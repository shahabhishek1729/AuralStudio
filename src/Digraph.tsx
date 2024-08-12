/**
 * @fileoverview Renders the acyclic digraph for a given source file. Handles the rendering of
 * individual pieces, nodes, blocks and subtrees.
 */

import { ArcherContainer, ArcherElement } from "react-archer";
import rattle_icon from "./assets/rattle_icon.png";
import { ReactNode } from "react";
import { RTLNode } from "./types";
import { ROW_STYLE, FLEX_COL, FLEX_ROW, BORDER_STYLE } from "./styles";
import { Token, RenderPiece } from "./PieceRenderer";
import { getColor } from "./utils";

const token_type = {
  FNDEF: "function",
  OUTPUT: "output",
  VARDECL: "variable",
  CONDTL: "conditional",
  CONDTLY: "yes",
  CONDTLN: "no",
  RETURN: "return",
};

// The kinds of nodes that would require an arrow to be drawn towards them.
// Currently includes:
//  - 'yes' branch of if-statements
//  - 'no' branch of if-statements
const ARROW_NODES = [token_type.CONDTLY, token_type.CONDTLN];
const GLOBAL_BLOCK_NODES = ["FNDEF"];
const LOCAL_BLOCK_NODES = ["CONDTL"];

const isBlock = (c: RTLNode) => GLOBAL_BLOCK_NODES.includes(c.kind);
const hasBlocks = (c: RTLNode) => !!c.children.find(isBlock);
const getBlocks = (c: RTLNode) => c.children.filter(isBlock);

/**
 * Move an address one space forward (e.g., 1.0.2.1.0 => 1.0.2.1.1)
 * @param addr The address to be moved forward from (typically the parent addr)
 * @param n The number of steps to move forward (typically the child's position)
 * @returns {string} The new address, equivalent to moving forward `n` spaces
 *					 from `addr`.
 */
function addrStep(addr: string, n: number = 1): string {
  let splits = addr.split(".");
  let prefix = splits.slice(0, -1);
  let suffix = parseInt(splits[splits.length - 1]);
  suffix += n;
  prefix.push(suffix.toString());
  return prefix.join(".");
}

export function Digraph(source: RTLNode[], selectedAddr: string) {
  return (
    <div style={{ overflowX: "auto" }}>
      <ArcherContainer
        strokeColor="white"
        lineStyle="curve"
        endShape={{ arrow: { arrowLength: 6, arrowThickness: 5 } }}
        endMarker={true}
      >
        <div
          style={{
            justifyContent: "center",
            overflowX: "scroll",
            ...FLEX_ROW,
          }}
        >
          <ArcherElement
            id="root"
            relations={source.map((d) => {
              return {
                targetId: d.address,
                targetAnchor: "top",
                sourceAnchor: "bottom",
              };
            })}
          >
            {FileNode("linalg.rattle")}
          </ArcherElement>
        </div>

        <div style={ROW_STYLE}>
          {source.map((subtree, i) =>
            RenderSubtree(i.toString(), subtree, selectedAddr),
          )}
        </div>
      </ArcherContainer>
    </div>
  );
}

function RenderSubtree(
  addr: string,
  subtreeRoot: RTLNode,
  selectedAddr: string,
): ReactNode {
  return (
    <div id={`${addr}`} style={BORDER_STYLE(addr === selectedAddr)}>
      <div style={FLEX_COL}>
        {RenderBlock(addr, subtreeRoot, selectedAddr, undefined)}
      </div>
      {hasBlocks(subtreeRoot) ? (
        <div
          id={`${addr}.1`}
          style={{
            gap: "30px",
            ...FLEX_ROW,
            ...BORDER_STYLE(selectedAddr === `${addr}.1`),
          }}
        >
          {getBlocks(subtreeRoot).map((sub, i) =>
            RenderSubtree(`${addr}.1.${i}`, sub, selectedAddr),
          )}
        </div>
      ) : null}
    </div>
  );
}

function RenderBlock(
  address: string,
  subtreeRoot: RTLNode,
  selectedAddr: string,
  subtreeParent?: RTLNode,
): ReactNode {
  const blockAddr =
    hasBlocks(subtreeRoot) || !subtreeParent ? `${address}.0` : address;
  return (
    <>
      <div style={{ height: "25px" }} />
      <ArcherElement
        id={blockAddr}
        relations={getBlocks(subtreeRoot).map((c) => {
          return {
            targetId: c.address,
            sourceAnchor: "bottom",
            targetAnchor: "top",
          };
        })}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            paddingLeft: "10px",
            paddingRight: "10px",
          }}
        >
          {RenderNode(
            subtreeRoot,
            blockAddr,
            selectedAddr,
            subtreeParent,
            false,
          )}
        </div>
      </ArcherElement>
      <div style={{ height: "25px" }} />
    </>
  );
}

function RenderNode(
  node: RTLNode,
  address: string,
  selectedAddr: string,
  parent?: RTLNode,
  check_blocks: boolean = true,
  recursive: boolean = true,
) {
  if (LOCAL_BLOCK_NODES.includes(node.kind) && check_blocks) {
    return (
      <div
        id={address}
        style={{ ...FLEX_COL, ...BORDER_STYLE(selectedAddr === address) }}
      >
        <div
          id={`${address}.0`}
          style={BORDER_STYLE(selectedAddr === `${address}.0`)}
        >
          {RenderNode(node, `${address}.0`, selectedAddr, parent, false, false)}
        </div>
        <div style={{ height: "50px" }} />
        <div
          id={`${address}.1`}
          style={{
            gap: "20px",
            ...FLEX_ROW,
            ...BORDER_STYLE(selectedAddr === `${address}.1`),
          }}
        >
          {node.children.map((n, i) => (
            <div
              id={`${address}.1.${i}`}
              style={BORDER_STYLE(selectedAddr === `${address}.1.${i}`)}
            >
              {RenderNode(n, `${address}.1.${i}`, selectedAddr, node)}
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (GLOBAL_BLOCK_NODES.includes(node.kind) && check_blocks) {
    return;
  }

  const indent = parent && parent.kind && parent.kind === "FNDEF";
  return (
    <div
      id={address}
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "10px",
        alignItems: ARROW_NODES.includes(
          token_type[node.kind as keyof typeof token_type],
        )
          ? "start" // TODO: Center or keep alignment as-is?
          : "start",
        ...BORDER_STYLE(selectedAddr === address),
      }}
    >
      <ArcherElement
        id={node.address}
        relations={node.children
          .filter((c) =>
            ARROW_NODES.includes(token_type[c.kind as keyof typeof token_type]),
          )
          .map((c) => {
            return {
              targetId: c.address,
              targetAnchor: "top",
              sourceAnchor: "bottom",
            };
          })}
      >
        <div
          id={node.children.length > 0 ? `${address}.0` : ""}
          style={{
            display: "flex",
            flexDirection: "row",
            width: "fit-content",
            justifyContent: "center",
            alignItems: "center",
            ...BORDER_STYLE(selectedAddr === `${address}.0`),
          }}
        >
          <Token
            token_type={token_type[node.kind as keyof typeof token_type]}
            puzzle_color={
              node.pieces.length > 0 ? getColor(node.pieces[0]) : "transparent"
            }
            first={indent && parent.children[0].line === node.line}
            indent={indent}
          />
          {node.pieces.map((piece, index) => RenderPiece(piece, index === 0))}
        </div>
      </ArcherElement>

      {recursive
        ? node.children.map((n, i) =>
            RenderNode(n, addrStep(`${address}.0`, i + 1), selectedAddr, node),
          )
        : null}
    </div>
  );
}

function FileNode(fname: string) {
  return (
    <div
      style={{
        background: "white",
        height: "40px",
        width: "fit-content",
        borderRadius: "10px",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "60px",
        paddingRight: "10px",
        ...FLEX_ROW,
      }}
    >
      <img
        src={rattle_icon}
        height="32px"
        style={{
          marginLeft: "-48px",
          marginRight: "4px",
        }}
      />

      <div
        style={{ height: "40px", width: "1px", backgroundColor: "#000000" }}
      ></div>

      <p
        style={{
          fontFamily: "JetBrains Mono",
          textAlign: "start",
          marginLeft: "7px",
          color: "black",
        }}
      >
        {fname}
      </p>
    </div>
  );
}
