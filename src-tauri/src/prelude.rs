pub use crate::error::SyntaxError;
use crate::scanner::rtl_token::RTLToken;
use crate::scanner::scanner::Token;

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
