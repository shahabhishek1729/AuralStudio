use crate::digraph::address::Address;
use crate::digraph::util::HORIZ_CHILDREN;
use crate::prelude::Stack;
use crate::scanner::rtl_token::RTLToken;
use crate::scanner::scanner::{Scanner, Token};
use crate::static_analysis::analyzer::SemanticError;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[macro_export]
macro_rules! piece {
    (IDENT $e:expr) => {{
        use crate::digraph::parser::Piece;
        Piece::IDENT($e.to_string())
    }};
    (#$e:expr) => {{
        use crate::digraph::parser::Piece;
        Piece::NUMBER($e as f64)
    }};
    (TEXT $e:expr) => {{
        use crate::digraph::parser::Piece;
        Piece::TEXT($e.to_string())
    }};
    (LIST [$($e:expr),*]) => {{
        use crate::digraph::parser::Piece;
        Piece::LIST(vec![$($e),*])
    }};
    (True) => {{
        use crate::digraph::parser::Piece;
        Piece::BOOL(true)
    }};
    (False) => {{
        use crate::digraph::parser::Piece;
        Piece::BOOL(false)
    }};
    () => {{
        use crate::digraph::parser::Piece;
        Piece::NOTHING
    }};
    (..#) => {{
        use crate::digraph::parser::Piece;
        Piece::PendingVal
    }};
    (..+) => {{
        use crate::digraph::parser::Piece;
        Piece::PendingOp
    }};
}

#[macro_export]
macro_rules! make_node {
    (line $line:literal -> $kind:path [$($piece:expr),*]) => {
        {
            use $crate::digraph::address::Address;
            use $crate::digraph::parser::Node;
            Node {
                line: $line,
                children: vec![],
                kind: $kind,
                pieces: vec![$($piece),*],
                addr: Address::new(vec![]),
                ..Default::default()
            }
        }
    };
    (line $line:literal -> $kind:path [$($piece:expr),*]; {$($child:expr),*}) => {
        {
            use $crate::digraph::address::Address;
            use $crate::digraph::parser::Node;
            Node {
                line: $line,
                children: vec![$($child),*],
                kind: $kind,
                pieces: vec![$($piece),*],
                addr: Address::new(vec![]),
                ..Default::default()
            }
        }
    };
    (L $line:literal @ $addr:expr => $kind:path [$($piece:expr),*]) => {
        {
            use $crate::digraph::parser::Node;
            Node {
                line: $line,
                children: vec![],
                kind: $kind,
                pieces: vec![$($piece),*],
                addr: $addr,
                ..Default::default()
            }
        }
    };
    (L $line:literal @ $($addr:literal),* -> $kind:path [$($piece:expr),*]) => {
        {
            use $crate::digraph::address::Address;
            use $crate::digraph::parser::Node;
            Node {
                line: $line,
                children: vec![],
                kind: $kind,
                pieces: vec![$($piece),*],
                addr: Address::new(vec![$($addr),*]),
                ..Default::default()
            }
        }
    };
    (L $line:literal @ $($addr:literal),* -> $kind:path [$($piece:expr),*]; {$($child:expr),*}) => {
        {
            use $crate::digraph::address::Address;
            use $crate::digraph::parser::Node;
            Node {
                line: $line,
                children: vec![$($child),*],
                kind: $kind,
                pieces: vec![$($piece),*],
                addr: Address::new(vec![$($addr),*]),
                ..Default::default()
            }
        }
    };
}

/// Represents a single line of code, which is rendered in a digraph as a single node.
///
/// A node consists of several pieces (for instance, an `output` node may consist of a `string`
/// piece, an `addition` operator piece and another `string piece`), and a node can have several
/// children (which are themselves nodes, and are analogous to indented segments of code in other
/// programming languages).
///
/// # Examples
/// ```
/// use crate::*;
/// // The following node represents the code `print("Hello, World!")` on line 1 of a Rattle file.
/// let node = Node {
///   line: 1,
///   children: vec![],
///   kind: NodeKind::OUTPUT,
///   pieces: vec!(Piece::TEXT(String::from("Hello, World!")))
/// };
///
/// // These can also be written with the `make_node!` macro.
/// let gen_node = make_node!(line 1 -> NodeKind::OUTPUT [piece!(TEXT "Hello, World!")]);
/// assert_eq!(gen_node, node);
/// ```
///
/// ```
/// // The following node represents the following code in a Rattle file:
/// // if true:
/// //    print("Hello, World!")
/// // else:
/// //    print("Goodbye, World!")
/// let node = Node {
///   line: 1,
///   children: vec![
///     Node {
///       line: 2,
///       children: vec![
///         Node {
///           line: 3,
///           children: vec![],
///           kind: NodeKind::OUTPUT,
///           pieces: vec![Piece::TEXT(String::from("Hello, World!"))]
///         },
///       ],
///       kind: NodeKind::CONDTLY,
///       pieces: vec![]
///     }
///     Node {
///       line: 4,
///       children: vec![
///         Node {
///           line: 5,
///           children: vec![],
///           kind: NodeKind::OUTPUT,
///           pieces: vec![Piece::TEXT(String::from("Goodbye, World!"))]
///         },
///       ],
///       kind: NodeKind::CONDTLN,
///       pieces: vec![]
///     }
///   ],
///   kind: NodeType::CONDTL,
///   pieces: vec![Piece::BOOL(true)]
/// };
///
/// // Or with the `make_node!` macro:
/// let gen_node = make_node!(line 1 -> CONDTL [piece!(True)]; {
///     make_node!(line 2 -> NodeKind::CONDTLY []; {
///         make_node!(line 3 -> NodeKind::OUTPUT [piece!(TEXT "Hello, World!")])
///     }),
///     make_node!(line 4 -> NodeKind::CONDTLN []; {
///         make_node!(line 5 -> NodeKind::OUTPUT [piece!(TEXT "Goodbye, World!")])
///     })
/// });
///
/// assert_eq!(node, gen_node);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Node {
    // Since each Node is itself a line, this serves as a primary key
    pub(crate) line: usize,
    // Used in functions, conditionals, loops, classes, etc.
    pub(crate) children: Vec<Node>,
    pub(crate) kind: NodeKind,
    // The rest of the token, if applicable
    pub(crate) pieces: Vec<Piece>,
    #[serde(rename = "address")]
    pub(crate) addr: Address,
    #[serde(rename = "parent")]
    pub(crate) parent_addr: Address,
    pub(crate) rtl: Option<String>,
    pub(crate) note: Option<String>,
    pub(crate) err: Option<SemanticError>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            line: 0,
            children: vec![],
            kind: NodeKind::OUTPUT,
            pieces: vec![],
            addr: Address::new(vec![]),
            parent_addr: Address::new(vec![]),
            rtl: None,
            note: None,
            err: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        // We don't need to check the `parent_addr` field because if addresses are equal, the nodes
        // must be equal (helps make testing macros more concise).
        self.line == other.line
            && self.children == other.children
            && self.kind == other.kind
            && self.pieces == other.pieces
            && self.addr == other.addr
    }
}

impl Node {
    pub(super) fn has_subtree(&self) -> bool {
        // NOTE:XXX: Claims that all sub-function definitions must be in the root children of a function
        // NOTE:XXX: Claims FNDEF & CONDTL are the only kind of sub-tree allowable (classes?)
        self.children
            .iter()
            .find(|&x| HORIZ_CHILDREN.contains(&x.kind))
            .is_some()
    }
}

/// Represents every kind of node we can have in Rattle.
/// Each node represents a single line of code, and the kind of the node is determined by the first
/// word in the line (e.g., 'define ...' means that the line is a Node of type `FNDEF`). For
/// function calls, however, the first word will be an identifier, and will be followed by 'of'.
#[derive(Debug, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) enum NodeKind {
    /// Function definitions
    FNDEF,
    /// Variable declarations
    VARDECL,
    /// Outputting to the console
    OUTPUT,
    /// Conditionals (if-else statements)
    CONDTL,
    /// The `true` branch of a conditional
    CONDTLY,
    /// The `false` branch of a conditional
    CONDTLN,
    /// For loops over iterables
    FORLOOP,
    /// While loops
    WHLLOOP,
    /// Breaking out of loops
    BREAK,
    /// Continuing on to the next iteration of loops
    CONTINUE,
    /// Returns from within functions
    RETURN,
    /// Calling a previously defined function
    FNCALL,
    /// Imports for foreign code
    GRABPKG,
    /// Nodes being actively edited
    PENDING,
}

/// An enum of every kind of piece supported in Rattle. A piece is an individual chunk of code,
/// often no more than a single token; a `Node` is made up of one or more pieces. Examples of
/// pieces include an individual constant (e.g., a number, string, boolean, etc.), opreator or
/// identifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub(crate) enum Piece {
    /// Identifiers (keywords or custom-defined)
    IDENT(String),
    /// Numbers (includes `integer`s and `float`s.
    NUMBER(f64),
    /// Strings (bounded by 'string...done'
    TEXT(String),
    /// Booleans (true or false)
    BOOL(bool),
    /// The NoneType
    NOTHING,
    /// An operator
    OP(OpKind),
    /// A function call
    FNCALL(Vec<Piece>),
    /// A list
    LIST(Vec<Piece>),
    /// Being edited (value)
    PendingVal,
    /// Being edited (operator)
    PendingOp,
    /// Placeholder type for when an invalid piece index is used
    NULL,
}

impl Piece {
    pub(crate) fn resolves_to_val(&self) -> bool {
        match self {
            Piece::OP(_) | Piece::PendingOp => false,
            _ => true,
        }
    }
}

pub(crate) struct PieceIdx<'a>(pub &'a [usize]);
impl std::ops::Index<PieceIdx<'_>> for Vec<Piece> {
    type Output = Piece;

    fn index(&self, index: PieceIdx) -> &Self::Output {
        let mut curr = self;
        for (i, curr_ix) in index.0.iter().enumerate() {
            if i == index.0.len() - 1 {
                return &curr[*curr_ix];
            }

            match curr[*curr_ix] {
                Piece::LIST(ref args) | Piece::FNCALL(ref args) => curr = args,
                ref piece => return piece,
            }
        }
        &Piece::NULL
    }
}

impl std::ops::IndexMut<PieceIdx<'_>> for Vec<Piece> {
    fn index_mut(&mut self, index: PieceIdx) -> &mut Self::Output {
        let mut curr = self;
        for (i, curr_ix) in index.0.iter().enumerate() {
            if i == index.0.len() - 1 {
                return &mut curr[*curr_ix];
            }

            match curr[*curr_ix] {
                Piece::LIST(ref mut args) | Piece::FNCALL(ref mut args) => curr = args,
                ref mut piece => return piece,
            }
        }
        unreachable!(
            "Too many indices {:?} out of bounds for piece vector",
            index.0
        );
    }
}

#[test]
fn indices_work() {
    let piece_vec = vec![
        piece!(IDENT "hi"),
        Piece::OP(OpKind::ASSN),
        piece!(LIST [piece!(IDENT "list"), piece!(..#)]),
    ];

    assert_eq!(piece_vec[PieceIdx(&[2, 0])], piece!(IDENT "list"));
    assert_eq!(piece_vec[PieceIdx(&[2, 1])], piece!(..#));
    assert_eq!(
        piece_vec[PieceIdx(&[2])],
        piece!(LIST [piece!(IDENT "list"), piece!(..#)])
    )
}

/// An enum of every kind of operator supported in Rattle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub(crate) enum OpKind {
    /// +
    ADD,
    /// -
    SUB,
    /// *
    MUL,
    /// /
    DIV,
    /// %
    MOD,
    /// ==
    EQ,
    /// !=
    NE,
    /// >
    GT,
    /// <
    LT,
    /// >=
    GE,
    /// <=
    LE,
    /// = or ->
    ASSN,
    /// && (and)
    AND,
    /// || (or)
    OR,
    /// !
    NOT,
    /// in
    IN,
    /// dot
    DOT,
    /// indexing ([])
    AT,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Didn't expect to have to explicitly handle token {}", .0.rtl_token)]
    UnexpectedToken(Token),
    #[error("Two tokens were expected to be on the same line, but were not!")]
    LineMismatch,
}

pub(crate) struct Parser {
    tokens: Vec<Token>,
    waiting_parents: Stack<Vec<usize>>,
    curr: usize,
    end: usize,
}

impl Parser {
    pub(crate) fn new(source: String) -> anyhow::Result<Self> {
        let mut scanner = Scanner::new(&source);
        // For now, we depend on a file with no syntax errors to render this tree.
        // TODO: To avoid this, we would need a String-based parser
        let Ok(tokens) = scanner.scan() else {
            anyhow::bail!("Syntax error found in your file, could not be scanned");
        };

        Ok(Self {
            tokens: tokens.clone(),
            curr: 0,
            waiting_parents: Stack::new(),
            end: tokens.len(),
        })
    }

    pub(crate) fn parse(&mut self) -> anyhow::Result<Vec<Node>> {
        let mut nodes = vec![];

        loop {
            let curr_token = self.advance_();

            match curr_token.rtl_token {
                RTLToken::ImportIdentifier => {
                    // 'grab ...'
                    let line = curr_token.line;
                    let (pieces, rtl) = self._collect_line()?;
                    let node = Node {
                        kind: NodeKind::GRABPKG,
                        line,
                        children: vec![],
                        pieces,
                        addr: Address::new(vec![]),
                        rtl: Some(format!("grab {}", rtl)),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;
                }

                RTLToken::PrintToken => {
                    // 'output ...'
                    let line = curr_token.line;
                    let (pieces, rtl) = self._collect_line()?;
                    let node = Node {
                        kind: NodeKind::OUTPUT,
                        line,
                        children: vec![],
                        pieces,
                        addr: Address::new(vec![]),
                        rtl: Some(format!("output {}", rtl)),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;
                }

                RTLToken::ReturnIdentifier => {
                    // 'output ...'
                    let line = curr_token.line;
                    let (pieces, rtl) = self._collect_line()?;
                    let node = Node {
                        kind: NodeKind::RETURN,
                        line,
                        children: vec![],
                        pieces,
                        addr: Address::new(vec![]),
                        rtl: Some(format!("return {}", rtl)),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;
                }

                RTLToken::FunctionIdentifier => {
                    // 'define ... of ...'
                    let curr_token = self.advance_();
                    let curr_line = curr_token.line;

                    let Some(ref fn_name) = curr_token.literal else {
                        anyhow::bail!(
                            "The 'define' keyword was not followed by the name of the function!"
                        );
                    };
                    let fn_name: String = fn_name.unwrap_identifier();
                    let (args, rtl): (Vec<Piece>, String) = self._collect_line()?;
                    let pieces: Vec<Piece> = [&[Piece::IDENT(fn_name.clone())], &args[..]].concat();

                    let node = Node {
                        kind: NodeKind::FNDEF,
                        line: curr_line,
                        children: vec![],
                        pieces,
                        addr: Address::new(vec![]),
                        rtl: Some(format!("define {} of {}", fn_name, rtl)),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;

                    let insert_loc = self._get_insert_loc(&mut nodes)?;
                    // self.waiting_parents.push(vec![insert_loc.len() - 1]);
                    // let insert_loc = self._get_insert_loc(&mut nodes)?;

                    match self.waiting_parents.peek() {
                        Some(last) => {
                            self.waiting_parents.push(
                                last.iter()
                                    .chain(&[insert_loc.len() - 1])
                                    .map(|&x| x)
                                    .collect::<Vec<_>>(),
                            );
                        }
                        None => self.waiting_parents.push(vec![insert_loc.len() - 1]),
                    }
                }

                RTLToken::VarIdentifier => {
                    // 'let ... be ...'
                    let curr_token = self.advance_();
                    let curr_line = curr_token.line;

                    let Some(ref var_name) = curr_token.literal else {
                        anyhow::bail!(
                            "The 'let' keyword was not followed by the name of the variable!"
                        );
                    };
                    let var_name = var_name.unwrap_identifier();

                    let (expr, rtl) = self._collect_line()?;
                    let pieces = [&[Piece::IDENT(var_name.clone())], &expr[..]].concat();

                    let node = Node {
                        kind: NodeKind::VARDECL,
                        line: curr_line,
                        children: vec![],
                        pieces,
                        addr: Address::new(vec![]),
                        rtl: Some(format!("let {} {}", var_name, rtl)),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;
                }

                RTLToken::IfIdentifier => {
                    // 'if ...'
                    let curr_line = curr_token.line;
                    let (protasis, rtl) = self._collect_line()?;

                    let node = Node {
                        kind: NodeKind::CONDTL,
                        line: curr_line,
                        children: vec![Node {
                            kind: NodeKind::CONDTLY,
                            line: curr_line + 1,
                            children: vec![],
                            pieces: vec![],
                            addr: Address::new(vec![]),
                            rtl: Some(format!("if {}", rtl)),
                            ..Default::default()
                        }],
                        pieces: protasis,
                        addr: Address::new(vec![]),
                        rtl: None,
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;

                    let insert_loc = self._get_insert_loc(&mut nodes)?;

                    let Some(last) = self.waiting_parents.peek() else {
                        anyhow::bail!(
                            "Found nothing in the stack when trying to push a conditional."
                        )
                    };

                    let last = last.clone();

                    self.waiting_parents.push(
                        last.iter()
                            .chain(&[insert_loc.len() - 1])
                            .map(|&x| x)
                            .collect::<Vec<_>>(),
                    );

                    self.waiting_parents.push(
                        last.iter()
                            .chain(&[insert_loc.len() - 1, 0])
                            .map(|&x| x)
                            .collect::<Vec<_>>(),
                    );
                }

                RTLToken::ElseIdentifier => {
                    // 'otherwise'
                    let node = Node {
                        kind: NodeKind::CONDTLN,
                        line: curr_token.line,
                        children: vec![],
                        pieces: vec![],
                        addr: Address::new(vec![]),
                        rtl: Some("otherwise".into()),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;

                    let Some(last) = self.waiting_parents.peek() else {
                        anyhow::bail!(
                            "Found nothing in the stack when trying to push `else` branch"
                        );
                    };

                    let last = last.clone();
                    self.waiting_parents
                        .push(last.iter().chain(&[1]).map(|&x| x).collect::<Vec<_>>());
                }

                RTLToken::ForIdentifier => {
                    // 'for ...'
                    let line = curr_token.line;
                    let (pieces, rtl) = self._collect_line()?;

                    let node = Node {
                        kind: NodeKind::FORLOOP,
                        line,
                        children: vec![],
                        pieces,
                        addr: Address::new(vec![]),
                        rtl: Some(format!("for {}", rtl)),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;
                    let insert_loc = self._get_insert_loc(&mut nodes)?;

                    let Some(last) = self.waiting_parents.peek() else {
                        anyhow::bail!("Found nothing in the stack when trying to push `for` loop");
                    };

                    let last = last.clone();
                    self.waiting_parents.push(
                        last.iter()
                            .chain(&[insert_loc.len() - 1])
                            .map(|&x| x)
                            .collect::<Vec<_>>(),
                    );
                }

                RTLToken::BlockEnd => {
                    // 'done ...'
                    let ended_str = curr_token.lexeme.clone();
                    // When we reach 'done otherwise', we need to pop both the else piece and the
                    // overall conditional itself
                    if &ended_str == "done otherwise" {
                        let piece_ended = self.waiting_parents.pop();
                        anyhow::ensure!(
                            piece_ended.is_some(),
                            "We ended a piece ({}) we never started!",
                            ended_str
                        );
                    }

                    let piece_ended = self.waiting_parents.pop();
                    anyhow::ensure!(
                        piece_ended.is_some(),
                        "We ended a piece ({}) we never started!",
                        ended_str
                    );
                }
                RTLToken::PENDING => {
                    // 'output ...'
                    let line = curr_token.line;
                    let node = Node {
                        kind: NodeKind::PENDING,
                        line,
                        children: vec![],
                        pieces: vec![Piece::PendingVal],
                        addr: Address::new(vec![]),
                        rtl: Some("pretend".into()),
                        ..Default::default()
                    };
                    self.push_node(node, &mut nodes)?;
                }

                RTLToken::LineBreak => continue,
                RTLToken::EOF => break,
                _ => anyhow::bail!(ParserError::UnexpectedToken(curr_token.clone())),
            };

            if self.curr >= self.end {
                break;
            }
        }

        Ok(nodes)
    }

    fn _collect_line(&mut self) -> anyhow::Result<(Vec<Piece>, String)> {
        let mut collected: Vec<Piece> = vec![];
        let mut rtl = String::new();

        loop {
            let curr = self.advance_();
            if curr.rtl_token == RTLToken::LineBreak {
                break Ok((collected, rtl));
            }

            let curr = curr.clone();
            collected.push(self._collect_piece(&curr)?);
            rtl.push_str(&curr.lexeme);

            if self.line_end_reached() {
                break Ok((collected, rtl));
            }

            // Haven't reached the end, something will follow, so push a space
            rtl.push(' ');
        }
    }

    fn _collect_piece(&mut self, token: &Token) -> anyhow::Result<Piece> {
        match token.rtl_token {
            // Operators
            RTLToken::AddOperation => Ok(Piece::OP(OpKind::ADD)),
            RTLToken::SubOperation => Ok(Piece::OP(OpKind::SUB)),
            RTLToken::MulOperation => Ok(Piece::OP(OpKind::MUL)),
            RTLToken::DivOperation => Ok(Piece::OP(OpKind::DIV)),
            RTLToken::ModOperation => Ok(Piece::OP(OpKind::MOD)),
            RTLToken::EqComparator => Ok(Piece::OP(OpKind::EQ)),
            RTLToken::NeComparator => Ok(Piece::OP(OpKind::NE)),
            RTLToken::GtComparator => Ok(Piece::OP(OpKind::GT)),
            RTLToken::LtComparator => Ok(Piece::OP(OpKind::LT)),
            RTLToken::GtEqComparator => Ok(Piece::OP(OpKind::GE)),
            RTLToken::LtEqComparator => Ok(Piece::OP(OpKind::LE)),
            RTLToken::AssnEq => Ok(Piece::OP(OpKind::ASSN)),
            RTLToken::AndLogical => Ok(Piece::OP(OpKind::AND)),
            RTLToken::OrLogical => Ok(Piece::OP(OpKind::OR)),
            RTLToken::NotLogical => Ok(Piece::OP(OpKind::NOT)),
            RTLToken::MembershipOperator => Ok(Piece::OP(OpKind::IN)),
            RTLToken::DotOperator => Ok(Piece::OP(OpKind::DOT)),
            RTLToken::IdxOperator => Ok(Piece::OP(OpKind::AT)),
            // Values
            RTLToken::NumericVal => Ok(Piece::NUMBER(token.unwrap_numeric())),
            RTLToken::BooleanVal => Ok(Piece::BOOL(token.unwrap_bool())),
            RTLToken::StringVal => Ok(Piece::TEXT(token.unwrap_string())),
            RTLToken::ObjIdentifier => Ok(Piece::IDENT(token.unwrap_identifier())),
            RTLToken::NoneVal => Ok(Piece::NOTHING),
            // Calls and Collections
            RTLToken::FnCallIdentifier => {
                //let mut signature = vec![Piece::IDENT(token.unwrap_identifier())];
                let mut signature = vec![];
                let mut token;
                loop {
                    token = self.advance_().clone();
                    if token.rtl_token == RTLToken::ExprEnd {
                        break;
                    }
                    signature.push(self._collect_piece(&token)?);
                }
                return Ok(Piece::FNCALL(signature));
            }
            RTLToken::ListVal => {
                let mut signature = vec![Piece::IDENT(token.unwrap_identifier())];
                let mut token;
                loop {
                    token = self.advance_().clone();
                    if token.rtl_token == RTLToken::ExprEnd {
                        break;
                    }
                    signature.push(self._collect_piece(&token)?);
                }
                return Ok(Piece::LIST(signature));
            }
            RTLToken::TupleVal => todo!(),
            RTLToken::DictVal => todo!(),
            kind @ _ => anyhow::bail!("Found a token that should never be here: {}", kind),
        }
    }

    // Returns the array of nodes to which the current node being parsed would be added
    fn line_end_reached(&self) -> bool {
        // Rust's `||` short-circuits so this is valid
        return self.curr >= self.end - 1
            || self.tokens[self.curr + 1].line != self.tokens[self.curr].line;
    }

    fn push_node(&mut self, node: Node, to: &mut Vec<Node>) -> anyhow::Result<()> {
        if self.waiting_parents.is_empty() {
            to.push(node);
        } else {
            anyhow::ensure!(
                to.len() > 0,
                "There were no nodes in the parsed list, but the stack had {} elements!",
                self.waiting_parents.len()
            );
            let Some(last_root) = self.waiting_parents.peek() else {
                anyhow::bail!("There must be elements in waiting_parents if we reach this stage");
            };

            let mut to = to;
            for (i, idx) in last_root.iter().enumerate() {
                anyhow::ensure!(
                    *idx < to.len(),
                    "Tried to push to the node at position {}, but there were only {} nodes in {:?}!",
                    idx,
                    to.len(),
                    last_root
                );
                if i == last_root.len() - 1 {
                    let mut node_ = node.clone();
                    if to[*idx].kind == NodeKind::CONDTLY {
                        node_.line = node.line + 1;
                    }
                    to[*idx].children.push(node_);
                    break;
                }

                to = &mut to[*idx].children;
            }
        }

        Ok(())
    }

    fn _get_insert_loc(&mut self, within: &mut Vec<Node>) -> anyhow::Result<Vec<Node>> {
        match self.waiting_parents.peek() {
            Some(waiting_parents) => {
                anyhow::ensure!(
                    within.len() > 0,
                    "There were no nodes in the parsed list, but the stack had elements!",
                );

                let mut within = within;
                for (i, idx) in waiting_parents.iter().enumerate() {
                    anyhow::ensure!(
                        *idx < within.len(),
                        "Tried to retrieve the node at position {}, but there were only {} nodes!",
                        idx,
                        within.len(),
                    );
                    if i == waiting_parents.len() - 1 {
                        return Ok(within[*idx].children.clone());
                    }

                    within = &mut within[*idx].children;
                }

                anyhow::bail!("This should be unreachable code.");
            }

            None => Ok(within.clone()),
        }
    }

    // Returns and consumes the current point
    fn advance_(&mut self) -> &Token {
        // Ensure we never move past the end of our list
        let tmp = &self.tokens[self.curr];
        self.curr += 1;
        tmp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digraph::parser::Piece;

    mod macros {
        use super::NodeKind::*;
        use super::OpKind::*;
        use super::{Address, Node, NodeKind, OpKind, Piece};

        #[test]
        fn pieces_no_children() {
            let node = make_node!(line 1 -> GRABPKG [piece!(IDENT "pandas")]);
            let true_node = Node {
                line: 1,
                children: vec![],
                kind: GRABPKG,
                pieces: vec![piece!(IDENT "pandas")],
                addr: Address::new(vec![]),
                ..Default::default()
            };

            assert_eq!(node, true_node);
        }

        #[test]
        fn no_pieces_children() {
            let node = make_node!(line 3 -> CONDTLY []);
            let true_node = Node {
                line: 3,
                children: vec![],
                kind: NodeKind::CONDTLY,
                pieces: vec![],
                addr: Address::new(vec![]),
                ..Default::default()
            };

            assert_eq!(node, true_node);
        }

        #[test]
        fn pieces_children() {
            let made_node = make_node!(line 5 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                make_node!(line 2 -> OUTPUT [piece!(IDENT "x")]),
                make_node!(line 3 -> VARDECL [piece!(IDENT "my_age"), Piece::OP(ASSN), piece!(# 3)])
            });

            let true_node = Node {
                line: 5,
                kind: NodeKind::FNDEF,
                pieces: vec![Piece::IDENT("f".to_string()), Piece::IDENT("x".to_string())],
                addr: Address::new(vec![]),
                children: vec![
                    Node {
                        line: 2,
                        kind: NodeKind::OUTPUT,
                        pieces: vec![Piece::IDENT("x".to_string())],
                        children: vec![],
                        addr: Address::new(vec![]),
                        ..Default::default()
                    },
                    Node {
                        line: 3,
                        kind: NodeKind::VARDECL,
                        pieces: vec![
                            Piece::IDENT("my_age".to_string()),
                            Piece::OP(OpKind::ASSN),
                            Piece::NUMBER(3.),
                        ],
                        children: vec![],
                        addr: Address::new(vec![]),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            };

            assert_eq!(made_node, true_node);
        }

        #[test]
        fn pieces_children_addrs() {
            let made_node = make_node!(L 5 @ 1,2,3,0 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                make_node!(L 2 @ 0,0,2,1 -> OUTPUT [piece!(IDENT "x")]),
                make_node!(L 3 @ 1,2,3 -> VARDECL [piece!(IDENT "my_age"), Piece::OP(ASSN), piece!(# 3)])
            });

            let true_node = Node {
                line: 5,
                kind: NodeKind::FNDEF,
                pieces: vec![Piece::IDENT("f".to_string()), Piece::IDENT("x".to_string())],
                addr: Address::new(vec![1, 2, 3, 0]),
                children: vec![
                    Node {
                        line: 2,
                        kind: NodeKind::OUTPUT,
                        pieces: vec![Piece::IDENT("x".to_string())],
                        children: vec![],
                        addr: Address::new(vec![0, 0, 2, 1]),
                        ..Default::default()
                    },
                    Node {
                        line: 3,
                        kind: NodeKind::VARDECL,
                        pieces: vec![
                            Piece::IDENT("my_age".to_string()),
                            Piece::OP(OpKind::ASSN),
                            Piece::NUMBER(3.),
                        ],
                        children: vec![],
                        addr: Address::new(vec![1, 2, 3]),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            };

            assert_eq!(made_node, true_node);
        }
    }

    mod parser {
        use super::NodeKind::*;
        use super::*;

        #[test]
        fn imports() {
            let source = "grab pandas";

            let mut parser = Parser::new(String::from(source)).unwrap();
            assert_eq!(
                parser.parse().unwrap(),
                vec![make_node!(line 1 -> GRABPKG [piece!(IDENT "pandas")])]
            );
        }

        #[test]
        fn print() {
            let source = "output string hello world done";

            let mut parser = Parser::new(String::from(source)).unwrap();
            assert_eq!(
                parser.parse().unwrap(),
                vec![make_node!(line 1 -> OUTPUT [piece!(TEXT "hello world")])]
            );
        }

        #[test]
        fn function() {
            let source = "define f of x\noutput string hello world done\ndone define";

            let mut parser = Parser::new(String::from(source)).unwrap();
            assert_eq!(
                parser.parse().unwrap(),
                vec![
                    make_node!(line 1 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                        make_node!(line 2 -> OUTPUT [piece!(TEXT "hello world")])
                    })
                ]
            );
        }

        #[test]
        fn variables() {
            let source = "let x be 3\noutput string hello world done";

            let mut parser = Parser::new(String::from(source)).unwrap();
            assert_eq!(
                parser.parse().unwrap(),
                vec![
                    make_node!(line 1 -> VARDECL [piece!(IDENT "x"), Piece::OP(OpKind::ASSN), piece!(# 3)]),
                    make_node!(line 2 -> OUTPUT [piece!(TEXT "hello world")])
                ]
            );
        }

        #[test]
        fn conditional() {
            let source = "define f of x\nif x equals 3\noutput string hello world done\ndone if\n\
                          otherwise\noutput string bye world done\ndone otherwise\ndone define";

            let mut parser = Parser::new(String::from(source)).unwrap();
            assert_eq!(
                parser.parse().unwrap(),
                vec![
                    make_node!(line 1 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                        make_node!(line 2 -> CONDTL [piece!(IDENT "x"), Piece::OP(OpKind::EQ), piece!(# 3)]; {
                            make_node!(line 3 -> CONDTLY []; {
                                make_node!(line 4 -> OUTPUT [piece!(TEXT "hello world")])
                            }),
                            make_node!(line 5 -> CONDTLN []; {
                                make_node!(line 6 -> OUTPUT [piece!(TEXT "bye world")])
                            })
                        })
                    })
                ]
            );
        }

        #[test]
        fn r#for() {
            let source = "define f of x\nfor x in my_list\noutput x\ndone for\ndone define";

            let mut parser = Parser::new(source.to_string()).unwrap();
            assert_eq!(
                parser.parse().unwrap(),
                vec![
                    make_node!(line 1 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                        make_node!(line 2 -> FORLOOP [piece!(IDENT "x"), Piece::OP(OpKind::IN), piece!(IDENT "my_list")]; {
                            make_node!(line 3 -> OUTPUT [piece!(IDENT "x")])
                        })
                    })
                ]
            );
        }

        #[test]
        fn list() {
            let source = "let x be list 1 2 done";

            let mut parser = Parser::new(source.to_string()).unwrap();
            assert_eq!(
                parser.parse().unwrap(),
                vec![
                    make_node!(line 1 -> VARDECL [piece!(IDENT "x"), Piece::OP(OpKind::ASSN), 
                        piece!(LIST [piece!(IDENT "list"), piece!(# 1), piece!(# 2)])])
                ]
            );
        }

        #[test]
        fn indexing() {
            let source = "let x be mylist at 3";

            let mut parser = Parser::new(String::from(source)).unwrap();
            let tokens = parser.parse().unwrap();
            assert_eq!(
                tokens,
                vec![
                    make_node!(line 1 -> VARDECL [piece!(IDENT "x"), Piece::OP(OpKind::ASSN), piece!(IDENT "mylist"), Piece::OP(OpKind::AT), piece!(# 3)])
                ]
            );
        }

        #[test]
        fn compound() {
            let source = "define f of x\noutput x\nlet my_age be 3\ndone define\ndefine g of x\n\
                          output string hi done\ndone define\ndefine h of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

            let mut parser = Parser::new(String::from(source)).unwrap();
            let tokens = parser.parse().unwrap();
            assert_eq!(
                tokens,
                vec![
                    make_node!(line 1 -> FNDEF [piece!(IDENT "f"), piece!(IDENT "x")]; {
                        make_node!(line 2 -> OUTPUT [piece!(IDENT "x")]),
                        make_node!(line 3 -> VARDECL [piece!(IDENT "my_age"), Piece::OP(OpKind::ASSN), piece!(# 3)])
                    }),
                    make_node!(line 5 -> FNDEF [piece!(IDENT "g"), piece!(IDENT "x")]; {
                        make_node!(line 6 -> OUTPUT [piece!(TEXT "hi")])
                    }),
                    make_node!(line 8 -> FNDEF [piece!(IDENT "h"), piece!(IDENT "x")]; {
                        make_node!(line 9 -> OUTPUT [piece!(IDENT "x"), Piece::OP(OpKind::ADD), piece!(# 1)]),
                        make_node!(line 10 -> CONDTL [piece!(IDENT "x"), Piece::OP(OpKind::EQ), piece!(# 3)]; {
                            make_node!(line 11 -> CONDTLY []; {
                                make_node!(line 12 -> OUTPUT [piece!(IDENT "x")])
                            }),
                            make_node!(line 13 -> CONDTLN []; {
                                make_node!(line 14 -> OUTPUT [piece!(IDENT "y")])
                            })
                        })
                    }),
                ]
            );
        }

        #[test]
        fn linalg() {
            let source = "define inverse of m
define determinant of m
let x be m at 0 times m at 3
let y be m at 1 times m at 2
return x minus y
done define
define adjoint of m 
let result be list m at 3 0 minus m at 1 0 minus m at 2 m at 0 done 
return result
done define
let d be determinant of m done
let a be adjoint of m done
let iterator be range of 4 done
for i in iterator
let m at i be 1 over d times a at i
return m
done for
done define\ndefine g of x\noutput x plus 1\nif x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone define\ndefine start of args\nlet matrix be list 1 2 3 4 done\ndone define";
            let mut parser = Parser::new(String::from(source)).unwrap();
            let _tokens = parser.parse().unwrap();
        }
    }
}
