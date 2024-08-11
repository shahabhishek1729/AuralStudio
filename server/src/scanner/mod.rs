//! The Scanner contains all the files necessary to take in a
//! Rattle file and output a series of tokens, the kinds of
//! which are defined in the RTLToken enum, that represent the
//! syntax of the program. These tokens can then be transpiled
//! into another language (e.g., Rattle -> Python), or directly
//! deocmpiled into machine code.
///
pub mod numeric_parser;
///
pub mod rtl_token;
///
pub mod scanner;
