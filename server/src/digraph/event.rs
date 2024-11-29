use super::parser::{NodeKind, Piece, PieceIdx};
use super::state::Expecting;
use super::state::{ADMode, CursorError};
use crate::digraph::address::Addressable;
use crate::digraph::command::Command;
use crate::digraph::edit::new_piece;
use crate::digraph::parser::OpKind;
use crate::digraph::state::{CursorDir, CursorState};
use crate::new_token;
use crate::{make_node, piece};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KeyboardEvent {
    pub key: String,
    /*
    #[serde(rename = "ctrlKey")]
    ctrl_key: bool,
    #[serde(rename = "metaKey")]
    meta_key: bool,
    #[serde(rename = "shiftKey")]
    shift_key: bool,
    #[serde(rename = "altKey")]
    alt_key: bool,
    */
}

impl KeyboardEvent {
    pub(crate) fn parse_command(&self, state: &mut CursorState) -> Result<(), CursorError> {
        let command = Command::from(&self.key, &state.mode);

        match *command {
            Command::NavUp
            | Command::NavDown
            | Command::NavLeft
            | Command::NavRight
            | Command::NavIn
            | Command::NavOut => {
                // This is a navigation command, move in the correct direction
                if let Ok(new_addr) = state.navigate(dir_map(*command)) {
                    state.block_loc = new_addr;
                    let _ = state.coerce();
                }
            }
            Command::EditMode => state.to_insert()?,
            Command::InplaceEditMode => {
                if !state._at_node() {
                    return Err(CursorError::AmbiguousEdit);
                }
                let hash = state.graph.get_hash_mut();
                let Some(curr_node) = hash.get(&state.block_loc) else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };
                let curr_node = unsafe { &mut **curr_node };

                if curr_node.kind == NodeKind::PENDING {
                    state.piece_ix = None;
                    state.mode = ADMode::EDIT(Expecting::Token);
                } else {
                    state._move_to_next(
                        curr_node,
                        Some(&mut vec![0usize]),
                        curr_node.kind == NodeKind::FNDEF,
                    )?;
                }
            }
            Command::ViewMode => state.to_view()?,
            Command::Escape => {
                let piece_ix = &state.piece_ix.clone().unwrap_or(vec![]);
                if piece_ix.len() > 1 {
                    let hash = state.graph.get_hash_mut();
                    let Some(curr_node) = hash.get(&state.block_loc) else {
                        return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                    };

                    let curr_node = unsafe { &mut **curr_node };

                    let parent_vec =
                        match curr_node.pieces[PieceIdx(&piece_ix[0..piece_ix.len() - 1])] {
                            Piece::LIST(ref mut args) | Piece::FNCALL(ref mut args) => args,
                            _ => return Err(CursorError::PieceAddrNotFound(piece_ix.to_vec())),
                        };

                    if [Some(&piece!(..#)), Some(&piece!(IDENT ""))].contains(&parent_vec.last()) {
                        parent_vec.pop();
                    }

                    if let Some(ref mut pix) = state.piece_ix {
                        pix.pop();
                    }
                    // Cannot be in an FNDEF if we're within a set of args (list/call)
                    state._move_to_next(curr_node, None, false)?;
                } else {
                    state.to_view()?;
                }
            }
            Command::InsertVar => {
                new_token! {
                    From state, NodeKind::VARDECL => [
                        piece!(IDENT ""), Piece::OP(OpKind::ASSN), piece!(..#)
                    ] @ vec![0]
                };
                state.mode = ADMode::TYPE;
            }
            Command::InsertIf => {
                new_token! { From state, NodeKind::CONDTL => [piece!(..#)] @ vec![0] ; {
                 make_node!(line 0 -> NodeKind::CONDTLY []; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                 }),
                 make_node!(line 0 -> NodeKind::CONDTLN []; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                 })
                }};
            }
            Command::InsertFor => {
                new_token! { From state, NodeKind::FORLOOP => [piece!(..#), Piece::OP(OpKind::IN), piece!(..#)] @ vec![0] ; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                }};
            }
            Command::InsertWhile => {
                new_token! { From state, NodeKind::WHLLOOP => [piece!(..#)] @ vec![0] ; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                }};
            }
            Command::InsertBreak => new_token! { From state, NodeKind::BREAK => [] },
            Command::InsertContinue => new_token! { From state, NodeKind::CONTINUE => [] },
            Command::InsertReturn => {
                new_token! { From state, NodeKind::RETURN => [piece!(..#)] @ vec![0] };
            }
            Command::InsertOutput => {
                new_token! { From state, NodeKind::OUTPUT => [piece!(..#)] @ vec![0] };
            }
            Command::InsertPending => {
                new_token! { From state, NodeKind::PENDING => [piece!(..#)] @ vec![0] };
            }
            Command::InsertImport => {
                new_token! { From state, NodeKind::GRABPKG => [
                    piece!(IDENT ""), piece!(..#) // TODO: Only "from" and "alias" can go here
                ] @ vec![0]};
                state.mode = ADMode::TYPE;
            }
            Command::AddVarName => new_piece(state, piece!(IDENT ""))?,
            Command::AddTrue => new_piece(state, piece!(True))?,
            Command::AddFalse => new_piece(state, piece!(False))?,
            Command::AddNothing => new_piece(state, piece!())?,
            Command::AddText => new_piece(state, piece!(TEXT ""))?,
            Command::AddNum => new_piece(state, piece!(# 0))?,
            Command::AddList => new_piece(state, piece!(LIST [piece!(IDENT "list"), piece!(..#)]))?,
            Command::AddCall => new_piece(state, Piece::FNCALL(vec![piece!(IDENT "")]))?,
            Command::ChainAdd => new_piece(state, Piece::OP(OpKind::ADD))?,
            Command::ChainSub => new_piece(state, Piece::OP(OpKind::SUB))?,
            Command::ChainMul => new_piece(state, Piece::OP(OpKind::MUL))?,
            Command::ChainDiv => new_piece(state, Piece::OP(OpKind::DIV))?,
            Command::ChainMod => new_piece(state, Piece::OP(OpKind::MOD))?,
            Command::ChainGt => new_piece(state, Piece::OP(OpKind::GT))?,
            Command::ChainLt => new_piece(state, Piece::OP(OpKind::LT))?,
            Command::ChainGe => new_piece(state, Piece::OP(OpKind::GE))?,
            Command::ChainLe => new_piece(state, Piece::OP(OpKind::LE))?,
            Command::ChainEq => new_piece(state, Piece::OP(OpKind::EQ))?,
            Command::ChainNot => new_piece(state, Piece::OP(OpKind::NOT))?,
            Command::ChainAnd => new_piece(state, Piece::OP(OpKind::AND))?,
            Command::ChainOr => new_piece(state, Piece::OP(OpKind::OR))?,
            Command::ChainIdx => new_piece(state, Piece::OP(OpKind::AT))?,
            Command::ChainIn => new_piece(state, Piece::OP(OpKind::IN))?,
            Command::ChainDot => new_piece(state, Piece::OP(OpKind::DOT))?,
            Command::TryBracket => {
                let hash = state.graph.get_hash_mut();
                let Some(curr_node) = hash.get(&state.block_loc) else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };

                let Some(ref piece_ix) = state.piece_ix.clone() else {
                    unreachable!("Cannot add a new piece without editing a piece first");
                };

                // SAFETY: We know this reference must be valid because we just retrieved the
                // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
                // been dropped since its creation (no concurrency), so this is safe.
                let curr_node = unsafe { &mut **curr_node };
                if curr_node.kind != NodeKind::VARDECL || piece_ix != &[2] {
                    // piece_ix must equal 2 since "let name -> _bracket_"
                    return Err(CursorError::MisplacedBracket);
                }
                curr_node.pieces.insert(1, piece!(..#));
                curr_node.pieces.insert(1, Piece::OP(OpKind::AT));
            }
            Command::CommitChunk => {
                // If we are within args, move on to the next element
                let piece_ix = &state.piece_ix.clone().unwrap_or(vec![]);
                if piece_ix.len() > 1 {
                    let hash = state.graph.get_hash_mut();
                    let Some(curr_node) = hash.get(&state.block_loc) else {
                        return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                    };

                    let curr_node = unsafe { &mut **curr_node };

                    let parent_vec =
                        match curr_node.pieces[PieceIdx(&piece_ix[0..piece_ix.len() - 1])] {
                            Piece::LIST(ref mut args) | Piece::FNCALL(ref mut args) => args,
                            _ => return Err(CursorError::PieceAddrNotFound(piece_ix.to_vec())),
                        };

                    assert_eq!(parent_vec.last(), Some(&piece!(..+)));
                    let last_ix = parent_vec.len() - 1;
                    parent_vec[last_ix] = piece!(..#);
                    state.mode = ADMode::EDIT(Expecting::Value);
                }
            }
            Command::DeleteNode => {
                let hash_ref = state.graph.get_hash();
                let Some(curr_node) = hash_ref.get(&state.node_loc) else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };

                let curr_addr = curr_node.addr.clone();
                let parent_addr = curr_node.parent_addr.clone();

                let hash = state.graph.get_hash_mut();
                let Some(parent_node) = hash.get(&parent_addr) else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };

                let parent_node = unsafe { &mut **parent_node };
                let parent_len = parent_node.children.len();
                let num_fn = parent_node
                    .children
                    .iter()
                    .filter(|c| c.kind == NodeKind::FNDEF)
                    .collect::<Vec<_>>()
                    .len();
                for (i, child) in parent_node.children.iter().enumerate() {
                    if child.addr == curr_addr {
                        parent_node.children.remove(i);
                        (&mut state.graph[..]).fill_addr();

                        // Handle deletions of the last node in a parent's children
                        if i == parent_len - 1 {
                            if i == 0 {
                                parent_node
                                    .children
                                    .push(make_node!(line 0 -> NodeKind::PENDING [piece!(..#)]))
                            } else {
                                state.block_loc = parent_node.children[i - 1].addr.clone();
                                state.coerce()?;
                            }
                        } else if i == num_fn - 1 {
                            // If deleting the only sub-function there is, go back to the parent.
                            // Otherwise, go to the last function.
                            state.block_loc = if i == 0 {
                                parent_addr
                            } else {
                                parent_node.children[i - 1].addr.clone()
                            };
                            state.coerce()?;
                            state.block_loc = state.navigate(CursorDir::OUT)?;
                            state.coerce()?;
                        }
                        break;
                    }
                }
            }
            Command::Run => {
                let rtl = state.to_rtl();
                let s = crate::runner::compile(rtl, "auralstudio_la.rattle".into());
                // TODO: Clean up
                state.output = Some(
                    String::from_utf8(
                        std::process::Command::new("python")
                            .arg("../auralstudio_la.py")
                            .output()
                            .unwrap()
                            .stdout,
                    )
                    .unwrap(),
                );
            }
            Command::TypeChar(c) => {
                if c == "Enter" {
                    let Some(ref mut piece_ix) = state.piece_ix.clone() else {
                        unreachable!("cannot be in TYPE mode without a `piece_ix`");
                    };

                    let hash = state.graph.get_hash_mut();
                    let Some(curr_node) = hash.get(&state.block_loc) else {
                        return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                    };

                    let curr_node = unsafe { &mut **curr_node };
                    match &curr_node.pieces[PieceIdx(piece_ix)] {
                        Piece::IDENT(s) if s.is_empty() => return Err(CursorError::EmptyIdent),
                        _ => {}
                    };

                    // TODO: let x at 3 be 2 -> x[3] = 2: How do we allow this?
                    state._move_to_next(
                        curr_node,
                        Some(piece_ix),
                        curr_node.kind == NodeKind::FNDEF,
                    )?;
                }
            }
            Command::NULL => eprintln!("Received a null command"),
        }

        return Ok(());
    }
}

fn dir_map(command: Command) -> CursorDir {
    match command {
        Command::NavUp => CursorDir::UP,
        Command::NavDown => CursorDir::DOWN,
        Command::NavLeft => CursorDir::LEFT,
        Command::NavRight => CursorDir::RIGHT,
        Command::NavIn => CursorDir::IN,
        Command::NavOut => CursorDir::OUT,
        _ => unreachable!("from dir_map"),
    }
}
