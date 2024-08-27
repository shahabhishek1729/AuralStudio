use super::state::CursorState;
use crate::digraph::address::{Address, Addressable};
use crate::digraph::util::*;
use crate::prelude::CursorError;

pub(crate) struct Editor<'a> {
    state: &'a CursorState,
    insert_loc: Option<Address>,
}

impl<'a> Editor<'a> {
    pub(crate) fn new(state: &'a CursorState) -> Result<Self, CursorError> {
        let Some(&curr_node) = state.graph.get_hash().get(&state.node_loc) else {
            return Err(CursorError::AddrNotFound(state.node_loc.clone()));
        };

        let insert_loc = match curr_node.kind {
            super::parser::NodeKind::CONDTL if state._at_node() => {
                return Err(CursorError::InsertConditional);
            }
            super::parser::NodeKind::FNDEF if !state._at_node() => {
                // On a function block, so we need to know how many global children this block has.
                let num_blocks = curr_node
                    .children
                    .iter()
                    .filter(|c| GLOBAL_BLOCKS.contains(&c.kind))
                    .collect::<Vec<_>>()
                    .len();

                if num_blocks == 0 {
                    // No global children, append:
                    // <1 (new level), 0 (first child in that level), 0 (root of the block)>
                    state.block_loc.join(&[1, 0, 0])
                } else {
                    // There are global children -> precondition = curr_addr.len() % 2 == 0
                    let Some(next_addr) = state.block_loc.next() else {
                        return Err(CursorError::EmptyAddr);
                    };
                    // Append <num_blocks (the index of the new node), 0 (root of the block)>
                    next_addr.join(&[num_blocks, 0])
                }
            }
            _ => {
                // If we're on a node, just make a new node below and as the new `insert_loc`
                let Some(next_addr) = state.block_loc.next() else {
                    return Err(CursorError::EmptyAddr);
                };
                next_addr
            }
        };

        Ok(Self {
            state,
            insert_loc: Some(insert_loc),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addr;
    use crate::digraph::address::Addressable;
    use crate::digraph::parser::Parser;
    use crate::digraph::state::ADMode;

    const SOURCE: &'static str = "define f of x\noutput x\ndefine f1 of x\noutput x\ndone define\ndefine \
                          f2 of x\noutput x\ndefine f21 of x\noutput x\ndone define\ndefine f22 of \
                          x\noutput x\ndone define\ndone define\ndone define\ndefine g of x\noutput x plus 1\nif \
                          x equals 3\noutput x\ndone if\notherwise\noutput y\ndone otherwise\ndone \
                          define";

    macro_rules! insert {
        // Move sequences that result in an attempted move "off the graph" should return errors
        (@ <$($id:literal),+> _in_ $src:ident -> <$($new_id:literal),+>) => {{
            let mut parser = Parser::new(String::from($src)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();

            let block_loc = addr!($($id),+);
            let node_loc = block_loc.coerce(&nodes.get_hash()).expect("Node coercion should work");
            let mut state = CursorState {
                block_loc,
                node_loc,
                mode: ADMode::VIEW,
                insert_at: None,
                graph: nodes.to_vec(),
            };
            let editor = Editor::new(&state);
            assert_eq!(editor.expect("Coercion should work").insert_loc, Some(addr!($($new_id),+)));
        }};
    }

    #[test]
    fn fn_insert_loc() {
        insert!(@ <0, 0> _in_ SOURCE -> <0, 1, 2, 0>)
    }
}
