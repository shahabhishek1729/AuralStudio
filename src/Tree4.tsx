import { ArcherContainer, ArcherElement } from "react-archer";
import { FileToken, RenderBlock } from "./Token";

import flatten, { node, flatnode } from "./flattener";

const rootStyle = { display: "flex", justifyContent: "center" };
const rowStyle = {
  marginTop: "100px",
  marginBottom: "100px",
  display: "flex",
  gap: "10px",
  justifyContent: "center",
};

const childNodes = [
  {
    id: 1,
    children: [],
	blocks: [
      {
        kind: "library",
        line: 1,
        name: "numpy",
      },
    ]
  },
  {
    id: 2,
    children: [
      {
        id: 3,
        children: [],
		blocks: [
		  {
			kind: "output",
			name: "hello",
			line: 1,
		  },
		]
      },
      {
        id: 4,
        children: [],
		blocks: [
		  {
			kind: "output",
			name: "bye",
			line: 1,
		  },
		]
      },
    ],
	blocks: [
      {
        kind: "function",
        name: "main(int argc, char **argv)",
        line: 1,
      },
      {
        kind: "output",
        line: 2,
        name: "hello + 42",
      },
      {
        kind: "variable",
        name: "hello",
        line: 3,
      },
      {
        kind: "arrow",
        value: "->",
        line: 3,
      },
      {
        kind: "constant",
        value: 6,
        line: 3,
      },
      {
        kind: "operator",
        value: "+",
        line: 3,
      },
      {
        kind: "constant",
        value: 43,
        line: 3,
      },
      {
        kind: "conditional",
        name: "hello == 49",
        line: 4,
      },
    ]
  },
];

// TODO: When selected, set border to `3px solid #FAD70F`
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
			  border: "1px dashed gray",
              borderRadius: "20px",
              padding: "10px",
              minWidth: "200px",
            }}
			key={d.id}
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
              {RenderBlock(d.blocks)}
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
  console.log("Flattened:")
  console.log(flattened);

  return (
    <div>
      <ArcherContainer strokeColor="white" lineStyle="curve" endShape={{arrow: {arrowLength: 6, arrowThickness: 5}}} endMarker={true}>
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
