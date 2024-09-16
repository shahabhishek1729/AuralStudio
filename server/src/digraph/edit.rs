use super::address::Address;
use super::parser::{Node, NodeKind, Piece};
use super::state::{ADMode, CursorState};
use crate::digraph::address::Addressable;
use crate::digraph::util::*;
use crate::prelude::CursorError;

impl CursorState {
    pub(super) fn to_insert(&mut self) -> Result<(), CursorError> {
        let hash = self.graph.get_hash();
        let Some(curr_node) = hash.get(&self.node_loc) else {
            return Err(CursorError::AddrNotFound(self.node_loc.clone()));
        };
        let parent_addr = curr_node.parent_addr.clone();
        let curr_addr = curr_node.addr.clone();

        let (insert_loc, expecting) = match curr_node.kind {
            super::parser::NodeKind::CONDTL if self._at_node() => {
                return Err(CursorError::InsertConditional);
            }
            super::parser::NodeKind::FNDEF if !self._at_node() => {
                // On a function block, so we need to know how many global children this block has.
                let num_blocks = curr_node
                    .children
                    .iter()
                    .filter(|c| GLOBAL_BLOCKS.contains(&c.kind))
                    .collect::<Vec<_>>()
                    .len();

                let insert_loc_ = if num_blocks == 0 {
                    // No global children, append:
                    // <1 (new level), 0 (first child in that level), 0 (root of the block)>
                    self.block_loc.join(&[1, 0, 0])
                } else {
                    // There are global children -> precondition: curr_addr.len() % 2 == 0
                    let Some(next_addr) = self.block_loc.next() else {
                        return Err(CursorError::EmptyAddr);
                    };
                    // Append <num_blocks (the index of the new node), 0 (root of the block)>
                    next_addr.join(&[num_blocks, 0])
                };

                self._insert_fn(&insert_loc_, curr_addr)?;

                (insert_loc_, Some(NodeKind::FNDEF))
            }
            _ => {
                // If we're on a node, just make a new node below and as the new `insert_loc`
                let Some(next_addr) = self.block_loc.next() else {
                    return Err(CursorError::EmptyAddr);
                };

                // NOTE: Subtraction is safe because this address must be >= 1 (parent is 0)
                let child_ix = next_addr.last().expect("child address cannot be empty") - 1;

                self._insert_other(&next_addr, parent_addr, child_ix)?;

                (next_addr, None) // There are a number of possible nodes that could follow
            }
        };

        self.mode = ADMode::EDIT(expecting);
        self.block_loc = insert_loc;

        // Re-sync up the graph with the new additions of nodes
        (&mut self.graph[..]).fill_addr();
        Ok(())
    }

    // Update graph with a new function (@ `at_addr`) as a child of the function @ `from_addr`.
    fn _insert_fn(&mut self, at_addr: &Address, from_addr: Address) -> Result<(), CursorError> {
        let hash_ = self.graph.get_hash_mut();
        let Some(parent_node) = hash_.get(&from_addr) else {
            return Err(CursorError::ParentNotFound(from_addr));
        };
        // SAFETY: The parent node must be valid because this node is not a root function
        // (which would be in the FNDEF branch) so `parent_addr` must point to a valid node.
        let parent_node = unsafe { &mut **parent_node };
        let new_node = Node {
            line: 0, // TODO: How to increment all subsequent line numbers efficiently?
            children: vec![],
            kind: NodeKind::FNDEF,
            pieces: vec![Piece::PENDING],
            addr: at_addr.clone(),
            parent_addr: from_addr,
        };
        parent_node.children.push(new_node);
        Ok(())
    }

    // Update graph with a new node (@ `at_addr`) as the `index`th child of the node @ `from_addr`.
    fn _insert_other(
        &mut self,
        at_addr: &Address,
        from_addr: Address,
        index: usize,
    ) -> Result<(), CursorError> {
        let hash_ = self.graph.get_hash_mut();
        let Some(parent_node) = hash_.get(&from_addr) else {
            return Err(CursorError::ParentNotFound(from_addr));
        };
        // SAFETY: The parent node must be valid because this node is not a root function
        // (which would be in the FNDEF branch) so `parent_addr` must point to a valid node.
        let parent_node = unsafe { &mut **parent_node };
        let new_node = Node {
            line: 0, // TODO: How to increment all subsequent line numbers efficiently?
            children: vec![],
            kind: NodeKind::PENDING,
            pieces: vec![Piece::PENDING],
            addr: at_addr.clone(),
            parent_addr: from_addr,
        };
        parent_node.children.insert(index, new_node);
        Ok(())
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

    mod insert_loc {
        use super::*;

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
                graph: nodes.to_vec(),
            };
            state.to_insert().expect("Could not toggle mode");
            assert_eq!(state.block_loc, addr!($($new_id),+));
        }};
    }

        #[test]
        fn insert_block() {
            insert!(@ <0, 0> _in_ SOURCE -> <0, 1, 2, 0>);
            insert!(@ <0, 1, 0> _in_ SOURCE -> <0, 1, 0, 1, 0, 0>);
            insert!(@ <0, 1, 1, 0> _in_ SOURCE -> <0, 1, 1, 1, 2, 0>);
            insert!(@ <0, 1, 1, 1, 0> _in_ SOURCE -> <0, 1, 1, 1, 0, 1, 0, 0>);
            insert!(@ <0, 1, 1, 1, 1> _in_ SOURCE -> <0, 1, 1, 1, 1, 1, 0, 0>);
        }

        #[test]
        fn insert_node() {
            insert!(@ <1, 0, 1> _in_ SOURCE -> <1, 0, 2>)
        }
    }

    mod insertion {
        use super::*;

        #[test]
        fn child_insertion() {
            let mut parser = Parser::new(String::from(SOURCE)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();

            let block_loc = addr!(1, 0, 1);
            let node_loc = block_loc
                .coerce(&nodes.get_hash())
                .expect("Node coercion should work");
            let mut state = CursorState {
                block_loc,
                node_loc,
                mode: ADMode::VIEW,
                graph: nodes.to_vec(),
            };
            state.to_insert().expect("Could not toggle node");
            assert_eq!(state.block_loc, addr!(1, 0, 2));

            let hash = state.graph.get_hash();
            assert_eq!(
                hash.get(&state.block_loc)
                    .expect("Should find a node at <1, 0, 2>")
                    .kind,
                NodeKind::PENDING
            );
        }
    }
}
