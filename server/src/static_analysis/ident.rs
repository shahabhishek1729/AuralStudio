use crate::digraph::{
    address::Address,
    parser::{Node, NodeKind, Piece},
    state::Canvas,
};
use serde::de::Deserialize;
use serde::ser::Serialize;
use serde::{Deserializer, Serializer};
use serde_derive::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    ops,
    rc::{Rc, Weak},
};

const GLOBALS: [&str; 156] = [
    "ArithmeticError",
    "AssertionError",
    "AttributeError",
    "BaseException",
    "BlockingIOError",
    "BrokenPipeError",
    "BufferError",
    "BytesWarning",
    "ChildProcessError",
    "ConnectionAbortedError",
    "ConnectionError",
    "ConnectionRefusedError",
    "ConnectionResetError",
    "DeprecationWarning",
    "EOFError",
    "Ellipsis",
    "EncodingWarning",
    "EnvironmentError",
    "Exception",
    "False",
    "FileExistsError",
    "FileNotFoundError",
    "FloatingPointError",
    "FutureWarning",
    "GeneratorExit",
    "IOError",
    "ImportError",
    "ImportWarning",
    "IndentationError",
    "IndexError",
    "InterruptedError",
    "IsADirectoryError",
    "KeyError",
    "KeyboardInterrupt",
    "LookupError",
    "MemoryError",
    "ModuleNotFoundError",
    "NameError",
    "None",
    "NotADirectoryError",
    "NotImplemented",
    "NotImplementedError",
    "OSError",
    "OverflowError",
    "PendingDeprecationWarning",
    "PermissionError",
    "ProcessLookupError",
    "RecursionError",
    "ReferenceError",
    "ResourceWarning",
    "RuntimeError",
    "RuntimeWarning",
    "StopAsyncIteration",
    "StopIteration",
    "SyntaxError",
    "SyntaxWarning",
    "SystemError",
    "SystemExit",
    "TabError",
    "TimeoutError",
    "True",
    "TypeError",
    "UnboundLocalError",
    "UnicodeDecodeError",
    "UnicodeEncodeError",
    "UnicodeError",
    "UnicodeTranslateError",
    "UnicodeWarning",
    "UserWarning",
    "ValueError",
    "Warning",
    "ZeroDivisionError",
    "_",
    "__build_class__",
    "__debug__",
    "__doc__",
    "__import__",
    "__loader__",
    "__name__",
    "__package__",
    "__spec__",
    "abs",
    "aiter",
    "all",
    "anext",
    "any",
    "ascii",
    "bin",
    "bool",
    "breakpoint",
    "bytearray",
    "bytes",
    "callable",
    "chr",
    "classmethod",
    "compile",
    "complex",
    "copyright",
    "credits",
    "delattr",
    "dict",
    "dir",
    "divmod",
    "enumerate",
    "eval",
    "exec",
    "exit",
    "filter",
    "float",
    "format",
    "frozenset",
    "getattr",
    "globals",
    "hasattr",
    "hash",
    "help",
    "hex",
    "id",
    "input",
    "int",
    "isinstance",
    "issubclass",
    "iter",
    "len",
    "license",
    "list",
    "locals",
    "map",
    "max",
    "memoryview",
    "min",
    "next",
    "object",
    "oct",
    "open",
    "ord",
    "pow",
    "print",
    "property",
    "quit",
    "range",
    "repr",
    "reversed",
    "round",
    "set",
    "setattr",
    "slice",
    "sorted",
    "staticmethod",
    "str",
    "sum",
    "super",
    "tuple",
    "type",
    "vars",
    "zip",
];

type Child<T> = Rc<RefCell<T>>;
type Parent<T> = Weak<RefCell<T>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) enum Ident {
    Var {
        name: String,
        #[serde(skip)]
        parent: Option<Parent<Ident>>,
        addr: Address,
        valid_idents: Vec<(String, Option<String>)>,
        val: Option<String>,
    },
    Fun {
        name: String,
        #[serde(deserialize_with = "deserialize_child")]
        #[serde(serialize_with = "serialize_child")]
        children: Vec<Child<Ident>>,
        args: Vec<String>,
        #[serde(skip)]
        parent: Option<Parent<Ident>>,
        addr: Address,
        valid_idents: Vec<(String, Option<String>)>,
    },
}

fn serialize_child<S, T>(items: &Vec<Child<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let v: Vec<_> = items.iter().map(|rc| rc.borrow()).collect();
    let v: Vec<&T> = v.iter().map(|r| &**r).collect();
    v.serialize(serializer)
}

fn deserialize_child<'de, D, T>(deserializer: D) -> Result<Vec<Child<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v: Vec<T> = Deserialize::deserialize(deserializer)?;
    Ok(v.into_iter()
        .map(|item| Rc::new(RefCell::new(item)))
        .collect())
}

impl Ident {
    pub(super) fn get_name<'a>(&'a self) -> &'a str {
        match self {
            Ident::Var { name, .. } | Ident::Fun { name, .. } => name,
        }
    }

    pub(super) fn get_value(&self) -> Option<String> {
        match self {
            Ident::Var { val, .. } => val.clone(),
            _ => None,
        }
    }

    fn get_addr<'a>(&'a self) -> &'a Address {
        match self {
            Ident::Var { addr, .. } | Ident::Fun { addr, .. } => addr,
        }
    }

    fn get_valid_idents<'a>(&'a self) -> &'a [(String, Option<String>)] {
        match self {
            Ident::Var { valid_idents, .. } | Ident::Fun { valid_idents, .. } => valid_idents,
        }
    }

    fn set_valid_idents(&mut self, new: Vec<(String, Option<String>)>) {
        match self {
            Ident::Var {
                ref mut valid_idents,
                ..
            }
            | Ident::Fun {
                ref mut valid_idents,
                ..
            } => *valid_idents = new,
        }
    }

    pub(super) fn is_valid(&self, name: &String) -> bool {
        let valid_names = match self {
            Ident::Var { valid_idents, .. } => valid_idents,
            Ident::Fun { valid_idents, .. } => valid_idents,
        };
        valid_names.iter().any(|(x, _)| x == name) || GLOBALS.iter().any(|&s| s == name)
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Ident::Var {
                    name: n1,
                    parent: p1,
                    addr: a1,
                    ..
                },
                Ident::Var {
                    name: n2,
                    parent: p2,
                    addr: a2,
                    ..
                },
            ) => {
                n1 == n2
                    && a1 == a2
                    && match (p1, p2) {
                        (Some(r1), Some(r2)) => r1.as_ptr() == r2.as_ptr(),
                        (None, None) => true,
                        _ => false,
                    }
            }
            (
                Ident::Fun {
                    name: n0,
                    children: c0,
                    args: arg0,
                    parent: p0,
                    addr: a0,
                    ..
                },
                Ident::Fun {
                    name: n1,
                    children: c1,
                    args: arg1,
                    parent: p1,
                    addr: a1,
                    ..
                },
            ) => {
                n0 == n1
                    && arg0 == arg1
                    && a0 == a1
                    && c0 == c1
                    && match (p0, p1) {
                        (Some(r0), Some(r1)) => r0.as_ptr() == r1.as_ptr(),
                        (None, None) => true,
                        _ => false,
                    }
            }
            _ => return false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IDGraph {
    #[serde(serialize_with = "serialize_child")]
    #[serde(deserialize_with = "deserialize_child")]
    pub(super) graph: Vec<Child<Ident>>,
}

impl IDGraph {
    pub(crate) fn from_state(state: &Canvas) -> Self {
        let mut graph: Vec<Child<Ident>> = vec![];

        fn _inner(
            _node: &Node,
            parent: Option<Parent<Ident>>,
            state: &Canvas,
        ) -> Option<Child<Ident>> {
            match _node.kind {
                NodeKind::FNDEF => {
                    let Piece::IDENT(ref name) = _node.pieces[0] else {
                        return None;
                    };

                    let args = _node
                        .pieces
                        .iter()
                        .skip(1) // Skip the function name which is piece #1
                        .map(|p| {
                            let Piece::IDENT(s) = p else {
                                unreachable!("FNDEF can only have IDENT pieces");
                            };
                            s.into()
                        })
                        .collect::<Vec<_>>();

                    let addr = _node.addr.clone();

                    let fun = Rc::new(RefCell::new(Ident::Fun {
                        name: name.into(),
                        args,
                        children: vec![],
                        parent: parent.clone(),
                        addr,
                        valid_idents: vec![],
                    }));

                    let children = _node
                        .children
                        .iter()
                        .filter_map(|child| _inner(child, Some(Rc::downgrade(&fun)), state))
                        .collect::<Vec<_>>();

                    if let Ident::Fun {
                        children: ref mut c,
                        ..
                    } = &mut *fun.borrow_mut()
                    {
                        *c = children;
                    }

                    Some(fun)
                }
                NodeKind::VARDECL => {
                    let Piece::IDENT(ref name) = _node.pieces[0] else {
                        return None;
                    };

                    let addr = _node.addr.clone();
                    Some(Rc::new(RefCell::new(Ident::Var {
                        name: name.into(),
                        parent,
                        addr,
                        valid_idents: vec![],
                        val: None,
                    })))
                }
                _ => None,
            }
        }

        for node in state.graph.iter() {
            if let Some(ident) = _inner(node, None, state) {
                graph.push(ident);
            }
        }

        Self { graph }
    }

    pub(crate) fn populate_valid_idents(&self) {
        for node in &self.graph {
            Self::update_valid_idents(node.clone(), None);
        }
    }

    fn update_valid_idents(node: Child<Ident>, parent: Option<&Ident>) {
        let mut valid_idents: Vec<(String, Option<String>)> = vec![];

        // Get parent's valid_idents if available
        // let parent = parent.borrow();
        if let Some(parent) = parent {
            let p_valid_idents = parent.get_valid_idents();
            let p_name = parent.get_name();
            let p_val = parent.get_value();
            valid_idents.extend_from_slice(p_valid_idents);
            valid_idents.push((p_name.to_string(), p_val));

            // XXX: NOTE: Optimization possible
            // If parent is a function, find this node's left siblings
            if let Ident::Fun { args, children, .. } = &*parent {
                valid_idents.extend(args.iter().map(|v| (v.clone(), None)));

                if let Some(index) = children.iter().position(|c| Rc::ptr_eq(c, &node)) {
                    for left_sibling in &children[..index] {
                        let name = left_sibling.borrow();
                        let val = name.get_value();
                        let name = name.get_name();
                        valid_idents.push((name.to_string(), val));
                    }
                }
            }
        }

        {
            // Update the node's valid_idents
            let mut node_mut = node.borrow_mut();
            node_mut.set_valid_idents(valid_idents);
        }

        let node = node.borrow();
        if let Ident::Fun { children, .. } = &*node {
            for child in children {
                Self::update_valid_idents(child.clone(), Some(&*node))
            }
        }
    }

    pub(super) fn get_hash(&self) -> (BTreeMap<Address, Ident>, HashMap<String, usize>) {
        let mut hm: BTreeMap<Address, Ident> = BTreeMap::new();
        let mut args_hm: HashMap<String, usize> = HashMap::new();
        // Recursively traverse the graph, adding each node to the map and handling its subtree.
        fn _inner(
            node: Child<Ident>,
            _hm: &mut BTreeMap<Address, Ident>,
            _args_hm: &mut HashMap<String, usize>,
        ) {
            let node = node.borrow().clone();
            let addr = node.get_addr();
            _hm.insert(addr.clone(), node.clone());
            if let Ident::Fun {
                name,
                children,
                args,
                ..
            } = node
            {
                _args_hm.insert(name, args.len());
                children
                    .into_iter()
                    .for_each(|n| _inner(n.clone(), _hm, _args_hm));
            }
        }
        self.graph
            .iter()
            .for_each(|n| _inner(n.clone(), &mut hm, &mut args_hm));
        (hm, args_hm)
    }

    pub(super) fn get_hash_mut(&mut self) -> HashMap<Address, Child<Ident>> {
        let mut hm: HashMap<Address, Child<Ident>> = HashMap::new();
        // Recursively traverse the graph, adding each node to the map and handling its subtree.
        fn _inner(node: Child<Ident>, _hm: &mut HashMap<Address, Child<Ident>>) {
            let node_ = node.borrow().clone();
            let addr = node_.get_addr();
            _hm.insert(addr.clone(), node.clone());
            if let Ident::Fun { children, .. } = node_ {
                children.into_iter().for_each(|n| _inner(n.clone(), _hm));
            }
        }
        self.graph.iter().for_each(|n| _inner(n.clone(), &mut hm));
        hm
    }
}

pub(super) fn serialize_piece(state: &Canvas, piece: &Piece, graph: &IDGraph) -> String {
    match piece {
        Piece::TEXT(s) => format!("\"{}\"", s),
        Piece::NUMBER(n) => n.to_string(),
        Piece::IDENT(s) => {
            let _dag = graph;
            let (hash, _) = _dag.get_hash();

            if let Some((_, ident)) = hash
                .range((
                    ops::Bound::Unbounded,
                    ops::Bound::Included(&state.block_loc),
                ))
                .next_back()
            {
                if ident.get_name() == s {
                    let Some(val) = ident.get_value() else {
                        return s.into();
                    };
                    return val;
                }

                let valid_idents = ident.get_valid_idents();
                if let Some(val) = valid_idents
                    .iter()
                    .filter(|(name, _)| name == s)
                    .next()
                    .unwrap_or(&("".into(), None))
                    .1
                    .clone()
                {
                    return val;
                } else {
                    return s.into();
                }
            } else {
                return s.into();
            }
        }
        Piece::BOOL(b) => (if *b { "True" } else { "False" }).into(),
        Piece::NOTHING => "None".into(),
        Piece::OP(op) => match op {
            crate::digraph::parser::OpKind::ADD => "+",
            crate::digraph::parser::OpKind::SUB => "-",
            crate::digraph::parser::OpKind::MUL => "*",
            crate::digraph::parser::OpKind::DIV => "/",
            crate::digraph::parser::OpKind::INTDIV => "//",
            crate::digraph::parser::OpKind::MOD => "%",
            crate::digraph::parser::OpKind::EQ => "==",
            crate::digraph::parser::OpKind::NE => "!=",
            crate::digraph::parser::OpKind::GT => ">",
            crate::digraph::parser::OpKind::LT => "<",
            crate::digraph::parser::OpKind::GE => ">=",
            crate::digraph::parser::OpKind::LE => "<=",
            crate::digraph::parser::OpKind::ASSN => "=",
            crate::digraph::parser::OpKind::NOT => "not",
            crate::digraph::parser::OpKind::IN => "in",
            crate::digraph::parser::OpKind::DOT => ".",
            crate::digraph::parser::OpKind::AT => todo!(),
        }
        .into(),
        Piece::FNCALL(args) => format!(
            "{}({})",
            serialize_piece(state, &args[0], graph),
            args[1..]
                .iter()
                .map(|x| serialize_piece(state, x, graph))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Piece::LIST(args) => format!(
            "[{}]",
            args[1..]
                .iter()
                .map(|x| serialize_piece(state, x, graph))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Piece::PendingVal => "...".into(),
        Piece::PendingOp => unimplemented!("Pending operators result in crashes"),
        Piece::NULL => unreachable!("NULL piece is a placeholder, should never appear here."),
    }
}

pub(super) fn anglicize_piece(piece: &Piece) -> String {
    match piece {
        Piece::TEXT(s) => format!("string {} done", s),
        Piece::NUMBER(n) => n.to_string(),
        Piece::IDENT(s) => s.into(),
        Piece::BOOL(b) => (if *b { "true" } else { "false" }).into(),
        Piece::NOTHING => "None".into(),
        Piece::OP(op) => match op {
            crate::digraph::parser::OpKind::ADD => "plus",
            crate::digraph::parser::OpKind::SUB => "minus",
            crate::digraph::parser::OpKind::MUL => "times",
            crate::digraph::parser::OpKind::DIV => "divided by",
            crate::digraph::parser::OpKind::INTDIV => "floor divided by",
            crate::digraph::parser::OpKind::MOD => "modulo",
            crate::digraph::parser::OpKind::EQ => "equal to",
            crate::digraph::parser::OpKind::NE => "not equal to",
            crate::digraph::parser::OpKind::GT => "greater than",
            crate::digraph::parser::OpKind::LT => "less than",
            crate::digraph::parser::OpKind::GE => "greater than or equal to",
            crate::digraph::parser::OpKind::LE => "less than or equal to",
            crate::digraph::parser::OpKind::ASSN => "equals",
            crate::digraph::parser::OpKind::NOT => "not",
            crate::digraph::parser::OpKind::IN => "in",
            crate::digraph::parser::OpKind::DOT => "dot",
            crate::digraph::parser::OpKind::AT => "at",
        }
        .into(),
        Piece::FNCALL(args) => format!(
            "{} of {} done",
            anglicize_piece(&args[0]),
            args[1..]
                .iter()
                .map(|x| anglicize_piece(x))
                .collect::<Vec<_>>()
                .join(" and ")
        ),
        Piece::LIST(args) => format!(
            "list of {}",
            args[1..]
                .iter()
                .map(|x| anglicize_piece(x))
                .collect::<Vec<_>>()
                .join(" and ")
        ),
        Piece::PendingVal => "...".into(),
        Piece::PendingOp => unimplemented!("Pending operators result in crashes"),
        Piece::NULL => unreachable!("NULL piece is a placeholder, should never appear here."),
    }
}

pub(super) fn serialize_expr(state: &Canvas, expr: &[Piece], graph: &IDGraph) -> String {
    expr.iter()
        .map(|x| serialize_piece(state, x, graph))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn anglicize_expr(expr: &[Piece]) -> String {
    expr.iter()
        .map(|x| anglicize_piece(x))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn eval_expr(state: &Canvas, expr: &[Piece], graph: &IDGraph) -> String {
    let expr = serialize_expr(state, expr, graph);
    let Ok(process) = std::process::Command::new("python3")
        .arg("-c")
        .arg(format!("print(repr(eval('{expr}')))"))
        .output()
    else {
        panic!()
    };

    let mut out = String::from_utf8(process.stdout).expect("stdout conversion can't fail");
    let _ = out.pop();
    let err = String::from_utf8(process.stderr).expect("stdout conversion can't fail");

    // XXX: NOTE: We are assuming there is either output or an error, since a single line of
    // Python can usually not produce both simultaneously ("output" can't be used in an expr)
    format! {"{}{}", out, err}
}

pub(super) fn format_expr(state: &Canvas, expr: &[Piece], graph: &IDGraph) -> String {
    let angl = anglicize_expr(expr);
    let eval = eval_expr(state, expr, graph);
    let el = eval.len() - 1;
    format!(
        "{}{}",
        angl,
        if angl == eval || angl == format!("string {} done", &eval[1..el]) {
            "".into()
        } else {
            format!(", which is {eval}")
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addr;
    use crate::digraph::address::Addressable;
    use crate::digraph::parser::Parser;
    use crate::digraph::state::ADMode;
    use crate::digraph::state::Canvas;

    const SOURCE: &'static str = "define f of x\noutput x\ndefine f1 of x\noutput x\ndone define\ndefine \
                          f2 of x\noutput x\ndefine f21 of x\noutput x\ndone define\ndefine f22 of \
                          x\noutput x\ndone define\ndone define\ndone define\ndefine g of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

    #[test]
    fn dag_compiles() {
        let mut parser = Parser::new(String::from(SOURCE)).unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = Canvas {
            filename: "".into(),
            block_loc: addr!(0, 0),
            node_loc: addr!(0, 0)
                .coerce(&nodes.get_hash())
                .expect("Coercion should work"),
            mode: ADMode::VIEW,
            graph: nodes.to_vec(),
            piece_ix: None,
            output: None,
            err: None,
            no_edit: false,
        };

        let _dag = IDGraph::from_state(&state);
    }

    #[test]
    fn ident_vals() {
        let mut parser = Parser::new(String::from(
            "define start of args\nlet x be 2\noutput x\ndone define",
        ))
        .unwrap();
        let mut nodes = parser.parse().unwrap();
        (&mut nodes[..]).fill_addr();
        let state = Canvas {
            filename: "".into(),
            block_loc: addr!(0, 0, 2),
            node_loc: addr!(0, 0, 2)
                .coerce(&nodes.get_hash())
                .expect("Coercion should work"),
            mode: ADMode::VIEW,
            graph: nodes.to_vec(),
            piece_ix: None,
            output: None,
            err: None,
            no_edit: false,
        };

        let _dag = IDGraph::from_state(&state);
    }
}
