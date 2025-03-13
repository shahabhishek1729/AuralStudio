use super::parser::{NodeKind, Piece, PieceIdx};
use super::state::Expecting;
use super::state::{ADMode, CursorError};
use crate::digraph::address::Addressable;
use crate::digraph::command::Command;
use crate::digraph::edit::new_piece;
use crate::digraph::parser::OpKind;
use crate::digraph::state::{Canvas, CursorDir};
use crate::file_utils::{auralstudio_dir, to_py};
use crate::static_analysis::analyzer::Analyzer;
use crate::static_analysis::ident::IDGraph;
use crate::{addr, new_token, play};
use crate::{make_node, piece};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;

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
    pub(crate) fn parse_command(&self, state: &mut Canvas) -> Result<(), CursorError> {
        let command = Command::from(&self.key, &state.mode);

        if state._at_node() {
            let continue_ = Self::_parse_node_level(state, *command)?;
            if !continue_ {
                return Ok(());
            }
        }

        match *command {
            Command::NavUp
            | Command::NavDown
            | Command::NavLeft
            | Command::NavRight
            | Command::NavIn
            | Command::NavOut => {
                // This is a navigation command, move in the correct direction
                match state.navigate(dir_map(*command)) {
                    Ok(new_addr) => {
                        state.block_loc = new_addr;
                        let _ = state.coerce();
                        state.piece_ix = None;
                    }
                    Err(CursorError::InvalidMotion(CursorDir::UP))
                        if state.block_loc.len() == 2 =>
                    {
                        // Going up from a global block goes up to the root
                        state.block_loc = addr!();
                    }
                    Err(e) => return Err(e),
                }
            }
            Command::EditMode => {
                if state.block_loc == addr!() {
                    state
                        .graph
                        .push(make_node!(line 0 -> NodeKind::FNDEF [piece!(IDENT "")]; {
                            make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                        }));
                    (&mut state.graph[..]).fill_addr();
                    state.block_loc = state.graph.last().expect("cannot be empty").addr.clone();
                    state.coerce()?;
                    state.piece_ix = Some(vec![0]);
                    state.mode = ADMode::TYPE;
                } else {
                    if state._at_node() {
                        let id_graph = IDGraph::from_state(&state);
                        id_graph.populate_valid_idents();
                        let res = Analyzer::analyze(&state, &id_graph);
                        match res {
                            Ok(_) => play!(from "../public/correct.mp3"),
                            Err(ref e) => {
                                let hash = state.graph.get_hash_mut();
                                let Some(curr_node) = hash.get(&state.block_loc) else {
                                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                                };

                                // SAFETY: We know this reference must be valid because we just retrieved the
                                // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
                                // been dropped since its creation (no concurrency), so this is safe.
                                let curr_node = unsafe { &mut **curr_node };
                                curr_node.err = Some(e.clone());

                                play!(from "../public/incorrect.mp3", @ vol 0.7);
                            }
                        }
                    }
                    state.to_insert()?;
                }
            }
            Command::InplaceEditMode => {
                if !state._at_node() {
                    return Err(CursorError::AmbiguousEdit);
                }

                let hash = state.graph.get_hash_mut();
                let Some(curr_node) = hash.get(&state.block_loc) else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };
                let curr_node = unsafe { &mut **curr_node };

                match curr_node.kind {
                    NodeKind::PENDING => {
                        state.piece_ix = None;
                        state.mode = ADMode::EDIT(Expecting::Token);
                    }
                    NodeKind::FNDEF => {
                        curr_node.pieces.push(piece!(IDENT ""));
                        state._move_to_next(curr_node, Some(&mut vec![0usize]), true)?;
                        state.mode = ADMode::TYPE;
                    }
                    _ => {
                        // Insert a blank node at the end
                        let Some(last) = curr_node.pieces.last() else {
                            unreachable!("Empty nodes are impossible");
                        };
                        curr_node.pieces.push(if last.resolves_to_val() {
                            piece!(..+)
                        } else {
                            piece!(..#)
                        });

                        state._move_to_next(curr_node, Some(&mut vec![0usize]), false)?;
                    }
                }
            }
            Command::ViewMode => state.to_view()?,
            Command::Escape => {
                let piece_ix = &state.piece_ix.clone().unwrap_or(vec![]);
                let hash = state.graph.get_hash_mut();
                let Some(curr_node) = hash.get(&state.block_loc) else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };

                let curr_node = unsafe { &mut **curr_node };

                // Allow escaping in TYPE mode only when adding function params.
                if state.mode == ADMode::TYPE
                    && !(curr_node.kind == NodeKind::FNDEF && *piece_ix != vec![0])
                {
                    return Err(CursorError::EscapeWhileType);
                }

                if piece_ix.len() > 1 {
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
            Command::InsertBreak | Command::InsertContinue => {
                // Ensure we're in a loop before we add either of these blocks
                let hash = state.graph.get_hash();
                let curr_node = hash.get(&state.block_loc);
                let Some(start_node) = curr_node else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };

                let mut curr_node = start_node;
                let mut in_loop = false;
                while curr_node.parent_addr.len() > 2 {
                    curr_node = hash
                        .get(&curr_node.parent_addr)
                        .expect("Cannot have no parent");
                    if curr_node.kind == NodeKind::FORLOOP || curr_node.kind == NodeKind::WHLLOOP {
                        in_loop = true;
                        break;
                    }
                }

                if !in_loop {
                    return Err(CursorError::NotInLoop);
                }

                let kind = match *command {
                    Command::InsertBreak => NodeKind::BREAK,
                    Command::InsertContinue => NodeKind::CONTINUE,
                    _ => unreachable!("within sub-match"),
                };
                new_token! { From state, kind => [] }
            }
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

                // First, we must ensure you're not deleting the root of a block.
                if curr_node.children.len() > 0 && state._at_node() {
                    return Err(CursorError::DeleteBlock);
                }

                let curr_kind = curr_node.kind.clone();
                let curr_addr = curr_node.addr.clone();
                let parent_addr = curr_node.parent_addr.clone();

                let hash = state.graph.get_hash_mut();

                if curr_kind == NodeKind::FNDEF && hash.get(&parent_addr).is_none() {
                    // Deleting root functions is handled separately, although logic is similar.
                    let ref mut parent_children = state.graph;
                    let parent_len = parent_children.len();
                    for (i, child) in parent_children.iter().enumerate() {
                        if child.addr == curr_addr {
                            if i == 0 {
                                return Err(CursorError::DeleteStartFn);
                            }
                            parent_children.remove(i);
                            let prev_addr = parent_children[i - 1].addr.clone();

                            (&mut state.graph[..]).fill_addr();

                            // Handle deletions of the last node in a parent's children
                            if i == parent_len - 1 {
                                state.block_loc = prev_addr;
                                state.coerce()?;
                                state.block_loc = state.navigate(CursorDir::OUT)?;
                            } else {
                                state.coerce()?;
                            }
                            break;
                        }
                    }
                } else {
                    // If we're deleting subfunctions, local blocks or individual nodes, the
                    // process is slightly more complicated.
                    let parent_children = match hash.get(&parent_addr) {
                        Some(_parent_node) => {
                            let parent_node = unsafe { &mut **_parent_node };
                            &mut parent_node.children
                        }
                        _ => return Err(CursorError::AddrNotFound(state.block_loc.clone())),
                    };

                    let parent_len = parent_children.len();
                    let num_fn = parent_children
                        .iter()
                        .filter(|c| c.kind == NodeKind::FNDEF)
                        .collect::<Vec<_>>()
                        .len();

                    for (i, child) in parent_children.iter().enumerate() {
                        if child.addr == curr_addr {
                            parent_children.remove(i);
                            (&mut state.graph[..]).fill_addr();

                            // Handle deletions of the last node in a parent's children
                            if i == parent_len - 1 {
                                if i == num_fn {
                                    parent_children.push(
                                        make_node!(L 0 @ state.block_loc.clone() => NodeKind::PENDING [piece!(..#)]),
                                    );
                                    state.coerce()?;
                                } else {
                                    state.block_loc = parent_children[i - 1].addr.clone();
                                    state.coerce()?;
                                }
                                (&mut state.graph[..]).fill_addr();
                            } else if num_fn > 0 && i == num_fn - 1 {
                                // If deleting the only sub-function there is, go back to the parent.
                                // Otherwise, go to the last function.
                                state.block_loc = if i == 0 {
                                    parent_addr
                                } else {
                                    parent_children[i - 1].addr.clone()
                                };
                                state.coerce()?;
                                state.block_loc = state.navigate(CursorDir::OUT)?;
                                state.coerce()?;
                                (&mut state.graph[..]).fill_addr();
                            } else {
                                state.coerce()?;
                            }
                            break;
                        }
                    }
                }
            }
            Command::Run => {
                let rtl = state.to_rtl();

                let filename = format!("{}.rattle", state.filename.trim());
                let target_dir = auralstudio_dir()?;
                let filename = target_dir.join(filename);
                let Some(filename) = filename.to_str() else {
                    panic!("Found an unreadable filename which shouldn't be possible")
                };

                let _ = crate::runner::compile(rtl, filename.to_string());
                let process = std::process::Command::new("python")
                    .arg(to_py(filename))
                    .output()?;

                let out = String::from_utf8(process.stdout).expect("stdout conversion can't fail");
                let mut err =
                    String::from_utf8(process.stderr).expect("stdout conversion can't fail");
                if !err.is_empty() {
                    err = format!("\n\nError:\n{}", err);
                }

                state.output = Some(format!("{}{}", out, err));
            }
            Command::SaveFile => {
                let rtl = state.to_rtl();

                let filename = format!("{}.rattle", state.filename.trim());
                let target_dir = auralstudio_dir()?;
                let filename = target_dir.join(filename);
                let Some(filename) = filename.to_str() else {
                    panic!("Found an unreadable filename which shouldn't be possible")
                };

                let mut file = std::fs::File::create(filename)?;
                file.write_all(rtl.as_bytes())?;
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

                    state._move_to_next(
                        curr_node,
                        Some(piece_ix),
                        curr_node.kind == NodeKind::FNDEF,
                    )?;
                }
            }
            Command::NULL => return Err(CursorError::InvalidCommand),
        }

        Ok(())
    }

    fn _parse_node_level(state: &mut Canvas, command: Command) -> Result<bool, CursorError> {
        match command {
            Command::NavIn | Command::NavRight | Command::NavLeft | Command::NavOut => {
                let dir = dir_map(command);
                match dir {
                    CursorDir::IN if state.piece_ix.is_none() => {
                        let mut pix = state.piece_ix.clone().unwrap_or_else(|| vec![]);
                        pix.push(0);

                        // Ensure that moving in results in a valid piece
                        let hash = state.graph.get_hash();
                        let Some(curr_node) = hash.get(&state.block_loc) else {
                            unreachable!("Node can never be null");
                        };
                        let curr_piece = &curr_node.pieces[PieceIdx(&pix)];
                        if curr_piece == &Piece::NULL {
                            return Err(CursorError::InvalidMotion(dir));
                        }
                        state.piece_ix = Some(pix);
                    }
                    CursorDir::OUT if state.piece_ix.is_some() => {
                        let mut pix = state.piece_ix.clone().expect("must have non-null piece_ix");
                        pix.pop();
                        if pix.is_empty() {
                            state.piece_ix = None
                        } else {
                            state.piece_ix = Some(pix);
                        }
                    }
                    CursorDir::LEFT if state.piece_ix.is_some() => {
                        let mut pix = state.piece_ix.clone().expect("must be non-null");
                        let len = pix.len();

                        if len == 0 || pix.last() == Some(&0) {
                            return Err(CursorError::InvalidMotion(dir));
                        }
                        pix[len - 1] -= 1;
                        state.piece_ix = Some(pix);
                    }
                    CursorDir::RIGHT if state.piece_ix.is_some() => {
                        let mut pix = state.piece_ix.clone().expect("must be non-null");
                        let len = pix.len();

                        if len == 0 {
                            return Err(CursorError::InvalidMotion(dir));
                        }
                        pix[len - 1] += 1;

                        // Ensure that moving in results in a valid piece
                        let hash = state.graph.get_hash();
                        let Some(curr_node) = hash.get(&state.block_loc) else {
                            unreachable!("Node can never be null");
                        };

                        let curr_piece = &curr_node.pieces[PieceIdx(&pix)];
                        if curr_piece == &Piece::NULL {
                            return Err(CursorError::InvalidMotion(dir));
                        }
                        state.piece_ix = Some(pix);
                    }
                    _ => return Err(CursorError::InvalidMotion(dir)),
                }
            }
            Command::InplaceEditMode if state.piece_ix.is_some() => {
                let hash = state.graph.get_hash();
                let Some(curr_node) = hash.get(&state.block_loc) else {
                    unreachable!("Node can never be null");
                };

                let Some(ref pix) = state.piece_ix else {
                    unreachable!("Can't be in match arm without passing guard clause");
                };
                let curr_piece = &curr_node.pieces[PieceIdx(pix)];
                match curr_piece {
                    Piece::IDENT(_) | Piece::NUMBER(_) | Piece::TEXT(_) => {
                        state.mode = ADMode::TYPE
                    }
                    _ => {}
                }
            }
            Command::DeleteNode if state.piece_ix.is_some() => {
                let hash = state.graph.get_hash_mut();
                let Some(curr_node) = hash.get(&state.block_loc) else {
                    unreachable!("Node can never be null");
                };

                let curr_node = unsafe { &mut **curr_node };
                let Some(ref mut pix) = state.piece_ix else {
                    unreachable!("Can't be in match arm without passing guard clause");
                };

                let curr_piece = &curr_node.pieces[PieceIdx(pix)].clone();

                let last_ix = pix.len() - 1;
                let (parent_vec, curr_ix) = if last_ix == 0 {
                    (&mut curr_node.pieces, pix[0])
                } else {
                    match curr_node.pieces[PieceIdx(&pix[0..last_ix])] {
                        Piece::LIST(ref mut args) | Piece::FNCALL(ref mut args) => {
                            (args, pix[last_ix])
                        }
                        _ => return Err(CursorError::PieceAddrNotFound(pix.to_vec())),
                    }
                };

                parent_vec.remove(curr_ix);

                // If there were two pending pieces in a row, remove both
                if parent_vec.len() > 1
                    && curr_ix < parent_vec.len()
                    && parent_vec[curr_ix].resolves_to_pending()
                {
                    parent_vec.remove(curr_ix);
                    pix[last_ix] = std::cmp::min(curr_ix, parent_vec.len() - 1);
                } else if parent_vec.len() > 1
                    && curr_ix > 0
                    && parent_vec[curr_ix - 1].resolves_to_pending()
                {
                    parent_vec.remove(curr_ix - 1);
                    pix[last_ix] = std::cmp::min(curr_ix, parent_vec.len() - 1);
                } else {
                    parent_vec.insert(
                        curr_ix,
                        if curr_piece.resolves_to_val() {
                            piece!(..#)
                        } else {
                            piece!(..+)
                        },
                    );
                }
            }
            _ => return Ok(true),
        }
        Ok(false)
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
