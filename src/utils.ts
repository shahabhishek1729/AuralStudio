import { SYMBOL_MAP } from "./PieceRenderer";
import { RTLPiece, extractPieceType, symbol_metadata } from "./types";

export function getColor(piece: RTLPiece): string | undefined {
  if (piece === "NOTHING") return undefined;

  const kind = extractPieceType(piece).toLowerCase();
  return SYMBOL_MAP[kind as keyof symbol_metadata][0];
} 
