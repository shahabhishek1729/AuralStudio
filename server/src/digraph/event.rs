use crate::digraph::state::{CursorDir, CursorState};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KeyboardEvent {
    // #[serde(rename = "altGraphKey")]
    // alt_graph_key: bool,
    // #[serde(rename = "altKey")]
    // alt_key: bool,
    // bubbles: bool,
    // #[serde(rename = "cancelBubble")]
    // cancel_bubble: bool,
    // cancelable: bool,
    // #[serde(rename = "charCode")]
    // char_code: usize,
    // code: String,
    // composed: bool,
    // #[serde(rename = "ctrlKey")]
    // ctrl_key: bool,
    // #[serde(rename = "currentTarget")]
    // current_target: Option<String>,
    // #[serde(rename = "defaultPrevented")]
    // default_prevented: bool,
    // detail: usize,
    // #[serde(rename = "eventPhase")]
    // event_phase: usize,
    // #[serde(rename = "isComposing")]
    // is_composing: bool,
    // #[serde(rename = "isTrusted")]
    // is_trusted: bool,
    pub key: String,
    // #[serde(rename = "keyCode")]
    // key_code: usize,
    // #[serde(rename = "keyIdentifier")]
    // key_identifier: String,
    // #[serde(rename = "keyLocation")]
    // key_location: usize,
    // #[serde(rename = "layerX")]
    // layer_x: usize,
    // #[serde(rename = "layerY")]
    // layer_y: usize,
    // location: usize,
    // #[serde(rename = "metaKey")]
    // meta_key: bool,
    // #[serde(rename = "pageX")]
    // page_x: usize,
    // #[serde(rename = "pageY")]
    // page_y: usize,
    // repeat: bool,
    // #[serde(rename = "returnValue")]
    // return_value: bool,
    // #[serde(rename = "shiftKey")]
    // shift_key: bool,
    // #[serde(rename = "timeStamp")]
    // time_stamp: u64,
    // #[serde(rename = "type")]
    // ty: String,
    // which: usize,
}

impl KeyboardEvent {
    pub(crate) fn parse_command(&self, state: &mut CursorState) {
        let dir = match &self.key[..] {
            "h" => CursorDir::LEFT,
            "j" => CursorDir::DOWN,
            "k" => CursorDir::UP,
            "l" => CursorDir::RIGHT,
            "Enter" => CursorDir::IN,
            "Backspace" => CursorDir::OUT,
            &_ => unreachable!("Only motion commands are currently supported"),
        };
        let Ok(new_addr) = state.navigate(dir) else {
            return;
        };
        state.block_loc = new_addr;
        let Ok(_) = state.coerce() else {
            return;
        };
    }
}
