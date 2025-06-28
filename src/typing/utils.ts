/**
 * Utility functions for working with digraph pieces in the AuralStudio project.
 *
 * These functions help extract and navigate pieces (tokens, values, etc.) in the digraph structure.
 *
 * Usage:
 *   import { pieceAtIx, extractPieceType, extractPieceValue } from './typing/utils';
 */

import type { RTLPiece } from './digraph';

/**
 * Returns the piece at a given index path in a nested RTLPiece array.
 * Throws an error if the path is invalid.
 *
 * @param pieces The array of RTLPiece to search
 * @param index The index path (array of indices)
 * @returns The RTLPiece at the given path
 */
export function pieceAtIx(pieces: RTLPiece[], index: number[] | null): RTLPiece {
  for (const i of (index ?? [])) {
    switch (extractPieceType(pieces[i])) {
      case "FNCALL":
        pieces = extractPieceValue(pieces[i]) as RTLPiece[];
        break;
      case "LIST":
        pieces = extractPieceValue(pieces[i]) as RTLPiece[];
        break;
      default:
        return pieces[i];
    }
  }
  throw new Error(`Invalid piece address ${index}`);
}

/**
 * Returns a string describing the type of the given RTLPiece.
 *
 * @param piece The RTLPiece to describe
 * @returns A string representing the type of the piece
 */
export function extractPieceType(piece: RTLPiece): string {
  if (piece === "NOTHING" || piece === "PendingVal" || piece === "PendingOp") return piece;
  const types = ["IDENT", "NUMBER", "OP", "TEXT", "BOOL", "FNCALL", "LIST"];
  for (const type_ of types) {
    if (type_ in piece) return type_;
  }
  throw new Error(`Invalid piece found: ${piece}`);
}

/**
 * Returns the value stored in a given RTLPiece, if any.
 *
 * @param piece The RTLPiece to extract the value from
 * @returns The value (string or RTLPiece[]), or undefined if not present
 */
export function extractPieceValue(piece: RTLPiece): string | undefined | RTLPiece[] {
  if (piece === "NOTHING" || piece === "PendingOp" || piece === "PendingVal") return undefined;
  switch (extractPieceType(piece)) {
    case "IDENT": return piece.IDENT;
    case "NUMBER": return `${piece.NUMBER}`;
    case "OP": return piece.OP;
    case "TEXT": return piece.TEXT;
    case "BOOL": return `${piece.BOOL}`;
    case "FNCALL": return piece.FNCALL;
    case "LIST": return piece.LIST;
  }
  return undefined;
} 