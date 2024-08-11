//! While none of the code in this file is used in the actual compiler itself, it serves as a
//! formal definition of the way grammar is parsed in the Rattle language, and provides the basics
//! of such a parsing implementation.
#![allow(dead_code)]
use crate::scanner::rtl_token::RTLToken;
use crate::scanner::scanner::{Literal, Token};

pub enum Primary {
    TRUE,
    FALSE,
    NONE,
    NUMERIC(f64),
    STRING(String),
    IDENTIFIER(String),
    EXPR(Expr),
}

impl Primary {
    pub fn new(s: Token) -> Self {
        match s.rtl_token {
            RTLToken::BooleanVal => {
                let b = s
                    .literal
                    .expect("Should not have a boolean without a value");
                match b {
                    Literal::RTLBoolean(true) => return Self::TRUE,
                    Literal::RTLBoolean(false) => return Self::FALSE,
                    _ => {
                        panic!("Should not have a boolean literal without a boolean value");
                    }
                }
            }
            RTLToken::NoneVal => return Self::NONE,
            RTLToken::NumericVal => {
                let n = s
                    .literal
                    .expect("Should not have a numeric without a value");
                match n {
                    Literal::RTLNumeric(n_) => return Self::NUMERIC(n_),
                    _ => {
                        panic!("Should not have a numeric literal without a numeric value");
                    }
                }
            }
            RTLToken::StringVal => {
                let n = s.literal.expect("Should not have a string without a value");
                match n {
                    Literal::RTLString(n_) => return Self::STRING(n_),
                    _ => {
                        panic!("Should not have a string literal without a string value");
                    }
                }
            }
            RTLToken::ObjIdentifier => {
                let n = s
                    .literal
                    .expect("Should not have an identifier without a parsed name");
                match n {
                    Literal::RTLIdentifier(n_) => return Self::IDENTIFIER(n_),
                    _ => {
                        panic!("Should not have an identifier without a parsed name");
                    }
                }
            }
            _ => {
                panic!("Should not be reached");
            }
        }
    }
}

// -> primary ( args? | "dot" IDENTIFIER )* "close" ;
pub struct Call {
    primary: Primary,
    chained_ids: Vec<RTLToken>,
    args: Vec<Expr>,
}

// -> "not" unary | call ;
pub struct Unary {
    unary: Box<Unary>,
    call: Call,
}

// -> unary ( ("multiply" | "divide") unary )* ;
pub struct Factor {}

// -> factor ( ("add" | "subtract") factor)* ;
pub struct Term {}

// -> term ( ("greater than" " equal to"?) | ("less than" " equal to"?) term )* ;
pub struct Cmp {}

// -> cmp ( ("not"? " equal to") cmp)* ;
pub struct Equality {}

// -> equality ("and" equality)* ;
pub struct LogicAnd {}

// -> logic_and ("or" logic_and)* ;
pub struct LogicOr {}

// -> (call "dot")? IDENTIFIER "equals" expr
//  | logic_or ;

#[derive(Debug, Clone)]
pub enum LiteralVariant {
    NumericVal(f64),
    StringVal(f64),
    TRUE,
    FALSE,
    NONE,
}

#[derive(Debug, Clone)]
pub enum Expr {
    PRIMARY { literal: LiteralVariant },
    CALL { fn_name: Box<Expr>, args: Vec<Expr> },
    GET { obj: Box<Expr> },
}

// -> IDENTIFIER params? block ;
struct Function {}

// -> iDENTIFIER IDENTIFIER* or IDENTIFIER+ ;
struct Params {}

// -> expr expr* OR expr+ ;
struct Args {}
