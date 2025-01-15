import { useState } from "react";
import { FLEX_ROW, FLEX_COL, BUTTON_STYLE } from "./styles";
import MenuIcon from "@mui/icons-material/Menu";

function SearchBar({ onSearch }) {
  const [searchTerm, setSearchTerm] = useState("");

  const handleInputChange = (event) => {
    setSearchTerm(event.target.value);
    onSearch(event.target.value); // Call the onSearch function with the updated search term
  };

  return (
    <input
      type="text"
      placeholder="Search projects..."
      value={searchTerm}
      onChange={handleInputChange}
      style={{
        fontFamily: "Futura",
        border: "",
        borderRadius: "40px",
        height: "1.5rem",
        background: "#444",
        alignSelf: "center",
      }}
    />
  );
}

export function Sidebar({ onClick }) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        rowGap: "0.5rem",
        padding: "10px",
        borderRadius: "10px",
        height: "100%",
        boxShadow: "0px 0px 5px black",
        alignItems: "center",
      }}
    >
      <span
        style={{
          background: `linear-gradient(45deg, #5C89FD, #00D1FF)`,
          ...BUTTON_STYLE,
        }}
      >
        Create Project
      </span>
      <span
        style={{
          background: "white",
          ...BUTTON_STYLE,
          color: "black",
          fontWeight: "normal",
        }}
      >
        Open Project
      </span>
    </div>
  );
}

export function DashboardFake() {
  return (
    <div style={{ ...FLEX_ROW, height: "100vh", width: "100vw" }}>
      <div
        id="sidebar"
        style={{ ...FLEX_COL, height: "fit-content", paddingTop: "1.2rem" }}
      >
        <MenuIcon style={{ alignSelf: "center", scale: "1.3" }} />
        <div style={{ height: "2rem" }} />
        <Sidebar
          onClick={() => {
            // TODO: handle clicks
          }}
        />
      </div>

      <div style={{ width: "2rem" }} />

      <div style={{ ...FLEX_COL }}>
        <div id="header" style={{ ...FLEX_ROW, height: "fit-content" }}>
          <div style={{ width: "5rem" }} />
          <h1 style={{ fontFamily: "Futura" }}>Dashboard</h1>
          <div style={{ width: "5rem" }} />
          <SearchBar
            onSearch={() => {
              // TODO: handle searches
            }}
          />
        </div>

        <div
          id="project_panel"
          style={{
            ...FLEX_COL,
            borderRadius: "100px",
            background: "#444455",
            height: "100vh",
            width: "70vw",
            paddingTop: "30px",
          }}
        >
          <div style={{ ...FLEX_ROW, justifyContent: "space-between" }}>
            <h2
              style={{ fontFamily: "Plus Jakarta Sans", paddingLeft: "3rem" }}
            >
              Projects
            </h2>
            <h3
              style={{
                fontFamily: "Plus Jakarta Sans",
                paddingRight: "3rem",
                alignSelf: "center",
              }}
            >
              January 2025
            </h3>
          </div>
        </div>
      </div>
    </div>
  );
}

export function Dashboard() {
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
