import "./App.css";

import add_folder from "./assets/add_folder.png";
import add_file from "./assets/add_file.png";
import open_terminal from "./assets/open_terminal.png";
import refresh_files from "./assets/refresh_files.png";

import TabsComponent from "./TabsComponent";

import { FileTree } from "./FileTree.tsx";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { DAG } from "./Digraph.tsx";
import { CursorState, _ExpectingPiece } from "./types.ts";

function App() {
  let [codeOutput, _] = useState<string>("Code output will appear here...");
  let [payload, setPayload] = useState<CursorState>({
    graph: [],
    blockLoc: "",
    nodeLoc: "",
    mode: "VIEW",
    pieceIx: null,
  });

  let final_source = `define inverse of m
define determinant of m
let x be m at 0 times m at 3
let y be m at 1 times m at 2
return x minus y
done define
define adjoint of m 
let result be list m at 3 0 minus m at 1 0 minus m at 2 m at 0 done 
return result
done define
let d be determinant of m done
let a be adjoint of m done
let iterator be range of 4 done
for i in iterator
let m at i be 1 over d times a at i
return m
done for
done define\ndefine g of x\noutput x plus 1\nif x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone define`;

  useEffect(() => {
    invoke("parse_file", { source: final_source }).then((o: any) => {
      setPayload({
        graph: o,
        blockLoc: "0.0",
        nodeLoc: "0.0.0",
        mode: "VIEW",
        pieceIx: null,
      });
    });
  }, []);

  const onKeyUp = (e: KeyboardEvent) => {
    const elem = document.getElementById(
      `${payload.blockLoc},${payload.pieceIx}`,
    );

    invoke("handle_event", {
      event: JSON.stringify({ key: e.key }),
      payload: payload,
      // (optimization) only send in a value when a value is committed.
      value: e.key === "Enter" ? elem?.value : null,
    }).then((state_editor: unknown) => {
      const new_payload = state_editor as CursorState;
      // Prevent useEffect loops by only setting `payload` on a change
      if (payload !== new_payload) setPayload(new_payload);
      console.log("NEW PAYLOAD");
      console.log(new_payload);
    });
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

          {DAG(payload.graph, payload)}

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
