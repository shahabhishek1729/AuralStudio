use super::state::CursorState;
use crate::digraph::address::{Address, Addressable};
use crate::digraph::util::*;
use crate::prelude::CursorError;

pub(crate) struct Editor {
    state: CursorState,
    insert_loc: Option<Address>,
}

impl Editor {
    pub(crate) fn new(state: CursorState) -> Result<Self, CursorError> {
        let Some(&curr_node) = state.graph.get_hash().get(&state.node_loc) else {
            return Err(CursorError::AddrNotFound(state.node_loc));
        };

        let insert_loc = match curr_node.kind {
            _ if state._at_node() => Some({
                // If we're on a node, just make a new node below and as the new `insert_loc`
                let Some(next_addr) = curr_node.addr.next() else {
                    return Err(CursorError::EmptyAddr);
                };
                next_addr
            }),
            super::parser::NodeKind::FNDEF => Some({
                // On a function block, so we need to know how many global children this block has.
                let num_blocks = _filter_children(curr_node).collect::<Vec<_>>().len();

                if num_blocks == 0 {
                    // No global children, append <1(new level), 0(first child in that level)>
                    state.block_loc.join(&[1, 0])
                } else {
                    // There are global children, so it must be that the current address has an
                    // even length. We need to change the last index to a 1, and append the
                    // child number to the end
                    let Some(next_addr) = state.block_loc.next() else {
                        return Err(CursorError::EmptyAddr);
                    };
                    next_addr.join(&[num_blocks])
                }
            }),
            super::parser::NodeKind::CONDTL => {
                return Err(CursorError::InsertConditional);
            }
            _ => None,
        };

        Ok(Self { state, insert_loc })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
