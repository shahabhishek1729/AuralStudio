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

// TODO: When selected, set border to `3px solid #FAD70F`
function renderRow(nodes: node[], flattened: flatnode[], selectedIdx: number) {
  return (
    <div style={rowStyle}>
      {nodes.map(d => {
        return (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
			  border: selectedIdx === d.id ? "3px solid #FAD70F" : "1px dashed gray",
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

            {d.children.length > 0 ? renderRow(d.children, flattened, selectedIdx=selectedIdx) : null}
          </div>
        );
      })}
    </div>
  );
}

function TestTree({ source, selectedIdx }) {
  let flattened = flatten(source);

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

        {renderRow(source, flattened, selectedIdx=selectedIdx)}
      </ArcherContainer>
    </div>
  );
}

export default TestTree;
