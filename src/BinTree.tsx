import Tree from "react-d3-tree";
import { useCenteredTree } from "./helpers";
import "./styles.css";
import gear from "./assets/action_cpu.png";
import question from "./assets/conditional_q4.png";
import cube from "./assets/value_box.png";
import book from "./assets/vuesax/bold/book.png";
import rattle_icon from "./assets/rattle_icon.png";

const containerStyles = {
  width: "100vw",
  height: "100vh",
  backgroundColor: "white",
};

const TOKEN_MAP = {
  file: ["#FFFFFF", rattle_icon],
  function: ["#FE4949", gear],
  variable: ["#49A7FE", cube],
  conditional: ["#06975A", question],
  library: ["#B140B4", book],
};

function Token({ nodeDatum, toggleNode }) {
  console.log("The node datum was");
  console.log(nodeDatum);
  return (
    <g>
      <div
        style={{
          background: TOKEN_MAP[nodeDatum.name][0],
          height: "40px",
          width: "fit-content",
          borderRadius: "10px",
          display: "flex",
          flexDirection: "row",
          justifyContent: "center", //Centered vertically
          alignItems: "center", //Centered horizontally
          paddingLeft: "15px",
          paddingRight: "15px",
        }}
      >
        <img
          src={TOKEN_MAP[nodeDatum.name][1]}
          height="32px"
          style={{
            marginLeft: "-8px",
            marginRight: "4px",
          }}
        />

        <div
          style={{ height: "40px", width: "1px", backgroundColor: "#FFFFFF" }}
        ></div>

        <p
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            marginLeft: "7px",
          }}
        >
          {nodeDatum.attributes?.department}
        </p>
      </div>
    </g>
  );
}

export function BinTree() {
  // Here we're using `renderCustomNodeElement` to represent each node
  // as an SVG `rect` instead of the default `circle`.
  const renderRectSvgNode = ({ nodeDatum, toggleNode }) => (
    <g>
      <circle r="10" onClick={toggleNode} />
      <text fill="white" strokeWidth="1" x="20">
        {nodeDatum.name}
      </text>
      {nodeDatum.attributes?.department && (
        <text fill="white" x="20" dy="20" strokeWidth="1">
          Department: {nodeDatum.attributes?.department}
        </text>
      )}
    </g>
  );

  const data = {
    name: "function",
    children: [
      {
        name: "variable",
        attributes: {
          department: "Production",
        },
        children: [
          {
            name: "function",
            attributes: {
              department: "Fabrication",
            },
            children: [
              {
                name: "function",
              },
            ],
          },
          {
            name: "library",
            attributes: {
              department: "Assembly",
            },
            children: [
              {
                name: "function",
              },
            ],
          },
        ],
      },
      {
        name: "variable",
        attributes: {
          department: "Marketing",
        },
        children: [
          {
            name: "function",
            attributes: {
              department: "A",
            },
            children: [
              {
                name: "function",
              },
            ],
          },
          {
            name: "function",
            attributes: {
              department: "B",
            },
            children: [
              {
                name: "function",
              },
            ],
          },
        ],
      },
    ],
  };

  const [translate, containerRef] = useCenteredTree();
  return (
    <div style={containerStyles} ref={containerRef}>
      <Tree
        data={data}
        translate={translate}
        allowForeignElements
        renderCustomNodeElement={Token}
        orientation="vertical"
        pathClassFunc={() => "custom_link"}
        styles={{
          // Define styles for 'link__to-leaf'
          links: {
            stroke: "blue", // Set the stroke color to white for links to leaf nodes
          },
        }}
      />
    </div>
  );
}
