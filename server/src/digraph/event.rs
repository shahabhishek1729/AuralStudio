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

const PENDING_PIECES: &'static [Piece; 3] = &[
    Piece::PendingVal,
    Piece::PendingOp,
    Piece::IDENT(String::new()),
];

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
            Command::ViewMode => state.to_view()?,
            Command::InsertVar => {
                new_token! {
                    From state, NodeKind::VARDECL => [
                        piece!(IDENT ""), Piece::OP(OpKind::ASSN), piece!(..#)
                    ] @ vec![0]
                }
            }
            Command::InsertIf => {
                new_token! { From state, NodeKind::CONDTL => [piece!(..#)] @ vec![0] ; {
                 make_node!(line 0 -> NodeKind::CONDTLY []; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                 }),
                 make_node!(line 0 -> NodeKind::CONDTLN []; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                 })
                }}
            }
            Command::InsertFor => {
                new_token! { From state, NodeKind::FORLOOP => [piece!(..#), Piece::OP(OpKind::IN), piece!(..#)] @ vec![0] ; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                }}
            }
            Command::InsertWhile => {
                new_token! { From state, NodeKind::WHLLOOP => [piece!(..#)] @ vec![0] ; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(..#)])
                }}
            }
            Command::InsertBreak => new_token! { From state, NodeKind::BREAK => [] },
            Command::InsertContinue => new_token! { From state, NodeKind::CONTINUE => [] },
            Command::InsertReturn => {
                new_token! { From state, NodeKind::RETURN => [piece!(..#)] @ vec![0] }
            }
            Command::InsertOutput => {
                new_token! { From state, NodeKind::OUTPUT => [piece!(..#)] @ vec![0] }
            }
            Command::InsertPending => {
                new_token! { From state, NodeKind::PENDING => [piece!(..#)] @ vec![0] }
            }
            Command::InsertImport => {
                new_token! { From state, NodeKind::GRABPKG => [
                    piece!(IDENT ""), piece!(..#) // TODO: Only "from" and "alias" can go here
                ] @ vec![0]}
            }
            Command::AddVarName => new_piece(state, piece!(IDENT ""))?,
            Command::AddTrue => new_piece(state, piece!(True))?,
            Command::AddFalse => new_piece(state, piece!(False))?,
            Command::AddNothing => new_piece(state, piece!())?,
            Command::AddText => new_piece(state, piece!(TEXT ""))?,
            Command::AddNum => new_piece(state, piece!(# 0))?,
            Command::AddList => new_piece(state, piece!(LIST [piece!(IDENT "list"), piece!(..#)]))?,
            Command::AddCall => {
                new_piece(state, Piece::FNCALL(vec![piece!(IDENT ""), piece!(..#)]))?
            }
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
            Command::CommitChunk => todo!(),
            Command::Run => todo!(),
            Command::TypeChar(c) => {
                if c == "Enter" {
                    let Some(mut piece_ix) = state.piece_ix.clone() else {
                        unreachable!("cannot be in TYPE mode without a `piece_ix`");
                    };

                    let hash = state.graph.get_hash_mut();
                    let Some(curr_node) = hash.get(&state.block_loc) else {
                        return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                    };

                    // SAFETY: We know this reference must be valid because we just retrieved the
                    // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
                    // been dropped since its creation (no concurrency), so this is safe.
                    let curr_node = unsafe { &mut **curr_node };

                    // TODO: let x at 3 be 2 -> x[3] = 2: How do we allow this?

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

                        if let Some(spi) = state.piece_ix.as_mut() {
                            if let Some(last) = spi.last_mut() {
                                *last = parent_vec.len();
                            }
                        }

                        state.mode = ADMode::EDIT(Expecting::Op); // Must be OP after a value
                        parent_vec.push(piece!(..+));
                        return Ok(());
                    }

                    // Find the next pending piece in the local piece[]
                    // A pending piece is either an unnamed identifier, or an explicit pending
                    let piece_ix_len = piece_ix.len() - 1;
                    let start_i = piece_ix[piece_ix_len] + 1;
                    for i in start_i..parent_vec.len() {
                        piece_ix[piece_ix_len] = i;
                        if PENDING_PIECES.contains(&curr_node.pieces[PieceIdx(&piece_ix)]) {
                            state.piece_ix = Some(piece_ix);
                            match &parent_vec[i - 1] {
                                piece @ _ if PENDING_PIECES.contains(piece) => {
                                    unreachable!("should have reached earlier")
                                }
                                Piece::OP(_) => state.mode = ADMode::EDIT(Expecting::Value),
                                _ => state.mode = ADMode::EDIT(Expecting::Op),
                            }
                            break;
                        }
                    }
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
