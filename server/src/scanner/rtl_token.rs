/// Contains each token that may be found in a Rattle script.
///
/// A Rattle script is parsed down into these tokens using the `Scanner`, and these tokens
/// are compiled to a Python script.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RTLToken {
    /// Opens an indented block in Python
    BlockStart,
    /// closes an indented block in Python
    BlockEnd,
    /// closes a one-line expression in Python
    ExprEnd,
    /// `=` (assignment) operator in Python
    AssnEq,
    /// `*` (unpack) operator in Python
    Unpack,
    /// `+` (add) operator in Python
    AddOperation,
    /// `-` (addition) operator in Python
    SubOperation,
    /// `*` (subtraction) operator in Python
    MulOperation,
    /// `/` (multiplication) operator in Python
    DivOperation,
    /// `%` (division) operator in Python
    ModOperation,
    /// `//` (integer division) operator in Python
    IntDivOperation,
    /// `==` (equality) operator in Python
    EqComparator,
    /// `!=` (inequality) operator in Python
    NeComparator,
    /// `>` (greater than) operator in Python
    GtComparator,
    /// `>=` (greater than or equal) operator in Python
    GtEqComparator,
    /// `<` (less than) operator in Python
    LtComparator,
    /// `<=` (less than or equal) operator in Python
    LtEqComparator,
    /// `and` operator in Python
    AndLogical,
    /// `or` operator in Python
    OrLogical,
    /// `not` operator in Python
    NotLogical,
    /// `is` operator in Python
    IdentityOperator,
    /// `in` operator in Python
    MembershipOperator,
    /// Python strings
    StringVal,
    /// Python numerics (includes `int`s and `float`s)
    NumericVal,
    /// Python booleans (`true` and `false`)
    BooleanVal,
    /// Python lists: [a, b, c, ...]
    ListVal,
    /// Python tuples: (a, b, c, ...)
    TupleVal,
    /// Python dictionaries: {a:b, c:d, e:f, ...}
    DictVal,
    /// Encodes `None` value in Python
    NoneVal,
    /// Identifier for valid variable, function, class and package names
    ObjIdentifier,
    /// Rattle keyword for chaining calls and attributes
    DotOperator,
    /// Rattle keyword for declaring a function (`def` in Python)
    FunctionIdentifier,
    /// Rattle keyword for calling a function
    FnCallIdentifier,
    /// Rattle keyword for declaring a variable
    VarIdentifier,
    /// Rattle keyword for declaring a class (`class` in Python)
    ClassIdentifier,
    /// Rattle keyword for opening an if statement (`if` in Python)
    IfIdentifier,
    /// Rattle keyword for opening an else if statement (`elif` in Python)
    ElifIdentifier,
    /// Rattle keyword for opening an else statement (`else` in Python)
    ElseIdentifier,
    /// Rattle keyword for opening an for loop (`for` in Python)
    ForIdentifier,
    /// Rattle keyword for opening an if statement (`while` in Python)
    WhileIdentifier,
    /// Keyword for aliasing package name (`as` in Python)
    AliasIdentifier,
    /// Keyword for asserts and tests (`assert` in Python)
    AssertIdentifier,
    /// Keyword for asynchronous functions (`async` in Python)
    AsyncIdentifier,
    /// Keyword to await asynchronous functions (`await` in Python)
    AwaitIdentifier,
    /// Keyword to break from loops (`break` in Python)
    BreakIdentifier,
    /// Keyword to continue a loop (`continue` in Python)
    ContinueIdentifier,
    /// Keyword to remove an object from memory (`del` in Python)
    DelIdentifier,
    /// Keyword to catch exceptions (`except` in Python)
    ExceptIdentifier,
    /// Keyword to run after try-catch block (`finally` in Python)
    FinallyIdentifier,
    /// Keyword to import a package from a specific location (`from` in Python)
    FromIdentifier,
    /// Likely will be unsupported in initial Rattle version
    GlobalIdentifier,
    /// Keyword to load in an external package (`import` in Python)
    ImportIdentifier,
    /// Keyword to declare an anonymous function (`lambda` in Python)
    LambdaIdentifier,
    /// Likely will be unsupported in initial Rattle version
    NonlocalIdentifier,
    /// Placeholder keyword (similar to `todo!()` in Rust, `pass` in Python)
    PassIdentifier,
    /// Keyword to throw an exception and stop the program (`raise` in Python)
    RaiseIdentifier,
    /// Keyword to return from a function (`return` in Python)
    ReturnIdentifier,
    /// Keyword to try a code block that might fail (`try` in Python)
    TryIdentitifer,
    /// Keyword to open a context-scope block  (`with` in Python)
    WithIdentifier,
    /// Keyword to return a promise to compute a value (`yield` in Python)
    YieldIdentifier,
    /// Rattle-only keyword to use a reserved keyword for other purposes
    // EscapeIdentifier,
    /// Raw sequence following an `EscapeIdentifier` that should not be parsed as any other token
    RawSequence,
    /// Special token representing a line break
    LineBreak,
    /// Special token representing the end of a file
    EOF,
    /// Special token for calls to print (for Python3, coerces to a function call)
    PrintToken,
    /// Rattle-specific keyword `of` (used with function definitions and function calls)
    OfToken,
    /// The name of a function or a function call; not strictly required, but convenient
    FnName,
    /// The and used to separate arguments in a function or a function call
    AndDelim,
    /// The token used to index into lists, tuples or dictionaries ('[' and ']' in Python)
    IdxOperator,
    /// A digraph operation that is still in progress
    PENDING,
}

impl std::fmt::Display for RTLToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
