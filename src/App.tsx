import "./App.css";

import { SetStateAction, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Canvas,
  Debugger,
  IDisplay,
  RDisplay,
  _ExpectingPiece,
  IDGraph,
} from "./types.ts";
import { Dashboard } from "./Dashboard.tsx";
import { DAG } from "./Digraph.tsx";
import { speak } from "./speechUtils.ts";
import {
  CheckmarkIcon,
  ErrorIcon,
  WindowButton,
} from "./components/Components.tsx";
import { SlidingBorder } from "./components/SlidingBorder.tsx";

export const FAIL_SOUND = new Audio("./src/assets/Blow.aiff");

function App() {
  let [display, setDisplay] = useState<IDisplay>("EDITOR");
  const [RDisplay, setRDisplay] = useState<RDisplay>("PANEL");
  const [debugger_, setDebugger_] = useState<Debugger | null>(null);
  let [payload, setPayload] = useState<Canvas>({
    filename: "unnamed",
    graph: [],
    blockLoc: "",
    nodeLoc: "",
    mode: "VIEW",
    pieceIx: null,
    output: null,
    err: null,
  });

  let [idg, setIdg] = useState<IDGraph>({
    graph: [],
  });

  const [editingFilename, setEditingFilename] = useState(false);
  const [activeNote, setActiveNote] = useState("");

  // Play welcome audio as soon as the user opens the app
  useEffect(() => {
    const audioContext = new window.AudioContext();

    const playAudio = async () => {
      try {
        const response = await fetch("./src/assets/Welcome_Bong.mp3");
        const audioData = await response.arrayBuffer();
        const audioBuffer = await audioContext.decodeAudioData(audioData);

        const source = audioContext.createBufferSource();
        source.buffer = audioBuffer;
        source.connect(audioContext.destination);
        source.start(0);

        const response2 = await fetch("./src/assets/Welcome_Voice.mp3");
        const audioData2 = await response2.arrayBuffer();
        const audioBuffer2 = await audioContext.decodeAudioData(audioData2);

        const source2 = audioContext.createBufferSource();
        source2.buffer = audioBuffer2;
        source2.connect(audioContext.destination);
        source2.start(0);
      } catch (error) {
        console.error("Error playing audio:", error);
      }
    };

    // Attempt to resume the AudioContext immediately
    if (audioContext.state === "suspended") {
      audioContext.resume();
    }

    // playAudio();

    return () => {
      if (audioContext.state !== "closed") {
        audioContext.close();
      }
    };
  }, []);

  useEffect(() => {
    let initial_source = `define start of args\npretend\ndone define`;
    invoke("parse_file", { source: initial_source }).then((o: any) => {
      setPayload({
        filename: "unnamed",
        graph: o,
        blockLoc: "0.0",
        nodeLoc: "0.0.0",
        mode: "VIEW",
        pieceIx: null,
        output: null,
        err: null,
      });
    });
  }, [display]);

  const handleEvent = (key: string) => {
    switch (key) {
      case "n":
        setDisplay("EDITOR");
        break;
      default:
        console.log(`Can't yet handle an ${key} keypress`);
    }
  };

  const onKeyUp = (e: KeyboardEvent) => {
    if (display === "HOME") {
      handleEvent(e.key);
    } else if (display === "EDITOR") {
      // Handle a few cases manually where the server is not required:
      //
      // 1. "n" in view mode to create a new note
      if (payload.mode === "VIEW" && e.key === "n") {
        // TODO: Implement
        setActiveNote(payload.blockLoc);
        const elem = document.getElementById(`note_${payload.blockLoc}`);
        if (elem) {
          elem.focus();
          invoke("fetch_note", { payload: payload }).then((result: unknown) => {
            const text = result as string;
            const _elem = elem as HTMLInputElement;
            _elem.value = text;
          });
        }
        payload.mode = "TYPE";
      }

      // 2. "s" in view mode on an unnamed file to give it a name
      if (payload.mode === "VIEW" && e.key === "s") {
        setEditingFilename(true);
        payload.blockLoc = "";
        return;
      }

      // 3. "Enter" after filename has been edited
      if (
        payload.mode === "TYPE" &&
        e.key === "Enter" &&
        (payload.blockLoc === "" || editingFilename)
      ) {
        // Save the entered filename and continue
        const elem = document.getElementById("edit_filename");
        if (elem) payload.filename = (elem as HTMLInputElement).value;
        setEditingFilename(false);
        return;
      }

      // 4. "Enter" after note has been edited
      if (payload.mode === "TYPE" && e.key === "Enter" && activeNote) {
        // Save the entered filename and continue
        const elem = document.getElementById(`note_${payload.blockLoc}`);
        invoke("save_note", {
          note: (elem as HTMLInputElement).value.trimEnd(),
          payload: payload,
        }).then((result: unknown) => {
          const new_payload = result as Canvas;
          if (new_payload !== payload) setPayload(new_payload);
        });
        setActiveNote("");
        return;
      }

      // 5. "e" on a node with an error
      if (payload.mode === "VIEW" && e.key == "e") {
        invoke("fetch_err", { payload: payload }).then((result: unknown) => {
          const err = result as string;
          if (err) speak(err);
          else FAIL_SOUND.play();
        });
        return;
      }

      // 6. "w" in VIEW mode to enter Walkthrough Mode
      if (payload.mode === "VIEW" && e.key == "w") {
        let dbg = debugger_;
        if (debugger_ === null) {
          dbg = {
            state: payload,
            call_stack: ["0.0.0"],
          };
          setDebugger_(dbg);
        }

        invoke("runWalkthrough", { debugger: dbg, idg: idg }).then(
          (result: unknown) => {
            //                        [DBG,      expl,   IDGraph, exit code]
            const new_dbg = result as [Debugger, string, IDGraph, number];
            setPayload(new_dbg[0].state);
            setDebugger_(new_dbg[0]);
            speak(new_dbg[1]);
            setIdg(new_dbg[2]);

            if (new_dbg[3] === 1) {
              setDebugger_(null);
              speak("Program terminated successfully");
            } else if (new_dbg[3] === -1) {
              setDebugger_(null);
              speak("Walkthrough terminated, program failure");
            }
          },
        );
        return;
      }

      const elem = document.getElementById(
        `${payload.blockLoc},${payload.pieceIx}`,
      );

      let value = null;
      if (elem && elem instanceof HTMLInputElement)
        value = (elem as HTMLInputElement).value;

      invoke("handle_event", {
        event: JSON.stringify({ key: e.key }),
        payload: payload,
        // (optimization) only send in a value when a value is committed.
        value: e.key === "Enter" ? value : null,
      }).then((result: unknown) => {
        const [succeeded, err, new_payload] = result as [
          boolean,
          string,
          Canvas,
        ];

        if (!succeeded) {
          FAIL_SOUND.play();
          if (err !== "") speak(err);
        }

        const elem = document.getElementById(`menu_${payload.blockLoc}`);
        if (elem) elem.style.display = "none";

        // Prevent useEffect loops by only setting `payload` on a change
        if (payload !== new_payload) setPayload(new_payload);
        console.log(new_payload);
      });
    }
  };

  useEffect(() => {
    if (payload.graph.length > 0) {
      const elem = document.getElementById(
        `${payload.blockLoc},${payload.pieceIx}`,
      );
      if (elem && payload.mode === "TYPE") elem.focus();

      invoke("sync_idents", { payload: payload }).then((res: unknown) => {
        let idg_ = res as IDGraph;
        setIdg(idg_);
      });

      window.addEventListener("keyup", onKeyUp);
      return () => {
        window.removeEventListener("keyup", onKeyUp);
      };
    }
  }, [payload]);

  useEffect(() => {
    if (editingFilename) {
      // Switch on editingFilename mode
      payload.mode = "TYPE";
      payload.blockLoc = "";
      const elem = document.getElementById("edit_filename");
      if (elem) elem.focus();
    } else {
      payload.mode = "VIEW";
      invoke("handle_event", {
        event: JSON.stringify({ key: "s" }),
        payload: payload,
        value: null,
      });
    }
  }, [editingFilename]);

  return (
    <div className="container overflow-visible bg-gray-900">
      <div className="h-screen w-screen flex flex-row">
        {/*<div id="sidebar" className="h-screen w-[20vw] bg-black/80">
          <h1
            className="font-extrabold text-start ml-6 mt-6 mb-8 font-[BalooChettan]"
            style={{
              fontSize: "40px",
              textAlign: "left",
            }}
          >
            Steps
          </h1>

          <SearchBar />
        </div>*/}

        <div className="overflow-scroll bg-gray-900 h-screen w-screen resize-x flex flex-col">
          <div className="overflow-hidden w-[100vw] h-screen">
            <div className="z-[2] relative">
              {DAG(payload, display !== "EDITOR", editingFilename, activeNote)}
            </div>

            {/* Sliding border overlay */}
            <SlidingBorder
              selectedAddr={payload.blockLoc}
              pieceIx={payload.pieceIx}
            />

            <div className="absolute bottom-10 right-10 z-[10]">
              {RunPanel(
                RDisplay,
                setRDisplay,
                payload.output ?? "Output will appear here...",
                payload.err,
              )}
            </div>

            <div className={display !== "HOME" ? "hidden" : ""}>
              <Dashboard />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function RunPanel(
  runDisplay: RDisplay,
  setRDisplay: React.Dispatch<SetStateAction<RDisplay>>,
  output: string,
  err: string | null,
) {
  return runDisplay === "FLOAT" ? (
    <div
      className="flex rounded-full"
      style={{
        width: "fit-content",
        background: "linear-gradient(to right, #49A7FE, #2C6498)",
        border: "2px solid #49A7FE",
        padding: "5px",
        cursor: "pointer",
      }}
      onClick={() => setRDisplay("PANEL")}
    >
      <svg
        width="70"
        height="70"
        viewBox="0 0 69 76"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M64.0974 18.7207L67.1945 20.7651C67.8184 21.1769 67.9922 22.017 67.5826 22.6416L62.3912 30.5574C61.9817 31.1819 61.1439 31.3543 60.52 30.9425L57.0858 28.6757C56.4619 28.2639 56.2881 27.4238 56.6977 26.7992C57.6225 25.3891 58.1319 24.5552 56.8415 23.1526C55.7402 21.9554 52.1079 19.573 50.7288 21.6759L49.0871 24.1793C48.6847 24.7927 47.8693 24.97 47.2492 24.5858C47.238 24.5789 47.2268 24.5717 47.2158 24.5644C45.663 23.5395 43.7598 22.619 42.3911 21.3586C41.8284 20.8404 41.7903 19.927 42.3098 19.3291C43.3494 18.1326 44.5882 16.4152 43.7213 15.843C42.3423 14.9327 41.3424 14.236 39.6927 13.4411C36.6965 11.9972 32.2703 11.384 29.2082 11.1244C28.7668 11.087 28.7105 10.3293 29.1444 10.241C29.1512 10.2396 29.1529 10.2394 29.1597 10.2376C29.3746 10.1813 36.4863 8.32738 41.1341 8.56209C44.1792 8.71586 45.9642 8.84769 48.8282 9.90053C52.4972 11.2493 55.6753 13.1614 57.1034 15.5477C58.5316 17.9339 56.8947 19.6301 58.774 20.2058C59.901 20.5511 61.1275 20.285 61.8386 19.3138C62.3626 18.5981 63.3563 18.2315 64.0974 18.7207Z"
          fill="white"
        />
        <rect
          width="5.57999"
          height="20.5532"
          transform="matrix(0.839614 0.543184 -0.540707 0.841211 43.002 20.6377)"
          fill="white"
        />
        <rect
          width="9.17833"
          height="32.4045"
          rx="2"
          transform="matrix(0.835481 0.549519 -0.547037 0.837108 30.834 36.1426)"
          fill="white"
        />
      </svg>
    </div>
  ) : (
    <div className="h-75 w-110 relative">
      <div
        className="h-70 w-100 absolute"
        style={{
          background: "linear-gradient(#FFFFFF1A, #191A1BFF)",
          borderRadius: "42px",
          padding: "2px",
          zIndex: "2",
          backdropFilter: "blur(8px)",
        }}
      >
        <div
          className="h-full w-full"
          style={{
            background: "radial-gradient(circle, #292B2C80, #18191B66)",
            borderRadius: "40px",
          }}
        >
          <div className="flex flex-col w-full absolute top-6 left-6">
            <div className="flex flex-row" style={{ gap: "8px" }}>
              {WindowButton("#FF605C", () => setRDisplay("FLOAT"))}
              {WindowButton("#FFBD44", () => setRDisplay("FLOAT"))}
              {WindowButton("#00CA4E", () => {})}
            </div>

            <div id="runStatus" className="flex flex-row mt-6 items-center">
              <h1 className="mr-3 font-bold text-5xl">
                {err ? "Failed" : "Success"}
              </h1>
              <div className="mt-2">
                {err ? <ErrorIcon /> : <CheckmarkIcon />}
              </div>
            </div>

            <p
              className="mt-3 text-lg"
              style={{ fontFamily: "JetBrains Mono" }}
            >
              {output}
              <br />
              {err}
            </p>

            <div
              className="rounded-full py-2 text-2xl mt-8"
              style={{
                width: "85%",
                background: "linear-gradient(to right, #49A7FE, #2C6498)",
                textAlign: "center",
                cursor: "pointer",
              }}
              onClick={() =>
                window.dispatchEvent(new KeyboardEvent("keyup", { key: "r" }))
              }
            >
              Run Code
            </div>
          </div>
        </div>
      </div>
      <div
        className="h-20 w-20 top-6 -left-8 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
      <div
        className="h-50 w-50 -top-9 right-0 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
      <div
        className="h-10 w-10 bottom-13 -left-2 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
      <div
        className="h-18 w-18 bottom-3 right-6 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
    </div>
  );
}

export default App;
