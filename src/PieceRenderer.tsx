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
} from "./types";

export function RenderPiece(piece: RTLPiece, first: boolean) {
  const kind = extractPieceType(piece);
  switch (kind) {
    case "IDENT":
      return (
        <RenderIdentifier
          id_name={piece["IDENT" as keyof RTLPiece]}
          chained={first}
        />
      );
    case "NUMBER":
      return (
        <RenderNumber num={piece["NUMBER" as keyof RTLPiece]} chained={first} />
      );
    case "OP":
      return <RenderOperator op_name={piece["OP" as keyof RTLPiece]} />;
    case "TEXT":
      return (
        <RenderText text={piece["TEXT" as keyof RTLPiece]} chained={first} />
      );
    case "BOOL":
      return (
        <RenderBoolean bool={piece["BOOL" as keyof RTLPiece]} chained={first} />
      );
    case "NOTHING":
      return <RenderNothing chained={first} />;
    case "LIST":
      return (
        <RenderList pieces={piece["LIST" as keyof RTLPiece]} chained={first} />
      );
    case "FNCALL":
      return (
        <RenderCall
          pieces={piece["FNCALL" as keyof RTLPiece]}
          chained={first}
        />
      );
    case "PENDING":
      return <RenderPending />;
    default:
      throw new Error("Invalid piece found!");
  }
}

export function RenderNumber({ num, chained }) {
  return Symbol("constant", chained ? "" : "transparent", num.toString());
}

export function RenderBoolean({ bool, chained }) {
  return Symbol("constant", chained ? "" : "transparent", bool.toString());
}

export function RenderText({ text, chained }) {
  return Symbol("text", chained ? "" : "transparent", text);
}

export function RenderNothing({ chained }) {
  return Symbol("constant", chained ? "" : "transparent", "nothing");
}

export function RenderIdentifier({ id_name, chained }) {
  return Symbol("ident", chained ? "" : "transparent", id_name);
}

export function RenderList({ pieces, chained }) {
  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      {ChainSymbol("constant", chained, false, "list")}
      <div
        style={{
          border: `1px solid ${SYMBOL_MAP["constant"][0]}`,
          display: "flex",
          flexDirection: "row",
        }}
      >
        {pieces.slice(1).map((b: RTLPiece) => (
          <>
            {RenderPiece(b, false)}
            <div
              style={{
                height: "36px",
                width: "1px",
                backgroundColor: SYMBOL_MAP["constant"][0],
              }}
            />
          </>
        ))}
      </div>
      {ChainSymbol("constant", chained, true, "done")}
    </div>
  );
}

export function RenderCall({ pieces, chained }) {
  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      {ChainSymbol("call", chained, false, pieces[0]["IDENT"])}
      <div
        style={{
          border: `1px solid ${SYMBOL_MAP["call"][0]}`,
          display: "flex",
          flexDirection: "row",
        }}
      >
        {pieces.slice(1).map((b: RTLPiece) => (
          <>
            {RenderPiece(b, false)}
            {/* Vertical line separator */}
            <div
              style={{
                height: "36px",
                width: "1px",
                backgroundColor: SYMBOL_MAP["call" as keyof symbol_metadata][0],
              }}
            />
          </>
        ))}
      </div>
      {ChainSymbol("call", chained, true, "done")}
    </div>
  );
}

export function RenderOperator({ op_name }) {
  const _type = op_name === "ASSN" ? "arrow" : "operator";
  const _name = OP_TO_NAME[op_name as keyof op_kind];

  return (
    <>
      <div style={{ width: "6px" }} />
      {Symbol(_type, "transparent", _name)}
      <div style={{ width: "6px" }} />
    </>
  );
}

export function RenderPending() {
  return Symbol("pending", "transparent", "  +  ");
}

const TOKEN_MAP: token_metadata = {
  file: ["#FFFFFF", rattle_icon],
  function: ["#FE4949", gear],
  variable: ["#49A7FE", cube],
  conditional: ["#06975A", question],
  yes: ["#06975A", question],
  no: ["#06975A", question],
  library: ["#B140B4", book],
  output: ["#BC272a", output],
  return: ["#B140B4", book],
  list: ["#06975A", question],
  pending: ["transparent", ""],
};

export const SYMBOL_MAP: symbol_metadata = {
  constant: ["#FF8A00", "#FFFFFF", 16, 36],
  arrow: ["transparent", "#FFCD4E", 20, 36],
  operator: ["#701490", "#FFFFFF", 20, 25],
  text: ["#374f40", "#FFFFFF", 16, 36],
  ident: ["#000000", "#FFFFFF", 16, 36],
  call: ["#FFFFFF", "#000000", 16, 36],
  pending: ["#000000", "#FFFFFF", 16, 36],
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
  IN: "in",
  DOT: ".",
  ASSN: "->",
};

export function ChainSymbol(
  type: string,
  chained: boolean,
  start: boolean,
  symbol?: string,
) {
  let radii = "10px 0px 0px 10px";
  if (chained && !start) radii = "0px 0px 0px 0px";
  else if (start) radii = "0px 10px 10px 0px";

  return (
    <div
      style={{
        background: SYMBOL_MAP[type as keyof symbol_metadata][0],
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
    </div>
  );
}

export function Symbol(type: string, puzzle_color: string, symbol?: string) {
  return (
    <div
      style={{
        background: SYMBOL_MAP[type as keyof symbol_metadata][0],
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
      <span
        style={{
          fontFamily: "JetBrains Mono",
          textAlign: "start",
          color: SYMBOL_MAP[type as keyof symbol_metadata][1],
          fontSize: `${SYMBOL_MAP[type as keyof symbol_metadata][2]}px`,
        }}
      >
        {symbol}
      </span>
    </div>
  );
}

/**
 * A token is the first piece in a node (e.g., the "variable" keyword)
 * @param token_type {string} the token's kind (e.g., function, variable, ...)
 * @param puzzle_color {string} "transparent" if this token has no subsequent
 *								tokens (e.g, "yes" token in a conditional),
 *								otherwise any other string (all are equivalent)
 * @param first {boolean} whether the this token is part of the first line of a
 *						  global block (tells us whether to show the arrow)
 * @param indent {boolean} whether to indent or not
 */
export function Token({ token_type, puzzle_color, first, indent }) {
  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      {indent ? (
        <img
          style={{
            height: "24px",
            marginRight: "5px",
            visibility: first ? "visible" : "hidden",
          }}
          src={function_arrow}
        />
      ) : null}
      <div
        style={{
          backgroundColor: TOKEN_MAP[token_type as keyof token_metadata][0],
          height: "36px",
          width: "fit-content",
          borderRadius:
            puzzle_color === "transparent" ? "10px" : "10px 0px 0px 10px",
          border: token_type === "pending" ? "1px dashed white" : "",
          display: "flex",
          flexDirection: "row",
          justifyContent: "center", //Centered vertically
          alignItems: "center", //Centered horizontally
          paddingLeft: "8px",
        }}
      >
        <span
          style={{
            fontFamily: "JetBrains Mono",
            fontWeight: token_type === "pending" ? "" : "bold",
            paddingRight: puzzle_color === "transparent" ? "8px" : "5px",
            whiteSpace: "pre-wrap",
            cursor: token_type === "pending" ? "pointer" : "default",
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
    </div>
  );
}
