import "./App.css";

import add_folder from "./assets/add_folder.png";
import add_file from "./assets/add_file.png";
import open_terminal from "./assets/open_terminal.png";
import refresh_files from "./assets/refresh_files.png";

import TabsComponent from "./TabsComponent";

import { FileTree } from "./FileTree.tsx";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { RenderDigraph } from "./DigraphRenderer.tsx";
import { listen } from "@tauri-apps/api/event";

function App() {
  invoke("grab_window", { window: appWindow });
  invoke("send_true");
  let [codeOutput, setCodeOutput] = useState("Code output will appear here...");
  let [treeSource, setTreeSource] = useState([]);
  // let source = "define f of x\noutput x\nlet my_age be 3\ndone define\ndefine h of x\noutput string hi done\ndone define\ndefine g of x\noutput x plus 1\nif x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone define";

  // let source = "define adjoint of a and b and c and d and det\nlet a be d over det\nlet b be -1 times b over det\nlet c be -1 times c over det\nlet d be a over det\nlet inv be list a b c d done\nreturn inv\ndone define\ndefine determinant of a and b and c and d\nlet answer be a times d minus b times c\nif answer equals 0\noutput string non-invertible! done\ndone if\notherwise\noutput string invertible! done\nreturn answer\ndone otherwise\ndone define\ndefine start of arguments\nlet a be 1\nlet b be 2\nlet c be 3\nlet d be 4\nlet det be determinant of a and b and c and d done\nlet inv be adjoint of a and b and c and d and det done\noutput inv\ndone define"
  let source =
    "define adjoint of a and b and c and d and det\nlet a be d over det\nlet b be -1 times b over det\nlet c be -1 times c over det\nlet d be a over det\ndefine build_list of a and b\nreturn list a b done\ndone define\nlet inv be list a b c d done\nreturn inv\ndone define\ndefine determinant of a and b and c and d\nlet answer be a times d minus b times c\nif answer equals 0\noutput string non-invertible! done\ndone if\notherwise\noutput string invertible! done\ndone otherwise\ndone define\ndefine start of arguments\nlet a be 1\nlet b be 2\nlet c be 3\nlet d be 4\nlet det be determinant of a and b and c and d done\nlet inv be adjoint of a and b and c and d and det done\noutput inv\ndone define";
  // let source = "define start of arguments\noutput string Hello, World! done\ndone define";

  let [selectedIdx, setSelectedIdx] = useState(1);

  //	RustySocketConnection2(() => {
  //		setSelectedIdx(selectedIdx - 1);
  //	});
  //});

  //type Payload = {
  //	event_type: number;
  //}
  //
  let done = false;

  async function startSerialEventListener() {
    await listen<Payload>("nav_event", (event) => {
      console.log(
        "Event triggered from rust!\nPayload: " + event.payload.event_type,
      );

      // let output = "Successful: invertible!\n[-2, 1, 1.5, -0.5]"
      let output =
        "Execution failed:\nOn line 2 of the adjoint function, you attempted to divide d by det. However, d was a number but det was nothing, and I do not know how\nto divide a number by nothing.\n\nConsider setting a default value for the variable det or ensure that it's value was properly initialized.";
      invoke("run_code", { code: source, path: "linalg.rattle" }).then(
        (result) => setCodeOutput(output),
      );

      let synth = window.speechSynthesis;
      let voices = [];

      if (voices.length === 0) {
        PopulateVoices();
      }
      if (speechSynthesis !== undefined) {
        speechSynthesis.onvoiceschanged = PopulateVoices;
      }
      let toSpeak = new SpeechSynthesisUtterance(output);
      toSpeak.voice = voices[voices.findIndex((v) => v.name === "Samantha")];
      if (!done) {
        setSelectedIdx(2);
        synth.speak(toSpeak);
        done = true;
      } else {
        done = false;
      }

      function PopulateVoices() {
        voices = synth.getVoices();
      }
    });
  }
  useEffect(() => {
    startSerialEventListener();
  }, []);

  //useEffect(() => {
  //	setSelectedID(selectedIdx);
  //}, [selectedIdx]);
  //

  useEffect(() => {
    invoke("parse_file", { source: source }).then((o: any) => {
      console.log("The parsed JSON was:");
      console.log(o);
      setTreeSource(o);
    });
  }, []);

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
        <div style={{ flexGrow: 3, flexDirection: "column", height: "100vh" }}>
          <div
            style={{ display: "flex", flexDirection: "row", marginTop: "50px" }}
          >
            <img
              src={add_folder}
              style={{
                height: "20px",
                marginLeft: "10px",
                marginRight: "15px",
              }}
            />
            <img
              src={add_file}
              style={{ height: "20px", marginRight: "15px" }}
            />
            <img
              src={open_terminal}
              style={{ height: "20px", marginRight: "15px" }}
            />
            <img
              src={refresh_files}
              style={{ height: "20px", marginRight: "15px" }}
            />
          </div>

          <FileTree />
        </div>

        <div
          style={{
            backgroundColor: "#0F0F0F",
            flexGrow: 5,
            height: "100vh",
            resize: "horizontal",
            flexDirection: "column",
          }}
        >
          <TabsComponent />

          <div style={{ height: "20px" }} />

          <RenderDigraph source={treeSource} selectedIdx={selectedIdx} />

          <div
            style={{
              display: "flex",
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
              {codeOutput}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
