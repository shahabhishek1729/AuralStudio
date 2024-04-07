import "./App.css";

import add_folder from "./assets/add_folder.png";
import add_file from "./assets/add_file.png";
import open_terminal from "./assets/open_terminal.png";
import refresh_files from "./assets/refresh_files.png";

import TabsComponent from "./TabsComponent";

import { FileTree } from "./FileTree.tsx";
import TestTree from "./Tree4.tsx";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  // let [codeText, setCodeText] = useState("");
  let [codeOutput] = useState("Code output will appear here...");

  invoke("parse_file").then((o: any) => {
	  console.log("The parsed JSON was:");
	  console.log(o)
	  }
  );

  // function handleSubmit() {
  // invoke("run_code", { code: codeText, path: "thisisatest.rattle" }).then(
  // (o: any) => setCodeOutput(o)
  // );
  // }
  //

  return (
    <div className="container" style={{overflow: "hidden"}}>
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
        <div style={{ flexGrow: 1, flexDirection: "column", height: "100vh" }}>
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

          {/* <p style={{marginLeft: "20px", lineHeight: "20px", fontFamily: "Helvetica", textAlign: 'start', whiteSpace: "pre-line"}}>{printFileTree(data)}</p> */}
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
          {/* <input
            type="text"
            value={codeText}
            onChange={(e) => setCodeText(e.target.value)}
          />
          <button onClick={handleSubmit}>Submit</button> */}
          <div style={{ height: "20px" }} />

          <div style={{ overflow: "auto", height: "60%" }}>{<TestTree />}</div>

          <div
            style={{
              display: "flex",
              backgroundColor: "#282828",
              flexGrow: 1,
              height: "30%",
            }}
          >
            <p
              style={{
                paddingLeft: "20px",
                fontFamily: "Andale Mono",
                fontSize: "20px",
              }}
            >
              {codeOutput}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
