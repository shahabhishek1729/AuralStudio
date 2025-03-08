/**
 * @fileoverview Renders the acyclic digraph for a given source file. Handles the rendering of
 * individual pieces, nodes, blocks and subtrees.
 */

import { useEffect, useRef, useState } from "react";
import {
  ArcherContainer,
  ArcherContainerRef,
  ArcherElement,
} from "react-archer";
import { ReactNode } from "react";
import { CursorState, RTLNode } from "./types";
import { ROW_STYLE, FLEX_COL, FLEX_ROW, BORDER_ANIMATION } from "./styles";
import { Token, RenderPiece } from "./PieceRenderer";
import { getColor } from "./utils";
import ReactCardFlip from "react-card-flip";
import { CheckmarkIcon, ErrorIcon, MessageIcon } from "./Components";

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
  BREAK: "break",
  CONTINUE: "continue",
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
export function DAG(
  payload: CursorState,
  hide: boolean,
  editFname: boolean,
  flipped: string,
) {
  const source = payload.graph;
  const selectedAddr = payload.blockLoc || "filenode";
  const [renderedAddr, setRenderedAddr] = useState("");
  const [fname, setFname] = useState(payload.filename);

  const borderRef = useRef<HTMLDivElement | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const archerRef = useRef<ArcherContainerRef | null>(null);

  // Builds a sliding border around selected nodes within the graph
  // TODO: Scale when resizing windows (moving border when selecting groups)
  useEffect(() => {
    const removeBorder = () => {
      setRenderedAddr(selectedAddr);
    };

    if (selectedAddr !== null && borderRef.current && containerRef.current) {
      setRenderedAddr("");
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
        borderRef.current.style.visibility = "visible";

        borderRef.current.addEventListener("transitionend", removeBorder);

        document.getElementById(selectedAddr)?.scrollIntoView({
          behavior: "smooth",
          block: "center",
        });

        let selectedId = `${selectedAddr === "filenode" ? "" : "selected_"}${selectedAddr}`;
        document.getElementById(selectedId)?.focus();
        document.getElementById(selectedAddr)?.focus();
      }
    }
    return () => {
      if (borderRef.current)
        borderRef.current.removeEventListener("transitionend", removeBorder);
    };
  }, [source, selectedAddr]);

  useEffect(() => {
    if (borderRef.current) {
      // Show the border only as it transitions, and hide it once done
      // (so that static border can take over).
      if (renderedAddr === "") borderRef.current.style.visibility = "visible";
      else borderRef.current.style.visibility = "hidden";
    }
  }, [renderedAddr]);

  const onScroll = () => {
    if (archerRef.current) archerRef.current.refreshScreen();
  };

  useEffect(() => {
    scrollRef.current?.addEventListener("scroll", onScroll);
    return () => scrollRef.current?.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <div style={{ display: hide ? "none" : "" }}>
      <div>
        <ArcherContainer
          ref={archerRef}
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
              {FileNode(
                fname,
                setFname,
                selectedAddr === "filenode",
                editFname,
              )}
            </ArcherElement>
          </div>

          <div
            ref={containerRef}
            style={{ position: "relative", overflow: "scroll" }}
          >
            <div
              id={`selected_${selectedAddr}`}
              ref={borderRef}
              style={BORDER_ANIMATION}
            />
            <div ref={scrollRef} style={{ ...ROW_STYLE, overflow: "scroll" }}>
              {source.map((subtree, i) =>
                RenderSubtree(
                  i.toString(),
                  subtree,
                  selectedAddr,
                  renderedAddr,
                  payload.pieceIx,
                  flipped,
                ),
              )}
            </div>
          </div>
        </ArcherContainer>
      </div>
    </div>
  );
}

function RenderSubtree(
  addr: string,
  subtreeRoot: RTLNode,
  selectedAddr: string,
  renderedAddr: string,
  pieceIx: number[] | null,
  flipped: string,
): ReactNode {
  return (
    <div key={addr}>
      <div style={FLEX_COL}>
        {RenderBlock(
          addr,
          subtreeRoot,
          selectedAddr,
          renderedAddr,
          pieceIx,
          addr.includes(".") ? subtreeRoot : undefined,
          flipped,
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
            RenderSubtree(
              `${addr}.1.${i}`,
              sub,
              selectedAddr,
              renderedAddr,
              pieceIx,
              flipped,
            ),
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
  renderedAddr: string,
  pieceIx: number[] | null,
  subtreeParent?: RTLNode,
  flipped: string = "",
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
            renderedAddr,
            pieceIx,
            subtreeParent,
            0,
            false,
            undefined,
            flipped,
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
  renderedAddr: string, // Address state that only updates after border moves
  pieceIx: number[] | null, // The piece being edited (not necessarily in `node`)
  parent?: RTLNode, // This node's parent
  parentIndents: number = 0, // How many times the parent node was indented
  check_blocks: boolean = true, // Used on recursive calls
  recursive: boolean = true, // Used on recursive calls
  flipped: string = "",
) {
  if (LOCAL_BLOCK_NODES.includes(node.kind) && check_blocks) {
    return (
      <div
        key={address}
        id={address}
        style={{
          ...FLEX_COL,
          border: renderedAddr === address ? "2px solid #f7dc28" : "",
          borderRadius: "10px",
        }}
      >
        <div id={`${address}.0`}>
          {RenderNode(
            node,
            `${address}.0`,
            selectedAddr,
            renderedAddr,
            pieceIx,
            parent,
            parentIndents,
            false,
            false,
            flipped,
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
              {RenderNode(
                n,
                `${address}.1.${i}`,
                selectedAddr,
                renderedAddr,
                pieceIx,
                node,
                undefined,
                undefined,
                undefined,
                flipped,
              )}
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
        border:
          renderedAddr === address
            ? "2px solid #f7dc28"
            : node.kind === "FNDEF"
              ? "2px solid #484848"
              : "",
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
          style={{ ...FLEX_ROW, alignItems: "center", gap: "8px" }}
        >
          <ReactCardFlip
            isFlipped={
              node.children.length > 0
                ? flipped === `${address}.0`
                : flipped === address
            }
            flipDirection="vertical"
          >
            <div
              style={{
                display: "flex",
                flexDirection: "row",
                width: "fit-content",
                justifyContent: "center",
                alignItems: "center",
                border:
                  renderedAddr === `${address}.0` ? "2px solid #f7dc28" : "",
                borderRadius: "10px",
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
                  !!(
                    indent &&
                    parent &&
                    excludeBlocks(parent)[0].address === node.address
                  )
                }
                indent={parentIndents + indent}
                addr={address}
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

            <div
              style={{
                display: "flex",
                flexDirection: "row",
                width: "fit-content",
                justifyContent: "center",
                alignItems: "center",
                background: "#333",
              }}
            >
              <textarea
                id={
                  node.children.length > 0
                    ? `note_${address}.0`
                    : `note_${address}`
                }
                onKeyUp={(e) => {
                  const target = e.target as HTMLTextAreaElement;
                  target.style.height = "0px";
                  target.style.height = -8 + target.scrollHeight + "px";
                }}
                style={{
                  width: "100%",
                  height: "fit-content",
                  textAlign: "start",
                  color: "#CCCCCC",
                  fontSize: "12px",
                  padding: "5px",
                }}
                placeholder="Enter note here..."
              />
            </div>
          </ReactCardFlip>
          {node.note ? <MessageIcon /> : null}
          {node.err ? (
            <ErrorIcon />
          ) : !["FNDEF", "PENDING"].includes(node.kind) ? (
            <CheckmarkIcon />
          ) : null}
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
                renderedAddr,
                pieceIx,
                node,
                parentIndents + indent,
                undefined,
                undefined,
                flipped,
              ),
            )
        : null}
    </div>
  );
}

function FileNode(
  fname: string,
  setFname: (arg0: string) => void,
  includeBorder: boolean,
  editing: boolean,
) {
  return (
    <div
      id="filenode"
      style={{
        background: `linear-gradient(45deg, #5C89FD, #00D1FF)`,
        height: "40px",
        width: "fit-content",
        border: includeBorder ? "2px solid #f7dc28" : "",
        borderRadius: "10px",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "10px",
        paddingRight: "10px",
        filter: `drop-shadow(-8px 2px 16px #5C89FD)`,
        ...FLEX_ROW,
      }}
    >
      {editing ? (
        <input
          id={`edit_filename`}
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: "black",
            background: "white",
            padding: "0px",
            width: `${fname.length + 2}ch`,
          }}
          value={fname}
          onChange={(e) => setFname(e.target.value.replace(" ", "_"))}
          onFocus={(e) => e.target.select()}
        />
      ) : (
        <p
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: "white",
          }}
        >
          {fname}
        </p>
      )}
    </div>
  );
}
