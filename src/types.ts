import { NodeTypes } from "reactflow";
import { RTLNodeComponent, FileNodeComponent } from "./components/Nodes";

export type IDisplay = "HOME" | "PROJECTS" | "EDITOR";
export type RDisplay = "FLOAT" | "PANEL" | "MAX";

// Custom node types
export const nodeTypes: NodeTypes = {
  rtlNode: RTLNodeComponent,
  fileNode: FileNodeComponent,
};


export interface RTLNode {
	line: number,
	children: RTLNode[],
	kind: string,
	pieces: RTLPiece[],
	address: string,
	parent?: string,
	rtl: string | null,
	note: string | null,
	err: SemanticError | null
}

interface SemanticError {
    UnreachableCode: string,
    UseBeforeDef: string,
    UnmatchedSignature: [string, number, number],
}

// There are several kinds of pieces in a digraph, as seen below.
// All but "NOTHING" and Pending* store internal data.
export type RTLPiece = "NOTHING" | "PendingVal" | "PendingOp" | _PieceInterface;
export interface _PieceInterface {
	IDENT?: string,
	NUMBER?: number,
	TEXT?: string,
	BOOL?: boolean,
	OP?: string,
	FNCALL?: RTLPiece[],
	LIST?: RTLPiece[],
}

// The state of the digraph currently, including:
export interface Canvas {
	// File currently being written to
	filename: string,
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
	// The error to be displayed in the output panel
	err: string | null,
}

// The state of the debugger
export interface Debugger {
	state: Canvas,
	call_stack: Address[],
}

export interface IDGraph {
	graph: Ident[];
}

export type Ident = IdentVar | IdentFun;
type ValidIdent = [string, string | null];
interface IdentVar {
	Var: {
		name: string;
		addr: Address;
		valid_idents: ValidIdent[];
		val: string | null;
	}
}

interface IdentFun {
	Fun: {
		name: string;
		addr: Address;
		valid_idents: ValidIdent[];
		args: string[];
		children: Ident[];
	}
}

// Either we are viewing or editing the digraph; if editing, we might be expecting a certain piece
export type ADMode = "VIEW" | "TYPE" | { "EDIT": _ExpectingPiece };
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
  constant: [string, string, number, number, string];
  number: [string, string, number, number, string];
  bool: [string, string, number, number, string];
  arrow: [string, string, number, number, string];
  operator: [string, string, number, number, string];
  text: [string, string, number, number, string];
  ident: [string, string, number, number, string];
  fncall: [string, string, number, number, string];
  list: [string, string, number, number, string];
  PendingVal: [string, string, number, number, string];
  PendingOp: [string, string, number, number, string];
}

export interface token_metadata {
  file: [string, string, string];
  function: [string, null, string];
  variable: [string, null, string];
  conditional: [string, null, string];
  yes: [string, null, string];
  no: [string, null, string];
  for: [string, null, string];
  while: [string, null, string];
  library: [string, null, string];
  output: [string, null, string];
  return: [string, null, string];
  list: [string, null, string];
  pending: [string, null, string];
  continue: [string, null, string];
  break: [string, null, string];
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

