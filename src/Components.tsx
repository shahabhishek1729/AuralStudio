import React from "react";
import { Search } from "lucide-react";

export const ErrorIcon = () => {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: "20px",
        height: "20px",
        borderRadius: "50%",
        backgroundColor: "#db4653",
      }}
    >
      <svg
        width="2800"
        height="2800"
        viewBox="0 0 24 24"
        fill="white"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path d="M12 16c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25S12.69 16 12 16zm-1-10h2v8h-2V6z" />
      </svg>
    </div>
  );
};

export const Circle = () => {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: "20px",
        height: "20px",
        borderRadius: "50%",
        backgroundColor: "#21b06b",
      }}
    />
  );
};

export const CheckmarkIcon = () => {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: "20px",
        height: "20px",
        borderRadius: "50%",
        backgroundColor: "#21b06b",
      }}
    >
      <svg
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="white"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path d="M9 16.2l-4.2-4.2L3 13.8l6 6 12-12-1.8-1.8L9 16.2z" />
      </svg>
    </div>
  );
};

export const MessageIcon = () => {
  return (
    <svg
      width="16"
      height="20"
      viewBox="0 0 24 24"
      fill="gray"
      xmlns="http://www.w3.org/2000/svg"
    >
      {/* Speech Bubble */}
      <path d="M20 2H4c-1.1 0-2 .9-2 2v16l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM4 14V4h16v10H6l-2 2z" />
    </svg>
  );
};

export const SearchBar = () => {
  return (
    <div
      style={{
        position: "relative",
        width: "100%",
      }}
    >
      <Search
        style={{
          position: "absolute",
          top: "50%",
          left: "16px",
          transform: "translateY(-50%)",
          width: "16px",
          height: "16px",
          color: "#FFF",
          pointerEvents: "none",
        }}
      />
      <input
        type="text"
        placeholder="Search"
        style={{
          display: "flex",
          width: "80%",
          marginLeft: "5px",
          justifyContent: "center",
          padding: "10px 12px 10px 34px", // Left padding for icon
          borderRadius: "999px", // pill shape
          border: "1px solid transparent",
          fontSize: "16px",
          fontFamily: "BalooChettan",
          color: "white",
          outline: "none",
          backgroundColor: "#88888866",
          transition: "border-color 0.2s",
        }}
        onFocus={(e) => (e.currentTarget.style.borderColor = "#666")}
        onBlur={(e) => (e.currentTarget.style.borderColor = "transparent")}
      />
    </div>
  );
};

export function WindowButton(color: string, onClick: () => void) {
  const text = color === "#FF605C" ? "×" : color === "#FFBD44" ? "-" : "+";
  return (
    <div
      className="relative group h-4 w-4 rounded-full flex items-center justify-center"
      style={{ background: color, cursor: "default" }}
      onClick={onClick}
    >
      <span className="absolute inset-0 flex items-center justify-center text-black font-mono text-[16px] opacity-0 group-hover:opacity-100 transition-opacity duration-200">
        {text}
      </span>
    </div>
  );
}
