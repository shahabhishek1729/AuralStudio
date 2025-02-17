/**
 * @fileoverview Renders individual pieces (tokens and otherwise) for an acyclic
 * digraph - can render operators, tokens, identifiers, numbers, chains, etc.
 */

import gear from "./assets/action_cpu.png";
import question from "./assets/conditional_q4.png";
import cube from "./assets/value_box.png";
import book from "./assets/vuesax/bold/book.png";
import rattle_icon from "./assets/rattle_icon.png";
import output from "./assets/output_icon.png";
import function_arrow from "./assets/function_arrow.png";
import {
  RTLPiece,
  extractPieceType,
  symbol_metadata,
  op_kind,
  token_metadata,
  extractPieceValue,
  _PieceInterface,
} from "./types";
import { useState } from "react";
import { arrayEquals } from "./utils";
import { TokenMenu } from "./MenuTemplates";
import { FAIL_SOUND } from "./App";

const OP_TYPES = ["OP", "PendingOp"];

export function RenderPiece(
  all_pieces: RTLPiece[],
  i: number[],
  pieceIx: number[],
  parentAddr: string,
  pieceIxFull?: number[], // Only used in RenderArgs to prevent slicing addrs.
) {
  const piece = all_pieces[i[i.length - 1]];
  const kind = extractPieceType(piece);
  const first = i[0] === 0;
  const selected = arrayEquals(pieceIx, i);

  // Inner wrapper function allows us to key each piece in a node below
  function _RenderPiece() {
    // Don't re-render indices that have already been compressed into the ident
    if (
      i[i.length - 1] > 0 &&
      (all_pieces[i[i.length - 1] - 1] as _PieceInterface)["OP"] === "AT" &&
      all_pieces[i[i.length - 1]] !== "PendingVal" &&
      (!selected ||
        !!(all_pieces[i[i.length - 1]] as _PieceInterface)["IDENT"] ||
        !!(all_pieces[i[i.length - 1]] as _PieceInterface)["NUMBER"] ||
        !!(all_pieces[i[i.length - 1]] as _PieceInterface)["TEXT"])
    )
      return <></>;

    switch (kind) {
      case "IDENT":
        return (
          <RenderIdent
            pieces={all_pieces}
            i={i}
            name={piece["IDENT" as keyof RTLPiece]}
            chained={first}
            selected={selected}
            parentAddr={parentAddr}
            pieceIx={pieceIxFull || pieceIx}
          />
        );
      case "NUMBER":
        return (
          <RenderNumber
            num={piece["NUMBER" as keyof RTLPiece]}
            pieceIx={pieceIxFull || pieceIx}
            chained={first}
            selected={selected}
            parentAddr={parentAddr}
          />
        );
      case "OP":
        return (
          <RenderOperator
            op_name={piece["OP" as keyof RTLPiece]}
            selected={selected}
          />
        );
      case "TEXT":
        return (
          <RenderText
            text={piece["TEXT" as keyof RTLPiece]}
            pieceIx={pieceIxFull || pieceIx}
            chained={first}
            selected={selected}
            parentAddr={parentAddr}
          />
        );
      case "BOOL":
        return (
          <RenderBoolean
            bool={piece["BOOL" as keyof RTLPiece]}
            chained={first}
            selected={selected}
          />
        );
      case "NOTHING":
        return <RenderNothing chained={first} selected={selected} />;
      case "LIST":
        return (
          <RenderList
            pieces={piece["LIST" as keyof RTLPiece]}
            chained={first}
            myIx={i}
            pieceIx={pieceIx}
            parentAddr={parentAddr}
          />
        );
      case "FNCALL":
        return (
          <RenderCall
            pieces={piece["FNCALL" as keyof RTLPiece]}
            chained={first}
            myIx={i}
            pieceIx={pieceIx}
            parentAddr={parentAddr}
          />
        );
      case "PendingVal":
        return (
          <RenderPending chained={first} selected={selected} kind={kind} />
        );
      case "PendingOp":
        return (
          <RenderPending chained={first} selected={selected} kind={kind} />
        );
      default:
        throw new Error("Invalid piece found!");
    }
  }

  return <div key={i.join(".")}>{_RenderPiece()}</div>;
}

interface NumberProps {
  num: number;
  pieceIx: number[];
  chained: boolean;
  selected: boolean;
  parentAddr: string;
}

export function RenderNumber({
  num,
  pieceIx,
  chained,
  selected,
  parentAddr,
}: NumberProps) {
  return Symbol(
    "constant",
    chained ? "" : "transparent",
    selected,
    num.toString(),
    parentAddr,
    pieceIx,
  );
}

interface BoolProps {
  bool: boolean;
  chained: boolean;
  selected: boolean;
}

export function RenderBoolean({ bool, chained, selected }: BoolProps) {
  return Symbol(
    "constant",
    chained ? "" : "transparent",
    selected,
    bool.toString(),
  );
}

interface TextProps {
  text: string;
  pieceIx: number[];
  chained: boolean;
  selected: boolean;
  parentAddr: string;
}

export function RenderText({
  text,
  pieceIx,
  chained,
  selected,
  parentAddr,
}: TextProps) {
  return Symbol(
    "text",
    chained ? "" : "transparent",
    selected,
    text,
    parentAddr,
    pieceIx,
  );
}

interface NullProps {
  chained: boolean;
  selected: boolean;
}

export function RenderNothing({ chained, selected }: NullProps) {
  return Symbol("constant", chained ? "" : "transparent", selected, "nothing");
}

interface IdentProps {
  pieces: RTLPiece[];
  i: number[];
  name: string;
  pieceIx: number[];
  chained: boolean;
  selected: boolean;
  parentAddr: string;
}

export function RenderIdent({
  pieces,
  i,
  name,
  chained,
  selected,
  parentAddr,
  pieceIx,
}: IdentProps) {
  const i_ = i[i.length - 1];
  const puzzle_color = chained ? "" : "transparent";
  const idxFollows =
    i_ < pieces.length - 2 && (pieces[i_ + 1] as _PieceInterface).OP === "AT";
  let idx = idxFollows ? ` #${extractPieceValue(pieces[i_ + 2]) ?? ""}` : "";

  const background = selected ? "white" : SYMBOL_MAP["ident"][0];
  const foreground = selected ? "black" : SYMBOL_MAP["ident"][1];

  const [value, setValue] = useState(name);

  return (
    <div
      style={{
        background: background,
        height: `${SYMBOL_MAP["ident"][3]}px`,
        width: "fit-content",
        minWidth: `${SYMBOL_MAP["ident"][3] - 10}px`,
        borderRadius:
          puzzle_color === "transparent" ? "10px" : "0px 10px 10px 0px",
        display: "flex",
        flexDirection: "row",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "5px",
        paddingRight: "5px",
      }}
    >
      {selected ? (
        <input
          id={`${parentAddr},${selected ? (pieceIx ?? []).join(",") : i}`}
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: foreground,
            background: "white",
            fontSize: `${SYMBOL_MAP["ident"][2]}px`,
            padding: "0px",
            width: `${value.length + 2}ch`,
            boxShadow: "none",
          }}
          value={value}
          onChange={(e) => setValue(e.target.value.replace(" ", "_"))}
          onFocus={(e) => e.target.select()}
        />
      ) : (
        <span
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: foreground,
            fontSize: `${SYMBOL_MAP["ident"][2]}px`,
          }}
        >
          {name}
        </span>
      )}
      {idxFollows ? (
        <span
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: SYMBOL_MAP["arrow"][1],
            fontSize: `${SYMBOL_MAP["constant"][2]}px`,
            whiteSpace: "pre",
          }}
        >
          {idx}
        </span>
      ) : null}
    </div>
  );
}

function elementDone(pieces: RTLPiece[], i: number): boolean {
  if (i === pieces.length - 1) return true;
  const currType = extractPieceType(pieces[i]);
  const nextType = extractPieceType(pieces[i + 1]);
  return !OP_TYPES.includes(currType) && !OP_TYPES.includes(nextType);
}

interface CompoundProps {
  pieces: RTLPiece[];
  myIx: number[];
  pieceIx: number[];
  chained: boolean;
  parentAddr: string;
}

export function RenderList({
  pieces,
  chained,
  myIx,
  pieceIx,
  parentAddr,
}: CompoundProps) {
  return RenderArgs("list", pieces, chained, myIx, pieceIx, parentAddr);
}

export function RenderCall({
  pieces,
  chained,
  myIx,
  pieceIx,
  parentAddr,
}: CompoundProps) {
  return RenderArgs("call", pieces, chained, myIx, pieceIx, parentAddr);
}

export function RenderArgs(
  kind: string,
  pieces: RTLPiece[],
  chained: boolean,
  myIx: number[],
  pieceIx: number[],
  parentAddr: string,
) {
  const colorKind = kind === "list" ? "constant" : kind;
  const name = kind === "call" ? (pieces[0] as _PieceInterface).IDENT : "list";

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        border: arrayEquals(myIx, pieceIx) ? "2px solid white" : "",
      }}
    >
      {ChainSymbol(
        colorKind,
        chained,
        false,
        parentAddr,
        arrayEquals(pieceIx.slice(0, myIx.length), myIx) ? pieceIx : [],
        name,
      )}
      <div
        style={{
          border: `1px solid ${SYMBOL_MAP[colorKind as keyof symbol_metadata][0]}`,
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
        }}
      >
        {pieces.slice(1).map((piece, i: number) => (
          <div key={i} style={{ display: "flex" }}>
            <div style={{ transform: "scale(0.9)", height: "fit-content" }}>
              {RenderPiece(
                pieces,
                [...myIx, i + 1],
                pieceIx,
                parentAddr,
                pieceIx,
              )}
            </div>
            {elementDone(pieces, i + 1) ? (
              <div
                style={{
                  height: piece === "PendingOp" ? "25px" : "36px",
                  width: "1px",
                  backgroundColor:
                    SYMBOL_MAP[colorKind as keyof symbol_metadata][0],
                }}
              />
            ) : null}
          </div>
        ))}
      </div>
      {ChainSymbol(colorKind, chained, true, parentAddr, [], "done")}
    </div>
  );
}

interface OpProps {
  op_name: string;
  selected: boolean;
}

export function RenderOperator({ op_name, selected }: OpProps) {
  const _type = op_name === "ASSN" ? "arrow" : "operator";
  const _name = OP_TO_NAME[op_name as keyof op_kind];

  if (_name === "") return <></>;

  return (
    <div style={{ display: "flex" }}>
      <div style={{ width: "6px" }} />
      {Symbol(_type, "transparent", selected, _name)}
      <div style={{ width: "6px" }} />
    </div>
  );
}

interface PendingProps {
  chained: boolean;
  selected: boolean;
  kind: string;
}

export function RenderPending({ chained, selected, kind }: PendingProps) {
  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      {!chained ? <div style={{ width: "5px" }} /> : null}
      {Symbol(kind, chained ? "" : "transparent", selected, "...")}
    </div>
  );
}

const TOKEN_MAP: token_metadata = {
  file: ["#FFFFFF", rattle_icon],
  function: ["#FE4949", gear],
  variable: ["#49A7FE", cube],
  conditional: ["#06975A", question],
  yes: ["#06975A", question],
  no: ["#06975A", question],
  for: ["#06975A", question],
  while: ["#06975A", question],
  library: ["#B140B4", book],
  output: ["#FF7A00", output],
  return: ["#B140B4", book],
  list: ["#06975A", question],
  pending: ["transparent", ""],
};

export const SYMBOL_MAP: symbol_metadata = {
  constant: ["#FF8A00", "#FFFFFF", 16, 36],
  arrow: ["transparent", "#FFCD4E", 20, 36],
  operator: ["#701490", "#FFFFFF", 20, 25],
  text: ["#374f40", "#FFFFFF", 16, 36],
  ident: ["#333333", "#FFFFFF", 16, 36],
  call: ["#179c8a", "#FFFFFF", 16, 36],
  PendingVal: ["transparent", "#FFFFFF", 16, 36],
  PendingOp: ["transparent", "#FFFFFF", 16, 25],
};

const OP_TO_NAME: op_kind = {
  ADD: "+",
  SUB: "-",
  MUL: "*",
  DIV: "÷",
  MOD: "%",
  EQ: "==",
  NE: "!=",
  GT: ">",
  LT: "<",
  GE: ">=",
  LE: "<=",
  AND: "and",
  OR: "or",
  NOT: "not",
  IN: ":",
  DOT: ".",
  ASSN: "->",
  AT: "",
};

export function ChainSymbol(
  type: string,
  chained: boolean,
  start: boolean,
  parentAddr: string,
  pieceIx: number[],
  symbol?: string,
) {
  let radii = "10px 0px 0px 10px";
  if (chained && !start) radii = "0px 0px 0px 0px";
  else if (start) radii = "0px 10px 10px 0px";

  const selected = pieceIx[pieceIx.length - 1] === 0;
  const background = selected
    ? "white"
    : SYMBOL_MAP[type as keyof symbol_metadata][0];
  const foreground = selected
    ? "black"
    : SYMBOL_MAP[type as keyof symbol_metadata][1];

  const [value, setValue] = useState("");

  return (
    <div
      style={{
        background: background,
        height: `${SYMBOL_MAP[type as keyof symbol_metadata][3]}px`,
        width: "fit-content",
        minWidth: `${SYMBOL_MAP[type as keyof symbol_metadata][3] - 10}px`,
        borderRadius: radii,
        display: "flex",
        flexDirection: "row",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "5px",
        paddingRight: "5px",
        paddingTop: "1px",
        paddingBottom: "1px",
      }}
    >
      {selected ? (
        <input
          id={`${parentAddr},${pieceIx}`}
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: foreground,
            background: "white",
            fontSize: `${SYMBOL_MAP[type as keyof symbol_metadata][2]}px`,
            padding: "0px",
            width: `${value.length + 2}ch`,
            boxShadow: "none",
          }}
          value={value}
          onChange={(e) => setValue(e.target.value.replace(" ", "_"))}
          onFocus={(e) => e.target.select()}
        />
      ) : (
        <p
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: SYMBOL_MAP[type as keyof symbol_metadata][1],
            fontSize: `${SYMBOL_MAP[type as keyof symbol_metadata][2]}px`,
          }}
        >
          {symbol}
        </p>
      )}
    </div>
  );
}

export function Symbol(
  type: string,
  puzzle_color: string,
  selected: boolean,
  symbol?: string,
  parentAddr?: string,
  pieceIx?: number[],
) {
  const background = selected
    ? "white"
    : SYMBOL_MAP[type as keyof symbol_metadata][0];
  const foreground = selected
    ? "black"
    : SYMBOL_MAP[type as keyof symbol_metadata][1];

  const [value, setValue] = useState("");

  return (
    <div
      style={{
        background: background,
        height: `${SYMBOL_MAP[type as keyof symbol_metadata][3]}px`,
        width: "fit-content",
        minWidth: `${SYMBOL_MAP[type as keyof symbol_metadata][3] - 10}px`,
        borderRadius:
          puzzle_color === "transparent" ? "10px" : "0px 10px 10px 0px",
        display: "flex",
        flexDirection: "row",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "5px",
        paddingRight: "5px",
      }}
    >
      {selected && ["text", "constant", "ident", "call"].includes(type) ? (
        <input
          id={`${parentAddr},${pieceIx}`}
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: foreground,
            background: "white",
            fontSize: `${SYMBOL_MAP[type as keyof symbol_metadata][2]}px`,
            padding: "0px",
            width: `${value.length + 2}ch`,
            boxShadow: "none",
          }}
          value={value}
          onChange={(e) => {
            if (type === "constant" && isNaN(+e.target.value)) {
              FAIL_SOUND.play();
              return;
            }
            setValue(e.target.value);
          }}
          onFocus={(e) => e.target.select()}
        />
      ) : (
        <span
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            color: foreground,
            fontSize: `${SYMBOL_MAP[type as keyof symbol_metadata][2]}px`,
          }}
        >
          {symbol}
        </span>
      )}
    </div>
  );
}

interface TokenProps {
  token_type: string;
  puzzle_color: string;
  first: boolean;
  indent: number;
  addr: string;
}

/**
 * A token is the first piece in a node (e.g., the "variable" keyword)
 * @param token_type {string} the token's kind (e.g., function, variable, ...)
 * @param puzzle_color {string} "transparent" if this token has no subsequent
 *								tokens (e.g, "yes" token in a conditional),
 *								otherwise any other string (all are equivalent)
 * @param first {boolean} whether the this token is part of the first line of a
 *						  global block (tells us whether to show the arrow)
 * @param indent {number} how many indents to apply
 * @param addr {string} the address of the node containing this token
 */
export function Token({
  token_type,
  puzzle_color,
  first,
  indent,
  addr,
}: TokenProps) {
  const [clicked, setClicked] = useState(false);

  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      {[...Array(indent)].map((_, i) => (
        <img
          key={i}
          style={{
            height: "24px",
            marginRight: "5px",
            visibility: first && i + 1 === indent ? "visible" : "hidden",
          }}
          src={function_arrow}
        />
      ))}
      <div style={{ display: "flex", flexDirection: "column" }}>
        {clicked ? (
          <TokenMenu
            onClick={() => {
              setClicked(false);
              const border = document.getElementById(`selected_${addr}`);
              if (border) border.style.visibility = "visible";
            }}
          />
        ) : (
          <div
            style={{
              backgroundColor: TOKEN_MAP[token_type as keyof token_metadata][0],
              height: "36px",
              width: token_type === "pending" ? "100%" : "fit-content",
              borderRadius:
                puzzle_color === "transparent" ? "10px" : "10px 0px 0px 10px",
              border: token_type === "pending" ? "1px dashed white" : "",
              display: "flex",
              flexDirection: "row",
              justifyContent: "center", //Centered vertically
              alignItems: "center", //Centered horizontally
              paddingLeft: token_type === "pending" ? "0px" : "8px",
            }}
            onClick={() => {
              setClicked(true);
              document.getElementById(`selected_${addr}`)!.style.visibility =
                "hidden";
            }}
          >
            <span
              style={{
                fontFamily: "JetBrains Mono",
                fontWeight: token_type === "pending" ? "" : "bold",
                paddingRight: puzzle_color === "transparent" ? "8px" : "5px",
                whiteSpace: "pre-wrap",
                cursor: token_type === "pending" ? "pointer" : "",
                fontSize: token_type === "pending" ? "28px" : "",
              }}
            >
              {token_type === "conditional"
                ? "is"
                : token_type === "pending"
                  ? "    +    "
                  : token_type}
            </span>
            {puzzle_color !== "transparent" ? (
              <svg
                width="20"
                height="23"
                viewBox="0 0 20 23"
                style={{ marginRight: "-1px" }}
              >
                <path
                  d="M-5.40914e-07 10.6253C-3.974e-06 6.30178 20 -8.74228e-07 20 -8.74228e-07L20 23C20 23 2.89217e-06 14.9489 -5.40914e-07 10.6253Z"
                  fill={puzzle_color}
                />
              </svg>
            ) : null}
          </div>
        )}
      </div>
    </div>
  );
}
