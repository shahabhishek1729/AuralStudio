import { useEffect, useRef, useState } from "react";
import { Handle, Position, useUpdateNodeInternals } from "reactflow";
import "reactflow/dist/style.css";
import { FLEX_COL, FLEX_ROW } from "../styles";
import { RTLNode } from "../typing/digraph";
import { Token, RenderPiece } from "../PieceRenderer";
import { arrEq, getColor } from "../utils";
import { CheckmarkIcon, ErrorIcon, MessageIcon } from "./Components";
import function_arrow from "../assets/function_arrow.png";
import { updateNodeSize } from "../Digraph";

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

const GLOBAL_BLOCK_NODES = ["FNDEF"];
const isBlock = (c: RTLNode) => GLOBAL_BLOCK_NODES.includes(c.kind);
const excludeBlocks = (c: RTLNode) => c.children.filter((c_) => !isBlock(c_));

export const RTLNodeComponent = ({ data }: { data: any }) => {
  function RenderNode(
    node: RTLNode,
    address: string,
    renderedAddr: string,
    selectedAddr: string,
    parentIndents: number,
    pieceIx: number[] | null,
    parent?: RTLNode,
  ) {
    return (
      <div
        id={address}
        key={address}
        style={{ ...FLEX_ROW, alignItems: "center", gap: "8px" }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            width: "fit-content",
            justifyContent: "center",
            alignItems: "center",
            border:
              renderedAddr === `${address}.0` && selectedAddr === `${address}.0`
                ? "2px solid #f7dc28"
                : "",
            borderRadius: "25px",
          }}
        >
          <Token
            token_type={token_type[node.kind as keyof typeof token_type]}
            puzzle_color={
              node.address === selectedAddr && arrEq(pieceIx ?? [1], [0])
                ? "white"
                : node.pieces.length > 0
                  ? getColor(node.pieces[0])
                  : "transparent"
            }
            first={
              !!(
                parentIndents &&
                parent &&
                excludeBlocks(parent)[0]?.address === node.address
              )
            }
            indent={parentIndents || 0}
            addr={address}
          />
          {node.pieces.map((_: any, ix: number) =>
            RenderPiece(
              node.pieces,
              [ix],
              node.address === selectedAddr ? pieceIx ?? [-1] : [-1],
              node.address,
            ),
          )}
        </div>
        {node.note ? <MessageIcon /> : null}
        {node.err ? (
          <ErrorIcon />
        ) : !["FNDEF", "PENDING"].includes(node.kind) ? (
          <CheckmarkIcon />
        ) : null}
      </div>
    );
  }

  const ref = useRef(null);
  const [_, setSize] = useState({ width: 150, height: 50 }); // defaults

  useEffect(() => {
    const observer = new ResizeObserver(([entry]) => {
      const { width, height } = entry.contentRect;
      setSize({ width, height });

      // Update global node size state
      updateNodeSize(data.node.address, width, height);

      console.log(`Width was ${width}, height was ${height}`);
    });

    if (ref.current) observer.observe(ref.current);
    return () => observer.disconnect();
  }, [ref, data.node.address]);

  const { node, selectedAddr, renderedAddr, pieceIx, parentIndents } = data;
  const address = node.address;
  const updateNodeInternals = useUpdateNodeInternals();

  useEffect(() => {
    updateNodeInternals(address);
  }, [address, updateNodeInternals]);

  return (
    <div
      key={address}
      ref={ref}
      style={{
        background:
          renderedAddr === `${address}.0` && selectedAddr === `${address}.0`
            ? "#f7dc28"
            : "linear-gradient(#FFFFFF2A, #191A1B99)",
        borderRadius: "25px",
        padding: "2px",
      }}
    >
      <div
        id={address.slice(0, address.length - 2)}
        style={{
          height: "100%",
          background:
            "radial-gradient(ellipse 60% 70% at center top, #292B4C, #18191B)",
          borderRadius: "23px",
          padding: "40px",
        }}
      >
        {/* Source handle for outgoing edges */}
        <Handle
          type="source"
          position={Position.Bottom}
          id={`${address}-source`}
          style={{ background: "transparent", color: "transparent" }}
        />

        {/* Target handle for incoming edges */}
        <Handle
          type="target"
          position={Position.Top}
          id={`${address}-target`}
          style={{ background: "white" }}
        />

        {RenderNode(
          node,
          node.address,
          renderedAddr,
          selectedAddr,
          parentIndents,
          pieceIx,
          undefined,
        )}

        <div style={{ ...FLEX_ROW, marginTop: "0.5rem" }}>
          <img
            style={{
              height: "24px",
              marginRight: "5px",
            }}
            src={function_arrow}
          />
          <div style={{ ...FLEX_COL, gap: "0.3rem" }}>
            {node.children
              .filter((child: RTLNode) => child.kind != "FNDEF")
              .map((child: RTLNode) =>
                RenderNode(
                  child,
                  child.address,
                  renderedAddr,
                  selectedAddr,
                  parentIndents,
                  pieceIx,
                  node,
                ),
              )}
          </div>
        </div>
      </div>
    </div>
  );
};

// Custom file node component for reactflow
export const FileNodeComponent = ({ data }: { data: any }) => {
  const { fname, setFname, includeBorder, editing } = data;
  const updateNodeInternals = useUpdateNodeInternals();
  const ref = useRef(null);

  useEffect(() => {
    updateNodeInternals("filenode");
  }, [updateNodeInternals]);

  useEffect(() => {
    const observer = new ResizeObserver(([entry]) => {
      const { width, height } = entry.contentRect;

      // Update global node size state for filenode
      updateNodeSize("filenode", width, height);

      console.log(`FileNode width was ${width}, height was ${height}`);
    });

    if (ref.current) observer.observe(ref.current);
    return () => observer.disconnect();
  }, [ref]);

  return (
    <div
      ref={ref}
      id="filenode"
      style={{
        background: `linear-gradient(45deg, #5C89FD, #00D1FF)`,
        height: "40px",
        width: "fit-content",
        border: includeBorder ? "2px solid #EEEEFFAA" : "",
        borderRadius: "10px",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "10px",
        paddingRight: "10px",
        // filter: `drop-shadow(-8px 2px 16px #5C89FD)`,
        ...FLEX_ROW,
      }}
    >
      {/* Source handle for outgoing edges to root functions */}
      <Handle
        type="source"
        position={Position.Bottom}
        id="filenode-source"
        style={{ background: "white" }}
      />

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
};
