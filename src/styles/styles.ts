import { CSSProperties } from "react";

export const FLEX_COL: CSSProperties = { display: "flex", flexDirection: "column" };
export const FLEX_ROW: CSSProperties = { display: "flex", flexDirection: "row" };
export const ROW_STYLE: CSSProperties = {
  marginTop: "100px",
  marginBottom: "100px",
  display: "flex",
  gap: "30px",
  justifyContent: "center",
};

export const BORDER_ANIMATION: CSSProperties = {
	position: "absolute",
	border: "2px solid #EEEEFFAA",
	width: "100%",
	borderRadius: "25px",
	padding: "1px",
	pointerEvents: "none",
	boxSizing: "content-box",
	display: "flex",
	transition: "transform 0.4s ease, width 0.4s ease, height 0.4s ease",
	// boxShadow: "0 0 40px 0px #f7dc28",
	zIndex: "2",
}

export const BUTTON_STYLE: CSSProperties = {
    height: "3rem",
    width: "fit-content",
    borderRadius: "15px",
    justifyContent: "center", //Centered vertically
    alignItems: "center", //Centered horizontally
    paddingLeft: "2rem",
    paddingRight: "2rem",
	fontWeight: "500",
    fontFamily: "Onest",
	cursor: "pointer",
	...FLEX_ROW
}
