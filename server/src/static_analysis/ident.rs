use crate::digraph::{
    address::Address,
    parser::{Node, NodeKind, Piece},
    state::Canvas,
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
        args: Vec<String>,
        parent: Option<Parent<Ident>>,
        addr: Address,
        valid_idents: Vec<String>,
    },
}

impl Ident {
    pub(super) fn get_name<'a>(&'a self) -> &'a str {
        match self {
            Ident::Var { name, .. } | Ident::Fun { name, .. } => name,
        }
    }

    fn get_addr<'a>(&'a self) -> &'a Address {
        match self {
            Ident::Var { addr, .. } | Ident::Fun { addr, .. } => addr,
        }
    }

    fn get_parent(&self) -> Option<Parent<Ident>> {
        match self {
            Ident::Var { parent, .. } | Ident::Fun { parent, .. } => parent.clone(),
        }
    }

    fn get_valid_idents<'a>(&'a self) -> &'a [String] {
        match self {
            Ident::Var { valid_idents, .. } | Ident::Fun { valid_idents, .. } => valid_idents,
        }
    }

    fn set_valid_idents(&mut self, new: Vec<String>) {
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
        valid_names.contains(name)
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

#[derive(Debug)]
pub(crate) struct IDGraph {
    pub(super) graph: Vec<Child<Ident>>,
}

impl IDGraph {
    pub(crate) fn from_state(state: &Canvas) -> Self {
        let mut graph: Vec<Child<Ident>> = vec![];

        fn _inner(_node: &Node, parent: Option<Parent<Ident>>) -> Option<Child<Ident>> {
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

    pub(crate) fn populate_valid_idents(&self) {
        for node in &self.graph {
            Self::update_valid_idents(node.clone());
        }
    }

    fn update_valid_idents(node: Child<Ident>) {
        let mut valid_idents = Vec::new();

        // Get parent's valid_idents if available
        if let Some(parent_weak) = node.borrow().get_parent() {
            if let Some(parent_rc) = parent_weak.upgrade() {
                let parent = parent_rc.borrow();
                let p_valid_idents = parent.get_valid_idents();
                let p_name = parent.get_name();
                valid_idents.extend_from_slice(p_valid_idents);
                valid_idents.push(p_name.to_string());

                // If parent is a function, find this node's left siblings
                if let Ident::Fun { args, children, .. } = &*parent {
                    valid_idents.extend_from_slice(&args[..]);

                    if let Some(index) = children.iter().position(|c| Rc::ptr_eq(c, &node)) {
                        for left_sibling in &children[..index] {
                            let name = left_sibling.borrow();
                            let name = name.get_name();
                            valid_idents.push(name.to_string());
                        }
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
                Self::update_valid_idents(child.clone())
            }
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
        };

        let _dag = IDGraph::from_state(&state);
    }
}
