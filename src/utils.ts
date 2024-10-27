import { SYMBOL_MAP } from "./PieceRenderer";
import { RTLPiece, extractPieceType, symbol_metadata } from "./types";

export function getColor(piece: RTLPiece): string | undefined {
  if (piece === "NOTHING") return undefined;

  const kind = extractPieceType(piece);
  if (kind === "PendingVal" || kind === "PendingOp") 
	  return SYMBOL_MAP[kind as keyof symbol_metadata][0];
  else 
	  return SYMBOL_MAP[kind.toLowerCase() as keyof symbol_metadata][0];
} 

export function arrayEquals(a: number[], b: number[]) {
    return a.length === b.length &&
        a.every((val, index) => val === b[index]);
}
