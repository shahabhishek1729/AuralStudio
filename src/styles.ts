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
	border: "2px solid #f7dc28",
	pointerEvents: "none",
	transition: "transform 0.4s ease, width 0.4s ease, height 0.4s ease",
	zIndex: "-1",
}

export function BORDER_STYLE(selected: boolean): CSSProperties {
	return {
		border: selected ? "2px solid #f7dc28" : "",
		borderRadius: "10px"
	}
}

