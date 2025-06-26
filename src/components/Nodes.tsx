import { useEffect } from "react";
import { Handle, Position, useUpdateNodeInternals } from "reactflow";
import "reactflow/dist/style.css";
import { RTLNode } from "../types";
import { FLEX_ROW } from "../styles";
import { Token, RenderPiece } from "../PieceRenderer";
import { arrEq, getColor } from "../utils";
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

const GLOBAL_BLOCK_NODES = ["FNDEF"];
const isBlock = (c: RTLNode) => GLOBAL_BLOCK_NODES.includes(c.kind);
const excludeBlocks = (c: RTLNode) => c.children.filter((c_) => !isBlock(c_));

export function RenderNode(
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

// Custom file node component for reactflow
export const FileNodeComponent = ({ data }: { data: any }) => {
  const { fname, setFname, includeBorder, editing } = data;
  const updateNodeInternals = useUpdateNodeInternals();

  useEffect(() => {
    updateNodeInternals("filenode");
  }, [updateNodeInternals]);

  return (
    <div
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
