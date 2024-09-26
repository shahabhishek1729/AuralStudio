export interface RTLNode {
	line: number,
	children: RTLNode[],
	kind: string,
	pieces: RTLPiece[],
	address: string,
	parent?: string,
	rtl: string | null,
}

// There are several kinds of pieces in a digraph, as seen below.
// All but "NOTHING" and "PENDING" store internal data.
export type RTLPiece = "NOTHING" | "PENDING" | _PieceInterface;
interface _PieceInterface {
	IDENT?: string,
	NUMBER?: number,
	TEXT?: string,
	BOOL?: boolean,
	OP?: string,
	FNCALL?: RTLPiece[],
	LIST?: RTLPiece[],
}

// The state of the digraph currently, including:
export interface CursorState {
	// The node graph itself
	graph: Array<RTLNode>,
	// The current location (either on an node or a block)
	block_loc: Address, 
	// The coerced location (always on an node)
	node_loc: Address,
	// See `ADMode` below
	mode: ADMode,
}

// Either we are viewing or editing the digraph; if editing, we might be expecting a certain piece
type ADMode = "VIEW" | { "EDIT": _ExpectingPiece };
/**
 * The kind of piece we're expecting to see next in the digraph.
 * When pieces are pending, there are three possibilities of the next piece we're expecting
 * ExprPiece: A piece that would be part of an expression (e.g., literals, operators) – unenforced
 * IdentPiece: A variable or package name - enforced
 * AnyPiece: We don't know what we're expecting next, so open to any piece
*/
export type _ExpectingPiece = "IdentPiece" | "ExprPiece" | "AnyPiece";

type Address = string; // Addresses are stored as IPv4-style strings in JSON 

/**
 * Turns a `RTLPiece` into a `string` concisely describing the kind of piece we 
 * dealing with.
 * @param piece The piece to be described as a string
 * @returns {string} A string-ified form of the piece
 */
export function extractPieceType(piece: RTLPiece): string {
	if (piece === "NOTHING" || piece === "PENDING") return piece;
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
  pending: [string, string, number, number];
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
  pending: [string, string];
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

