use super::edit::Editor;
use crate::digraph::command::Command;
use crate::digraph::state::{CursorDir, CursorState};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KeyboardEvent {
    pub key: String,
    /*
    #[serde(rename = "altGraphKey")]
    alt_graph_key: bool,
    #[serde(rename = "altKey")]
    alt_key: bool,
    bubbles: bool,
    #[serde(rename = "cancelBubble")]
    cancel_bubble: bool,
    cancelable: bool,
    #[serde(rename = "charCode")]
    char_code: usize,
    code: String,
    composed: bool,
    #[serde(rename = "ctrlKey")]
    ctrl_key: bool,
    #[serde(rename = "currentTarget")]
    current_target: Option<String>,
    #[serde(rename = "defaultPrevented")]
    default_prevented: bool,
    detail: usize,
    #[serde(rename = "eventPhase")]
    event_phase: usize,
    #[serde(rename = "isComposing")]
    is_composing: bool,
    #[serde(rename = "isTrusted")]
    is_trusted: bool,
    #[serde(rename = "keyCode")]
    key_code: usize,
    #[serde(rename = "keyIdentifier")]
    key_identifier: String,
    #[serde(rename = "keyLocation")]
    key_location: usize,
    #[serde(rename = "layerX")]
    layer_x: usize,
    #[serde(rename = "layerY")]
    layer_y: usize,
    location: usize,
    #[serde(rename = "metaKey")]
    meta_key: bool,
    #[serde(rename = "pageX")]
    page_x: usize,
    #[serde(rename = "pageY")]
    page_y: usize,
    repeat: bool,
    #[serde(rename = "returnValue")]
    return_value: bool,
    #[serde(rename = "shiftKey")]
    shift_key: bool,
    #[serde(rename = "timeStamp")]
    time_stamp: u64,
    #[serde(rename = "type")]
    ty: String,
    which: usize,
    */
}

impl KeyboardEvent {
    pub(crate) fn parse_command(
        &self,
        state: &mut CursorState,
        editor: Option<Editor>,
    ) -> Option<Editor> {
        dbg!("Called");
        let command = Command::from(&self.key, &state.mode);
        dbg!(&command);

        match command {
            Command::NavUp
            | Command::NavDown
            | Command::NavLeft
            | Command::NavRight
            | Command::NavIn
            | Command::NavOut => {
                // This is a navigation command, move in the correct direction
                let dir = match command {
                    Command::NavLeft => CursorDir::LEFT,
                    Command::NavDown => CursorDir::DOWN,
                    Command::NavUp => CursorDir::UP,
                    Command::NavRight => CursorDir::RIGHT,
                    Command::NavIn => CursorDir::IN,
                    Command::NavOut => CursorDir::OUT,
                    _ => unreachable!("from navigation command match"),
                };
                if let Ok(new_addr) = state.navigate(dir) {
                    state.block_loc = new_addr;
                    let _ = state.coerce();
                }
                return None;
            }
            Command::Insert => {
                // Create a new node "below" the current location ("below" = at_insert_loc)
                state.mode.toggle();
                // NOTE: XXX: This might be possible without a clone if we state is owned
                let Ok(editor) = Editor::new(state.clone()) else {
                    return None;
                };
                return Some(editor);
            }
            Command::Run => todo!(),
            _ => todo!(),
        }
    }
}
