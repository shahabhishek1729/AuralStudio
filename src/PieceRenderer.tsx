import gear from "./assets/action_cpu.png";
import question from "./assets/conditional_q4.png";
import cube from "./assets/value_box.png";
import book from "./assets/vuesax/bold/book.png";
import rattle_icon from "./assets/rattle_icon.png";
import output from "./assets/output_icon.png";
import function_arrow from "./assets/function_arrow.png";

export function RenderPiece(piece, first) {
  if ("IDENT" in piece) {
    return <RenderIdentifier id_name={piece["IDENT"]} chained={first} />;
  } else if ("NUMBER" in piece) {
    return <RenderNumber num={piece["NUMBER"]} chained={first} />;
  } else if ("OP" in piece) {
    return <RenderOperator op_name={piece["OP"]} chained={first} />;
  } else if ("TEXT" in piece) {
    return <RenderText text={piece["TEXT"]} chained={first} />;
  } else if ("BOOL" in piece) {
    return <RenderBoolean bool={piece["BOOL"]} chained={first} />;
  } else if ("NOTHING" in piece) {
    return <RenderNothing chained={first} />;
  } else if ("LIST" in piece) {
    return <RenderList pieces={piece["LIST"]} chained={first} />;
  } else if ("FNCALL" in piece) {
    return <RenderCall pieces={piece["FNCALL"]} chained={first} />;
  } else {
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
  return Symbol("text", "", text);
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
        {pieces.slice(1).map((b) => (
          <>
            {RenderPiece(b)}
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
        {pieces.slice(1).map((b) => (
          <>
            {RenderPiece(b)}
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

interface token_metadata {
  file: [string, string];
  function: [string, string];
  variable: [string, string];
  conditional: [string, string];
  yes: [string, string];
  no: [string, string];
  library: [string, string];
  output: [string, string];
  return: [string, string];
  list: [string, string];
}

interface symbol_metadata {
  constant: [string, string, number, number];
  arrow: [string, string, number, number];
  operator: [string, string, number, number];
  text: [string, string, number, number];
}

interface op_kind {
  ADD: string;
  SUB: string;
  MUL: string;
  DIV: string;
  MOD: string;
  EQ: string;
  NE: string;
  GT: string;
  LT: string;
  GE: string;
  LE: string;
  AND: string;
  OR: string;
  NOT: string;
  IN: string;
  DOT: string;
  ASSN: string;
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
};

const SYMBOL_MAP: symbol_metadata = {
  constant: ["#FF8A00", "#FFFFFF", 16, 36],
  arrow: ["transparent", "#FFCD4E", 20, 36],
  operator: ["#701490", "#FFFFFF", 20, 25],
  text: ["#374f40", "#FFFFFF", 16, 36],
  ident: ["#000000", "#FFFFFF", 16, 36],
  call: ["#FFFFFF", "#000000", 16, 36],
};

export function getColor(piece) {
  if ("IDENT" in piece) return SYMBOL_MAP["ident"][0];
  else if ("TEXT" in piece) return SYMBOL_MAP["text"][0];
  else if ("LIST" in piece) return SYMBOL_MAP["constant"][0];
}

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
          display: "flex",
          flexDirection: "row",
          justifyContent: "center", //Centered vertically
          alignItems: "center", //Centered horizontally
          paddingLeft: "8px",
        }}
      >
        <p
          style={{
            fontFamily: "JetBrains Mono",
            fontWeight: "bold",
            paddingRight: puzzle_color === "transparent" ? "8px" : "5px",
          }}
        >
          {token_type === "conditional" ? "is" : token_type}
        </p>
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
