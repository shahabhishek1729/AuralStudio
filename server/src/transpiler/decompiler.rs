use std::cmp;

use crate::error::{
    MiscellaneousSE, NegativeIndentsSE, PoorlyFormattedSE, SyntaxError, TokenOutOfBoundsSE,
    TokenOutOfLineSE, UnequalArgsSE, UnexpectedTokenSE, UnimplementedSE, UnsupportedFeatureSE,
};
use crate::prelude::*;
use crate::scanner::rtl_token::RTLToken;
use crate::scanner::scanner::{Scanner, Token};
use crate::transpiler::formatter::PyFormatter;

/// Similar to the lookahead macro in the `Scanner` class, this
/// allows us to peek at the next tokens in sequence without
/// consuming them as advance would. If only a shared reference
/// to `self` is passed, 1 is used as the default value to look-
/// ahead by. Otherwise, the passed in value is used.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate rattlesnake;
/// use rattlesnake::transpiler::decompiler::Decompiler;
///
/// let decompiler = Decompiler::new("print string hello world over").expect("Failed to create
/// decompiler");
/// // Returns the next token in sequence without consuming it
/// let next = lookahead_!(decompiler);
/// // Returns the 3rd token from now without consuming it
/// let third_next = lookahead_!(decompiler, 3);
/// ```
macro_rules! lookahead_ {
    ($self:expr) => {
        $self.lookahead_(1)
    };
    ($self:expr, $n:expr) => {
        $self.lookahead_($n)
    };
}

/// Checks if two tokens are equal, given their index from the
/// current position. Returns a `SyntaxError`, returning
/// an `Err(String)` if the two tokens are not equal and `Ok(())`
/// otherwise. ALl macro invocations require a shared reference
/// to `self`, and the token to compare against (`b`). If another
/// integer `a` is passed, a lookahead of `a` units is first
/// performed to get the token being compared.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate rattlesnake;
/// use rattlesnake::prelude::*;
/// use rattlesnake::transpiler::decompiler::Decompiler;
/// use rattlesnake::scanner::rtl_token::RTLToken;
/// use rattlesnake::error::*;
///
/// let mut decompiler = Decompiler::new("print string hello world over")
///     .expect("Failed to create decompiler from source");
///
/// // Since only two argumenets are provided, the first token in sequence will be used
/// token_eq!(decompiler, RTLToken::PrintToken)?; // Will return Ok(())
/// // The following two examples lookahead by 1 token before comparing
/// // token_eq!(decompiler, RTLToken::PrintToken)?; // Will return Err("Tokens not matched")
/// dbg!(decompiler.curr);
/// token_eq!(decompiler, 2, RTLToken::StringVal)?; // Will return Ok(())
/// // An equivalent way to achieve the above:
/// // let _ = decompiler.advance_(); // Consumes the PrintToken
/// // token_eq!(decompiler, RTLToken::PrintToken)?; // Will return Err("Tokens not matched")
/// // token_eq!(decompiler, RTLToken::StringVal)?; // Will return Ok(())
/// # Ok::<(), Box<dyn SyntaxError>>(())
/// ```
#[macro_export]
macro_rules! token_eq {
    ($self: expr, $a:expr, $b:expr, $c:expr) => {
        // Lookahead by `a` tokens
        if $self.tokens.len() <= $self.curr + $a - 1 {
            RESULT::from(Err(Box::new(TokenOutOfBoundsSE::new(
                $self.tokens[$self.curr - 1].clone(),
                $b,
                $a,
            ))))
        } else if $self.tokens[$self.curr - 1].line != $self.tokens[$self.curr + $a - 1].line {
            RESULT::from(Err(Box::new(TokenOutOfLineSE::new(
                $self.tokens[$self.curr - 1].clone(),
                $b,
                $a,
            ))))
        } else if (&[$b, $c]).contains(&$self.tokens[$self.curr + $a - 1].rtl_token) {
            RESULT::from(Ok(()))
        } else {
            RESULT::from(Err(Box::new(UnexpectedTokenSE::new(
                $self.tokens[$self.curr + $a - 1].clone(),
                $b,
            ))))
        }
    };
    ($self: expr, $a:expr, $b:expr) => {
        // Lookahead by `a` tokens
        if $self.tokens.len() <= $self.curr + $a - 1 {
            RESULT::from(Err(Box::new(TokenOutOfBoundsSE::new(
                $self.tokens[$self.curr - 1].clone(),
                $b,
                $a,
            ))))
        } else if $self.tokens[$self.curr].line != $self.tokens[$self.curr + $a - 1].line {
            RESULT::from(Err(Box::new(TokenOutOfLineSE::new(
                $self.tokens[$self.curr - 1].clone(),
                $b,
                $a,
            ))))
        } else if $self.tokens[$self.curr + $a - 1].rtl_token == $b {
            RESULT::from(Ok(()))
        } else {
            RESULT::from(Err(Box::new(UnexpectedTokenSE::new(
                $self.tokens[$self.curr + $a - 1].clone(),
                $b,
            ))))
        }
    };
    ($self:expr, $b:expr) => {
        if $self.tokens[$self.curr].rtl_token == $b {
            RESULT::from(Ok(()))
        } else {
            RESULT::from(Err(Box::new(UnexpectedTokenSE::new(
                $self.tokens[$self.curr].clone(),
                $b,
            ))))
        }
    };
}

///
pub const N_SPACES_INDENT: usize = 4usize;

/// A transpiler that translates Rattle scripts into Python.
///
/// A `Decompiler` is useful for taking a Rattle source file, applying
/// a scanner to break down the file into its base tokens, and using
/// those tokens to build up a Python source file that can then be
/// executed by the user.
///
/// The `Decompiler` will return Errors if syntax errors are found in
/// the Rattle program, but errors that would be caught by a Python
/// compiler are ignored, and left to the Python interpreter of the
/// user's choosing.
///
/// The only methods from this struct that are accessible to the user
/// are `Decompiler::new` and `Decompiler::decompile`, whereas all
/// other methods are private.
///
/// # Examples
/// ```
/// use rattlesnake::transpiler::decompiler::Decompiler;
///
/// let source = "print string hello world over";
/// let mut decompiler = Decompiler::new(source).expect("Failed to create decompiler from source.");
/// decompiler.decompile();
/// assert_eq!(decompiler.py, "print(\"hello world\")")
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Decompiler {
    /// The output of our Scanner (the list of tokens found in our program sequentially)
    pub tokens: Vec<Token>,
    /// The transpiled Python equivalent of the `tokens` in our program.
    pub py: String,
    /// The token index from which we start (set to 0 initially)
    pub start: usize,
    /// The token index we are currently processing
    pub curr: usize,
    _line: usize,
    /// The depth of indentation levels we have reached in the transpiled Python
    pub idnt: i8,
}

impl Decompiler {
    /// Allows us to create a new Decompiler with the following defaults:
    /// `tokens`: set to the result of running `Scanner::scan()` on our input source file.
    /// `start`: Set to 0, since that is where we begin parsing from.
    /// `curr`: Set to 0, since we must begin at `start`.
    /// `_line`: Set to 1, since decompilation always begins at line 1.
    /// `idnt`: Set to 0, since a brand new program has no indentation.
    /// `py`: `String::new()`, since we haven't generated any Python yet.
    pub fn new(source: &str) -> Result<Self, String> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan()?;

        Ok(Self {
            tokens,
            start: 0,
            curr: 0,
            _line: 1,
            idnt: 0,
            py: String::new(),
        })
    }

    /// Runs the Decompiler, and is the primary function in this primitive
    pub fn decompile(&mut self) -> Result<(), String> {
        let mut errors: Vec<String> = vec![];

        // while self.curr < self.tokens.len() {
        while self.tokens[self.curr].rtl_token != RTLToken::EOF {
            self.start = self.curr;
            match self.decompile_line_() {
                Ok(_) => {}
                Err(se) => {
                    errors.push(format!("Line {}: {}", se.get_line_num(), se.get_msg()));
                    break;
                }
            }
        }

        // If we have errors, we'll join them into a single string separated
        // by newlines. (UPDATE: only one error will be shown at a time, to
        // prevent related errors from clogging up the workspace.)
        if errors.len() > 0 {
            let joined = errors
                .iter()
                .fold(String::new(), |acc, e| format!("{}\n{}", acc, e));
            return Err(joined);
        }

        Ok(())
    }

    /// Decompiles a single line of Rattle code, and returns
    /// a Result depending on whether the syntax of that line
    /// was valid or not. It reutnrs `Ok(())` if the syntax was
    /// valid and `Err<Box<dyn SyntaxError>>` otherwise.
    pub fn decompile_line_(&mut self) -> RESULT {
        let next = &lookahead_!(self).unwrap();

        match next.rtl_token {
            RTLToken::VarIdentifier => self.decompile_vars_()?,
            RTLToken::IfIdentifier => self.decompile_if_()?,
            RTLToken::ElseIdentifier => self.decompile_else_()?,
            RTLToken::WhileIdentifier => self.decompile_while_()?,
            RTLToken::ForIdentifier => self.decompile_for_()?,
            RTLToken::AssertIdentifier => self.decompile_asserts_()?,
            RTLToken::FunctionIdentifier => self.decompile_fn_()?,
            RTLToken::ReturnIdentifier => self.decompile_returns_()?,
            RTLToken::PrintToken => self.decompile_prints_()?,
            RTLToken::FnCallIdentifier => {
                let call = self.decompile_calls_(false)?;
                self.push_str(&call);
            }
            RTLToken::ClassIdentifier => self.decompile_class_()?,
            RTLToken::ImportIdentifier => self.decompile_package_()?,
            RTLToken::NumericVal => self.push_str(&self.tokens[self.curr].unwrap_numeric().fmt()),
            RTLToken::StringVal => self.push_str(&self.tokens[self.curr].unwrap_string().fmt()),
            RTLToken::BooleanVal => self.push_str(&self.tokens[self.curr].unwrap_bool().fmt()),
            RTLToken::ListVal => {
                let list = self.decompile_lists(false)?;
                self.push_str(&list);
            }
            RTLToken::TupleVal => {
                let tup = self.decompile_tuples(false)?;
                self.push_str(&tup);
            }
            RTLToken::DictVal => {
                let dict = self.decompile_dicts(false)?;
                self.push_str(&dict);
            }
            RTLToken::DotOperator => self.push_str("."),
            RTLToken::AddOperation => self.push_str(" + "),
            RTLToken::SubOperation => self.push_str(" - "),
            RTLToken::MulOperation => self.push_str(" * "),
            RTLToken::DivOperation => self.push_str(" / "),
            RTLToken::ModOperation => self.push_str(" % "),
            RTLToken::IntDivOperation => self.push_str(" // "),
            RTLToken::AndLogical => self.push_str(" and "),
            RTLToken::OrLogical => self.push_str(" or "),
            RTLToken::NotLogical => self.push_str(" not "),
            RTLToken::Unpack => self.push_str("*"),
            RTLToken::EqComparator => self.push_str(" == "),
            RTLToken::NeComparator => self.push_str(" != "),
            RTLToken::GtComparator => self.push_str(" > "),
            RTLToken::GtEqComparator => self.push_str(" >= "),
            RTLToken::LtComparator => self.push_str(" <= "),
            RTLToken::LtEqComparator => self.push_str(" <= "),
            RTLToken::IdxOperator => {
                self.parse_idx(true, false)?;
            }
            RTLToken::ObjIdentifier => self.push_str(&self.tokens[self.curr].unwrap_identifier()),
            RTLToken::LineBreak => {
                let n_indents = cmp::max(self.idnt, 0);
                if n_indents < 0 {
                    return Err(Box::new(NegativeIndentsSE::new(next.line)));
                }
                let full_indent = " ".repeat(N_SPACES_INDENT).repeat(n_indents as usize);
                self.push_str(&format!("\n{}", full_indent))
            }
            RTLToken::BlockEnd => {
                self.dedent_();
            }
            RTLToken::EOF => {}
            RTLToken::LambdaIdentifier => {
                return Err(Box::new(UnsupportedFeatureSE::new(
                    RTLToken::LambdaIdentifier,
                    next.line,
                )));
            }
            RTLToken::NonlocalIdentifier => {
                return Err(Box::new(UnsupportedFeatureSE::new(
                    RTLToken::NonlocalIdentifier,
                    next.line,
                )));
            }
            RTLToken::GlobalIdentifier => {
                return Err(Box::new(UnsupportedFeatureSE::new(
                    RTLToken::GlobalIdentifier,
                    next.line,
                )));
            }
            RTLToken::FromIdentifier => {
                return Err(Box::new(UnsupportedFeatureSE::new(
                    RTLToken::FromIdentifier,
                    next.line,
                )));
            }
            RTLToken::PENDING => {
                if self.curr > 0 && Self::resolves_to_val_(&self.tokens[self.curr - 1]) {
                    return Err(Box::new(UnimplementedSE::new(next.line)));
                }
                self.push_str("...");
            }
            _ => {
                return Err(Box::new(MiscellaneousSE::new(
                    format!(
                        "Found a token that could not be handled. The token was a {}",
                        next.rtl_token
                    ),
                    next.line,
                )));
            }
        }

        let _ = self.advance_();

        Ok(())
    }

    fn parse_idx(&mut self, inplace: bool, rec: bool) -> RESVAL<Option<String>> {
        let curr_line = self.tokens[self.curr].line;
        let next_op = self.advance_();

        if next_op.line != curr_line {
            return Err(Box::new(PoorlyFormattedSE::new(
                        "index".into(),
                        Some("After using the at keyword, specify the index that you would like to retrieve from your list".into()),
                        self._line,
                    )));
        }

        let idx_expr = if next_op.unwrap_identifier() == "result" {
            self.decompile_calls_(false)?
        } else {
            if next_op.literal.is_none() {
                return Err(Box::new(PoorlyFormattedSE::new(
                            "index".into(),
                            Some("I expected to find a value after the \"at\" keyword, but didn't. Make sure to specify the index you wish to retrieve, or if you intended to use a more complex expression, make sure you wrap it in a result".into()),
                            self._line,
                        )));
            }
            format!("{}", next_op.literal.unwrap())
        };

        let idx_expr = format!(
            "{}",
            match lookahead_!(self, 2) {
                Some(next_op2) if next_op2.rtl_token == RTLToken::SliceOperator => {
                    let _ = self.advance_(); // Moves to the SliceOperator
                    format!(
                        "[{}:{}]",
                        idx_expr,
                        self.parse_idx(false, true)?
                            .expect("returns a value when inplace is false")
                    )
                }
                _ if rec => format!("{}", idx_expr),
                _ => format!("[{}]", idx_expr),
            }
        );

        if inplace {
            self.push_str(&idx_expr);
            Ok(None)
        } else {
            Ok(Some(idx_expr))
        }
    }

    /// Moves one token forward and consumes the current token
    pub fn advance_(&mut self) -> Token {
        if self.curr + 1 < self.tokens.len() {
            self.curr += 1;
            let c = self.tokens[self.curr].to_owned();
            return c;
        }
        return Token::new(RTLToken::EOF, "".into(), None, 1);
    }

    /// Peeks at the next token without consuming the current token.
    /// This function is meant to be used by the lookahead_!() macro
    /// and so should not be called directly. Please use the macro
    /// instead.
    pub fn lookahead_(&self, n: usize) -> Option<Token> {
        if self.curr + n >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.curr + n - 1].to_owned())
        }
    }

    /// Decompiles function definitions
    /// Rattle: function main x y open ... function over
    /// Python: def main(x, y):
    pub fn decompile_fn_(&mut self) -> RESULT {
        // The first token should be a FunctionIdentifier (since this function
        // should have no other entry points)
        token_eq!(self, RTLToken::FunctionIdentifier)?;
        let next = self.advance_();
        // The name of the function itself
        token_eq!(self, RTLToken::ObjIdentifier)?;

        // Extract the name from the token
        let fn_name = next.unwrap_identifier();

        let next_a = lookahead_!(self, 2);
        if next_a.is_none() {
            // return Err(String::from(
            //     "Poorly formatted function signature".into(),
            // ));
            return Err(Box::new(PoorlyFormattedSE::new(
                String::from("function signature"),
                None,
                next.line,
            )));
        }

        let next = next_a.unwrap();

        // let next = self.advance_();
        match next.rtl_token {
            // The function takes no parameters
            RTLToken::LineBreak => {
                // Python functions -> def name():
                self.push_str(&format!("def {}():", fn_name));
                self.indent_();
            }
            // The function does take parameters
            RTLToken::ObjIdentifier => {
                let mut args: Vec<String> = vec![]; // Stores the list of parameters
                let mut next = next;

                while next.rtl_token != RTLToken::LineBreak {
                    self.advance_();
                    // Another parameter found
                    let t = next.unwrap_identifier();
                    args.push(t);

                    if lookahead_!(self, 2).is_none() {
                        break;
                    }

                    next = lookahead_!(self, 2).unwrap();
                }

                // Python functions -> def name(args):
                self.push_str(&format!("def {}({}):", fn_name, args.join(", ")));
                self.indent_();
            }
            _ => {
                return Err(Box::new(PoorlyFormattedSE::new(
                    String::from("function signature"),
                    None,
                    next.line,
                )));
            }
        }

        Ok(())
    }

    /// Decompiles return statements
    /// Rattle: return numeric three over
    /// Python return 3
    pub fn decompile_returns_(&mut self) -> RESULT {
        token_eq!(self, RTLToken::ReturnIdentifier)?;
        let _ = self.advance_();
        let rexpr = self.decompile_expr(false)?;

        self.push_str(&format!("return {}", rexpr));
        Ok(())
    }

    /// Decompiles variable declarations
    /// Rattle: variable snake hello world over equals numeric two over
    /// Python: hello_world = 2
    pub fn decompile_vars_(&mut self) -> RESULT {
        // The first token should be a VarIdentifier (since this function
        // should have no other entry points)
        token_eq!(self, RTLToken::VarIdentifier)?;
        let next = self.advance_();

        // The name of the variable itself
        token_eq!(self, RTLToken::ObjIdentifier)?;
        // Extract the name from the token
        let var_name = self.chained_identifiers_(next)?;

        let mut next = self.advance_();

        self.push_str(&var_name);

        if next.rtl_token == RTLToken::IdxOperator {
            let _ = self.parse_idx(true, false)?;
            next = self.advance_();
        }

        if next.rtl_token == RTLToken::ObjIdentifier
            && next.literal.unwrap().unwrap_identifier() == "b"
        {
            // This means the ASR engine heard a "b" instead of a "be", but there is no syntax
            // error on the user's part
        }
        // This should be followed by an equals sign
        else {
            token_eq!(self, RTLToken::AssnEq)?;
        }

        self.push_str(" = ");

        Ok(())
    }

    /// Decompiles if statements
    /// Rattle: if x equals to numeric zero over open ... if over
    /// Python if x == 0:
    pub fn decompile_if_(&mut self) -> RESULT {
        token_eq!(self, RTLToken::IfIdentifier)?;
        let _ = self.advance_();
        let cond = self.decompile_expr(false)?;

        self.push_str(&format!("if {}:", cond));
        self.indent_();

        Ok(())
    }

    /// Decompiles else statements
    /// Rattle: else open ... else over
    /// Python else:
    pub fn decompile_else_(&mut self) -> RESULT {
        token_eq!(self, RTLToken::ElseIdentifier)?;

        self.push_str(&String::from("else:"));
        self.indent_();
        Ok(())
    }

    /// Decompiles while loops
    /// Rattle: while i less than numeric ten over open ... while over
    /// Python: while i < 10:
    pub fn decompile_while_(&mut self) -> RESULT {
        token_eq!(self, RTLToken::WhileIdentifier)?;
        let _ = self.advance_();
        let cond = self.decompile_expr(false)?;

        self.push_str(&format!("while {}:", cond));
        self.indent_();
        Ok(())
    }

    /// Decompiles for loops
    /// Rattle: for i in list open ... for over
    /// Python: for i in list:
    pub fn decompile_for_(&mut self) -> RESULT {
        token_eq!(self, RTLToken::ForIdentifier)?;
        let _ = self.advance_();
        let cond = self.decompile_expr(false)?;

        self.push_str(&format!("for {}:", cond));
        self.indent_();
        Ok(())
    }

    /// Decompiles for loops
    /// Rattle: for i in list open ... for over
    /// Python: for i in list:
    pub fn decompile_asserts_(&mut self) -> RESULT {
        token_eq!(self, RTLToken::AssertIdentifier)?;
        let _ = self.advance_();
        let cond = self.decompile_expr(false)?;

        self.push_str(&format!("assert {}", cond));
        Ok(())
    }

    /// Decompiles print statements
    /// Rattle: print string hello world over
    /// Python: print("hello world")
    pub fn decompile_prints_(&mut self) -> RESULT {
        // The first token should be a PrintToken (since this function
        // should have no other entry points)
        token_eq!(self, RTLToken::PrintToken)?;

        let _ = self.advance_();

        let expr = self.decompile_expr(false)?;

        self.push_str(&format!("print({})", expr));

        Ok(())
    }

    /// Decompiles class declarations
    /// Rattle: class dog open ... class over
    /// Python: class Dog:
    pub fn decompile_class_(&mut self) -> RESULT {
        // The first token should be a ClassIdentifier (since this function
        // should have no other entry points)
        token_eq!(self, RTLToken::ClassIdentifier)?;
        let next = self.advance_();
        // The name of the function itself
        token_eq!(self, RTLToken::ObjIdentifier)?;

        // Extract the name from the token
        let cname = next.unwrap_identifier();

        // let next = self.advance_();
        let next = lookahead_!(self, 2).unwrap();
        match next.rtl_token {
            // The class has no parent
            RTLToken::LineBreak => {
                // Python classes -> class Name:
                self.push_str(&format!("class {}:", cname));
                self.indent_();
            }
            // The class has a parent
            RTLToken::ObjIdentifier => {
                let next = self.advance_();
                // Extract the name of the super class from the token
                let superc = next.unwrap_identifier();
                // Python classes -> class Name(Super):
                self.push_str(&format!("class {}({}):", cname, superc));
                self.indent_();
            }
            _ => {
                // return Err("Poorly formatted class declaration".into());
                return Err(Box::new(PoorlyFormattedSE::new(
                    String::from("class declaration"),
                    None,
                    next.line,
                )));
            }
        }

        Ok(())
    }

    /// Decompiles import/package statements
    /// Rattle: package pandas alias pd
    /// Python: import pandas as pd
    pub fn decompile_package_(&mut self) -> RESULT {
        token_eq!(self, RTLToken::ImportIdentifier)?;

        let next = self.advance_();
        let pkg_name = self.chained_identifiers_(next)?;

        match token_eq!(self, 2, RTLToken::AliasIdentifier) {
            Ok(_) => {
                let _ = self.advance_();
                let pkg_alias = self.advance_();
                token_eq!(self, RTLToken::ObjIdentifier)?;

                self.push_str(&format!(
                    "import {} as {}",
                    pkg_name,
                    pkg_alias.unwrap_identifier()
                ))
            }
            Err(_) => {
                self.push_str(&format!("import {}", pkg_name));
            }
        }

        Ok(())
    }

    /// Decompiles function calls in Python
    /// Rattle: call main x numerc two over over
    /// Python: main(x, 2)
    pub fn decompile_calls_(&mut self, in_expr: bool) -> Result<String, Box<dyn SyntaxError>> {
        // The first token should be a FnCallIdentifier (since this function
        // should have no other entry points)
        if in_expr {
            self.advance_();
        }
        token_eq!(self, RTLToken::FnCallIdentifier)?;
        let next = self.advance_();
        // Extract the name of the function to be called (the "callee")
        let mut fn_name = if next.rtl_token == RTLToken::PrintToken {
            // print instead of output
            String::from("print")
        } else {
            self.chained_identifiers_(next)?
        };

        if fn_name == "quantity" {
            fn_name = String::from("");
        }

        // TODO: Expr functions (function_generator()(a, b))
        // TODO: Handle special function names

        let mut next = self.advance_();

        let mut args = vec![];
        while next.rtl_token != RTLToken::ExprEnd {
            let arg = self.decompile_expr(true)?;
            args.push(arg);
            next = self.advance_();
        }

        if (fn_name == "both" || fn_name == "either") && args.len() <= 1 {
            return Err(Box::new(UnequalArgsSE::new(
                fn_name,
                "2 or more".into(),
                1usize,
                self._line,
            )));
        }

        if (fn_name == "result" || fn_name == "inverse") && args.len() > 1 {
            return Err(Box::new(UnequalArgsSE::new(
                fn_name,
                "1".into(),
                args.len(),
                self._line,
            )));
        }

        if (fn_name == "increase" || fn_name == "decrease") && args.len() != 2 {
            return Err(Box::new(UnequalArgsSE::new(
                fn_name,
                "2".into(),
                args.len(),
                self._line,
            )));
        }

        return match &fn_name[..] {
            "both" => Ok(args.join(" and ")),
            "either" => Ok(args.join(" or ")),
            "inverse" => Ok(format!("not {}", args.join(""))),
            "result" => Ok(format!("({})", args.join(""))),
            "increase" => Ok(args.join(" += ")),
            "decrease" => Ok(args.join(" -= ")),
            _ => Ok(format!("{}({})", fn_name, args.join(", "))),
        };
    }

    /// Decompiles list declarations
    /// Rattle: list numeric one over, numeric two over, numeric three over over
    /// Python: [1, 2, 3]
    pub fn decompile_lists(&mut self, in_expr: bool) -> Result<String, Box<dyn SyntaxError>> {
        if in_expr {
            self.advance_();
        }
        token_eq!(self, RTLToken::ListVal)?;
        self.advance_();
        let args = self.decompile_args()?;
        Ok(format!("[{}]", args))
    }

    /// Decompiles tuple declarations
    /// Rattle: tuple numeric one over, numeric two over, numeric three over over
    /// Python: (1, 2, 3)
    pub fn decompile_tuples(&mut self, in_expr: bool) -> Result<String, Box<dyn SyntaxError>> {
        if in_expr {
            self.advance_();
        }
        token_eq!(self, RTLToken::TupleVal)?;
        self.advance_();
        let args = self.decompile_args()?;
        Ok(format!("({})", args))
    }

    /// Decompiles dictionary declarations
    /// Rattle: dictionary string hello over numeric one over over
    /// Python: { "hello"; 1 }
    pub fn decompile_dicts(&mut self, in_expr: bool) -> Result<String, Box<dyn SyntaxError>> {
        if in_expr {
            self.advance_();
        }
        token_eq!(self, RTLToken::DictVal)?;
        self.advance_();

        let mut args: Vec<String> = vec![];
        let mut next = self.tokens[self.curr].to_owned();
        while next.rtl_token != RTLToken::ExprEnd {
            let arg = self.decompile_expr(true)?;
            args.push(arg);
            let _ = self.advance_();
            next = self.tokens[self.curr].to_owned();
        }

        if args.len() % 2 == 1 {
            return Err(Box::new(PoorlyFormattedSE::new(
                String::from("dictionary"),
                None,
                next.line,
            )));
            // return Err("Poorly formatted dictionary. Every key should have a value".into());
        }

        let mut dict = String::from("{");
        let mut i = 0;
        while i < args.len() {
            dict.push_str(&&format!("{}: {}, ", args[i], args[i + 1]));
            i += 2;
        }

        Ok(format!("{}}}", &dict[..dict.len() - 2]))
    }

    /// Decompiles generic list of arguments. Args are a list of expressions
    /// separated by commas.
    /// Rattle: x y numeric two over numeric three over add numeric four over z
    /// Python: x, y, 2, 3 + 4, z
    pub fn decompile_args(&mut self) -> Result<String, Box<dyn SyntaxError>> {
        let mut args: Vec<String> = vec![];
        let mut next = self.tokens[self.curr].to_owned();
        while next.rtl_token != RTLToken::ExprEnd {
            let arg = self.decompile_expr(true)?;
            args.push(arg);
            let _ = self.advance_();
            next = self.tokens[self.curr].to_owned();
        }

        Ok(args.join(", "))
    }

    fn chained_identifiers_(&mut self, mut next: Token) -> Result<String, Box<dyn SyntaxError>> {
        let name1 = if next.rtl_token == RTLToken::ObjIdentifier {
            next.unwrap_identifier()
        } else {
            next.lexeme
        };

        let mut names = vec![name1];

        let mut _next2 = lookahead_!(self, 2);
        let mut i = 0;

        while _next2 != None && _next2.clone().unwrap().rtl_token == RTLToken::DotOperator {
            let _ = self.advance_();
            // while next.rtl_token == RTLToken::DotOperator {
            i += 1;
            if i > 100 {
                // In this case, we probably have an infinite loop
                break;
            }
            next = self.advance_();
            token_eq!(self, 1, RTLToken::ObjIdentifier, RTLToken::RawSequence)?;
            let name_x = next.unwrap_identifier();
            names.push(name_x);
            // next = self.advance_();
            _next2 = lookahead_!(self, 2);
        }

        let _x = lookahead_!(self);

        Ok(names.join("."))
    }

    /// Decompiles expressions (i.e., anything that can be on the RHS of an assignment.
    pub fn decompile_expr(&mut self, from_call: bool) -> Result<String, Box<dyn SyntaxError>> {
        let prev = lookahead_!(self);
        if prev.is_none() {
            return Err(Box::new(PoorlyFormattedSE::new(
                String::from("expression"),
                None,
                0,
            )));
        }
        let mut prev = prev.unwrap();

        if !(Self::resolves_to_val_(&prev) || self.is_unary_()) {
            return Err(Box::new(PoorlyFormattedSE::new(
                String::from("expression"),
                None,
                prev.line,
            )));
        }

        let mut expr = String::new();

        expr.push_str(&&match prev.rtl_token {
            RTLToken::NotLogical => String::from("not "),
            RTLToken::Unpack => String::from("*"),
            RTLToken::ObjIdentifier | RTLToken::RawSequence => {
                self.chained_identifiers_(prev.clone())?
            }
            RTLToken::NumericVal => prev.unwrap_numeric().fmt(),
            RTLToken::StringVal => prev.unwrap_string().fmt(),
            RTLToken::BooleanVal => prev.unwrap_bool().fmt(),
            RTLToken::ListVal => self.decompile_lists(false)?,
            RTLToken::TupleVal => self.decompile_tuples(false)?,
            RTLToken::FnCallIdentifier => self.decompile_calls_(false)?,
            RTLToken::ExprEnd => "".into(),
            _ => return Err(Box::new(MiscellaneousSE::new(String::from(""), prev.line))),
        });

        let mut broken = false;

        // let _ = self.advance_();
        for _ in 0..usize::MAX {
            let curr_a = lookahead_!(self, 2);
            if curr_a.is_none() {
                break;
            }
            let curr = curr_a.unwrap();

            if from_call
                && (curr.rtl_token == RTLToken::ExprEnd || prev.rtl_token == RTLToken::ExprEnd)
            {
                broken = true;
                break;
            }

            if Self::resolves_to_val_(&prev) {
                match curr.rtl_token {
                    RTLToken::AddOperation => {
                        expr.push_str(" + ");
                    }
                    RTLToken::SubOperation => {
                        expr.push_str(" - ");
                    }
                    RTLToken::MulOperation => {
                        expr.push_str(" * ");
                    }
                    RTLToken::ModOperation => {
                        expr.push_str(" % ");
                    }
                    RTLToken::DivOperation => {
                        expr.push_str(" / ");
                    }
                    RTLToken::IntDivOperation => {
                        expr.push_str(" // ");
                    }
                    RTLToken::AndLogical => {
                        expr.push_str(" and ");
                    }
                    RTLToken::OrLogical => {
                        expr.push_str(" or ");
                    }
                    RTLToken::EqComparator => {
                        expr.push_str(" == ");
                    }
                    RTLToken::NeComparator => {
                        expr.push_str(" != ");
                    }
                    RTLToken::GtComparator => {
                        expr.push_str(" > ");
                    }
                    RTLToken::GtEqComparator => {
                        expr.push_str(" >= ");
                    }
                    RTLToken::LtComparator => {
                        expr.push_str(" < ");
                    }
                    RTLToken::LtEqComparator => {
                        expr.push_str(" <= ");
                    }
                    RTLToken::MembershipOperator => {
                        expr.push_str(" in ");
                    }
                    RTLToken::IdentityOperator => {
                        expr.push_str(" is ");
                    }
                    RTLToken::AssnEq => {
                        expr.push_str(" = ");
                    }
                    RTLToken::ExprEnd => {}
                    RTLToken::IdxOperator => {
                        let _ = self.advance_();
                        expr.push_str(&self.parse_idx(false, false)?.unwrap_or(String::new()));
                        broken = true;
                        break;
                    }
                    RTLToken::PENDING => {
                        return Err(Box::new(UnimplementedSE {
                            line_num: self._line,
                        }))
                    }
                    _ => {
                        broken = true;
                        break;
                    }
                }
            } else {
                assert!(Self::resolves_to_op_(&prev));
                match curr.rtl_token {
                    RTLToken::NotLogical => {
                        expr.push_str(&&String::from("not "));
                    }
                    RTLToken::Unpack => {
                        expr.push_str(&&String::from("*"));
                    }
                    RTLToken::ObjIdentifier => {
                        expr.push_str(&&curr.unwrap_identifier());
                    }
                    RTLToken::NumericVal => {
                        expr.push_str(&&curr.unwrap_numeric().fmt());
                    }
                    RTLToken::StringVal => {
                        expr.push_str(&&curr.unwrap_string().fmt());
                    }
                    RTLToken::BooleanVal => {
                        expr.push_str(&&curr.unwrap_bool().fmt());
                    }
                    RTLToken::ListVal => expr.push_str(&&self.decompile_lists(true)?),
                    RTLToken::TupleVal => expr.push_str(&&self.decompile_tuples(true)?),
                    RTLToken::FnCallIdentifier => expr.push_str(&&self.decompile_calls_(true)?),
                    _ => {
                        broken = true;
                        break;
                    }
                }
            }
            prev = curr;
            let _ = self.advance_();
        }

        if !broken {
            let _ = self.advance_();
        }

        Ok(expr)
    }

    fn is_unary_(&self) -> bool {
        let curr_ = self.tokens[self.curr].rtl_token;
        let next_ = lookahead_!(self, 2);
        (curr_ == RTLToken::NotLogical || curr_ == RTLToken::Unpack)
            && next_.is_some()
            && Self::resolves_to_val_(&next_.unwrap())
    }

    fn resolves_to_val_(t: &Token) -> bool {
        vec![
            RTLToken::ExprEnd,
            RTLToken::ObjIdentifier,
            RTLToken::RawSequence,
            RTLToken::NumericVal,
            RTLToken::BooleanVal,
            RTLToken::StringVal,
            RTLToken::NoneVal,
            RTLToken::FnCallIdentifier,
            RTLToken::FnCallIdentifier,
            RTLToken::ListVal,
            RTLToken::TupleVal,
            RTLToken::DictVal,
        ]
        .contains(&t.rtl_token)
    }

    fn resolves_to_op_(t: &Token) -> bool {
        vec![
            RTLToken::AddOperation,
            RTLToken::SubOperation,
            RTLToken::MulOperation,
            RTLToken::DivOperation,
            RTLToken::ModOperation,
            RTLToken::IntDivOperation,
            RTLToken::Unpack,
            RTLToken::AndLogical,
            RTLToken::OrLogical,
            RTLToken::NotLogical,
            RTLToken::EqComparator,
            RTLToken::NeComparator,
            RTLToken::GtComparator,
            RTLToken::GtEqComparator,
            RTLToken::LtComparator,
            RTLToken::LtEqComparator,
            RTLToken::MembershipOperator,
            RTLToken::IdentityOperator,
            RTLToken::IdxOperator,
            RTLToken::AssnEq,
        ]
        .contains(&t.rtl_token)
    }

    fn push_str(&mut self, line: &str) {
        self.py.push_str(&line);
    }

    fn indent_(&mut self) {
        self.idnt += 1;
    }

    fn dedent_(&mut self) {
        self.idnt -= 1;
    }
}
