/**
 * Metadata types for symbols, tokens, and operators in the AuralStudio project.
 *
 * These interfaces describe the metadata structures used for symbol tables, token information,
 * and operator kinds in the digraph and static analysis logic.
 *
 * Usage:
 *   import type { symbol_metadata, token_metadata, op_kind } from './typing/metadata';
 */

/**
 * Metadata for various symbol types (constants, numbers, booleans, etc.).
 * Each entry is a tuple: [display, type, line, column, description].
 */
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

/**
 * Metadata for token types (file, function, variable, etc.).
 * Each entry is a tuple: [display, type, description].
 */
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

/**
 * Operator kind mapping for all supported operators in the language.
 * Each property is the string representation of the operator.
 */
export interface op_kind {
  ADD: string;
  SUB: string;
  MUL: string;
  DIV: string;
  INTDIV: string;
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
