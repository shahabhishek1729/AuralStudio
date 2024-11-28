/**
 * @fileoverview Renders the acyclic digraph for a given source file. Handles the rendering of
 * individual pieces, nodes, blocks and subtrees.
 */

import { useEffect, useRef } from "react";
import { ArcherContainer, ArcherElement } from "react-archer";
import { ReactNode } from "react";
import { CursorState, RTLNode } from "./types";
import { ROW_STYLE, FLEX_COL, FLEX_ROW, BORDER_ANIMATION } from "./styles";
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
  PENDING: "pending",
  FORLOOP: "for",
  WHLLOOP: "while",
};

// The kinds of nodes that would require an arrow to be drawn towards them.
// Currently includes:
//  - 'yes' branch of if-statements
//  - 'no' branch of if-statements
const ARROW_NODES = [token_type.CONDTLY, token_type.CONDTLN];
const GLOBAL_BLOCK_NODES = ["FNDEF"];
const LOCAL_BLOCK_NODES = ["CONDTL"];
const INDENT_NODES = GLOBAL_BLOCK_NODES.concat(["FORLOOP", "WHLLOOP"]);

const isBlock = (c: RTLNode) => GLOBAL_BLOCK_NODES.includes(c.kind);
const hasBlocks = (c: RTLNode) => !!c.children.find(isBlock);
const getBlocks = (c: RTLNode) => c.children.filter(isBlock);
const excludeBlocks = (c: RTLNode) => c.children.filter((c_) => !isBlock(c_));

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

// TODO: When editing, we need to render text boxes for strings + identifiers.
export function DAG(source: RTLNode[], state: CursorState) {
  const selectedAddr = state.blockLoc;
  const borderRef = useRef<HTMLDivElement | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);

  // Builds a sliding border around selected nodes within the graph
  // TODO: Scale when resizing windows (moving border when selecting groups)
  useEffect(() => {
    if (selectedAddr !== null && borderRef.current && containerRef.current) {
      const activeElement = document.getElementById(selectedAddr);
      const containerElement = containerRef.current;

      if (activeElement) {
        const activeRect = activeElement.getBoundingClientRect();
        const containerRect = containerElement.getBoundingClientRect();

        const top = activeRect.top - containerRect.top;
        const left = activeRect.left - containerRect.left;

        const width = activeRect.width - 5;
        const height = activeRect.height - 5;

        borderRef.current.style.transform = `translate(${left}px, ${top}px)`;
        borderRef.current.style.width = `${width}px`;
        borderRef.current.style.height = `${height}px`;
      }
    }
  }, [source, selectedAddr]);

  return (
    <div>
      <ArcherContainer
        strokeColor="white"
        lineStyle="curve"
        endShape={{ arrow: { arrowLength: 6, arrowThickness: 5 } }}
        endMarker={true}
        offset={5}
      >
        <div
          style={{
            justifyContent: "center",
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
            {FileNode("linalg.rattle") /* TODO: Remove hardcoded file name */}
          </ArcherElement>
        </div>

        <div ref={containerRef} style={{ position: "relative" }}>
          <div
            id={`selected_${selectedAddr}`}
            ref={borderRef}
            style={BORDER_ANIMATION}
          />
          <div style={ROW_STYLE}>
            {source.map((subtree, i) =>
              RenderSubtree(i.toString(), subtree, selectedAddr, state.pieceIx),
            )}
          </div>
        </div>
      </ArcherContainer>
    </div>
  );
}

function RenderSubtree(
  addr: string,
  subtreeRoot: RTLNode,
  selectedAddr: string,
  pieceIx: number[] | null,
): ReactNode {
  return (
    <div key={addr}>
      <div style={FLEX_COL}>
        {RenderBlock(
          addr,
          subtreeRoot,
          selectedAddr,
          pieceIx,
          addr.includes(".") ? subtreeRoot : undefined,
        )}
      </div>
      {hasBlocks(subtreeRoot) ? (
        <div
          id={`${addr}.1`}
          style={{
            gap: "30px",
            ...FLEX_ROW,
          }}
        >
          {getBlocks(subtreeRoot).map((sub, i) =>
            RenderSubtree(`${addr}.1.${i}`, sub, selectedAddr, pieceIx),
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
  pieceIx: number[] | null,
  subtreeParent?: RTLNode,
): ReactNode {
  const blockAddr =
    hasBlocks(subtreeRoot) || !subtreeParent ? `${address}.0` : address;
  return (
    <div key={address}>
      <div style={{ height: "30px" }} />
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
            paddingBottom: "5px",
          }}
        >
          {RenderNode(
            subtreeRoot,
            blockAddr,
            selectedAddr,
            pieceIx,
            subtreeParent,
            0,
            false,
          )}
        </div>
      </ArcherElement>
      <div style={{ height: "25px" }} />
    </div>
  );
}

function RenderNode(
  node: RTLNode, // The current node to be rendered
  address: string, // The address at which to render it (for div IDs)
  selectedAddr: string, // The address with the cursor (is it this one?)
  pieceIx: number[] | null, // The piece being edited (not necessarily in `node`)
  parent?: RTLNode, // This node's parent
  parentIndents: number = 0, // How many times the parent node was indented
  check_blocks: boolean = true, // Used on recursive calls
  recursive: boolean = true, // Used on recursive calls
) {
  if (LOCAL_BLOCK_NODES.includes(node.kind) && check_blocks) {
    return (
      <div key={address} id={address} style={FLEX_COL}>
        <div id={`${address}.0`}>
          {RenderNode(
            node,
            `${address}.0`,
            selectedAddr,
            pieceIx,
            parent,
            parentIndents,
            false,
            false,
          )}
        </div>
        <div style={{ height: "50px" }} />
        <div
          id={`${address}.1`}
          style={{
            gap: "20px",
            ...FLEX_ROW,
          }}
        >
          {node.children.map((n, i) => (
            <div key={i} id={`${address}.1.${i}`}>
              {RenderNode(n, `${address}.1.${i}`, selectedAddr, pieceIx, node)}
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (GLOBAL_BLOCK_NODES.includes(node.kind) && check_blocks) {
    return;
  }

  // If this node is part of a block (and is not one itself), indent.
  const indent = +(
    !!parent &&
    !!parent.kind &&
    INDENT_NODES.includes(parent.kind) &&
    !isBlock(node)
  );

  return (
    <div
      key={address}
      id={address}
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "10px",
        padding: node.kind === "FNDEF" ? "10px" : "",
        border: node.kind === "FNDEF" ? "2px solid #484848" : "",
        borderRadius: "10px",
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
          id={node.children.length > 0 ? `${address}.0` : ""}
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
              node.address === selectedAddr && (pieceIx ?? [1])[0] === 0
                ? "white"
                : node.pieces.length > 0
                  ? getColor(node.pieces[0])
                  : "transparent"
            }
            first={
              indent &&
              !!parent &&
              excludeBlocks(parent)[0].address === node.address
            }
            indent={parentIndents + indent}
          />
          {node.pieces.map((_, ix) =>
            RenderPiece(
              node.pieces,
              [ix],
              node.address === selectedAddr ? pieceIx ?? [-1] : [-1],
              node.address,
            ),
          )}
        </div>
      </ArcherElement>

      {recursive
        ? node.children
            .filter((c) => !GLOBAL_BLOCK_NODES.includes(c.kind))
            .map((n, i) =>
              RenderNode(
                n,
                addrStep(`${address}.0`, i + 1),
                selectedAddr,
                pieceIx,
                node,
                parentIndents + indent,
              ),
            )
        : null}
    </div>
  );
}

function FileNode(fname: string) {
  return (
    <div
      style={{
        background: `linear-gradient(45deg, #5C89FD, #00D1FF)`,
        height: "40px",
        width: "fit-content",
        borderRadius: "10px",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "10px",
        paddingRight: "10px",
        ...FLEX_ROW,
      }}
    >
      <p
        style={{
          fontFamily: "JetBrains Mono",
          textAlign: "start",
          color: "white",
        }}
      >
        {fname}
      </p>
    </div>
  );
}
