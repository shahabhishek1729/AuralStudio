use crate::prelude::*;
use crate::scanner::rtl_token::RTLToken;
use regex::{Captures, Regex};
use std::collections::HashMap;

/// Allows us to expand on `lookahead_` to check if the next word in
/// the source file is equal to a certain value
#[macro_export]
macro_rules! next_eq {
    ($s: expr, $w: expr) => {
        lookahead_!($s, 1) == $w
    };
}

/// Allows us to call `lookahead_` with a default argument of `n = 1`
/// (looking one word ahead)
#[macro_export]
macro_rules! lookahead_ {
    ($s: expr, $a: expr) => {
        $s.lookahead_($a)
    };
    ($s: expr) => {
        $s.lookahead_(1)
    };
}

// Import the macros for use
pub(crate) use lookahead_;
pub(crate) use next_eq;

// The maximum length of a Rattle keyword (in this case, "greater
// than equal to" and "less than equal to" are four words each).
// pub const MAX_KW_LEN: usize = 4;

// A HashMap representing every Rattle string and its associated
// RTLToken. Tokens are processed in a longest-first manner (i.e.,
// greedy matches), where, for instance, "otherwise if" would be given
// precedence over "if".
lazy_static! {
    static ref BINDINGS: HashMap<&'static str, RTLToken> = {
        let mut m = HashMap::new();
        m.insert("output", RTLToken::PrintToken);
        m.insert("done define", RTLToken::BlockEnd);
        m.insert("done type", RTLToken::BlockEnd);
        m.insert("done otherwise if", RTLToken::BlockEnd);
        m.insert("done if", RTLToken::BlockEnd);
        m.insert("done otherwise", RTLToken::BlockEnd);
        m.insert("done for", RTLToken::BlockEnd);
        m.insert("done while", RTLToken::BlockEnd);
        m.insert("done", RTLToken::ExprEnd);
        m.insert("be", RTLToken::AssnEq);
        m.insert("star", RTLToken::Unpack);
        m.insert("plus", RTLToken::AddOperation);
        m.insert("minus", RTLToken::SubOperation);
        m.insert("times", RTLToken::MulOperation);
        m.insert("over", RTLToken::DivOperation);
        m.insert("modulo", RTLToken::ModOperation);
        m.insert("floor divide", RTLToken::IntDivOperation);
        m.insert("equals", RTLToken::EqComparator);
        m.insert("not equals", RTLToken::NeComparator);
        m.insert("greater than", RTLToken::GtComparator);
        m.insert("greater than equals", RTLToken::GtEqComparator);
        m.insert("less than", RTLToken::LtComparator);
        m.insert("less than equals", RTLToken::LtEqComparator);
        m.insert("true", RTLToken::BooleanVal);
        m.insert("false", RTLToken::BooleanVal);
        m.insert("none", RTLToken::NoneVal);
        m.insert("is", RTLToken::IdentityOperator);
        m.insert("in", RTLToken::MembershipOperator);
        m.insert("not", RTLToken::NotLogical);
        m.insert("dot", RTLToken::DotOperator);
        m.insert("define", RTLToken::FunctionIdentifier);
        m.insert("call", RTLToken::FnCallIdentifier);
        m.insert("let", RTLToken::VarIdentifier);
        m.insert("type", RTLToken::ClassIdentifier);
        m.insert("if", RTLToken::IfIdentifier);
        m.insert("otherwise if", RTLToken::ElifIdentifier);
        m.insert("otherwise", RTLToken::ElseIdentifier);
        m.insert("for", RTLToken::ForIdentifier);
        m.insert("while", RTLToken::WhileIdentifier);
        m.insert("alias", RTLToken::AliasIdentifier);
        m.insert("ensure", RTLToken::AssertIdentifier);
        m.insert("asynchronous", RTLToken::AsyncIdentifier);
        m.insert("await", RTLToken::AwaitIdentifier);
        m.insert("break", RTLToken::BreakIdentifier);
        m.insert("continue", RTLToken::ContinueIdentifier);
        m.insert("destroy", RTLToken::DelIdentifier);
        m.insert("except", RTLToken::ExceptIdentifier);
        m.insert("finally", RTLToken::FinallyIdentifier);
        m.insert("from", RTLToken::FromIdentifier);
        m.insert("global", RTLToken::GlobalIdentifier);
        m.insert("grab", RTLToken::ImportIdentifier);
        m.insert("lambda", RTLToken::LambdaIdentifier);
        m.insert("nonlocal", RTLToken::NonlocalIdentifier);
        m.insert("pass", RTLToken::PassIdentifier);
        m.insert("raise", RTLToken::RaiseIdentifier);
        m.insert("return", RTLToken::ReturnIdentifier);
        m.insert("try", RTLToken::TryIdentitifer);
        m.insert("with", RTLToken::WithIdentifier);
        m.insert("yield", RTLToken::YieldIdentifier);
        m.insert("list", RTLToken::ListVal);
        m.insert("tuple", RTLToken::TupleVal);
        m.insert("dictionary", RTLToken::DictVal);
        m.insert("at", RTLToken::IdxOperator);
        m.insert("pretend", RTLToken::PENDING);
        // m.insert("escape", RTLToken::EscapeIdentifier);
        m
    };
}

/// Given the contents of a Rattle file, lexes it into its base tokens.
///
/// While meant to be used by the `Decompiler` to parse the file down
/// into tokens and then build back a Python source file from the tokens,
/// the `Scanner` can also be a useful tool in isolation.
///
/// The only methods from this struct that are accessible to the user
/// are `Scanner::new` and `Scanner::scan`, whereas all
/// other methods are private.
///
/// # Examples
/// ```
/// use rattlesnake::scanner::scanner::{Scanner, Literal};
/// use rattlesnake::scanner::rtl_token::RTLToken;
///
/// let script: &str = "variable x equals numeric five over";
/// let mut scanner: Scanner = Scanner::new(script);
/// let tokens = scanner.scan().expect("Invalid Rattle script.");
/// // Our Scanner has now broken down the source file into its individual
/// // tokens, which can be checked as shown below.
/// assert_eq!(scanner.tokens[0].rtl_token, RTLToken::VarIdentifier);
/// assert_eq!(scanner.tokens[1].rtl_token, RTLToken::ObjIdentifier);
/// assert_eq!(
///     *scanner.tokens[1].literal.as_ref().unwrap(),
///     Literal::RTLIdentifier(String::from("x"))
/// );
/// assert_eq!(scanner.tokens[2].rtl_token, RTLToken::AssnEq);
/// assert_eq!(scanner.tokens[3].rtl_token, RTLToken::NumericVal);
/// assert_eq!(
///     *scanner.tokens[3].literal.as_ref().unwrap(),
///     Literal::RTLNumeric(5f64)
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Scanner {
    ///
    pub source: Vec<String>,
    ///
    pub tokens: Vec<Token>,
    ///
    pub start: usize,
    ///
    pub curr: usize,
    ///
    pub line: usize,
}

impl Scanner {
    /// Constructs a new instance of a scanner given the contents of
    /// a Rattle source file.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::scanner::scanner::Scanner;
    ///
    /// let script: &str = "variable x equals numeric five over";
    /// let scanner: Scanner = Scanner::new(script);
    /// // You now have a Scanner that can be accessed using the
    /// // methods below.
    /// ```
    pub fn new(source: &str) -> Self {
        let words = source
            .replace("\n", " \n ")
            .split([' '])
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Self {
            source: words,
            tokens: vec![],
            start: 0,
            curr: 0,
            line: 1,
        }
    }

    /// Scans a file and breaks it down into base tokens needed
    /// to compile the file.
    ///
    /// # Errors
    /// Returns a String message if a syntax error was detected in
    /// your Rattle script (this may include unrecognized tokens,
    /// unterminated expressions, etc.)
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::scanner::scanner::{Scanner, Token};
    ///
    /// let script: &str = "variable x equals numeric five over";
    /// let mut scanner: Scanner = Scanner::new(script);
    /// let tokens: Result<Vec<Token>, String> = scanner.scan();
    /// // The tokens can now be used as a `Vector` after unwrapping, e.g.,
    /// let first_token = tokens.as_ref().unwrap().get(0);
    /// let second_token = tokens.as_ref().unwrap().get(1);
    /// // ...
    /// ```
    pub fn scan(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        // Keeps track of whether the current token is an EscapeIdentifier,
        // which would force the next token to be a RawSequence.
        let mut flags = ScannerFlags {
            escaped: false,
            force_ident: false,
            fn_args: 0usize,
        };

        while !self.end_reached_() {
            self.start = self.curr;
            match self.scan_once_(&mut flags) {
                Ok(sf) => {
                    flags = sf;
                }
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens
            .push(Token::new(RTLToken::EOF, String::new(), None, self.line));

        // If we have errors, we'll join them into a single string separated
        // by newlines.
        if errors.len() > 0 {
            let joined = errors
                .iter()
                .fold(String::new(), |acc, e| format!("{}\n{}", acc, e));
            return Err(joined);
        }

        Ok(self.tokens.to_owned())
    }

    fn scan_once_(&mut self, flags: &mut ScannerFlags) -> Result<ScannerFlags, String> {
        let mut next = self.advance_();

        let ScannerFlags {
            escaped,
            force_ident,
            fn_args,
        } = flags;

        if *escaped {
            self.add_token_(RTLToken::RawSequence);
            *escaped = false;
            return Ok(ScannerFlags {
                escaped: *escaped,
                force_ident: false,
                fn_args: fn_args.clone(),
            });
        }

        if *force_ident {
            self.identifier_(&next, true)?;
            return Ok(ScannerFlags {
                escaped: *escaped,
                force_ident: false,
                fn_args: fn_args.clone(),
            });
        }

        while *fn_args > 0usize {
            // while !(n == "" || n == "\n" || n == " ") && !self.end_reached_() {
            if next == "and" {
                next = self.advance_();
                self.start = self.curr - 1;
                break;
            } else if next == "done" {
                self.add_token_(RTLToken::ExprEnd);
                next = self.advance_();
                self.start = self.curr;
                *fn_args -= 1;
            } else if next == "\n" || next == "" {
                *fn_args -= 1;
            } else {
                break;
            }
        }

        while next_eq!(self, "of") {
            let is_def = self.tokens.len() >= 1
                && self.tokens[self.tokens.len() - 1].rtl_token == RTLToken::FunctionIdentifier;

            if next == "tuple" {
                self.add_token_(RTLToken::TupleVal);
            } else if next == "list" {
                self.add_token_(RTLToken::ListVal);
            } else if next == "dictionary" {
                self.add_token_(RTLToken::DictVal);
            } else {
                if !is_def {
                    self.add_token_(RTLToken::FnCallIdentifier);
                }
                self.add_token_with_val_(
                    RTLToken::ObjIdentifier,
                    Some(Literal::RTLIdentifier(next)),
                );
            }

            let _ = self.advance_();
            next = self.advance_();
            self.start = self.curr - 1;
            *fn_args += 1;
        }

        match &next.replace("\r", "")[..] {
            "escape" => *escaped = true,
            "string" => self.string_()?,
            "true" | "false" => self.boolean_(&next),
            "equal" => {
                if next_eq!(self, "to") {
                    self.advance_();
                    self.add_token_(RTLToken::EqComparator);
                }
            }
            "" | " " | "\t" => {}
            "\n" => {
                self.add_token_(RTLToken::LineBreak);
                self.line += 1;
            }
            c => {
                if let Ok(number) = c.parse::<f64>() {
                    self.add_token_with_val_(
                        RTLToken::NumericVal,
                        Some(Literal::RTLNumeric(number)),
                    );
                } else {
                    self.identifier_(c, false)?;
                    if let Some(t) = self.tokens.last() {
                        match t.rtl_token {
                            RTLToken::ImportIdentifier => *force_ident = true,
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(ScannerFlags {
            escaped: *escaped,
            force_ident: *force_ident,
            fn_args: *fn_args,
        })
    }

    fn identifier_(&mut self, c: &str, force: bool) -> Result<(), String> {
        // Regex for valid variable, function and class names
        let identifier_re = Regex::new("^[a-zA-Z_][a-zA-Z_0-9]*$").unwrap();
        if identifier_re.is_match(c) {
            // The token matches a valid Python identifier name
            let mut final_is: Vec<usize> = Vec::new();
            let tokens = (0..=MAX_KW_LEN)
                .rev()
                .filter_map(|i| {
                    let (substr, final_i_) = if i == 0 {
                        (String::from(c), 0)
                    } else {
                        (
                            format!(
                                "{}{}",
                                c,
                                // Builds up a string from the current token to that `j` down the
                                // sequence.
                                // E.g., if the token being matched is "greater than equals"
                                // (length 3), the next three words in the source sequence will be
                                // examined.
                                (1..i).fold(String::new(), |acc, j| {
                                    format!("{} {}", acc, lookahead_!(self, j))
                                })
                            ),
                            i,
                        )
                    };

                    if let Some(t) = BINDINGS.get(&substr[..]) {
                        final_is.push(final_i_);
                        Some(*t)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if tokens.len() == 0 || force {
                self.add_token_with_val_(
                    RTLToken::ObjIdentifier,
                    Some(Literal::RTLIdentifier(String::from(c))),
                );
            } else {
                for _ in 0..final_is[0] - 1 {
                    self.advance_();
                }
                self.add_token_(tokens[0]);
            }
        } else {
            // The token did not match a valid Python identifier name
            return Err(format!("Unrecognized token on line {}: '{}'", self.line, c));
        }

        Ok(())
    }

    fn string_(&mut self) -> Result<(), String> {
        // let mut prev_esc;
        // let mut val = String::new();
        while !next_eq!(self, "done") && !self.end_reached_() {
            // prev_esc = false;
            if next_eq!(self, "\n") {
                self.line += 1
            } else if next_eq!(self, "escape") {
                // prev_esc = true;
            }
            self.advance_();
            // if prev_esc {}
        }

        if self.end_reached_() {
            return Err(String::from("Unterminated string"));
        }

        self.advance_();

        let val = self.source[self.start + 1..self.curr - 1].join(" ");

        // TODO: Check for escapes

        let mut s = val.clone();
        for i in 0..RTL_SYMBOLS.len() {
            let rtl_re = Regex::new(RTL_SYMBOLS[i]).unwrap();
            s = rtl_re
                .replace_all(&s, |_: &Captures<'_>| PY_SYMBOLS[i])
                .to_string();
        }

        self.add_token_with_val_(RTLToken::StringVal, Some(Literal::RTLString(s)));
        Ok(())
    }

    fn boolean_(&mut self, next: &str) {
        let boolean = &next.to_lowercase() == "true";
        assert!(boolean || &next.to_lowercase() == "false");
        self.add_token_with_val_(RTLToken::BooleanVal, Some(Literal::RTLBoolean(boolean)));
    }

    fn advance_(&mut self) -> String {
        if self.curr >= self.source.len() {
            return String::from("");
        }

        let c = &self.source[self.curr];
        self.curr += 1;
        c.to_string()
    }

    fn add_token_(&mut self, token: RTLToken) {
        self.add_token_with_val_(token, None);
    }

    fn lookahead_(&self, n: usize) -> String {
        if self.curr + (n - 1) >= self.source.len() {
            String::from("\0")
        } else {
            self.source[self.curr + n - 1].to_owned()
        }
    }

    fn add_token_with_val_(&mut self, token: RTLToken, literal: Option<Literal>) {
        let text = self.source[self.start..self.curr].join(" ");
        self.tokens
            .push(Token::new(token, text, literal, self.line));
    }

    fn end_reached_(&self) -> bool {
        self.curr >= self.source.len()
    }
}

/// Stores the basic information about a token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The class which the Token belongs to, from the `RTLToken`
    /// enum (defined below).
    pub rtl_token: RTLToken,
    /// The actual Rattle string found in the source file which
    /// corresponds to this token.
    pub lexeme: String,
    /// Certain tokens (e.g., strings, numerics, booleans,
    /// modified identifiers, etc.) are preprocessed during the
    /// parsing, the results of which are stored here.
    pub literal: Option<Literal>,
    /// The line number in which this token appears in the
    /// original Rattle source file
    pub line: usize,
}

impl Token {
    /// Creates a new instance of a Token
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::scanner::scanner::{Scanner, Literal, Token};
    /// use rattlesnake::scanner::rtl_token::RTLToken;
    ///
    /// let rtl_token: RTLToken = RTLToken::StringVal;
    /// let lexeme: String = String::from("string hello world over");
    /// let literal: Option<Literal> = Some(Literal::RTLString(String::from("hello world")));
    /// let line: usize = 1usize;
    /// let token: Token = Token::new(rtl_token, lexeme, literal, line);
    /// ```
    ///
    pub const fn new(
        rtl_token: RTLToken,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            rtl_token,
            lexeme,
            literal,
            line,
        }
    }

    ///
    pub fn unwrap_identifier(&self) -> String {
        if self.rtl_token == RTLToken::ObjIdentifier {
            self.literal
                .as_ref()
                .expect("Cannot have an ObjIdentifier without a literal value")
                .unwrap_identifier()
        } else {
            self.lexeme.clone()
        }
    }

    ///
    pub fn unwrap_numeric(&self) -> f64 {
        self.literal
            .as_ref()
            .expect("Cannot have an ObjIdentifier without a literal value")
            .unwrap_numeric()
    }

    ///
    pub fn unwrap_string(&self) -> String {
        self.literal
            .as_ref()
            .expect("Cannot have an ObjIdentifier without a literal value")
            .unwrap_string()
    }

    ///
    pub fn unwrap_bool(&self) -> bool {
        self.literal
            .as_ref()
            .expect("Cannot have an ObjIdentifier without a literal value")
            .unwrap_bool()
    }
}

/// Stores values which were altered/parsed during tokenization.
///
/// This includes extracting the string itself from  `string ... over` expressions,
/// parsing floats from `numeric ... over` expressions, extracting true or false values from
/// Rattle booleans and applying modifiers (collapse, camel, pascal & snake) on identifiers when
/// requested.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Extracts numeric values from a `numeric ... over` expression
    RTLNumeric(f64),
    /// Extracts a string from a `string ... over` expression
    RTLString(String),
    /// Extracts a Rust boolean from a Rattle boolean
    RTLBoolean(bool),
    /// Applies requested modifiers to generate a valid Python identifier
    RTLIdentifier(String),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            Self::RTLNumeric(s) => s.to_string(),
            Self::RTLString(s) => s.to_string(),
            Self::RTLBoolean(s) => s.to_string(),
            Self::RTLIdentifier(s) => s.to_string(),
        };
        write!(f, "{}", inner)
    }
}

impl Literal {
    ///
    pub fn unwrap_identifier(&self) -> String {
        if let Literal::RTLIdentifier(s) = self {
            return String::from(s);
        } else {
            assert!(
                false,
                "Should not run unwrap_identifier() on any Literal besides an RTLIdentifier"
            );
            return String::new();
        }
    }

    ///
    pub fn unwrap_numeric(&self) -> f64 {
        if let Literal::RTLNumeric(s) = self {
            *s
        } else {
            assert!(
                false,
                "Should not run unwrap_identifier() on any Literal besides an RTLIdentifier"
            );
            0f64
        }
    }

    ///
    pub fn unwrap_string(&self) -> String {
        if let Literal::RTLString(s) = self {
            String::from(s)
        } else {
            assert!(
                false,
                "Should not run unwrap_identifier() on any Literal besides an RTLIdentifier"
            );
            String::new()
        }
    }

    ///
    pub fn unwrap_bool(&self) -> bool {
        if let Literal::RTLBoolean(s) = self {
            *s
        } else {
            assert!(
                false,
                "Should not run unwrap_identifier() on any Literal besides an RTLIdentifier"
            );
            false
        }
    }
}

struct ScannerFlags {
    escaped: bool,
    force_ident: bool,
    fn_args: usize,
}
