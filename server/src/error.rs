use std::fmt::Debug;

use crate::prelude::*;
use crate::scanner::rtl_token::RTLToken;
use crate::scanner::scanner::Token;
use crate::transpiler::dyn_wrapper::ToAny;

/// A generic syntax error that is specific to Rattle - certain
/// types of syntax errors may be detected in Python, but not
/// Rattle, and are not included as structs here.
pub trait SyntaxError: ToAny + Debug {
    /// Returns a String that describes the error to the user.
    /// The string should be concise and readable, to allow
    /// for quick error fixing.
    fn get_msg(&self) -> String;
    /// Returns the line number (as a `usize` because line numbers
    /// cannot be negative) of the error's occurrence. Since Rattle
    /// does not support multi-line statements, all errors should
    /// be traceable to a single line.
    fn get_line_num(&self) -> usize;
}

/// Indicates that a certain type of token was expected, but another
/// kind was found (e.g., something other than an identifier follows
/// a functio keyword)
#[derive(Debug, Clone)]
pub struct UnexpectedTokenSE {
    /// The token that was found at a specific position
    pub token_found: Token,
    /// The token the compiler expected to find at that position
    pub token_expected: RTLToken,
}

/// Used when the compiler attempts a lookahead beyond the 'end of
/// file' token.
#[derive(Debug, Clone)]
pub struct TokenOutOfBoundsSE {
    ///
    pub token_expected: RTLToken,
    ///
    pub token_start: Token,
    ///
    pub lookahead_amt: usize,
}

/// Used when the compiler attempts a lookahead that moves beyond
/// the current line - since Rattle does not support multi-line
/// statements, the expected token should also be on the current
/// line.
#[derive(Debug, Clone)]
pub struct TokenOutOfLineSE {
    /// The token the compiler expected to find at the position
    pub token_expected: RTLToken,
    /// The current token the compiler is on.
    pub token_start: Token,
    /// The amount by which the lookahead was attempted (usize
    /// because lookbehind is not supported, so the amount must
    /// be a positive integer)
    pub lookahead_amt: usize,
}

/// Returned when the compiler attempted a dedent although there
/// were already no indents.
#[derive(Debug, Clone)]
pub struct NegativeIndentsSE {
    /// The line number on which the dedent was attempted.
    pub line_num: usize,
}

/// Used when a user attempts to use a feature that is not
/// currently supported in Rattle. These features currently
/// include:
///  1. The `global` and `nonlocal` keywords in Python
///  2. The use of `from` in import statements (this can be
///     sidestepped by using `as` and aliasing the import)
///  3. Lambda/anonymous functions in Python
#[derive(Debug, Clone)]
pub struct UnsupportedFeatureSE {
    /// The feature the user attempted to use
    pub feature_token: RTLToken,
    /// The line on which this feature was used
    pub line_num: usize,
}

/// Used for function headers, class declarations, conditionals
/// and other expressions that are not formatted in a way the
/// compiler can understand.
#[derive(Debug, Clone)]
pub struct PoorlyFormattedSE {
    /// The poorly formatted expression itself
    pub expr: String,
    /// The line on which the error occurred
    pub line_num: usize,
    /// Any other useful messages to help the user determine
    /// what the issue is (and often how to resolve it)
    pub additional_msg: Option<String>,
}

/// Certain functions (such as `result`), can only take in a single
/// argument, or a set number of arguments. If the user attempts
/// to provide more of fewer arguments than this number, this error
/// is returned.
#[derive(Debug, Clone)]
pub struct UnequalArgsSE {
    /// The name of the function that was called
    pub fn_name: String,
    /// The number of arguments the function expected (this is a `String` because for certain
    /// functions, the number of args might be "2 or more", for instance)
    pub args_expected: String,
    /// The number of arguments the function received
    pub args_received: usize,
    /// The line number where the error occurred
    pub line_num: usize,
}

/// A miscellaneous error type for errors that do not fit into
/// one of the above descriptions.
#[derive(Debug, Clone)]
pub struct MiscellaneousSE {
    /// The message to share with the user
    pub msg: String,
    /// The line number on which the error occurerd
    pub line_num: usize,
}

#[derive(Debug, Clone)]
pub struct UnimplementedSE {
    pub line_num: usize,
}

impl UnexpectedTokenSE {
    /// Creates a new instance of an UnexpectedTokenSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::UnexpectedTokenSE;
    /// use rattlesnake::scanner::rtl_token::RTLToken;
    /// use rattlesnake::prelude::*;
    ///
    /// let error = UnexpectedTokenSE::new(GENERIC_BLANK_TOKEN, RTLToken::FunctionIdentifier);
    /// ````
    pub fn new(token_found: Token, token_expected: RTLToken) -> Self {
        Self {
            token_found,
            token_expected,
        }
    }
}

impl From<Result<(), Box<dyn SyntaxError>>> for UnexpectedTokenSE {
    fn from(value: Result<(), Box<dyn SyntaxError>>) -> Self {
        match value {
            Ok(_) => return Self::new(GENERIC_BLANK_TOKEN, RTLToken::EOF),
            Err(r) => {
                let b = &*r.as_any();
                match b.downcast_ref::<Self>() {
                    Some(i) => return i.clone(),
                    _ => return Self::new(GENERIC_BLANK_TOKEN, RTLToken::EOF),
                }
            }
        }
    }
}

impl SyntaxError for UnexpectedTokenSE {
    fn get_msg(&self) -> String {
        format!(
            "I found a {} but I expected a {}. Your tokens don't match.",
            self.token_found.rtl_token, self.token_expected
        )
    }

    fn get_line_num(&self) -> usize {
        self.token_found.line
    }
}

impl TokenOutOfBoundsSE {
    /// Creates a new instance of a TokenOutOfBoundsSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::TokenOutOfBoundsSE;
    /// use rattlesnake::scanner::rtl_token::RTLToken;
    /// use rattlesnake::prelude::*;
    ///
    /// let error = TokenOutOfBoundsSE::new(GENERIC_BLANK_TOKEN, RTLToken::FromIdentifier, 1usize);
    /// ````
    pub fn new(token_start: Token, token_expected: RTLToken, lookahead_amt: usize) -> Self {
        Self {
            token_expected,
            token_start,
            lookahead_amt,
        }
    }
}

impl From<Result<(), Box<dyn SyntaxError>>> for TokenOutOfBoundsSE {
    fn from(value: Result<(), Box<dyn SyntaxError>>) -> Self {
        match value {
            Ok(_) => return Self::new(GENERIC_BLANK_TOKEN, RTLToken::EOF, 0),
            Err(r) => {
                let b = &*r.as_any();
                match b.downcast_ref::<Self>() {
                    Some(i) => return i.clone(),
                    _ => return Self::new(GENERIC_BLANK_TOKEN, RTLToken::EOF, 0),
                }
            }
        }
    }
}

impl SyntaxError for TokenOutOfBoundsSE {
    fn get_msg(&self) -> String {
        format!(
            "I expected to see a {} {} tokens later, but instead, the file ended before I could get there.", 
            self.token_expected, self.lookahead_amt
        )
    }

    fn get_line_num(&self) -> usize {
        self.token_start.line
    }
}

impl TokenOutOfLineSE {
    ///
    /// Creates a new instance of a TokenOutOfLineSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::TokenOutOfLineSE;
    /// use rattlesnake::scanner::rtl_token::RTLToken;
    /// use rattlesnake::prelude::*;
    ///
    /// let error = TokenOutOfLineSE::new(GENERIC_BLANK_TOKEN, RTLToken::FunctionIdentifier, 1usize);
    /// ````
    pub fn new(token_start: Token, token_expected: RTLToken, lookahead_amt: usize) -> Self {
        Self {
            token_expected,
            token_start,
            lookahead_amt,
        }
    }
}

impl From<Result<(), Box<dyn SyntaxError>>> for TokenOutOfLineSE {
    fn from(value: Result<(), Box<dyn SyntaxError>>) -> Self {
        match value {
            Ok(_) => return Self::new(GENERIC_BLANK_TOKEN, RTLToken::EOF, 0),
            Err(r) => {
                let b = &*r.as_any();
                match b.downcast_ref::<Self>() {
                    Some(i) => return i.clone(),
                    _ => return Self::new(GENERIC_BLANK_TOKEN, RTLToken::EOF, 0),
                }
            }
        }
    }
}

impl SyntaxError for TokenOutOfLineSE {
    fn get_msg(&self) -> String {
        format!(
            "I expected to see a {} {} tokens later, but instead, the line ended before I could get there.", 
            self.token_expected, self.lookahead_amt
        )
    }

    fn get_line_num(&self) -> usize {
        self.token_start.line
    }
}

impl NegativeIndentsSE {
    ///
    /// Creates a new instance of a NegativeIndentsSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::NegativeIndentsSE;
    ///
    /// let error = NegativeIndentsSE::new(1usize);
    /// ````
    pub fn new(line_num: usize) -> Self {
        Self { line_num }
    }
}

impl From<Result<(), Box<dyn SyntaxError>>> for NegativeIndentsSE {
    fn from(value: Result<(), Box<dyn SyntaxError>>) -> Self {
        match value {
            Ok(_) => return Self::new(0),
            Err(r) => {
                let b = &*r.as_any();
                match b.downcast_ref::<Self>() {
                    Some(i) => return i.clone(),
                    _ => return Self::new(0),
                }
            }
        }
    }
}

impl SyntaxError for NegativeIndentsSE {
    fn get_msg(&self) -> String {
        format!("You tried to unindent when there were already zero indents, leading to negative indents.")
    }

    fn get_line_num(&self) -> usize {
        self.line_num
    }
}

impl UnsupportedFeatureSE {
    /// Creates a new instance of a UnsupportedFeatureSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::UnsupportedFeatureSE;
    /// use rattlesnake::scanner::rtl_token::RTLToken;
    ///
    /// let error = UnsupportedFeatureSE::new(RTLToken::FromIdentifier, 1usize);
    /// ````
    pub fn new(feature_token: RTLToken, line_num: usize) -> Self {
        Self {
            feature_token,
            line_num,
        }
    }
}

impl From<Result<(), Box<dyn SyntaxError>>> for UnsupportedFeatureSE {
    fn from(value: Result<(), Box<dyn SyntaxError>>) -> Self {
        match value {
            Ok(_) => return Self::new(RTLToken::EOF, 0),
            Err(r) => {
                let b = &*r.as_any();
                match b.downcast_ref::<Self>() {
                    Some(i) => return i.clone(),
                    _ => return Self::new(RTLToken::EOF, 0),
                }
            }
        }
    }
}

impl SyntaxError for UnsupportedFeatureSE {
    fn get_msg(&self) -> String {
        match self.feature_token {
            RTLToken::LambdaIdentifier => {
                "Lambda and anonymous functions are not presently supported in Rattle. Define a named function instead.".to_string()
            }
            RTLToken::NonlocalIdentifier => {
                    "Nonlocals are not presently supported in Rattle. If this feature is critical to your project, consider editing the Python generated source file instead.".to_string()
            }
            RTLToken::GlobalIdentifier => {
                    "Globals are not presently supported in Rattle. If this feature is critical to your project, consider editing the Python generated source file instead.".to_string()
            }
            RTLToken::FromIdentifier => {
                "From keywords in package imports are not presently supported in Rattle. However, using the following pattern: package torch dot nn alias nn will provide the same functionality as the from keyword: package nn from torch.".to_string()
            }
            _ => format!("{} are not presently supported in Rattle", self.feature_token)
        }
    }

    fn get_line_num(&self) -> usize {
        self.line_num
    }
}

impl UnimplementedSE {
    /// Creates a new instance of a UnsupportedFeatureSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::UnimplementedSE;
    /// let error = UnimplementedSE::new(1usize);
    /// ````
    pub fn new(line_num: usize) -> Self {
        Self { line_num }
    }
}

impl From<Result<(), Box<dyn SyntaxError>>> for UnimplementedSE {
    fn from(value: Result<(), Box<dyn SyntaxError>>) -> Self {
        match value {
            Ok(_) => return Self::new(0),
            Err(r) => {
                let b = &*r.as_any();
                match b.downcast_ref::<Self>() {
                    Some(i) => return i.clone(),
                    _ => return Self::new(0),
                }
            }
        }
    }
}

impl SyntaxError for UnimplementedSE {
    fn get_msg(&self) -> String {
        format!(
            "Found code that is still not finished on line {}",
            self.line_num
        )
    }

    fn get_line_num(&self) -> usize {
        self.line_num
    }
}

impl PoorlyFormattedSE {
    /// Creates a new instance of a PoorlyFormattedSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::PoorlyFormattedSE;
    /// let error = PoorlyFormattedSE::new(
    ///     String::from("function"),
    ///     None,
    ///     1usize,
    /// );
    /// ````
    pub fn new(expr: String, additional_msg: Option<String>, line_num: usize) -> Self {
        Self {
            expr,
            line_num,
            additional_msg,
        }
    }
}

impl From<Result<(), Box<dyn SyntaxError>>> for PoorlyFormattedSE {
    fn from(value: Result<(), Box<dyn SyntaxError>>) -> Self {
        match value {
            Ok(_) => return Self::new(String::from(""), None, 0),
            Err(r) => {
                let b = &*r.as_any();
                match b.downcast_ref::<Self>() {
                    Some(i) => return i.clone(),
                    _ => return Self::new(String::from(""), None, 0),
                }
            }
        }
    }
}

impl SyntaxError for PoorlyFormattedSE {
    fn get_msg(&self) -> String {
        let mut additional_msg_ = String::new();
        if let Some(am) = &self.additional_msg {
            additional_msg_ = am.to_string();
        }

        format!(
            "Your {} was not correctly implemented. {}",
            self.expr, additional_msg_
        )
    }

    fn get_line_num(&self) -> usize {
        self.line_num
    }
}

impl MiscellaneousSE {
    /// Creates a new instance of a MiscellaneousSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::MiscellaneousSE;
    /// let error = MiscellaneousSE::new(String::from("Error message for user"), 1usize);
    /// ````
    pub fn new(msg: String, line_num: usize) -> Self {
        Self { msg, line_num }
    }
}

impl SyntaxError for MiscellaneousSE {
    fn get_msg(&self) -> String {
        self.msg.clone()
    }

    fn get_line_num(&self) -> usize {
        self.line_num
    }
}

impl UnequalArgsSE {
    /// Creates a new instance of a UnequalArgsSE error.
    ///
    /// # Examples
    /// ```rust
    /// use rattlesnake::error::UnequalArgsSE;
    /// let error = UnequalArgs::new(String::from("result"), 1, 3, 1usize);
    /// ````
    pub fn new(
        fn_name: String,
        args_expected: String,
        args_received: usize,
        line_num: usize,
    ) -> Self {
        Self {
            fn_name,
            args_expected,
            args_received,
            line_num,
        }
    }
}

impl SyntaxError for UnequalArgsSE {
    fn get_msg(&self) -> String {
        let suffix1 = if self.args_expected == "1" { "s" } else { "" };
        let suffix2 = if self.args_received == 1 { "s" } else { "" };
        format!(
            "Incorrect number of arguments for the function {}. It expected {} argument{}, but you passed in {} argument{}",
            self.fn_name, self.args_expected, suffix1, self.args_received, suffix2
        )
    }

    fn get_line_num(&self) -> usize {
        return self.line_num;
    }
}
