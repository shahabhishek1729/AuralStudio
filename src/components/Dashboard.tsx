import { FLEX_ROW, FLEX_COL, BUTTON_STYLE } from "../styles/styles";

export default function Dashboard() {
  return (
    <div
      style={{
        display: "flex",
        width: "100vw",
        height: "100vh",
        backgroundImage: `url("./src/assets/DashboardBackground.png")`,
        backgroundSize: "cover",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div
        style={{
          width: "70vw",
          height: "60vh",
          background: `linear-gradient(135deg, #FFFFFF66, #FFFFFF33)`,
          animation: "popUp 0.8s ease-out",
          borderRadius: "80px",
          alignItems: "center",
          justifyContent: "space-around",
          boxShadow: "0px 10px 20px #1A414F1C",
          ...FLEX_ROW,
        }}
      >
        <div style={{ ...FLEX_COL, gap: "0px" }}>
          <p
            style={{
              fontFamily: "Onest",
              fontWeight: "bold",
              height: "fit-content",
              fontSize: "3.25vw",
              color: "#2679C5",
              margin: "0px",
              padding: "0px",
            }}
          >
            Welcome to AuralStudio
          </p>

          <p
            style={{
              fontFamily: "Onest",
              height: "fit-content",
              color: "#2679C5",
              fontSize: "1.5vw",
              padding: "10px 0px 0px 4px",
            }}
          >
            What would you like to work on?
          </p>

          <div style={{ ...FLEX_ROW, gap: "1rem", paddingTop: "3rem" }}>
            <span
              style={{
                background: `linear-gradient(45deg, #5C89FD, #00D1FF)`,
                ...BUTTON_STYLE,
                fontSize: "1.3vw",
              }}
              onClick={() => {
                window.dispatchEvent(new KeyboardEvent("keyup", { key: "n" }));
              }}
            >
              New Project
            </span>
            <span
              style={{
                background: "white",
                ...BUTTON_STYLE,
                color: "black",
                fontSize: "1.3vw",
              }}
              onClick={() => {
                window.dispatchEvent(new KeyboardEvent("keyup", { key: "o" }));
              }}
            >
              Open Project
            </span>
          </div>
        </div>
        <div
          id="img_logo"
          style={{
            display: "flex",
            width: "40vmin",
            height: "40vmin",
            maxWidth: "28rem",
            maxHeight: "28rem",
            backgroundImage: `url("./src/assets/DashboardLogo.png")`,
            backgroundSize: "cover",
            alignItems: "center",
            justifyContent: "center",
          }}
        />
      </div>
    </div>
  );
}
