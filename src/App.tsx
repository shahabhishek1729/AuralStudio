import "./App.css";

import { SetStateAction, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Canvas,
  Debugger,
  IDGraph,
  RTLNode,
  _ExpectingPiece,
} from "./typing/digraph";
import { IDisplay, RDisplay } from "./typing/display";
import { Dashboard } from "./components/Dashboard.tsx";
import { DAG } from "./Digraph.tsx";
import { speak } from "./utils/speechUtils.ts";
import {
  CheckmarkIcon,
  ErrorIcon,
  WindowButton,
} from "./components/Components.tsx";
import { SlidingBorder } from "./components/SlidingBorder.tsx";
import RunPanel from "./components/RunPanel.tsx";

export const FAIL_SOUND = new Audio("./src/assets/Blow.aiff");

function App() {
  let [display, setDisplay] = useState<IDisplay>("EDITOR");
  const [RDisplay, setRDisplay] = useState<RDisplay>("PANEL");
  const [debugger_, setDebugger_] = useState<Debugger | null>(null);
  let [payload, setPayload] = useState<Canvas>({
    filename: "unnamed",
    graph: [] as RTLNode[],
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

    playAudio();

    return () => {
      if (audioContext.state !== "closed") {
        audioContext.close();
      }
    };
  }, []);

  useEffect(() => {
    let initial_source = `define start of args\npretend\ndone define`;
    invoke<RTLNode[]>("parse_file", { source: initial_source }).then((o) => {
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
          invoke<string>("fetch_note", { payload: payload }).then((text) => {
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
        invoke<Canvas>("save_note", {
          note: (elem as HTMLInputElement).value.trimEnd(),
          payload: payload,
        }).then((new_payload) => {
          if (new_payload !== payload) setPayload(new_payload);
        });
        setActiveNote("");
        return;
      }

      // 5. "e" on a node with an error
      if (payload.mode === "VIEW" && e.key == "e") {
        invoke<string>("fetch_err", { payload: payload }).then((err) => {
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

        invoke<[Debugger, string, IDGraph, number]>("runWalkthrough", {
          debugger: dbg,
          idg: idg,
        }).then((new_dbg) => {
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
        });
        return;
      }

      const elem = document.getElementById(
        `${payload.blockLoc},${payload.pieceIx}`,
      );

      let value = null;
      if (elem && elem instanceof HTMLInputElement)
        value = (elem as HTMLInputElement).value;

      invoke<[boolean, string, Canvas]>("handle_event", {
        event: JSON.stringify({ key: e.key }),
        payload: payload,
        // (optimization) only send in a value when a value is committed.
        value: e.key === "Enter" ? value : null,
      }).then((result) => {
        const [succeeded, err, new_payload] = result;

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

      invoke<IDGraph>("sync_idents", { payload: payload }).then((idg_) =>
        setIdg(idg_),
      );

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
              {payload.graph.length > 0
                ? DAG(
                    payload,
                    display !== "EDITOR",
                    editingFilename,
                    activeNote,
                  )
                : null}
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

export default App;
