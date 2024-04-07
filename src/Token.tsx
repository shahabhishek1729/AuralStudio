import gear from "./assets/action_cpu.png";
import question from "./assets/conditional_q4.png";
import cube from "./assets/value_box.png";
import book from "./assets/vuesax/bold/book.png";
import rattle_icon from "./assets/rattle_icon.png";
import output from "./assets/output_icon.png";
import function_arrow from "./assets/function_arrow.png";
import { ReactNode } from "react";
import { block } from "./flattener";

interface token_metadata {
  file: [string, string];
  function: [string, string];
  variable: [string, string];
  conditional: [string, string];
  library: [string, string];
  output: [string, string];
}

const TOKEN_MAP = {
  file: ["#FFFFFF", rattle_icon],
  function: ["#FE4949", gear],
  variable: ["#49A7FE", cube],
  conditional: ["#06975A", question],
  library: ["#B140B4", book],
  output: ["#000000", output],
};

export function FileToken(fname: string) {
  return (
    <div
      style={{
        display: "flex",
        background: "white",
        height: "40px",
        width: "fit-content",
        borderRadius: "10px",
        flexDirection: "row",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "60px",
        paddingRight: "10px",
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
          // width: "90px",
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

export function Token(
  token_type: string,
  name?: string,
  internals: ReactNode | null = null
) {
  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      <div
        style={{
          background: TOKEN_MAP[token_type as keyof token_metadata][0],
          height: "40px",
          width: "fit-content",
          borderRadius: "10px",
          display: "flex",
          flexDirection: "row",
          justifyContent: "center", //Centered vertically
          alignItems: "center", //Centered horizontally
          paddingLeft: "15px",
          paddingRight: "10px",
        }}
      >
        <img
          src={TOKEN_MAP[token_type as keyof token_metadata][1]}
          height="32px"
          style={{
            marginLeft: "-8px",
            marginRight: "4px",
          }}
        />

        <div
          style={{ height: "40px", width: "1px", backgroundColor: "#FFFFFF" }}
        ></div>

        <p
          style={{
            fontFamily: "JetBrains Mono",
            textAlign: "start",
            marginLeft: "7px",
          }}
        >
          {name}
        </p>

        <div style={{ width: "5px" }} />
      </div>
      {internals}
    </div>
  );
}

interface symbol_metadata {
  constant: [string, string, number, number];
  arrow: [string, string, number, number];
  operator: [string, string, number, number];
  text: [string, string, number, number];
}

const SYMBOL_MAP: symbol_metadata = {
  constant: ["#FF8A00", "#FFFFFF", 16, 30],
  arrow: ["transparent", "#FFCD4E", 20, 30],
  operator: ["#701490", "#FFFFFF", 20, 25],
  text: ["#7E8E1C", "#FFFFFF", 16, 30],
};

export function Symbol(type: string, symbol: string) {
  return (
    <div
      style={{
        background: SYMBOL_MAP[type as keyof symbol_metadata][0],
        height: `${SYMBOL_MAP[type as keyof symbol_metadata][3]}px`,
        width: "fit-content",
        minWidth: `${SYMBOL_MAP[type as keyof symbol_metadata][3] - 10}px`,
        borderRadius: "10px",
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

const expr_: block[] = [
  {
    kind: "output",
    line: 1,
    name: "output",
  },
  {
    kind: "text",
    value: "hello ",
    line: 1,
  },
  {
    kind: "operator",
    value: "+",
    line: 1,
  },
  {
    kind: "constant",
    value: 42,
    line: 1,
  },
  {
    kind: "variable",
    name: "hello",
    line: 2,
  },
  {
    kind: "arrow",
    value: "->",
    line: 2,
  },
  {
    kind: "constant",
    value: 6,
    line: 2,
  },
  {
    kind: "operator",
    value: "+",
    line: 2,
  },
  {
    kind: "constant",
    value: 42,
    line: 2,
  },
  {
    kind: "function",
    name: "main(x, y, z)",
    line: 3,
  },
];

export function RenderBlock(block_: block[] = expr_) {
  let numLines = block_[block_.length - 1].line;
  let indents = 0;
  let next = false; // Whether or not to update indents on the next iteration
  let after_next = false; // Whether or not to update indents on the next iteration

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
      {[...Array(numLines)].map((_, idx) => {
        const currTokens = block_.filter((d) => d.line === idx + 1);
        if (next) indents++;
        after_next = next;
        next = currTokens[0].type === "function";

        return (
          <div style={{ display: "flex", flexDirection: "row" }}>
            {after_next ? (
              <img style={{ height: "24px" }} src={function_arrow} />
            ) : null}
            {
              <div
                style={{
                  width: `${36 * (indents - +after_next)}px`,
                }}
              />
            }
            {RenderExpr(block_.filter((d) => d.line === idx + 1))}
          </div>
        );
      })}
    </div>
  );
}

export function RenderExpr(expr: rtl_token[]) {
  const internals = (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
      }}
    >
      {expr.slice(1).map((e: rtl_token) => {
        return (
          <>
            {e.kind === "operator" ? <div style={{ width: "6px" }} /> : null}
            {Symbol(e.kind, e.value as string)}
            {e.kind === "operator" ? <div style={{ width: "6px" }} /> : null}
          </>
        );
      })}
    </div>
  );

  return Token(expr[0].kind, expr[0].name, internals);
}
