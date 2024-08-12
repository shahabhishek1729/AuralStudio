export interface RTLNode {
	line: number,
	children: RTLNode[],
	kind: string,
	pieces: RTLPiece[],
	address: string,
	parent?: string,
}

export type RTLPiece = "NOTHING" | _PieceInterface;
interface _PieceInterface {
	IDENT?: string,
	NUMBER?: number,
	TEXT?: string,
	BOOL?: boolean,
	OP?: string,
	FNCALL?: RTLPiece[],
	LIST?: RTLPiece[],
}

export function extractPieceType(piece: RTLPiece): string {
	if (piece === "NOTHING") return piece;
	const types = ["IDENT", "NUMBER", "OP", "TEXT", "BOOL", "FNCALL", "LIST"];
	for (const type_ of types) {
		if (type_ in piece) return type_;
	}
	throw new Error(`Invalid piece found: ${piece}`);
}

export interface symbol_metadata {
  constant: [string, string, number, number];
  arrow: [string, string, number, number];
  operator: [string, string, number, number];
  text: [string, string, number, number];
  ident: [string, string, number, number];
  call: [string, string, number, number];
}

