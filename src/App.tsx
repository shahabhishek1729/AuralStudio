import "./App.css";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { CursorState, IDisplay, _ExpectingPiece } from "./types.ts";
import { Dashboard } from "./Dashboard.tsx";
import { DAG } from "./Digraph.tsx";

function App() {
  let [display, setDisplay] = useState<IDisplay>("HOME");
  let [payload, setPayload] = useState<CursorState>({
    graph: [],
    blockLoc: "",
    nodeLoc: "",
    mode: "VIEW",
    pieceIx: null,
    output: null,
  });

  useEffect(() => {
    const audioContext = new (window.AudioContext ||
      window.webkitAudioContext)();
    const playAudio = async () => {
      try {
        const response = await fetch("./src/assets/Welcome_Bong.mp3");
        const audioData = await response.arrayBuffer();
        const audioBuffer = await audioContext.decodeAudioData(audioData);

        const source = audioContext.createBufferSource();
        source.buffer = audioBuffer;
        source.connect(audioContext.destination);
        source.start(0);
      } catch (error) {
        console.error("Error playing audio:", error);
      }

      try {
        const response = await fetch("./src/assets/Welcome_Voice.mp3");
        const audioData = await response.arrayBuffer();
        const audioBuffer = await audioContext.decodeAudioData(audioData);

        const source = audioContext.createBufferSource();
        source.buffer = audioBuffer;
        source.connect(audioContext.destination);
        source.start(0);
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
    invoke("parse_file", { source: initial_source }).then((o: any) => {
      setPayload({
        graph: o,
        blockLoc: "0.0",
        nodeLoc: "0.0.0",
        mode: "VIEW",
        pieceIx: null,
        output: null,
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
      const elem = document.getElementById(
        `${payload.blockLoc},${payload.pieceIx}`,
      );

      invoke("handle_event", {
        event: JSON.stringify({ key: e.key }),
        payload: payload,
        // (optimization) only send in a value when a value is committed.
        value: e.key === "Enter" ? elem?.value : null,
      }).then((state_editor: unknown) => {
        const elem = document.getElementById(`menu_${payload.blockLoc}`);
        if (elem) elem.style.display = "none";

        const new_payload = state_editor as CursorState;
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

      window.addEventListener("keyup", onKeyUp);
      return () => {
        window.removeEventListener("keyup", onKeyUp);
      };
    }
  }, [payload]);

  return (
    <div className="container" style={{ overflow: "hidden" }}>
      <div
        data-tauri-drag-region
        style={{
          position: "absolute",
          top: "0",
          height: "2rem",
          width: "100%",
        }}
      />

      <div style={{ display: "flex", flexDirection: "row" }}>
        <div
          style={{
            overflow: "scroll",
            backgroundColor: "#0F0F0F",
            height: "100vh",
            resize: "horizontal",
            flexDirection: "column",
          }}
        >
          <div style={{ overflow: "hidden", width: "100vw", height: "100vh" }}>
            {DAG(payload, display !== "EDITOR")}
            <div
              style={{
                display: display === "EDITOR" ? "flex" : "none",
                backgroundColor: "#282828",
                flexGrow: 1,
                height: "60%",
              }}
            >
              <pre
                style={{
                  paddingLeft: "20px",
                  fontFamily: "Andale Mono",
                  fontSize: "20px",
                }}
              >
                {payload.output ?? "Code output will appear here..."}
              </pre>
            </div>
            <div style={{ display: display !== "HOME" ? "none" : "" }}>
              <Dashboard />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
