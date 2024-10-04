use super::parser::NodeKind;
use super::state::{ADMode, CursorError};
use crate::digraph::address::Addressable;
use crate::digraph::command::Command;
use crate::digraph::state::{CursorDir, CursorState};
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
            Command::ViewMode => state.to_view(),
            Command::InsertVar => {
                let hash = state.graph.get_hash_mut();
                let Some(curr_node) = hash.get(&state.block_loc) else {
                    return Err(CursorError::AddrNotFound(state.block_loc.clone()));
                };

                // SAFETY: We know this reference must be valid because we just retrieved the
                // `curr_node` pointer from our graph's hash. The nodes in that hash cannot have
                // been dropped since its creation (no concurrency), so this is safe.
                let curr_node = unsafe { &mut **curr_node };
                curr_node.kind = NodeKind::VARDECL;
                state.mode = ADMode::EDIT(super::state::Expecting::IdentPiece);
            }
            Command::Run => todo!(),
            Command::TypeChar(c) => todo!(),
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
