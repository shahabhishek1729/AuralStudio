use super::address::Address;
use super::parser::{Node, NodeKind, Piece};
use super::state::{ADMode, CursorState, Expecting};
use crate::digraph::address::Addressable;
use crate::digraph::util::*;
use crate::prelude::CursorError;
use std::ops::ControlFlow;

impl CursorState {
    /// Transforms a `CursorState` object from viewing to editing mode.
    ///
    /// From a given node, when a user attempts to "insert below", determines what the address and
    /// kind of the new node should be (global inserts create functions, local inserts create
    /// placeholder nodes).
    ///
    /// # Examples
    /// ```rust
    /// // Assume `nodes` references the parsed version of SOURCE.
    /// let block_loc = addr!(0, 0);
    ///
    /// let mut state = CursorState {
    ///     block_loc,
    ///     node_loc, // Assume `node_loc` is defined as a coerced `block_loc`
    ///     mode: ADMode::VIEW,
    ///     graph: nodes.to_vec(),
    /// };
    /// state.to_insert().expect("Could not toggle node");
    ///
    /// // Automatically creates a new address for this node and adjusts all other addresses.
    /// assert_eq!(state.block_loc, addr!(0, 1, 2, 0));

    /// let hash = state.graph.get_hash();
    /// // We can auto-infer that the new node is of kind FNDEF.
    /// assert_eq!(
    ///     hash.get(&state.block_loc)
    ///         .expect(&format!("Should find a node at {:?}", state.block_loc))
    ///         .kind,
    ///     NodeKind::FNDEF
    /// );
    /// ```
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
                (insert_loc_, Expecting::IdentPiece) // function name must follow
            }
            super::parser::NodeKind::FNDEF if self._at_node() => {
                // If we're on a node, just make a new node below and as the new `insert_loc`
                let Some(next_addr) = self.block_loc.next() else {
                    return Err(CursorError::EmptyAddr);
                };
                self._insert_other(&next_addr, curr_addr, 0)?;
                (next_addr, Expecting::AnyPiece) // any possible `Token` could follow
            }
            _ => {
                // If we're on a node, just make a new node below and as the new `insert_loc`
                let Some(next_addr) = self.block_loc.next() else {
                    return Err(CursorError::EmptyAddr);
                };
                let child_ix = *self
                    .block_loc
                    .last()
                    .expect("child address cannot be empty");
                self._insert_other(&next_addr, parent_addr, child_ix)?;
                (next_addr, Expecting::AnyPiece) // any possible `Token` could follow
            }
        };

        self.mode = ADMode::EDIT(expecting);
        self.block_loc = insert_loc;
        // Recompute hash because nodes have changed
        self.node_loc = self.block_loc.coerce(&self.graph.get_hash())?;

        // Re-sync the graph addresses with the new additions of nodes
        (&mut self.graph[..]).fill_addr();
        Ok(())
    }

    /// Toggles `CursorState` back to viewing mode
    #[inline(always)]
    pub(super) fn to_view(&mut self) {
        self.mode = ADMode::VIEW;
    }

    // Update graph with a new function (@ `at_addr`) as a child of the function @ `from_addr`.
    #[inline]
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
            children: vec![Node {
                line: 0,
                kind: NodeKind::PENDING,
                pieces: vec![],
                addr: at_addr
                    .next()
                    .expect("Insertion location cannot be an empty address"),
                parent_addr: at_addr.clone(),
                children: vec![],
                rtl: Some("pretend".into()),
            }],
            kind: NodeKind::FNDEF,
            pieces: vec![Piece::PENDING],
            addr: at_addr.clone(),
            parent_addr: from_addr,
            rtl: Some("define pretend".into()),
        };
        parent_node.children.push(new_node);
        Ok(())
    }

    #[inline]
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

        let recomputed_ix = match parent_node.children.iter().enumerate().try_fold(
            (0, index),
            |(ri, i), (curr_i, child)| {
                if i == 0 {
                    return ControlFlow::Break((ri, i));
                }
                if GLOBAL_BLOCKS.contains(&child.kind) {
                    ControlFlow::Continue((curr_i + 1, i))
                } else {
                    ControlFlow::Continue((curr_i + 1, i - 1))
                }
            },
        ) {
            ControlFlow::Continue((ri, _)) => ri,
            ControlFlow::Break((ri, _)) => ri,
        };

        let new_node = Node {
            line: 0, // TODO: How to increment all subsequent line numbers efficiently?
            children: vec![],
            kind: NodeKind::PENDING,
            pieces: vec![Piece::PENDING],
            addr: at_addr.clone(),
            parent_addr: from_addr,
            rtl: Some("pretend".into()),
        };
        parent_node.children.insert(recomputed_ix, new_node);
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

        #[test]
        fn function_insertion() {
            let mut parser = Parser::new(String::from(SOURCE)).unwrap();
            let mut nodes = parser.parse().unwrap();
            (&mut nodes[..]).fill_addr();

            let block_loc = addr!(0, 0);
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
            assert_eq!(state.block_loc, addr!(0, 1, 2, 0));

            let hash = state.graph.get_hash();
            assert_eq!(
                hash.get(&state.block_loc)
                    .expect(&format!("Should find a node at {:?}", state.block_loc))
                    .kind,
                NodeKind::FNDEF
            );
        }
    }
}
