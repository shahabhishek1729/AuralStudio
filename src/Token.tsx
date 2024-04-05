import gear from "./assets/action_cpu.png";
import question from "./assets/conditional_q4.png";
import cube from "./assets/value_box.png";
import book from "./assets/vuesax/bold/book.png";
import rattle_icon from "./assets/rattle_icon.png";
import output from "./assets/output_icon.png";

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
  output: ["#7E8E1C", output],
};

// export function TokenTree() {
//   return (
//     <div
//       style={{
//         display: "flex",
//         width: "100%",
//         flexDirection: "column",
//         alignItems: "center",
//       }}
//     >
//       <FileToken fname="helloworld.rattle" />
//       <div style={{ height: "30px" }} />
//       <div
//         style={{
//           display: "flex",
//           width: "100%",
//           flexDirection: "row",
//           justifyContent: "space-between",
//         }}
//       >
//         <Token token_type={"library"} metadata={{ name: "numpy" }} />
//         <Token token_type={"library"} metadata={{ name: "pandas" }} />
//         <Token token_type={"variable"} metadata={{ name: "global" }} />
//         <Token token_type={"function"} metadata={{ name: "execute1()" }} />
//         <Token token_type={"function"} metadata={{ name: "execute2()" }} />
//       </div>
//     </div>
//   );
// }

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

export function Token(token_type: string, name: string) {
  return (
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
        paddingRight: "15px",
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
    </div>
  );
}

interface symbol_metadata {
  constant: [string, string, number, number];
  arrow: [string, string, number, number];
  operator: [string, string, number, number];
}

const SYMBOL_MAP: symbol_metadata = {
  constant: ["#FF8A00", "#FFFFFF", 16, 40],
  arrow: ["transparent", "#FFCD4E", 20, 30],
  operator: ["#701490", "#FFFFFF", 20, 30],
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

interface rtl_token {
  type: string;
  value?: string | number | boolean;
  line: number;
}
const render_line: rtl_token[] = [
  {
    type: "output",
    value: undefined,
    line: 1,
  },
  {
    type: "string",
    value: "hello ",
    line: 1,
  },
  {
    type: "operator",
    value: "+",
    line: 1,
  },
  {
    type: "number",
    value: 42,
    line: 1,
  },
];
export function RenderExpr(expr: string) {
  // TODO: Implement
  expr;
}
