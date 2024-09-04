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

export interface CursorState {
	graph: Array<RTLNode>,
	block_loc: Address, 
	node_loc: Address,
	mode: ADMode,
	insert_at: Address | null,
}

export interface Editor {
	state: CursorState,
	insert_loc: Address | null,
	expecting: string | null
}

type ADMode = "VIEW" | "EDIT";
type Address = string; // Addresses are stored as IPv4-style strings in JSON 

/**
 * Turns a `RTLPiece` into a `string` concisely describing the kind of piece we 
 * dealing with.
 * @param piece The piece to be described as a string
 * @returns {string} A string-ified form of the piece
 */
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

export interface token_metadata {
  file: [string, string];
  function: [string, string];
  variable: [string, string];
  conditional: [string, string];
  yes: [string, string];
  no: [string, string];
  library: [string, string];
  output: [string, string];
  return: [string, string];
  list: [string, string];
}

export interface op_kind {
  ADD: string;
  SUB: string;
  MUL: string;
  DIV: string;
  MOD: string;
  EQ: string;
  NE: string;
  GT: string;
  LT: string;
  GE: string;
  LE: string;
  AND: string;
  OR: string;
  NOT: string;
  IN: string;
  DOT: string;
  ASSN: string;
}

