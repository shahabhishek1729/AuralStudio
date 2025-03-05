import React from "react";

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
