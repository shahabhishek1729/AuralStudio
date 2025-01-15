export type IDisplay = "HOME" | "PROJECTS" | "EDITOR";

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
// All but "NOTHING" and Pending* store internal data.
export type RTLPiece = "NOTHING" | "PendingVal" | "PendingOp" | _PieceInterface;
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
	blockLoc: Address,
	// The coerced location (always on an node)
	nodeLoc: Address,
	// See `ADMode` below
	mode: ADMode,
	// The piece we are currently editing (if any)
	pieceIx: number[] | null,
	// The output to be displayed in the output panel
	output: string | null,
}

// Either we are viewing or editing the digraph; if editing, we might be expecting a certain piece
type ADMode = "VIEW" | "TYPE" | { "EDIT": _ExpectingPiece };
/**
 * The kind of piece we're expecting to see next in the digraph.
 * When pieces are pending, there are three possibilities of the next piece we're expecting
 * ExprPiece: A piece that would be part of an expression (e.g., literals, operators) – unenforced
 * IdentPiece: A variable or package name - enforced
 * AnyPiece: We don't know what we're expecting next, so open to any piece
*/
export type _ExpectingPiece = "ExprPiece" | "Token";

type Address = string; // Addresses are stored as IPv4-style strings in JSON 

export function pieceAtIx(pieces: RTLPiece[], index: number[] | null) {
	for (const i of (index ?? [])) {
		switch (extractPieceType(pieces[i])) {
			case "FNCALL":
				pieces = extractPieceValue(pieces[i]) as RTLPiece[];
				break;
			case "LIST":
				pieces = extractPieceValue(pieces[i]) as RTLPiece[];
				break;
			default:
				return pieces[i]
		}
	}

	throw new Error(`Invalid piece address ${index}`);
}

/**
 * Turns a `RTLPiece` into a `string` concisely describing the kind of piece we 
 * dealing with.
 * @param piece The piece to be described as a string
 * @returns {string} A string-ified form of the piece
 */
export function extractPieceType(piece: RTLPiece): string {
	if (piece === "NOTHING" || piece === "PendingVal" || piece === "PendingOp") return piece;

	const types = ["IDENT", "NUMBER", "OP", "TEXT", "BOOL", "FNCALL", "LIST"];
	for (const type_ of types) {
		if (type_ in piece) return type_;
	}
	throw new Error(`Invalid piece found: ${piece}`);
}

export function extractPieceValue(piece: RTLPiece): string | undefined | RTLPiece[] {
	if (piece === "NOTHING" || piece === "PendingOp" || piece === "PendingVal") return undefined;
	switch (extractPieceType(piece)) {
		case "IDENT": return piece.IDENT
		case "NUMBER": return `${piece.NUMBER}`
		case "OP": return piece.OP
		case "TEXT": return piece.TEXT
		case "BOOL": return `${piece.BOOL}`
		case "FNCALL": return piece.FNCALL
		case "LIST": return piece.LIST
	}
	return undefined;
}

export interface symbol_metadata {
  constant: [string, string, number, number];
  arrow: [string, string, number, number];
  operator: [string, string, number, number];
  text: [string, string, number, number];
  ident: [string, string, number, number];
  call: [string, string, number, number];
  PendingVal: [string, string, number, number];
  PendingOp: [string, string, number, number];
}

export interface token_metadata {
  file: [string, string];
  function: [string, string];
  variable: [string, string];
  conditional: [string, string];
  yes: [string, string];
  no: [string, string];
  for: [string, string];
  while: [string, string];
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
  AT: string;
}

