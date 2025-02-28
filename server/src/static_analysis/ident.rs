use crate::digraph::{
    address::Address,
    parser::{Node, NodeKind, Piece},
    state::CursorState,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

type Child<T> = Rc<RefCell<T>>;
type Parent<T> = Weak<RefCell<T>>;

#[derive(Debug, Clone)]
pub(super) enum Ident {
    Var {
        name: String,
        parent: Option<Parent<Ident>>,
        addr: Address,
        valid_idents: Vec<String>,
    },
    Fun {
        name: String,
        children: Vec<Child<Ident>>,
        n_args: usize,
        parent: Option<Parent<Ident>>,
        addr: Address,
        valid_idents: Vec<String>,
    },
}

impl Ident {
    pub(super) fn get_name<'a>(&'a self) -> &'a str {
        match self {
            Ident::Var { name, .. } => name,
            Ident::Fun { name, .. } => name,
        }
    }

    fn get_addr<'a>(&'a self) -> &'a Address {
        match self {
            Ident::Var { addr, .. } => addr,
            Ident::Fun { addr, .. } => addr,
        }
    }

    pub(super) fn is_valid(&self) -> bool {
        let valid_names = match self {
            Ident::Var { valid_idents, .. } => valid_idents,
            Ident::Fun { valid_idents, .. } => valid_idents,
        };
        valid_names.contains(&self.get_name().into())
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
                    name: n1,
                    children: c1,
                    n_args: na1,
                    parent: p1,
                    addr: a1,
                    ..
                },
                Ident::Fun {
                    name: n2,
                    children: c2,
                    n_args: na2,
                    parent: p2,
                    addr: a2,
                    ..
                },
            ) => {
                n1 == n2
                    && na1 == na2
                    && a1 == a2
                    && c1 == c2
                    && match (p1, p2) {
                        (Some(r1), Some(r2)) => r1.as_ptr() == r2.as_ptr(),
                        (None, None) => true,
                        _ => false,
                    }
            }
            _ => return false,
        }
    }
}

#[derive(Debug)]
pub(super) struct IDGraph {
    pub(super) graph: Vec<Child<Ident>>,
}

impl IDGraph {
    pub(super) fn from_state(state: &CursorState) -> Self {
        let mut graph: Vec<Child<Ident>> = vec![];

        fn _inner(_node: &Node, parent: Option<Parent<Ident>>) -> Option<Child<Ident>> {
            match _node.kind {
                NodeKind::FNDEF => {
                    let Piece::IDENT(ref name) = _node.pieces[0] else {
                        return None;
                    };
                    let n_args = _node.pieces.len() - 1;
                    let addr = _node.addr.clone();

                    let fun = Rc::new(RefCell::new(Ident::Fun {
                        name: name.into(),
                        n_args,
                        children: vec![],
                        parent: parent.clone(),
                        addr,
                        valid_idents: vec![],
                    }));

                    let children = _node
                        .children
                        .iter()
                        .filter_map(|child| _inner(child, Some(Rc::downgrade(&fun))))
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
                    })))
                }
                _ => None,
            }
        }

        for node in state.graph.iter() {
            if let Some(ident) = _inner(node, None) {
                graph.push(ident);
            }
        }

        Self { graph }
    }

    fn valid_identifiers(&self) {
        fn _inner(_acc: &mut Vec<String>, ident: &mut Ident, full_graph: &Vec<Child<Ident>>) {
            // Appends `ident`'s parent and all LHS siblings to `_acc`
            let Some(ref p) = (match ident {
                Ident::Var { ref parent, .. } => parent,
                Ident::Fun { ref parent, .. } => parent,
            }) else {
                _acc.extend(full_graph.iter().map(|c| c.borrow().get_name().into()));
                return;
            };

            let p = unsafe { p.as_ptr().read() };
            let p = &*(p.borrow());
            match p {
                Ident::Fun { name, children, .. } => {
                    // Append parent name
                    _acc.push(name.into());
                    // Append all LHS siblings' names
                    for child in children.iter() {
                        let child = child.borrow();
                        if *child == *ident {
                            break;
                        }
                        _acc.push(child.get_name().into());
                    }
                }
                _ => unreachable!("parent must be function"),
            }

            match ident {
                Ident::Var {
                    ref mut valid_idents,
                    ..
                } => *valid_idents = _acc.to_vec(),
                Ident::Fun {
                    ref mut valid_idents,
                    ref children,
                    ..
                } => {
                    *valid_idents = _acc.to_vec();
                    for child in children {
                        _inner(_acc, &mut child.borrow_mut(), &vec![])
                    }
                }
            }
        }

        for node in self.graph.iter() {
            _inner(&mut vec![], &mut node.borrow_mut(), &self.graph);
        }
    }

    pub(super) fn get_hash(&self) -> (HashMap<Address, Ident>, HashMap<String, usize>) {
        let mut hm: HashMap<Address, Ident> = HashMap::new();
        let mut args_hm: HashMap<String, usize> = HashMap::new();
        // Recursively traverse the graph, adding each node to the map and handling its subtree.
        fn _inner(
            node: Child<Ident>,
            _hm: &mut HashMap<Address, Ident>,
            _args_hm: &mut HashMap<String, usize>,
        ) {
            let node = node.borrow().clone();
            let addr = node.get_addr();
            _hm.insert(addr.clone(), node.clone());
            if let Ident::Fun {
                name,
                children,
                n_args,
                ..
            } = node
            {
                _args_hm.insert(name, n_args);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addr;
    use crate::digraph::address::Addressable;
    use crate::digraph::parser::Parser;
    use crate::digraph::state::ADMode;
    use crate::digraph::state::CursorState;

    macro_rules! move_cursor {
        // Move sequences that result in an attempted move "off the graph" should return errors
        ($($dir:tt),+ _in_ $src:ident -> $err:tt) => {{
            let mut parser = Parser::new(String::from($src)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();
            let mut state = CursorState {
                filename: "".into(),
                block_loc: addr!(0, 0),
                node_loc: addr!(0, 0).coerce(&nodes.get_hash()).expect("Coercion should work"),
                mode: ADMode::VIEW,
                graph: nodes.to_vec(),
                piece_ix: None,
                output: None,
            };
            let mut failed = false;
            $(
                let coerced = state.block_loc.coerce(&(state.graph).get_hash()).expect("Coercion should work");
                let dst = if coerced != state.block_loc && GLOBAL_BLOCKS.contains(&(state.graph).get_hash().get(&coerced).expect("Retrieval should work").kind) {
                    move_cursor!($dir).move_global(&state)
                } else {
                    move_cursor!($dir).move_local(&state)
                };
                failed = failed || matches!(dst, Err($err(_)));
                if let Ok(dst) = dst {
                    state.block_loc = dst;
                    let _ = state.coerce().expect("Post-motion coercion should succeed");
                }
            )+
            assert!(failed);
        }};
        ($($dir:tt),+ _in_ $src:ident -> <$($id:literal),+>) => {{
            let mut parser = Parser::new(String::from($src)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();
            let mut state = CursorState {
                filename: "".into(),
                block_loc: addr!(0, 0),
                node_loc: addr!(0, 0).coerce(&nodes.get_hash()).expect("Coercion should work"),
                mode: ADMode::VIEW,
                graph: nodes.to_vec(),
                piece_ix: None,
                output: None,
            };
            $(
                let coerced = state.block_loc.coerce(&(state.graph).get_hash()).expect("Coercion should work");
                let dst = if coerced != state.block_loc && GLOBAL_BLOCKS.contains(&(state.graph).get_hash().get(&coerced).expect("Retrieval should work").kind) {
                    move_cursor!($dir).move_global(&state).expect("Motion should succeed")
                } else {
                    move_cursor!($dir).move_local(&state).expect("Motion should succeed")
                };
                state.block_loc = dst.clone();
                let _ = state.coerce().expect("Post-motion coercion should succeed");
            )+
            assert_eq!(dst, addr!($($id),+));
        }};
        // Cycles are sequences of motions that end up at the leftmost root (i.e., <0, 0>)
        ($($dir:tt),+ _in_ $src:ident ||) => {{
            move_cursor!($($dir),+ _in_ $src -> <0, 0>);
        }};
        ($($dir:tt),+ _in_ $src:ident ##) => {{
            move_cursor!($($dir),+ _in_ $src -> <1, 0>);
        }};
        (U) => {$crate::digraph::state::CursorDir::UP};
        (D) => {$crate::digraph::state::CursorDir::DOWN};
        (L) => {$crate::digraph::state::CursorDir::LEFT};
        (R) => {$crate::digraph::state::CursorDir::RIGHT};
        (I) => {$crate::digraph::state::CursorDir::IN};
        (O) => {$crate::digraph::state::CursorDir::OUT};
    }

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
        let state = CursorState {
            filename: "".into(),
            block_loc: addr!(0, 0),
            node_loc: addr!(0, 0)
                .coerce(&nodes.get_hash())
                .expect("Coercion should work"),
            mode: ADMode::VIEW,
            graph: nodes.to_vec(),
            piece_ix: None,
            output: None,
        };

        let dag = IDGraph::from_state(&state);
    }
}
