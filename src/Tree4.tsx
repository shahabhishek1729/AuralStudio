import { ArcherContainer, ArcherElement } from "react-archer";
import { Token, FileToken } from "./Token";

import flatten, { node, flatnode } from "./flattener";

const rootStyle = { display: "flex", justifyContent: "center" };
const rowStyle = {
  marginTop: "100px",
  marginBottom: "100px",
  display: "flex",
  gap: "10px",
  justifyContent: "center",
};

const childNodes: node[] = [
  {
    id: 1,
    type: "library",
    name: "numpy",
    children: [
      {
        id: 2,
        type: "library",
        name: "pandas",
        children: [
          {
            id: 3,
            type: "library",
            name: "matplotlib",
            children: [],
          },
        ],
      },
    ],
  },
  {
    id: 4,
    type: "function",
    name: "main",
    children: [
      {
        id: 5,
        type: "variable",
        name: "global_count",
        children: [],
      },
      {
        id: 6,
        type: "conditional",
        name: "global_count == 1",
        children: [
          {
            id: 7,
            type: "variable",
            name: "helloworld",
            children: [],
          },
          {
            id: 9,
            type: "variable",
            name: "byeworld",
            children: [],
          },
        ],
      },
    ],
  },
  {
    id: 10,
    type: "function",
    name: "print_all",
    children: [
      {
        id: 11,
        type: "variable",
        name: "global_count -> 1",
        children: [],
      },
      {
        id: 12,
        type: "conditional",
        name: "global_count == 1",
        children: [
          {
            id: 13,
            type: "variable",
            name: "helloworld",
            children: [],
          },
          {
            id: 14,
            type: "output",
            name: "string how are you done",
            children: [],
          },
        ],
      },
    ],
  },
];

function renderRow(nodes: node[], flattened: flatnode[]) {
  return (
    <div style={rowStyle}>
      {nodes.map((d) => {
        return (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              backgroundColor: "#282828",
              borderRadius: "20px",
              padding: "10px",
              minWidth: "200px",
            }}
          >
            <ArcherElement
              id={`node${d.id}`}
              relations={flattened
                .filter((child) => child.parent === d.id)
                .map((child) => {
                  return {
                    targetId: `node${child.id}`,
                    targetAnchor: "top",
                    sourceAnchor: "bottom",
                  };
                })}
            >
              {Token(d.type, d.name)}
            </ArcherElement>

            {d.children.length > 0 ? renderRow(d.children, flattened) : null}
          </div>
        );
      })}
    </div>
  );
}

function TestTree() {
  let flattened = flatten(childNodes);

  return (
    <div>
      <ArcherContainer strokeColor="white" lineStyle="curve" endMarker={false}>
        <div style={rootStyle}>
          <ArcherElement
            id="root"
            relations={flattened
              .filter((d: flatnode) => d.level === 1)
              .map((d: flatnode) => {
                return {
                  targetId: `node${d.id}`,
                  targetAnchor: "top",
                  sourceAnchor: "bottom",
                };
              })}
          >
            {FileToken("helloworld.rattle")}
          </ArcherElement>
        </div>

        {renderRow(childNodes, flattened)}
      </ArcherContainer>
    </div>
  );
}

export default TestTree;
