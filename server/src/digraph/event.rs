use super::parser::{NodeKind, Piece};
use super::state::{ADMode, CursorError};
use crate::digraph::address::Addressable;
use crate::digraph::command::Command;
use crate::digraph::parser::OpKind;
use crate::digraph::state::{CursorDir, CursorState};
use crate::{make_node, piece};
use serde_derive::{Deserialize, Serialize};

macro_rules! insert {
    (From $state:ident, $kind:expr => [$($piece:expr),*] $(@ $piece_ix:literal),*) => {{
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

        $state.mode = ADMode::TYPE;
        $state.piece_ix = None;
        $($state.piece_ix = Some($piece_ix)),*
    }};

    (From $state:ident, $kind:expr => [$($piece:expr),*] $(@ $piece_ix:literal),* < {$($node:expr),+}) => {{
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

        $state.mode = ADMode::TYPE;
        $state.piece_ix = None;
        $($state.piece_ix = Some($piece_ix)),*
    }};
}

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
            Command::ViewMode => state.to_view(),
            Command::InsertVar => {
                insert! {
                    From state, NodeKind::VARDECL => [
                        piece!(IDENT ""), Piece::OP(OpKind::ASSN), piece!(...)
                    ] @ 0
                }
            }
            Command::InsertIf => {
                insert! { From state, NodeKind::CONDTL => [piece!(...)] @ 0 < {
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
                insert! { From state, kind => [piece!(...)] @ 0 < {
                     make_node!(line 0 -> NodeKind::PENDING [piece!(...)])
                }}
            }
            Command::InsertBreak => insert! { From state, NodeKind::BREAK => [] },
            Command::InsertContinue => insert! { From state, NodeKind::CONTINUE => [] },
            Command::InsertReturn => insert! { From state, NodeKind::RETURN => [piece!(...)] @ 0 },
            Command::InsertOutput => insert! { From state, NodeKind::OUTPUT => [piece!(...)] @ 0 },
            Command::InsertPending => {
                insert! { From state, NodeKind::PENDING => [piece!(...)] @ 0 }
            }
            Command::InsertImport => {
                insert! { From state, NodeKind::GRABPKG => [
                    piece!(IDENT ""), piece!(...) // TODO: Only "from" and "alias" can go here
                ] @ 0}
            }
            Command::Run => todo!(),
            Command::TypeChar(c) => {
                if c == "Enter" {
                    let Some(piece_ix) = state.piece_ix else {
                        unreachable!("cannot be in TYPE mode without a `piece_ix`");
                    };
                    let hash = state.graph.get_hash();
                    let Some(curr_node) = hash.get(&state.block_loc) else {
                        return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                    };

                    for i in piece_ix..curr_node.pieces.len() {
                        if curr_node.pieces[i] == Piece::PENDING
                            || curr_node.pieces[i] == Piece::IDENT("".into())
                        {
                            state.piece_ix = Some(i);
                            break;
                        }
                    }
                    // Typing always indicates we are working on a value, so next is an operator
                    state.mode = ADMode::EDIT(super::state::Expecting::Op);
                }
            }
            _ => {}
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
