import "./App.css";

import add_folder from "./assets/add_folder.png";
import add_file from "./assets/add_file.png";
import open_terminal from "./assets/open_terminal.png";
import refresh_files from "./assets/refresh_files.png";

import TabsComponent from "./TabsComponent";

import { FileTree } from "./FileTree.tsx";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Digraph } from "./Digraph.tsx";

function App() {
  let [codeOutput, _] = useState("Code output will appear here...");
  let [treeSource, setTreeSource] = useState([]);

  let source = `define f of x\noutput x\ndefine f1 of x\noutput x\ndone define\ndefine f2 of x\noutput x\ndefine f21 of x\noutput x\ndone define\ndefine f22 of x\noutput x\ndone define\ndone define\ndone define\ndefine g of x\noutput x plus 1\nif x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone define`;

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

          {Digraph(treeSource)}

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
