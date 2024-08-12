/**
 * @fileoverview Renders the acyclic digraph for a given source file. Handles the rendering of
 * individual pieces, nodes, blocks and subtrees.
 */

import { ArcherContainer, ArcherElement } from "react-archer";
import rattle_icon from "./assets/rattle_icon.png";
import { ReactNode } from "react";
import { RTLNode } from "./types";
import { ROW_STYLE, FLEX_COL, FLEX_ROW } from "./styles";
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

export function Digraph(source: RTLNode[]) {
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
          {source.map((subtree, i) => RenderSubtree(i.toString(), subtree))}
        </div>
      </ArcherContainer>
    </div>
  );
}

function RenderSubtree(addr: string, subtree_root: RTLNode): ReactNode {
  return (
    <div id={`${addr}`} style={{ border: "1px dashed white" }}>
      <div id={`${addr}.0`} style={FLEX_COL}>
        {RenderBlock(`${addr}.0`, subtree_root, undefined)}
      </div>
      {hasBlocks(subtree_root) ? (
        <div id={`${addr}.1`} style={FLEX_ROW}>
          {getBlocks(subtree_root).map((sub, i) =>
            RenderSubtree(`${addr}.1.${i}`, sub),
          )}
        </div>
      ) : null}
    </div>
  );
}

function RenderBlock(
  address: string,
  subtree_root: RTLNode,
  subtree_parent?: RTLNode,
): ReactNode {
  // TODO: Implement
  return (
    <div id={`${address}.0`}>
      {RenderNode(subtree_root, `${address}.0.0`, subtree_parent, false)}
    </div>
  );
}

function RenderNode(
  node: RTLNode,
  address: string,
  parent?: RTLNode,
  check_blocks: boolean = true,
  recursive: boolean = true,
) {
  if (LOCAL_BLOCK_NODES.includes(node.kind) && check_blocks) {
    return (
      <div id={address} style={FLEX_COL}>
        <div id={`${address}.0`}>
          {RenderNode(node, `${address}.0.0`, parent, false, false)}
        </div>
        <div style={{ height: "50px" }} />
        <div id={`${address}.1`} style={FLEX_ROW}>
          {node.children.map((n, i) => (
            <div id={`${address}.1.${i}`}>
              {RenderNode(n, `${address}.1.${i}.0`, node)}
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
          style={{
            display: "flex",
            flexDirection: "row",
            width: "fit-content",
            justifyContent: "center",
            alignItems: "center",
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

      {/* Add support for local blocks and subtrees
      {LOCAL_BLOCK_NODES.includes(node.kind) ? (
      ) : (
        node.children.map((n, i) =>
          RenderNode(n, node, addrStep(address, i + 1)),
        )
      )}
	  */}
      {recursive
        ? node.children.map((n, i) =>
            RenderNode(n, addrStep(address, i + 1), node),
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
