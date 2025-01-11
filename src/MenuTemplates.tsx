import { invoke } from "@tauri-apps/api/tauri";
import {
  RenderBoolean,
  RenderCall,
  RenderIdent,
  RenderList,
  RenderNothing,
  RenderNumber,
  RenderOperator,
  RenderPending,
  Token,
} from "./PieceRenderer";
import { getColor } from "./utils";

export function TokenMenu({ onClick }) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "20rem",
        rowGap: "0.5rem",
        padding: "10px",
        borderRadius: "10px",
        background: "#181818",
        boxShadow: "0px 0px 5px black",
        border: "2px solid #f7dc28",
      }}
    >
      <TokenTemplate shortcut={"⏎"} onClick={onClick} />
      <TokenTemplate shortcut={"v"} onClick={onClick} />
      <TokenTemplate shortcut={"r"} onClick={onClick} />
      <TokenTemplate shortcut={"o"} onClick={onClick} />
      <TokenTemplate shortcut={"f"} onClick={onClick} />
      <TokenTemplate shortcut={"w"} onClick={onClick} />
      <TokenTemplate shortcut={"i"} onClick={onClick} />
    </div>
  );
}

export function ValueMenu() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "15rem",
        border: "1px solid gray",
        rowGap: "0.5rem",
        paddingTop: "10px",
        paddingBottom: "10px",
        borderRadius: "10px",
      }}
    >
      <ValueTemplate shortcut={"v"} />
      <ValueTemplate shortcut={"n"} />
      <ValueTemplate shortcut={"t"} />
      <ValueTemplate shortcut={"f"} />
      <ValueTemplate shortcut={"␣"} />
      <ValueTemplate shortcut={"l"} />
      <ValueTemplate shortcut={"c"} />
    </div>
  );
}

export function OpMenu() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "5rem",
        border: "1px solid gray",
        rowGap: "0.5rem",
        paddingTop: "10px",
        paddingBottom: "10px",
        borderRadius: "10px",
      }}
    >
      <OpTemplate shortcut={"p"} />
      <OpTemplate shortcut={"m"} />
      <OpTemplate shortcut={"t"} />
      <OpTemplate shortcut={"d"} />
      <OpTemplate shortcut={"r"} />
      <OpTemplate shortcut={"e"} />
      <OpTemplate shortcut={"g"} />
      <OpTemplate shortcut={"l"} />
      <OpTemplate shortcut={"x"} />
      <OpTemplate shortcut={"s"} />
    </div>
  );
}

function TokenTemplate({ shortcut, onClick }) {
  let template;
  switch (shortcut) {
    case "⏎":
      template = <FunctionTemplate />;
      break;
    case "v":
      template = <VariableTemplate />;
      break;
    case "r":
      template = <ReturnTemplate />;
      break;
    case "o":
      template = <OutputTemplate />;
      break;
    case "f":
      template = <ForTemplate />;
      break;
    case "w":
      template = <WhileTemplate />;
      break;
    case "i":
      template = <IfTemplate />;
      break;
    default:
      throw new Error(`Couldn't find function for key '${shortcut}'`);
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        paddingLeft: "0.5rem",
        paddingRight: "0.5rem",
        alignItems: "center",
        justifyContent: "space-between",
        cursor: "pointer",
      }}
      onClick={() => {
        window.dispatchEvent(new KeyboardEvent("keyup", { key: shortcut }));
        onClick();
      }}
    >
      {template}
      <span style={{ fontFamily: "JetBrains Mono", color: "#AAA" }}>
        {shortcut}
      </span>
    </div>
  );
}

function FunctionTemplate() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Token
        token_type={"function"}
        puzzle_color={getColor({ IDENT: "..." })}
        first={true}
        indent={0}
      />
      <RenderIdent
        pieces={[]}
        i={0}
        name={"name"}
        chained={true}
        selected={false}
        parentAddr={"0"}
        pieceIx={0}
      />
      <RenderIdent
        pieces={[]}
        i={1}
        name={"parameters"}
        chained={false}
        selected={false}
        parentAddr={"0"}
        pieceIx={0}
      />
    </div>
  );
}

function VariableTemplate() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Token
        token_type={"variable"}
        puzzle_color={getColor({ IDENT: "..." })}
        first={true}
        indent={0}
      />
      <RenderIdent
        pieces={[]}
        i={0}
        name={"name"}
        chained={true}
        selected={false}
        parentAddr={"0"}
        pieceIx={0}
      />
      <RenderOperator op_name={"ASSN"} selected={false} />
      <RenderPending chained={false} selected={false} kind={"PendingVal"} />
    </div>
  );
}

function ReturnTemplate() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Token
        token_type={"return"}
        puzzle_color={"black"}
        first={true}
        indent={0}
      />
      <RenderPending chained={true} selected={false} kind={"PendingVal"} />
    </div>
  );
}

function OutputTemplate() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Token
        token_type={"output"}
        puzzle_color={"black"}
        first={true}
        indent={0}
      />
      <RenderPending chained={true} selected={false} kind={"PendingVal"} />
    </div>
  );
}

function ForTemplate() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Token
        token_type={"for"}
        puzzle_color={getColor({ IDENT: "" })}
        first={true}
        indent={0}
      />
      <RenderIdent
        pieces={[]}
        i={0}
        name={"name"}
        chained={true}
        selected={false}
        parentAddr={"0"}
        pieceIx={0}
      />
      <RenderOperator op_name={"IN"} selected={false} />
      <RenderPending chained={false} selected={false} kind={"PendingVal"} />
    </div>
  );
}

function WhileTemplate() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Token
        token_type={"while"}
        puzzle_color={"black"}
        first={true}
        indent={0}
      />
      <RenderPending chained={false} selected={false} kind={"PendingVal"} />
    </div>
  );
}

function IfTemplate() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Token
        token_type={"conditional"}
        puzzle_color={"black"}
        first={true}
        indent={0}
      />
      <RenderPending chained={false} selected={false} kind={"PendingVal"} />
    </div>
  );
}

function ValueTemplate({ shortcut }) {
  let template;
  switch (shortcut) {
    case "v":
      template = (
        <RenderIdent
          pieces={[]}
          i={0}
          name={"name"}
          chained={false}
          selected={false}
          parentAddr={"0"}
          pieceIx={0}
        />
      );
      break;
    case "n":
      template = (
        <RenderNumber
          num={0}
          pieceIx={0}
          chained={false}
          selected={false}
          parentAddr={"0"}
        />
      );
      break;
    case "t":
      template = <RenderBoolean bool={true} chained={false} selected={false} />;
      break;
    case "f":
      template = (
        <RenderBoolean bool={false} chained={false} selected={false} />
      );
      break;
    case "␣":
      template = <RenderNothing chained={false} selected={false} />;
      break;
    case "l":
      template = (
        <RenderList
          pieces={[{ IDENT: "list" }, "PendingVal"]}
          chained={false}
          myIx={[0]}
          pieceIx={[-1]}
          parentAddr={"0"}
        />
      );
      break;
    case "c":
      template = (
        <RenderCall
          pieces={[{ IDENT: "name" }, "PendingVal"]}
          chained={false}
          myIx={[0]}
          pieceIx={[-1]}
          parentAddr={"0"}
        />
      );
      break;
    default:
      throw new Error(`Couldn't find value for key '${shortcut}'`);
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        paddingLeft: "0.5rem",
        paddingRight: "0.5rem",
        alignItems: "center",
        justifyContent: "space-between",
      }}
    >
      {template}
      <span style={{ fontFamily: "JetBrains Mono", color: "#AAA" }}>
        {shortcut}
      </span>
    </div>
  );
}

function OpTemplate({ shortcut }) {
  let template;
  switch (shortcut) {
    case "p":
      template = <RenderOperator op_name={"ADD"} selected={false} />;
      break;
    case "m":
      template = <RenderOperator op_name={"SUB"} selected={false} />;
      break;
    case "t":
      template = <RenderOperator op_name={"MUL"} selected={false} />;
      break;
    case "d":
      template = <RenderOperator op_name={"DIV"} selected={false} />;
      break;
    case "r":
      template = <RenderOperator op_name={"MOD"} selected={false} />;
      break;
    case "g":
      template = <RenderOperator op_name={"GT"} selected={false} />;
      break;
    case "l":
      template = <RenderOperator op_name={"LT"} selected={false} />;
      break;
    case "x":
      template = <RenderOperator op_name={"GE"} selected={false} />;
      break;
    case "s":
      template = <RenderOperator op_name={"LE"} selected={false} />;
      break;
    case "e":
      template = <RenderOperator op_name={"EQ"} selected={false} />;
      break;
    default:
      throw new Error(`Couldn't find value for key '${shortcut}'`);
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        paddingLeft: "0.5rem",
        paddingRight: "0.5rem",
        alignItems: "center",
        justifyContent: "space-between",
      }}
    >
      {template}
      <span style={{ fontFamily: "JetBrains Mono", color: "#AAA" }}>
        {shortcut}
      </span>
    </div>
  );
}
