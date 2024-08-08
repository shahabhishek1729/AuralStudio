use crate::check;
use crate::digraph::address::{Address, Addressable};
use crate::digraph::parser::NodeKind;
use crate::Node;
use anyhow;
use phf::phf_map;
use std::collections::HashMap;
use thiserror::Error;

static N_ROOT_CHILDREN: phf::Map<&'static str, u8> = phf_map! {
    "IF" => 2,
    "IFY" => 1,
    "IFN" => 1,
    "WHILE" => 1,
    "FOR" => 1
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

                    // Ensure the validity of the returned address (i.e., if going right from the
                    // rightmost block, return an InvalidMotion error, not a CoercionError).
                    let Ok(mut dst) = dst.coerce(&state.graph_hash) else {
                        return Err(CursorError::InvalidMotion(*self));
                    };
                    let _ = dst.addr.pop();

                    return Ok(dst);
                }
                return Err(CursorError::InvalidMotion(*self));
            }
            CursorDir::IN => {
                // Essentially coerces a block to its root node
                todo!()
            }
            _ => return Err(CursorError::InvalidMotion(*self)),
        }
    }

    fn move_local<'cur>(
        &'cur self,
        state: &CursorState<'cur>,
    ) -> anyhow::Result<Address, CursorError> {
        // When going up and down, no coercion - check children for local blocks
        // Left and right involves finding the parent's next child
        // In and out involves directly finding parent and child and moving (up until global block)
        match self {
            CursorDir::UP => todo!(),
            CursorDir::DOWN => todo!(),
            CursorDir::LEFT => todo!(),
            CursorDir::RIGHT => todo!(),
            CursorDir::OUT => todo!(),
            CursorDir::IN => todo!(),
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
                let dst = move_cursor!($dir).move_global(&state);
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
                let dst = move_cursor!($dir).move_global(&state).expect("motion should succeed");
                state.block_loc = dst.clone();
            )+
            assert_eq!(dst, addr!($($id),+));
        }};
        // Cycles are sequences of motions that end up at the leftmost root (i.e., <0, 0>)
        ($($dir:tt),+ _in_ $src:ident ||) => {{
            move_cursor!($($dir),+ _in_ $src -> <0, 0>);
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
                          x\noutput x\ndone define\ndone define\ndone define\ndefine h of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

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
