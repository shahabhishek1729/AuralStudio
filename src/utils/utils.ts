import { SYMBOL_MAP } from "../PieceRenderer";
import { RTLPiece } from "../typing/digraph";
import { extractPieceType } from "../typing/utils";
import type { symbol_metadata } from "../typing/metadata";

export function getColor(piece: RTLPiece): string {
  if (piece === "NOTHING") return "";

  const kind = extractPieceType(piece);
  if (kind === "PendingVal" || kind === "PendingOp") 
	  return SYMBOL_MAP[kind as keyof symbol_metadata][0];
  else 
	  return SYMBOL_MAP[kind.toLowerCase() as keyof symbol_metadata][0];
} 

export function arrEq(a: number[], b: number[]) {
    return a.length === b.length &&
        a.every((val, index) => val === b[index]);
}
