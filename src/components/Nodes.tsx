import React from "react";
import { useEffect, useRef, useState } from "react";
import { Handle, Position, useUpdateNodeInternals } from "reactflow";
import "reactflow/dist/style.css";
import { NodeKind, RTLNode } from "../typing/digraph";
import { FLEX_COL, FLEX_ROW } from "../styles/styles";
import { Token, RenderPiece } from "../PieceRenderer";
import { arrEq, getColor } from "../utils/utils";
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

export const RTLNodeComponent = React.memo(({ data }: { data: any }) => {
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
      <div id={outerBlockId(node)} key={address}>
        <div
          id={address}
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
                renderedAddr === `${address}.0` &&
                selectedAddr === `${address}.0`
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
          ) : showIcon(node.kind) ? (
            <CheckmarkIcon />
          ) : null}
        </div>
        {["FORLOOP", "WHLLOOP", "CONDTLY", "CONDTLN"].includes(node.kind) ? (
          <div style={{ ...FLEX_ROW }}>
            {node.kind === "FORLOOP" || node.kind === "WHLLOOP" ? (
              <img
                style={{
                  height: "24px",
                  marginRight: "5px",
                }}
                src={function_arrow}
              />
            ) : null}
            <div style={FLEX_COL}>
              <div
                style={{
                  height: ["CONDTLY", "CONDTLN"].includes(node.kind)
                    ? "2px"
                    : "8px",
                }}
              />
              {node.children.map((child) => (
                <div>
                  {RenderNode(
                    child,
                    child.address,
                    renderedAddr,
                    selectedAddr,
                    parentIndents,
                    pieceIx,
                  )}
                  <div style={{ height: "8px" }} />
                </div>
              ))}
            </div>
          </div>
        ) : node.kind === "CONDTL" ? (
          <div style={FLEX_COL}>
            <svg
              width="100%"
              preserveAspectRatio="none"
              viewBox="0 0.8 7 2.5"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M0.964645 3.01289C0.984171 3.03242 1.01583 3.03242 1.03536 3.01289L1.35355 2.6947C1.37308 2.67517 1.37308 2.64351 1.35355 2.62399C1.33403 2.60446 1.30237 2.60446 1.28284 2.62399L1 2.90683L0.717157 2.62399C0.697631 2.60446 0.665973 2.60446 0.646446 2.62399C0.62692 2.64351 0.62692 2.67517 0.646447 2.6947L0.964645 3.01289ZM1.05 1.00053C1.05 0.972913 1.02761 0.950527 0.999999 0.950527C0.972385 0.950527 0.949999 0.972913 0.949999 1.00053L1.05 1.00053ZM1 2.97754L1.05 2.97754L1.05 1.00053L0.999999 1.00053L0.949999 1.00053L0.95 2.97754L1 2.97754Z"
                fill="white"
              />
              <path
                d="M1.00227 1.67147L1.05227 1.67118L1.00227 1.67147ZM5.68556 3.03536C5.70509 3.05488 5.73675 3.05488 5.75627 3.03536L6.07447 2.71716C6.094 2.69763 6.094 2.66597 6.07447 2.64645C6.05495 2.62692 6.02329 2.62692 6.00376 2.64645L5.72092 2.92929L5.43808 2.64645C5.41855 2.62692 5.38689 2.62692 5.36736 2.64645C5.34784 2.66597 5.34784 2.69763 5.36736 2.71716L5.68556 3.03536ZM1.00226 1.03009C1.05225 1.02903 1.05225 1.02903 1.05225 1.02903C1.05225 1.02903 1.05225 1.02902 1.05225 1.02902C1.05225 1.02901 1.05225 1.02899 1.05225 1.02898C1.05225 1.02895 1.05225 1.0289 1.05224 1.02884C1.05224 1.02872 1.05224 1.02854 1.05223 1.02832C1.05222 1.02787 1.05221 1.02723 1.05219 1.02643C1.05215 1.02484 1.0521 1.02263 1.05203 1.02013C1.0519 1.01523 1.05171 1.00886 1.05149 1.00396C1.0514 1.0018 1.05125 0.998805 1.051 0.996278C1.05093 0.99559 1.05081 0.994398 1.0506 0.992999C1.0506 0.99298 1.04997 0.988007 1.04785 0.982386C1.0472 0.980684 1.04581 0.977258 1.04333 0.973326C1.04145 0.970348 1.03538 0.961217 1.02322 0.955195C1.00695 0.947139 0.988611 0.948856 0.974652 0.957519C0.964163 0.964027 0.959095 0.972553 0.957495 0.975377C0.953864 0.981783 0.952566 0.987527 0.952352 0.988411C0.951867 0.990421 0.951602 0.992063 0.951473 0.992917C0.950829 0.997199 0.950703 1.00231 0.950639 1.00442C0.95053 1.00799 0.950436 1.01286 0.950355 1.01923C0.949713 1.06985 0.949722 1.23075 0.952275 1.67176L1.00227 1.67147L1.05227 1.67118C1.04972 1.22976 1.04972 1.06997 1.05035 1.0205C1.05043 1.01436 1.05051 1.01017 1.05059 1.00746C1.05072 1.00329 1.0508 1.0049 1.05036 1.00779C1.05026 1.00847 1.05002 1.00997 1.04956 1.01188C1.04937 1.01265 1.0481 1.01832 1.0445 1.02468C1.04291 1.02748 1.03786 1.03599 1.02738 1.04249C1.01343 1.05114 0.995103 1.05286 0.978847 1.04481C0.966699 1.03879 0.960647 1.02968 0.958779 1.02672C0.95631 1.02281 0.954935 1.01941 0.954305 1.01774C0.953007 1.01431 0.952402 1.01152 0.952212 1.01063C0.951947 1.00938 0.95179 1.0084 0.951711 1.00787C0.951554 1.00683 0.951482 1.00609 0.951468 1.00594C0.951434 1.00559 0.951493 1.00622 0.951594 1.00847C0.951771 1.01238 0.95194 1.01795 0.952068 1.02278C0.952131 1.02515 0.952182 1.02725 0.952217 1.02876C0.952235 1.02951 0.952248 1.03012 0.952257 1.03053C0.952262 1.03074 0.952265 1.03089 0.952268 1.031C0.952269 1.03105 0.95227 1.03109 0.95227 1.03111C0.95227 1.03113 0.952271 1.03114 0.952271 1.03114C0.952271 1.03114 0.952271 1.03114 0.952271 1.03115C0.952271 1.03115 0.952271 1.03115 1.00226 1.03009ZM1.00227 1.67147L0.952275 1.67176C0.952556 1.72032 0.98358 1.75601 1.01908 1.78072C1.05493 1.80567 1.10399 1.82588 1.16105 1.84301C1.27581 1.87747 1.43724 1.90355 1.62818 1.92418C2.01087 1.96554 2.52368 1.98615 3.04541 2.00415C3.56816 2.02219 4.10015 2.03761 4.52424 2.0684C4.73632 2.08379 4.91998 2.10294 5.0612 2.1279C5.13185 2.14038 5.19072 2.15412 5.2367 2.16914C5.28382 2.18454 5.31267 2.19981 5.32741 2.21286L5.36054 2.17541L5.39367 2.13797C5.36337 2.11116 5.31877 2.09076 5.26777 2.07409C5.21562 2.05705 5.1517 2.04234 5.0786 2.02942C4.93233 2.00358 4.74469 1.98414 4.53148 1.96866C4.10496 1.93769 3.57035 1.92221 3.04886 1.90421C2.52637 1.88619 2.01732 1.86565 1.63893 1.82476C1.44932 1.80427 1.29541 1.77895 1.18981 1.74724C1.13668 1.73129 1.09932 1.71473 1.0762 1.69864C1.05275 1.68232 1.05228 1.67273 1.05227 1.67118L1.00227 1.67147ZM5.36054 2.17541L5.32741 2.21286C5.49776 2.36358 5.58406 2.55959 5.62761 2.72034C5.64932 2.80047 5.66016 2.871 5.66557 2.92134C5.66828 2.94648 5.66961 2.96651 5.67027 2.98008C5.6706 2.98686 5.67076 2.99203 5.67084 2.99541C5.67088 2.99709 5.6709 2.99834 5.67091 2.99911C5.67091 2.99949 5.67092 2.99976 5.67092 2.99991C5.67092 2.99999 5.67092 3.00003 5.67092 3.00005C5.67092 3.00005 5.67092 3.00005 5.67092 3.00005C5.67092 3.00004 5.67092 3.00003 5.67092 3.00003C5.67092 3.00002 5.67092 3 5.72092 3C5.77092 3 5.77092 2.99998 5.77092 2.99996C5.77092 2.99995 5.77092 2.99993 5.77092 2.99991C5.77092 2.99987 5.77092 2.99982 5.77092 2.99977C5.77092 2.99966 5.77092 2.99952 5.77092 2.99935C5.77091 2.99901 5.77091 2.99855 5.7709 2.99797C5.77089 2.99681 5.77087 2.99517 5.77082 2.99308C5.77072 2.98889 5.77053 2.98288 5.77015 2.97521C5.76941 2.95988 5.76793 2.93791 5.765 2.91065C5.75915 2.8562 5.74747 2.78036 5.72413 2.6942C5.67758 2.52236 5.5837 2.30609 5.39367 2.13797L5.36054 2.17541Z"
                fill="white"
              />
            </svg>

            <div style={FLEX_ROW}>
              {/* NOTE: Should ony have 2 children (CONDTLY, CONDTLN) */}
              {node.children.map((child) => (
                <div>
                  {RenderNode(
                    child,
                    child.address,
                    renderedAddr,
                    selectedAddr,
                    parentIndents,
                    pieceIx,
                  )}
                  <div style={{ height: "8px" }} />
                </div>
              ))}
            </div>
          </div>
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
});

function outerBlockId(node: RTLNode): string | undefined {
  let offset =
    node.kind === "CONDTL"
      ? -4
      : ["FORLOOP", "WHLLOOP", "CONDTLY", "CONDTLN"].includes(node.kind)
        ? -2
        : undefined;
  if (offset) return node.address.substring(0, node.address.length + offset);
}

// Certain nodes can never contain errors, either because they are not
// user-defined or they have not been filled in yet (i.e., placeholders). For
// these, don't show success/failure icons.
function showIcon(kind: NodeKind): boolean {
  return !["CONDTLY", "CONDTLN", "PENDING", "FNDEF"].includes(kind);
}

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
