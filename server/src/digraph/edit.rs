use super::address::Address;
use super::parser::{Node, NodeKind, Piece, PieceIdx};
use super::state::{ADMode, Canvas, Expecting};
use crate::digraph::address::Addressable;
use crate::digraph::util::*;
use crate::piece;
use crate::prelude::CursorError;
use crate::static_analysis::analyzer::SemanticError;
use std::ops::ControlFlow;

const PENDING_PIECES: &'static [Piece; 3] = &[
    Piece::PendingVal,
    Piece::PendingOp,
    Piece::IDENT(String::new()),
];

#[macro_export]
macro_rules! new_token {
    (From $state:ident, $kind:expr => [$($piece:expr),*] $(@ $piece_ix:expr),*) => {{
        let hash = $state.graph.get_hash_mut();
        let Some(curr_node) = hash.get(&$state.block_loc) else {
            return Err(CursorError::AddrNotFound($state.block_loc.clone()));
        };

        // SAFETY: We know this reference must be valid because we just retrieved the
        // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
        // been dropped since its creation (no concurrency), so this is safe.
        let curr_node = unsafe { &mut **curr_node };
        curr_node.kind = $kind;
        curr_node.pieces = vec![$($piece),*];

        $state.mode = ADMode::EDIT(Expecting::Value);
        $state.piece_ix = None;
        $($state.piece_ix = Some($piece_ix.to_vec())),*
    }};

    (From $state:ident, $kind:expr => [$($piece:expr),*] $(@ $piece_ix:expr),* ; {$($node:expr),+}) => {{
        let hash = $state.graph.get_hash_mut();
        let Some(curr_node) = hash.get(&$state.block_loc) else {
            return Err(CursorError::AddrNotFound($state.block_loc.clone()));
        };

        // SAFETY: We know this reference must be valid because we just retrieved the
        // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
        // been dropped since its creation (no concurrency), so this is safe.
        let curr_node = unsafe { &mut **curr_node };
        curr_node.kind = $kind;
        curr_node.pieces = vec![$($piece),*];
        curr_node.children = vec![$($node),+];

        (&mut $state.graph[..]).fill_addr();
        $state.block_loc = curr_node.addr.clone();
        $state.node_loc = $state.block_loc.coerce(&$state.graph.get_hash()).expect("Coercion failed");

        $state.mode = ADMode::EDIT($crate::digraph::state::Expecting::Value);
        $state.piece_ix = None;
        $($state.piece_ix = Some($piece_ix)),*
    }};
}

/// Inserts a new piece within a node
pub(super) fn new_piece(state: &mut Canvas, piece: Piece) -> Result<(), CursorError> {
    let hash = state.graph.get_hash_mut();
    let Some(curr_node) = hash.get(&state.block_loc) else {
        return Err(CursorError::AddrNotFound(state.block_loc.clone()));
    };

    let Some(piece_ix) = state.piece_ix.clone() else {
        unreachable!("Cannot add a new piece without editing a piece first");
    };

    // SAFETY: We know this reference must be valid because we just retrieved the
    // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
    // been dropped since its creation (no concurrency), so this is safe.
    let curr_node = unsafe { &mut **curr_node };

    match piece {
        Piece::IDENT(_) | Piece::TEXT(_) | Piece::NUMBER(_) => state.mode = ADMode::TYPE,
        Piece::BOOL(_) | Piece::NOTHING | Piece::PendingVal | Piece::PendingOp | Piece::OP(_) => {
            let ADMode::EDIT(expecting) = state.mode else {
                unreachable!("Cannot insert a piece without being in EDIT mode");
            };

            // 1. Retrieve parent index
            // 2. Add a pending node to the parent index
            // 3. Update index to +1
            let last_piece_ix = piece_ix.len() - 1;
            let parent_vec = if last_piece_ix == 0 {
                &mut curr_node.pieces
            } else {
                match curr_node.pieces[PieceIdx(&piece_ix[0..last_piece_ix])] {
                    Piece::LIST(ref mut args) | Piece::FNCALL(ref mut args) => args,
                    _ => return Err(CursorError::PieceAddrNotFound(piece_ix.to_vec())),
                }
            };

            state.piece_ix.as_mut().expect("Cannot be empty")[last_piece_ix] =
                piece_ix[last_piece_ix] + 1;

            if piece_ix[last_piece_ix] == parent_vec.len() - 1 {
                // Reached last piece, need to add another
                parent_vec.push(match expecting {
                    Expecting::Op => piece!(..#),
                    Expecting::Value => piece!(..+),
                    Expecting::Token => unreachable!("cannot manually add a token piece"),
                });
                state.mode = ADMode::EDIT(!expecting);
            } else {
                // Still another piece there, do nothing
                state.mode = ADMode::VIEW;
            }
        }
        Piece::LIST(_) => {
            state.mode = ADMode::EDIT(Expecting::Value);
            state.piece_ix.as_mut().expect("Cannot be empty").push(1);
        }
        Piece::FNCALL(_) => {
            state.mode = ADMode::TYPE;
            state.piece_ix.as_mut().expect("Cannot be empty").push(0);
        }
        Piece::NULL => unreachable!("this is a placeholder type and should never be present"),
    }

    curr_node.pieces[PieceIdx(&piece_ix)] = piece;

    Ok(())
}

impl Canvas {
    /// Transforms a `Canvas` object from viewing to editing mode.
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
    /// let mut state = Canvas {
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

        let insert_loc = match curr_node.kind {
            NodeKind::CONDTL if self._at_node() => {
                // TODO: Test
                match curr_node.children.len() {
                    0 => unreachable!("Cannot ever delete both 'yes' and 'no' branches"),
                    1 => {
                        // NOTE: This is a shortcut to determine which branch we are missing. If
                        // we're missing the "yes", we want to insert the new node at index 0 and
                        // append a 0 to the address; otherwise, insert at ix 1 and append a 1.
                        let insert_ix = if curr_node.children[0].kind == NodeKind::CONDTLN {
                            0
                        } else {
                            1
                        };

                        let kind = if insert_ix == 0 {
                            NodeKind::CONDTLY
                        } else {
                            NodeKind::CONDTLN
                        };

                        let hash = self.graph.get_hash_mut();
                        let Some(curr_node) = hash.get(&self.node_loc) else {
                            return Err(CursorError::AddrNotFound(self.node_loc.clone()));
                        };

                        let mut addr_base = curr_addr.clone();
                        if curr_addr.len() < 2 {
                            return Err(CursorError::InvalidAddress(addr_base));
                        }
                        // Remove the last 0.0
                        let _ = addr_base.addr.pop();
                        let _ = addr_base.addr.pop();
                        // Append a 1 because the yes and no branches are on level 1.
                        addr_base.addr.push(1);

                        let mut addry = addr_base.clone();
                        addry.addr.push(insert_ix);

                        let mut pretendy = addry.clone();

                        addry.addr.push(0);
                        pretendy.addr.push(1);

                        self.block_loc = addry.clone();
                        self.node_loc = addry.clone();

                        let new_node = Node {
                            line: 0,
                            children: vec![Node {
                                line: 0,
                                kind: NodeKind::PENDING,
                                pieces: vec![piece!(..#)],
                                addr: pretendy,
                                parent_addr: addry.clone(),
                                children: vec![],
                                rtl: Some("pretend".into()),
                                note: None,
                                err: None,
                            }],
                            kind,
                            pieces: vec![],
                            addr: addry,
                            parent_addr: curr_addr,
                            rtl: Some("define pretend".into()),
                            note: None,
                            err: None,
                        };

                        let curr_node = unsafe { &mut **curr_node };
                        curr_node.children.insert(insert_ix, new_node);

                        // Update adddresses for the no branch since it has now been pushed right.
                        if kind == NodeKind::CONDTLY {
                            fn _inner(
                                node: &mut Node,
                                i: isize,
                                parent_addr: &Address,
                                horiz_: bool,
                            ) {
                                let mut addr: Vec<usize> = (*parent_addr.clone()).clone();
                                let horiz = HORIZ_CHILDREN.contains(&node.kind) && horiz_;
                                if horiz {
                                    let last_idx = addr.len() - 2;
                                    // Increment second-to-last since these children are on the level below the parent.
                                    addr[last_idx] += 1;
                                    // Increment last for each child's distinct horizontal position within that level.
                                    addr[last_idx + 1] += i as usize;
                                } else {
                                    let last_idx = addr.len() - 1;
                                    addr[last_idx] += (1 + i) as usize;
                                }

                                // For FNDEF, CONDTL and other nodes with horizontal children, the first 0 pushed
                                // references the vertical "level", and the 2nd 0 references the node's horizontal
                                // position within that level.
                                if !node.children.is_empty() {
                                    addr.push(0);
                                    if node.has_subtree() && horiz_ {
                                        addr.push(0);
                                    }
                                }
                                node.addr = Address::new(addr);
                                node.parent_addr = parent_addr.clone();

                                let mut fn_idx: isize = -1;
                                node.children.iter_mut().enumerate().for_each(|(i_, n_)| {
                                    match n_.kind {
                                        NodeKind::FNDEF => {
                                            fn_idx += 1;
                                            _inner(n_, fn_idx as isize, &node.addr, true);
                                        }
                                        _ => _inner(n_, i_ as isize - fn_idx - 1, &node.addr, true),
                                    };
                                });
                            }

                            _inner(&mut curr_node.children[1], 1, &curr_node.addr, true);
                        }
                    }
                    2 => return Err(CursorError::InsertConditional),
                    2.. => unreachable!(),
                }
                return Ok(());
            }
            NodeKind::FNDEF if !self._at_node() => {
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
                    let last_ix = self.block_loc.len() - 1;
                    if curr_node.parent_addr.coerce(&hash)? == self.node_loc {
                        Address::new(self.block_loc[0..last_ix].to_vec()).join(&[1, 0, 0])
                    } else {
                        self.block_loc.join(&[1, 0, 0])
                    }
                } else {
                    // There are global children -> precondition: curr_addr.len() % 2 == 0
                    let Some(next_addr) = self.block_loc.next() else {
                        return Err(CursorError::EmptyAddr);
                    };
                    // Append <num_blocks (the index of the new node), 0 (root of the block)>
                    next_addr.join(&[num_blocks, 0])
                };
                self._insert_fn(&insert_loc_, curr_addr, num_blocks)?;
                self.mode = ADMode::TYPE;
                self.piece_ix = Some(vec![0usize]);
                insert_loc_ // function name must follow
            }
            NodeKind::FNDEF if self._at_node() => {
                // If we're on a node, just make a new node below and as the new `insert_loc`
                let Some(next_addr) = self.block_loc.next() else {
                    return Err(CursorError::EmptyAddr);
                };
                self._insert_other(&next_addr, curr_addr, 0)?;
                self.mode = ADMode::EDIT(Expecting::Token);
                next_addr // any possible `Token` could follow
            }
            NodeKind::FORLOOP | NodeKind::WHLLOOP | NodeKind::CONDTLY | NodeKind::CONDTLN
                if self._at_node() =>
            {
                // If we're on a node, just make a new node below and as the new `insert_loc`
                let Some(next_addr) = self.block_loc.next() else {
                    return Err(CursorError::EmptyAddr);
                };

                self._insert_as_child(&next_addr, curr_addr)?;
                self.mode = ADMode::EDIT(Expecting::Token);
                next_addr // any possible `Token` could follow
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
                self.mode = ADMode::EDIT(Expecting::Token);
                next_addr // any possible `Token` could follow
            }
        };

        self.block_loc = insert_loc;
        // Recompute hash because nodes have changed
        self.node_loc = self.block_loc.coerce(&self.graph.get_hash())?;

        // Re-sync the graph addresses with the new additions of nodes
        (&mut self.graph[..]).fill_addr();
        Ok(())
    }

    /// Toggles `Canvas` back to viewing mode
    #[inline(always)]
    pub(super) fn to_view(&mut self) -> Result<(), CursorError> {
        self.mode = ADMode::VIEW;
        self.piece_ix = None;
        let hash = self.graph.get_hash_mut();
        let Some(curr_node) = hash.get(&self.block_loc) else {
            return Err(CursorError::AddrNotFound(self.block_loc.clone()));
        };

        // SAFETY: We know this reference must be valid because we just retrieved the
        // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
        // been dropped since its creation (no concurrency), so this is safe.
        let curr_node = unsafe { &mut **curr_node };

        let len = curr_node.pieces.len();
        let last_pending_op = curr_node.pieces[len - 1] == piece!(..+);
        let last_pending_param = len >= 2
            && curr_node.pieces[len - 1] == piece!(IDENT "")
            && matches!(curr_node.pieces[len - 2], Piece::IDENT(_));

        if len > 1 && (last_pending_op || last_pending_param) {
            curr_node.pieces.pop();
        }

        Ok(())
    }

    // Update graph with a new function (@ `at_addr`) as a child of the function @ `from_addr`.
    #[inline]
    fn _insert_fn(
        &mut self,
        at_addr: &Address,
        from_addr: Address,
        num_blocks: usize,
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
                note: None,
                err: None,
            }],
            kind: NodeKind::FNDEF,
            pieces: vec![Piece::IDENT("".into())],
            addr: at_addr.clone(),
            parent_addr: from_addr,
            rtl: Some("define pretend".into()),
            note: None,
            err: None,
        };
        parent_node.children.insert(num_blocks, new_node);
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
            pieces: vec![Piece::PendingVal],
            addr: at_addr.clone(),
            parent_addr: from_addr,
            rtl: Some("pretend".into()),
            note: None,
            err: None,
        };
        parent_node.children.insert(recomputed_ix, new_node);
        Ok(())
    }

    #[inline]
    // Update graph with a new node (@ `at_addr`) as the first child of a loop.
    fn _insert_as_child(
        &mut self,
        at_addr: &Address,
        from_addr: Address,
    ) -> Result<(), CursorError> {
        let hash_ = self.graph.get_hash_mut();
        let Some(curr_node) = hash_.get(&from_addr) else {
            return Err(CursorError::ParentNotFound(from_addr));
        };

        // SAFETY: The parent node must be valid because this node is not a root function
        // (which would be in the FNDEF branch) so `parent_addr` must point to a valid node.
        let curr_node = unsafe { &mut **curr_node };

        let new_node = Node {
            line: 0, // TODO: How to increment all subsequent line numbers efficiently?
            children: vec![],
            kind: NodeKind::PENDING,
            pieces: vec![Piece::PendingVal],
            addr: at_addr.clone(),
            parent_addr: from_addr,
            rtl: Some("pretend".into()),
            note: None,
            err: None,
        };
        curr_node.children.insert(0, new_node);
        Ok(())
    }

    /// Updates a specific piece (typicaly an ident or literal) based on keyboard input (`value`).
    pub(crate) fn update_value(&mut self, value: String) -> Result<(), CursorError> {
        let Some(ref piece_ix) = self.piece_ix else {
            unreachable!("Cannot update a value without editing a piece first");
        };

        let hash = self.graph.get_hash_mut();
        let Some(curr_node) = hash.get(&self.block_loc) else {
            return Err(CursorError::AddrNotFound(self.block_loc.clone()));
        };

        // SAFETY: This node must be valid because it comes from the current block_loc of the
        // PayloadState. block_loc must point to a node since otherwise we would have returned Err.
        let curr_node = unsafe { &mut **curr_node };
        let new_piece = match curr_node.pieces[PieceIdx(&piece_ix)] {
            Piece::IDENT(_) => {
                let _ = valid_varname(&value)?;
                Piece::IDENT(value)
            }
            Piece::NUMBER(_) => Piece::NUMBER(value.parse::<f64>()?),
            Piece::TEXT(_) => Piece::TEXT(value),
            _ => unreachable!("cannot update value for non-typed pieces"),
        };
        curr_node.pieces[PieceIdx(&piece_ix)] = new_piece;

        Ok(())
    }

    pub(crate) fn _move_to_next(
        &mut self,
        curr_node: &mut Node,
        piece_ix: Option<&mut Vec<usize>>,
        next_ident: bool, // In a function definition, all pieces must be IDENTs
    ) -> Result<(), CursorError> {
        let mut piece_ix = if piece_ix.is_none() {
            self.piece_ix.clone().expect("Can never be None")
        } else {
            piece_ix.unwrap().to_vec()
        };

        let parent_vec = if piece_ix.len() == 1 {
            &curr_node.pieces
        } else {
            match curr_node.pieces[PieceIdx(&piece_ix[0..piece_ix.len() - 1])] {
                Piece::LIST(ref args) | Piece::FNCALL(ref args) => args,
                _ => return Err(CursorError::PieceAddrNotFound(piece_ix.to_vec())),
            }
        };
        // If we are at the end of our local piece[], we add a new one
        if piece_ix.last() == Some(&(parent_vec.len() - 1)) {
            let parent_vec = if piece_ix.len() == 1 {
                &mut curr_node.pieces
            } else {
                match curr_node.pieces[PieceIdx(&piece_ix[0..piece_ix.len() - 1])] {
                    Piece::LIST(ref mut args) | Piece::FNCALL(ref mut args) => args,
                    _ => return Err(CursorError::PieceAddrNotFound(piece_ix.to_vec())),
                }
            };

            if let Some(spi) = self.piece_ix.as_mut() {
                if let Some(last) = spi.last_mut() {
                    *last = parent_vec.len();
                }
            }

            if next_ident {
                self.mode = ADMode::TYPE; // Next parameter must be an IDENT as well.
                parent_vec.push(piece!(IDENT ""));
            } else if piece_ix.len() > 1 && parent_vec.len() == 1 {
                self.mode = ADMode::EDIT(Expecting::Value);
                parent_vec.push(piece!(..#));
            } else {
                self.mode = ADMode::EDIT(Expecting::Op); // Must be OP after a value.
                parent_vec.push(piece!(..+));
            }

            return Ok(());
        }

        if piece_ix == vec![0usize] && curr_node.pieces[PieceIdx(&piece_ix)] == piece!(..#) {
            self.piece_ix = Some(piece_ix);
            self.mode = ADMode::EDIT(Expecting::Value);
            return Ok(());
        }

        // Find the next pending piece in the local piece[]
        // A pending piece is either an unnamed identifier, or an explicit pending
        let piece_ix_len = piece_ix.len() - 1;
        let start_i = piece_ix[piece_ix_len] + 1;
        let mut broken = false;
        for i in start_i..parent_vec.len() {
            piece_ix[piece_ix_len] = i;
            if PENDING_PIECES.contains(&curr_node.pieces[PieceIdx(&piece_ix)]) {
                self.piece_ix = Some(piece_ix.to_vec());
                match &parent_vec[i - 1] {
                    piece @ _ if PENDING_PIECES.contains(piece) => {
                        unreachable!("should have reached earlier")
                    }
                    Piece::OP(_) => self.mode = ADMode::EDIT(Expecting::Value),
                    _ => self.mode = ADMode::EDIT(Expecting::Op),
                }
                dbg!("Found one!");
                broken = true;
                break;
            }
        }

        if !broken {
            dbg!("Couldn't be broken");
            self.mode = ADMode::VIEW;
        }

        Ok(())
    }

    pub(crate) fn _move_right(
        &mut self,
        curr_node: &mut Node,
        piece_ix: Option<&mut Vec<usize>>,
    ) -> Result<(), CursorError> {
        let mut piece_ix = if piece_ix.is_none() {
            self.piece_ix.clone().expect("Can never be None")
        } else {
            piece_ix.unwrap().to_vec()
        };

        let parent_vec = if piece_ix.len() == 1 {
            &curr_node.pieces
        } else {
            match curr_node.pieces[PieceIdx(&piece_ix[0..piece_ix.len() - 1])] {
                Piece::LIST(ref args) | Piece::FNCALL(ref args) => args,
                _ => return Err(CursorError::PieceAddrNotFound(piece_ix.to_vec())),
            }
        };

        // If we are at the end of our local piece[], we stay
        if piece_ix.last() == Some(&(parent_vec.len() - 1)) {
            self.mode = ADMode::VIEW;
            return Ok(());
        }

        let piece_ix_len = piece_ix.len() - 1;
        piece_ix[piece_ix_len] += 1;
        let curr_piece = &curr_node.pieces[PieceIdx(&piece_ix)];
        if curr_piece != &Piece::NULL {
            self.piece_ix = Some(piece_ix.to_vec());
            self.mode = ADMode::VIEW;
        }

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

        macro_rules! ensure_insert {
        // Move sequences that result in an attempted move "off the graph" should return errors
        (@ <$($id:literal),+> _in_ $src:ident -> <$($new_id:literal),+>) => {{
            {
                let mut parser = Parser::new(String::from($src)).unwrap();
                let mut nodes = parser.parse().unwrap();
                (&mut nodes[..]).fill_addr();

                let block_loc = addr!($($id),+);
                let node_loc = block_loc.coerce(&nodes.get_hash()).expect("Node coercion should work");
                let mut state = Canvas {
                    filename: "".into(),
                    output: Some("".into()),
                    block_loc,
                    node_loc,
                    mode: ADMode::VIEW,
                    graph: nodes.to_vec(),
                    piece_ix: None,
                    err: None,
                };
                state.to_insert().expect("Could not toggle mode");
                assert_eq!(state.block_loc, addr!($($new_id),+));
                state
            }
        }};
    }

        #[test]
        fn insert_block() {
            ensure_insert!(@ <0, 0> _in_ SOURCE -> <0, 1, 2, 0>);
            ensure_insert!(@ <0, 1, 0> _in_ SOURCE -> <0, 1, 0, 1, 0, 0>);
            ensure_insert!(@ <0, 1, 1, 0> _in_ SOURCE -> <0, 1, 1, 1, 2, 0>);
            ensure_insert!(@ <0, 1, 1, 1, 0> _in_ SOURCE -> <0, 1, 1, 1, 0, 1, 0, 0>);
            ensure_insert!(@ <0, 1, 1, 1, 1> _in_ SOURCE -> <0, 1, 1, 1, 1, 1, 0, 0>);
        }

        #[test]
        fn insert_node() {
            ensure_insert!(@ <1, 0, 1> _in_ SOURCE -> <1, 0, 2>);
        }

        /// Insert a node as the first element of a for loop.
        #[test]
        fn insert_to_for() {
            let LOCAL_SOURCE: &'static str =
                "define f of x\nfor i in l\npretend\ndone for\ndone define";
            let state = ensure_insert!(@ <0, 0, 1, 0> _in_ LOCAL_SOURCE -> <0, 0, 1, 1>);
            assert_eq!(state.graph[0].children.len(), 1);

            let child = &state.graph[0].children[0];
            assert_eq!(child.kind, NodeKind::FORLOOP);

            assert_eq!(child.children.len(), 2);
            for (i, subchild) in child.children.iter().enumerate() {
                assert_eq!(subchild.kind, NodeKind::PENDING);
                assert_eq!(*subchild.addr, vec![0, 0, 1, i + 1]);
            }
        }

        /// Insert a node as the first element of a while loop.
        #[test]
        fn insert_to_while() {
            let LOCAL_SOURCE: &'static str =
                "define f of x\nwhile i greater than 0\npretend\ndone while\ndone define";
            let state = ensure_insert!(@ <0, 0, 1, 0> _in_ LOCAL_SOURCE -> <0, 0, 1, 1>);
            assert_eq!(state.graph[0].children.len(), 1);

            let child = &state.graph[0].children[0];
            assert_eq!(child.kind, NodeKind::WHLLOOP);

            assert_eq!(child.children.len(), 2);
            for (i, subchild) in child.children.iter().enumerate() {
                assert_eq!(subchild.kind, NodeKind::PENDING);
                assert_eq!(*subchild.addr, vec![0, 0, 1, i + 1]);
            }
        }

        /// Insert a node as the first element of a conditional "yes" branch
        #[test]
        fn insert_to_condtly() {
            let LOCAL_SOURCE: &'static str =
                "define f of x\nif i greater than 0\noutput string yes done\ndone if\notherwise\noutput string no done\ndone otherwise\ndone define";
            let state =
                ensure_insert!(@ <0, 0, 1, 1, 0, 0> _in_ LOCAL_SOURCE -> <0, 0, 1, 1, 0, 1>);
            assert_eq!(state.graph[0].children.len(), 1);

            let child = &state.graph[0].children[0];
            assert_eq!(child.kind, NodeKind::CONDTL);

            dbg!(&child.children);
            assert_eq!(child.children.len(), 2);

            let condtly = &child.children[0];
            let condtln = &child.children[1];
            assert_eq!(condtly.kind, NodeKind::CONDTLY);
            assert_eq!(condtln.kind, NodeKind::CONDTLN);

            assert_eq!(*condtly.children[0].addr, vec![0, 0, 1, 1, 0, 1]);
            assert_eq!(condtly.children[0].kind, NodeKind::PENDING);
            assert_eq!(*condtly.children[1].addr, vec![0, 0, 1, 1, 0, 2]);
            assert_eq!(condtly.children[1].kind, NodeKind::OUTPUT);
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
            let mut state = Canvas {
                filename: "".into(),
                output: Some("".into()),
                block_loc,
                node_loc,
                mode: ADMode::VIEW,
                graph: nodes.to_vec(),
                piece_ix: None,
                err: None,
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
            let mut state = Canvas {
                filename: "".into(),
                output: Some("".into()),
                block_loc,
                node_loc,
                mode: ADMode::VIEW,
                graph: nodes.to_vec(),
                piece_ix: None,
                err: None,
            };
            state.to_insert().expect("Could not toggle node");
            assert_eq!(state.block_loc, addr!(0, 1, 2, 0));

            state.coerce().expect("Coercion should work");
            let hash = state.graph.get_hash();

            assert_eq!(
                hash.get(&state.node_loc)
                    .expect(&format!("Should find a node at {:?}", state.node_loc))
                    .kind,
                NodeKind::FNDEF
            );
        }
    }
}

fn valid_varname(name: &str) -> Result<(), SemanticError> {
    // List of Python keywords (Python 3.x)
    let keywords = [
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
        "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global",
        "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
        "try", "while", "with", "yield",
    ];

    // Check if the name is empty
    if name.is_empty() {
        return Err(SemanticError::InvalidVarName(name.into()));
    }

    // Check if it's a Python keyword
    if keywords.contains(&name) {
        return Err(SemanticError::KeywordVarName(name.into()));
    }

    // Check if the first character is valid (letter or underscore)
    let mut chars = name.chars();
    if let Some(first) = chars.next() {
        if !first.is_alphabetic() && first != '_' {
            return Err(SemanticError::InvalidVarName(name.into()));
        }
    }

    // Check if the remaining characters are valid (letters, numbers, or underscore)
    if !chars.all(|c| c.is_alphanumeric() || c == '_') {
        return Err(SemanticError::InvalidVarName(name.into()));
    }

    Ok(())
}
