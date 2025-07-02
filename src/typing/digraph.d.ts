/**
 * Core digraph types for the AuralStudio project.
 *
 * These types describe the structure of the acyclic digraph, its nodes, pieces, and editing state.
 *
 * Usage:
 *   import type { RTLNode, RTLPiece, Canvas, ADMode, ... } from './typing/digraph';
 */

/**
 * Represents a semantic error attached to a node.
 */
export interface SemanticError {
  /** Code that is unreachable */
  UnreachableCode: string;
  /** Use of a variable before it is defined */
  UseBeforeDef: string;
  /** Signature mismatch: [function name, expected, actual] */
  UnmatchedSignature: [string, number, number];
}

/**
 * Nodes can be one of 14 kinds:
 * FNDEF: A function signature, including its name and arguments
 * VARDECL: A variable declaration of the form "let [name] be [expr]".
 * OUTPUT: A print statement to the console.
 * CONDTL: The condition portition of a conditional ("is [boolean]?").
 * CONDTLY: The yes branch of a conditional (analogous to if block).
 * CONDTLN: The no branch of a conditional (analogous to else block).
 * FORLOOP: A loop which iterates over an "iterable".
 * WHLLOOP: A loop which iterates until some condition is met.
 * BREAK: A statement that pulls execution out of a loop.
 * CONTINUE: A statement that moves execution to the next loop iteration.
 * RETURN: Returns a value from a function and terminates function execution.
 * FNCALL: A call to a pre-defined function, filling in required arguments.
 * GRABPKG: Analogous to an import statement, pulls in external code.
 * PENDING: A placeholder node that has yet to be filled in.
 */
export type NodeKind = "FNDEF" | "VARDECL" | "OUTPUT" | "CONDTL" | "CONDTLY" | 
	"CONDTLN" | "FORLOOP" | "WHLLOOP" | "BREAK" | "CONTINUE" | "RETURN" | 
	"FNCALL" | "GRABPKG" | "PENDING";
/**
 * Represents a node in the digraph (function, block, etc.).
 */
export interface RTLNode {
  /** Line number in the source file */
  line: number;
  /** Child nodes (functions, blocks, etc.) */
  children: RTLNode[];
  /** Kind of node (e.g., 'FNDEF', 'BLOCK', etc.) */
  kind: NodeKind;
  /** Pieces (tokens, expressions, etc.) attached to this node */
  pieces: RTLPiece[];
  /** Unique address for this node */
  address: string;
  /** Optional parent node address */
  parent?: string;
  /** Raw RTL string (if any) */
  rtl: string | null;
  /** Optional note or comment */
  note: string | null;
  /** Optional semantic error attached to this node */
  err: SemanticError | null;
}

/**
 * There are several kinds of pieces in a digraph. All but 'NOTHING' and Pending* store internal data.
 */
export type RTLPiece = "NOTHING" | "PendingVal" | "PendingOp" | _PieceInterface;

/**
 * Internal structure for a digraph piece (token, value, operator, etc.).
 */
export interface _PieceInterface {
  IDENT?: string;
  NUMBER?: number;
  TEXT?: string;
  BOOL?: boolean;
  OP?: string;
  FNCALL?: RTLPiece[];
  LIST?: RTLPiece[];
}

/**
 * The state of the digraph currently, including file, graph, location, mode, etc.
 */
export interface Canvas {
  /** File currently being written to */
  filename: string;
  /** The node graph itself */
  graph: RTLNode[];
  /** The current location (either on a node or a block) */
  blockLoc: Address;
  /** The coerced location (always on a node) */
  nodeLoc: Address;
  /** See ADMode below */
  mode: ADMode;
  /** The piece we are currently editing (if any) */
  pieceIx: number[] | null;
  /** The output to be displayed in the output panel */
  output: string | null;
  /** The error to be displayed in the output panel */
  err: string | null;
}

/**
 * The state of the debugger, including the current Canvas and call stack.
 */
export interface Debugger {
  /** Current digraph state */
  state: Canvas;
  /** Call stack (addresses) */
  call_stack: Address[];
}

/**
 * A graph of identifiers (variables, functions, etc.).
 */
export interface IDGraph {
  /** List of identifiers in the graph */
  graph: Ident[];
}

/**
 * An identifier can be a variable or a function.
 */
export type Ident = IdentVar | IdentFun;

/**
 * A valid identifier tuple: [name, optional namespace].
 */
export type ValidIdent = [string, string | null];

/**
 * Variable identifier structure.
 */
export interface IdentVar {
  Var: {
    name: string;
    addr: Address;
    valid_idents: ValidIdent[];
    val: string | null;
  };
}

/**
 * Function identifier structure.
 */
export interface IdentFun {
  Fun: {
    name: string;
    addr: Address;
    valid_idents: ValidIdent[];
    args: string[];
    children: Ident[];
  };
}

/**
 * The current mode of the digraph editor.
 * - VIEW: Viewing only
 * - TYPE: Typing
 * - EDIT: Editing a specific piece
 */
export type ADMode = "VIEW" | "TYPE" | { "EDIT": _ExpectingPiece };

/**
 * The kind of piece we're expecting to see next in the digraph.
 * - ExprPiece: A piece that would be part of an expression (e.g., literals, operators)
 * - Token: A variable or package name
 */
export type _ExpectingPiece = "ExprPiece" | "Token";

/**
 * Address type (IPv4-style string in JSON).
 */
export type Address = string;
