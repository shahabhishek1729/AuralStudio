use super::parser::{NodeKind, Piece};
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
            Command::ViewMode => state.to_view()?,
            Command::InsertVar => {
                new_token! {
                    From state, NodeKind::VARDECL => [
                        piece!(IDENT ""), Piece::OP(OpKind::ASSN), piece!(...)
                    ] @ 0
                }
            }
            Command::InsertIf => {
                new_token! { From state, NodeKind::CONDTL => [piece!(...)] @ 0 < {
                 make_node!(line 0 -> NodeKind::CONDTLY []; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(...)])
                 }),
                 make_node!(line 0 -> NodeKind::CONDTLN []; {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(...)])
                 })
                }}
            }
            Command::InsertFor | Command::InsertWhile => {
                let kind = match *command {
                    Command::InsertFor => NodeKind::FORLOOP,
                    Command::InsertWhile => NodeKind::WHLLOOP,
                    _ => unreachable!(),
                };
                new_token! { From state, kind => [piece!(...)] @ 0 < {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(...)])
                }}
            }
            Command::InsertBreak => new_token! { From state, NodeKind::BREAK => [] },
            Command::InsertContinue => new_token! { From state, NodeKind::CONTINUE => [] },
            Command::InsertReturn => {
                new_token! { From state, NodeKind::RETURN => [piece!(...)] @ 0 }
            }
            Command::InsertOutput => {
                new_token! { From state, NodeKind::OUTPUT => [piece!(...)] @ 0 }
            }
            Command::InsertPending => {
                new_token! { From state, NodeKind::PENDING => [piece!(...)] @ 0 }
            }
            Command::InsertImport => {
                new_token! { From state, NodeKind::GRABPKG => [
                    piece!(IDENT ""), piece!(...) // TODO: Only "from" and "alias" can go here
                ] @ 0}
            }
            Command::AddVarName => new_piece(state, piece!(IDENT ""))?,
            Command::AddTrue => new_piece(state, piece!(True))?,
            Command::AddFalse => new_piece(state, piece!(False))?,
            Command::AddNothing => new_piece(state, piece!())?,
            Command::AddText => new_piece(state, piece!(TEXT ""))?,
            Command::AddNum => new_piece(state, piece!(# 0))?,
            Command::AddList => todo!(),
            Command::AddCall => todo!(),
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
            Command::Run => todo!(),
            Command::TypeChar(c) => {
                if c == "Enter" {
                    let Some(piece_ix) = state.piece_ix else {
                        unreachable!("cannot be in TYPE mode without a `piece_ix`");
                    };

                    if state.piece_ix == Some(0) {
                        // This was likely a variable name, so next is another value
                        // TODO: let x at 3 be 2 -> x[3] = 2: How to allow this?
                        state.mode = ADMode::EDIT(super::state::Expecting::Value);
                    } else {
                        // Typing always indicates we are working on a value, so next is an operator
                        state.mode = ADMode::EDIT(super::state::Expecting::Op);
                    }

                    let hash = state.graph.get_hash_mut();
                    let Some(curr_node) = hash.get(&state.block_loc) else {
                        return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                    };

                    // SAFETY: We know this reference must be valid because we just retrieved the
                    // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
                    // been dropped since its creation (no concurrency), so this is safe.
                    let curr_node = unsafe { &mut **curr_node };
                    if piece_ix == curr_node.pieces.len() - 1 {
                        state.piece_ix = Some(curr_node.pieces.len());
                        curr_node.pieces.push(piece!(...));
                        return Ok(());
                    }

                    for i in piece_ix..curr_node.pieces.len() {
                        if curr_node.pieces[i] == Piece::PENDING
                            || curr_node.pieces[i] == Piece::IDENT("".into())
                        {
                            state.piece_ix = Some(i);
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
