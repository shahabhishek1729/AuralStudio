use crate::check;
use crate::digraph::address::{Address, Addressable};
use crate::digraph::parser::NodeKind;
use crate::Node;
use anyhow;
use phf::phf_map;
use std::collections::HashMap;
use thiserror::Error;

static N_ROOT_CHILDREN: phf::Map<&'static str, u8> = phf_map! {
    "CONDTL" => 2,
    "CONDTLY" => 1,
    "CONDTLN" => 1,
    "WHLLOOP" => 1,
    "FORLOOP" => 1,
    "FNDEF" => 1,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum CursorDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    IN,
    OUT,
}

impl std::fmt::Display for CursorDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, Error)]
pub(crate) enum CursorError {
    #[error("the address {0} does not exist in this tree")]
    InvalidAddress(Address),
    #[error("the motion {0} is not available at this position")]
    InvalidMotion(CursorDir),
    #[error("Couldn't find address: {} in the file", .0)]
    AddrNotFound(Address),
}

impl CursorDir {
    fn _modifying_val(&self) -> bool {
        match self {
            CursorDir::UP | CursorDir::LEFT => false,
            CursorDir::DOWN | CursorDir::RIGHT => true,
            _ => unreachable!("Cannot retrieve modifying value for a IN or OUT operation"),
        }
    }

    fn _parity_val(&self) -> usize {
        match self {
            CursorDir::UP | CursorDir::DOWN => 1,
            CursorDir::LEFT | CursorDir::RIGHT => 0,
            _ => unreachable!("Cannot retrieve modifying value for a IN or OUT operation"),
        }
    }

    fn _ensure_global_validity(&self, src: &Address) -> anyhow::Result<(), CursorError> {
        // 0.0.0, 1.0.0, 2.0.0.0, etc. -> can't go up anymore without hitting the root
        if *self == CursorDir::UP && src[1..].iter().sum::<usize>() == 0 {
            return Err(CursorError::InvalidMotion(*self));
        }

        // If the length is even, the last index is odd (i.e., represents a level). If this is not
        // true, this must mean there are no levels below the current block in the current subtree.
        if *self == CursorDir::DOWN && src.len() % 2 == 1 {
            return Err(CursorError::InvalidMotion(*self));
        }

        Ok(())
    }

    fn _find_parent_child<'a>(
        &self,
        state: &'a CursorState,
        src: &'a Address,
    ) -> Result<(&'a Node, usize), CursorError> {
        let src = src.coerce(&state.graph_hash)?;
        let Some(&node) = state.graph_hash.get(&src) else {
            return Err(CursorError::InvalidAddress(src.clone()));
        };
        let Some(ref parent_addr) = node.parent_addr else {
            // This node is a FNDEF/CLSDECL; cannot go up without going out first.
            return Err(CursorError::InvalidMotion(*self));
        };
        let Some(&parent) = state
            .graph_hash
            .get(&parent_addr.coerce(&state.graph_hash)?)
        else {
            return Err(CursorError::InvalidAddress(parent_addr.clone()));
        };

        let (i, curr_node): (usize, &Node) = parent
            .children
            .iter()
            .enumerate()
            .filter(|(_, n)| n.addr == src)
            .next()
            .unwrap();
        assert_eq!(curr_node, node);

        Ok((parent, i))
    }

    fn move_global<'a>(&'a self, state: &CursorState<'a>) -> anyhow::Result<Address, CursorError> {
        let src = &state.block_loc;

        self._ensure_global_validity(src)?;

        match self {
            CursorDir::UP | CursorDir::DOWN | CursorDir::LEFT | CursorDir::RIGHT => {
                let mut dst = src.clone().addr;
                let l = dst.len();
                for i in (0..l).filter(|x| x % 2 == self._parity_val()).rev() {
                    // If we're going up, make sure we can and move. Otherwise, go down.
                    check!(self._modifying_val() => dst[i] += 1 ; check!(dst[i] > 0 => dst[i] -= 1 ; continue));
                    let dst = Address::new(dst[0..(i + 1)].to_vec());

                    // Ensure the motion worked (i.e., not going right from the rightmost block).
                    let Ok(mut dst) = dst.coerce(&state.graph_hash) else {
                        return Err(CursorError::InvalidMotion(*self));
                    };
                    let _ = dst.addr.pop();

                    return Ok(dst);
                }
                return Err(CursorError::InvalidMotion(*self));
            }
            CursorDir::IN => {
                let dst = src.coerce(&state.graph_hash)?;
                Ok(dst)
            }
            _ => return Err(CursorError::InvalidMotion(*self)),
        }
    }

    fn move_local<'cur>(
        &'cur self,
        state: &CursorState<'cur>,
    ) -> anyhow::Result<Address, CursorError> {
        let src: &Address = &state.block_loc;

        // When going up and down, no coercion - check children for local blocks
        // Left and right involves finding the parent's next child
        // In and out involves directly finding parent and child and moving (up until global block)
        match self {
            CursorDir::UP => {
                let (parent, i) = self._find_parent_child(state, src)?;
                // If this is the parent's root child, go out and coerce to the nearest node
                if i < N_ROOT_CHILDREN[&format!("{:?}", parent.kind)] as usize {
                    let dst = CursorDir::OUT.move_local(state)?;
                    let dst = dst.coerce(&state.graph_hash)?;
                    return Ok(dst);
                }
                // Return the parent's previous child -> this should be the one right above
                Ok(parent.children[i - 1].addr.clone())
            }
            CursorDir::DOWN => {
                let Some(&node) = state.graph_hash.get(&src.coerce(&state.graph_hash)?) else {
                    return Err(CursorError::InvalidAddress(src.clone()));
                };
                if N_ROOT_CHILDREN.contains_key(&format!("{:?}", node.kind)[..]) {
                    return Ok(node.children[0].addr.clone());
                }

                let (parent, i) = self._find_parent_child(state, src)?;
                if i == parent.children.len() - 1 {
                    return Err(CursorError::InvalidMotion(*self));
                }

                // Return the parent's next child -> this should be the one right below
                // NOTE: XXX: Need the returned address always have the same length as `src`?
                Ok(Address::new(
                    parent.children[i + 1].addr.addr[..src.len()].to_vec(),
                ))
            }
            CursorDir::LEFT => {
                let (parent, i) = self._find_parent_child(state, src)?;
                match parent.children[i].kind {
                    NodeKind::CONDTLN => Ok(parent.children[i - 1].addr.clone()),
                    _ => Err(CursorError::InvalidMotion(*self)),
                }
            }
            CursorDir::RIGHT => {
                let (parent, i) = self._find_parent_child(state, src)?;
                match parent.children[i].kind {
                    NodeKind::CONDTLY => Ok(parent.children[i + 1].addr.clone()),
                    _ => Err(CursorError::InvalidMotion(*self)),
                }
            }
            CursorDir::IN => {
                // let mut src = &mut src.clone();
                // src.addr.push(0);
                // let Ok(_) = src.coerce(&state.graph_hash) else {
                //     return Err(CursorError::InvalidMotion(*self));
                // };
                // Ok(src.clone())
                self.move_global(state)
            }
            CursorDir::OUT => {
                let mut src = &mut src.clone();
                let _ = src.addr.pop();
                Ok(src.clone())
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct CursorState<'dag> {
    pub(crate) block_loc: Address,
    pub(crate) node_loc: Address,
    graph: &'dag [Node],
    graph_hash: HashMap<Address, &'dag Node>,
}

impl<'dag> CursorState<'dag> {
    pub(crate) fn new(
        graph: &'dag [Node],
        graph_hash: HashMap<Address, &'dag Node>,
    ) -> Result<Self, CursorError> {
        let block_loc = Address::new(vec![0, 0]);
        let node_loc = block_loc.coerce(&graph.get_hash())?;

        Ok(Self {
            block_loc,
            node_loc,
            graph,
            graph_hash,
        })
    }

    fn _move_cursor_node(&mut self, dir: CursorDir) -> Result<(), ()> {
        let Some(node) = self.graph_hash.get(&self.block_loc) else {
            return Err(());
        };

        match node.kind {
            NodeKind::CONDTLY | NodeKind::CONDTLN => {}
            _ => {
                if dir == CursorDir::LEFT || dir == CursorDir::RIGHT {
                    return Err(());
                }
            }
        }
        if node.kind == NodeKind::CONDTLY || node.kind == NodeKind::CONDTLN {
            // All motions are allowed
        } else {
            // Only vertial motions are allowed
            if dir == CursorDir::LEFT || dir == CursorDir::RIGHT {
                return Err(());
            }
        }

        Ok(())
    }

    fn coerce(&mut self) -> Result<(), CursorError> {
        self.node_loc = self.block_loc.coerce(&self.graph_hash)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::addr;
    use crate::digraph::address::{Address, Addressable};
    use crate::digraph::parser::NodeKind;
    use crate::digraph::parser::Parser;
    use crate::digraph::state::{CursorError::InvalidMotion, CursorState};

    macro_rules! move_cursor {
        // Move sequences that result in an attempted move "off the graph" should return errors
        ($($dir:tt),+ _in_ $src:ident -> $err:tt) => {{
            let mut parser = Parser::new(String::from($src)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();
            let graph = &nodes[..];
            let mut state = CursorState::new(graph, graph.get_hash()).unwrap();
            let mut failed = false;
            $(
                let coerced = state.block_loc.coerce(&state.graph_hash).expect("Coercion should work");
                let dst = if coerced != state.block_loc && state.graph_hash.get(&coerced).expect("Retrieval should work").kind == NodeKind::FNDEF {
                    move_cursor!($dir).move_global(&state)
                } else {
                    move_cursor!($dir).move_local(&state)
                };
                failed = failed || matches!(dst, Err($err(_)));
                if let Ok(dst) = dst {
                    state.block_loc = dst;
                }
            )+
            assert!(failed);
        }};
        ($($dir:tt),+ _in_ $src:ident -> <$($id:literal),+>) => {{
            let mut parser = Parser::new(String::from($src)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();
            let graph = &nodes[..];
            let mut state = CursorState::new(graph, graph.get_hash()).unwrap();
            $(
                let coerced = state.block_loc.coerce(&state.graph_hash).expect("Coercion should work");
                let dst = if coerced != state.block_loc && state.graph_hash.get(&coerced).expect("Retrieval should work").kind == NodeKind::FNDEF {
                    move_cursor!($dir).move_global(&state).expect("Motion should succeed")
                } else {
                    move_cursor!($dir).move_local(&state).expect("Motion should succeed")
                };
                state.block_loc = dst.clone();
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

    const SOURCE: &str = "define f of x\noutput x\ndefine f1 of x\noutput x\ndone define\ndefine \
                          f2 of x\noutput x\ndefine f21 of x\noutput x\ndone define\ndefine f22 of \
                          x\noutput x\ndone define\ndone define\ndone define\ndefine g of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

    mod global {
        use super::*;

        #[test]
        fn global_invalid() {
            move_cursor!(D, D _in_ SOURCE -> InvalidMotion);
            move_cursor!(D, R, D, R, R _in_ SOURCE -> InvalidMotion);
            move_cursor!(D, R, D, R, D _in_ SOURCE -> InvalidMotion);
            move_cursor!(D, R, D, D _in_ SOURCE -> InvalidMotion);
        }

        #[test]
        fn global_compound() {
            move_cursor!(D, R _in_ SOURCE -> <0, 1, 1, 0>);
            move_cursor!(D, R, D, R _in_ SOURCE -> <0, 1, 1, 1, 1>);
            move_cursor!(D, R, D, R, L _in_ SOURCE -> <0, 1, 1, 1, 0>);
        }

        #[test]
        fn global_cycles() {
            move_cursor!(R, L _in_ SOURCE ||);
            move_cursor!(D, U _in_ SOURCE ||);
            move_cursor!(D, R, U _in_ SOURCE ||);
            move_cursor!(D, R, L, R, U _in_ SOURCE ||);
            move_cursor!(D, R, D, R, U, L, U _in_ SOURCE ||);
        }
    }

    mod local {
        use super::*;

        #[test]
        fn local_linear() {
            move_cursor!(R, I _in_ SOURCE -> <1, 0, 0>);
            move_cursor!(R, I, D _in_ SOURCE -> <1, 0, 1>);
            move_cursor!(R, I, D, D _in_ SOURCE -> <1, 0, 2>);
        }

        #[test]
        fn local_depth() {
            move_cursor!(R, I, D, D, I _in_ SOURCE -> <1, 0, 2, 0, 0>);
            move_cursor!(R, I, D, D, I, D _in_ SOURCE -> <1, 0, 2, 1, 0, 0>);
            move_cursor!(R, I, D, D, I, D, D _in_ SOURCE -> <1, 0, 2, 1, 0, 1>);
            move_cursor!(R, I, D, D, I, D, R _in_ SOURCE -> <1, 0, 2, 1, 1, 0>);
            move_cursor!(R, I, D, D, I, D, D, D _in_ SOURCE -> InvalidMotion);
        }

        #[test]
        fn local_cycles() {
            move_cursor!(R, I, D, U, O _in_ SOURCE ##);
            move_cursor!(R, I, D, D, U, U, O _in_ SOURCE ##);
            move_cursor!(R, I, D, D, U, U, O _in_ SOURCE ##);
            move_cursor!(R, I, D, D, O _in_ SOURCE ##);
        }
    }
}
