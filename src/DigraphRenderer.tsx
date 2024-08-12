import { ArcherContainer, ArcherElement } from "react-archer";
import { getColor, RenderPiece, Token } from "./PieceRenderer";
import { FileToken } from "./archive/Token";

const token_type = {
  FNDEF: "function",
  OUTPUT: "output",
  VARDECL: "variable",
  CONDTL: "conditional",
  CONDTLY: "yes",
  CONDTLN: "no",
  RETURN: "return",
};

const rootStyle = {
  display: "flex",
  justifyContent: "center",
  overflowX: "scroll",
};

const rowStyle = {
  marginTop: "100px",
  marginBottom: "100px",
  display: "flex",
  gap: "30px",
  justifyContent: "center",
};

// The kinds of nodes that would require an arrow to be drawn towards them.
// Currently includes:
//  - 'yes' branch of if-statements
//  - 'no' branch of if-statements
const ARROW_NODES = [token_type.CONDTLY, token_type.CONDTLN];

function RenderNode({ node, parent, selectedIdx, address }) {
  const indent = parent && parent.kind && parent.kind === "FNDEF";
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "10px",
        alignItems: ARROW_NODES.includes(token_type[node.kind])
          ? "start" // TODO: Center or keep alignment as-is?
          : "start",
      }}
    >
      <ArcherElement
        id={node.line}
        relations={node.children
          .filter((c) => ARROW_NODES.includes(token_type[c.kind]))
          .map((c) => {
            return {
              targetId: c.line,
              targetAnchor: "top",
              sourceAnchor: "bottom",
            };
          })}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            width: "fit-content",
            justifyContent: "center",
            alignItems: "center",
            border: selectedIdx === node.line ? "2px solid #f7dc28" : "",
            borderRadius: "10px",
          }}
        >
          <Token
            token_type={token_type[node.kind]}
            puzzle_color={
              node.pieces.length > 0 ? getColor(node.pieces[0]) : "transparent"
            }
            first={indent && parent.children[0].line === node.line}
            indent={indent}
          />
          {node.pieces.map((piece, index) => RenderPiece(piece, index === 0))}
        </div>
      </ArcherElement>

      {token_type[node.kind] !== "conditional" ? (
        node.children.map((n) => (
          <RenderNode node={n} parent={node} selectedIdx={selectedIdx} />
        ))
      ) : (
        <div style={{ display: "flex", flexDirection: "column" }}>
          <div style={{ height: "50px" }} />
          <div style={{ display: "flex", flexDirection: "row" }}>
            {node.children.map((n) => (
              <RenderNode node={n} parent={node} selectedIdx={selectedIdx} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

export function RenderDigraph({ source, selectedIdx }) {
  return (
    <div style={{ overflowX: "auto" }}>
      <ArcherContainer
        strokeColor="white"
        lineStyle="curve"
        endShape={{ arrow: { arrowLength: 6, arrowThickness: 5 } }}
        endMarker={true}
      >
        <div style={rootStyle}>
          <ArcherElement
            id="root"
            relations={source.map((d) => {
              return {
                targetId: d.line,
                targetAnchor: "top",
                sourceAnchor: "bottom",
              };
            })}
          >
            {FileToken("linalg.rattle")}
          </ArcherElement>
        </div>

        <div style={rowStyle}>
          {source.map((i, s) => (
            <RenderNode
              address={`${i}`}
              node={s}
              parent={null}
              selectedIdx={selectedIdx}
            />
          ))}
        </div>
      </ArcherContainer>
    </div>
  );
}
