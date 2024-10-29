use crate::digraph::address::Address;
use crate::digraph::state::CursorDir;
pub use crate::error::SyntaxError;
use crate::scanner::rtl_token::RTLToken;
use crate::scanner::scanner::Token;
use thiserror::Error;

/// A custom result type with two variants:
/// 1. Ok(()) -> Indicates proper Rattle syntax
/// 2. Err(Box<dyn SyntaxError>) -> Indicates a syntax error that
///    must be one of the nine defined errors in the language.
pub type RESULT = Result<(), Box<dyn SyntaxError>>;

/// Another custom, generic result type with two variants:
/// 1. Ok(T) -> Indicates proper Rattle syntax and returns a value of type T
/// 2. Err(Box<dyn SyntaxError>) -> Indicates a syntax error that
///    must be one of the nine defined errors in the language.
pub type RESVAL<T> = Result<T, Box<dyn SyntaxError>>;

/// A Rust equivalent of the ternary operator in C/C++.
/// cpp: bool ? yes : no ||| rust: check!(bool => yes ; no)
#[macro_export]
macro_rules! check {
    ($test:expr => $true_expr:expr ; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

/// Blank tokens will be 'end of file' tokens
const BLANK_TOKEN_KIND: RTLToken = RTLToken::EOF;

/// A generic, blank token (freuqently to be used as a default value or
/// an example for documentatino purposes)
pub const GENERIC_BLANK_TOKEN: Token = Token::new(BLANK_TOKEN_KIND, String::new(), None, 0);

/// The maximum length of a Rattle keyword (in this case, "greater
/// than equal to" and "less than equal to" are four words each).
pub const MAX_KW_LEN: usize = 3;

/// A list of symbols that can be used (and have valid meanings) in Python.
pub const PY_SYMBOLS: &[&'static str] = &[
    "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", ".", ",", "/", "?", "+", "-", "_", "=", "<",
    ">", "~", "`", "\"", "'", "[", "{", "}", "]", "|", "\\", ";", ":", " ",
];

/// Corresponding keywords that can be used to represent the Python keywords
/// (in the order listed in `PY_SYMBOLS` above)
pub const RTL_SYMBOLS: &[&'static str] = &[
    r"\s*exclamations? sign\s*",
    r"\s*ats? sign\s*",
    r"\s*pounds? sign\s*",
    r"\s*dollars? sign\s*",
    r"\s*percents? sign\s*",
    r"\s*carets? sign\s*",
    r"\s*ampersands? sign\s*",
    r"\s*stars? sign\s*",
    r"\s*left parenthes[ie]s",
    r"\s*right parenthes[ie]s",
    r"\s*dots? sign\s*",
    r"\s*commas? sign\s*",
    r"\s*slashs? sign\s*",
    r"\s*questions? sign\s*",
    r"\s*pluss? sign\s*",
    r"\s*minuss? sign\s*",
    r"\s*underscores? sign\s*",
    r"\s*equals? sign\s*",
    r"\s*less thans? sign\s*",
    r"\s*greater thans? sign\s*",
    r"\s*tildes? sign\s*",
    r"\s*ticks? sign\s*",
    r"\s*quote sign\s*",
    r"\s*apostrophe sign\s*",
    r"\s*left brackets? sign\s*",
    r"\s*left brace sign\s*",
    r"\s*right brace sign\s*",
    r"\s*right brackets? sign\s*",
    r"\s*pipes? sign\s*",
    r"\s*back\s*slashs? sign\s*",
    r"\s*semicolons? sign\s*",
    r"\s*colons? sign\s*",
    r"\s*spaces? sign\s*",
];

/// A wrapper around Rust's `Vec` that only supports LIFO operations.
///
/// # Examples
/// ```
/// use crate::*;
///
/// // Creates an empty stack that we can manually fill up
/// let mut stack: Stack<usize> = Stack::new();
/// stack.push(3);
/// stack.push(2);
/// stack.push(6);
/// assert_eq!(stack.len(), 3);
/// assert_eq!(stack.pop(), 6);
/// assert_eq!(stack.len(), 2);
/// let _ = stack.pop();
/// let _ = stack.pop();
/// assert!(stack.is_empty());
/// ```
///
/// ```
/// // We can also initialize `Stack`s from `Vec`s
/// use crate::*;
///
/// let vec: Vec<usize> = vec![3, 6, 2];
/// let mut stack: Stack<usize> = Stack::from_vec(vec);
/// assert_eq!(stack.len(), 3);
/// assert_eq!(stack.pop(), 6);
/// assert_eq!(stack.len(), 2);
/// let _ = stack.pop();
/// let _ = stack.pop();
/// assert!(stack.is_empty());
/// ```
#[derive(Debug, Clone)]
pub(crate) struct Stack<T>
where
    T: Clone,
{
    pub elements: Vec<T>,
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub(crate) fn new() -> Self {
        Self { elements: vec![] }
    }

    pub(crate) fn push(&mut self, elem: T) {
        self.elements.push(elem);
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub(crate) fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.elements.len()
    }
}

#[derive(Debug, Error)]
pub(crate) enum CursorError {
    #[error("the address {0} does not exist in this tree")]
    InvalidAddress(Address),
    #[error("the motion {0} is not available at this position")]
    InvalidMotion(CursorDir),
    #[error("couldn't find address: {} in the file", .0)]
    AddrNotFound(Address),
    #[error("found an invalid piece address: {:?}", .0)]
    PieceAddrNotFound(Vec<usize>),
    #[error("couldn't find parent (whose address should be {}", .0)]
    ParentNotFound(Address),
    #[error("cannot increment an empty address")]
    EmptyAddr,
    #[error("cannot add to a conditional that already has a 'yes' and 'no' branch")]
    InsertConditional,
    #[error("found an invalid number (only 0-9 and a single decimal place can be used)")]
    InvalidNumber,
    #[error("cannot edit inplace unless a node is currently selected")]
    AmbiguousEdit,
}

impl From<std::num::ParseFloatError> for CursorError {
    fn from(_: std::num::ParseFloatError) -> Self {
        return Self::InvalidNumber;
    }
}

#[derive(Debug, Error)]
pub(crate) enum TranspileError {}
