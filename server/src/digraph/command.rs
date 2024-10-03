use super::state::ADMode;
use phf::phf_map;
use std::borrow::Cow;

/// Kinds of commands that can be made within the Editor.
/// Basic nomenclature:
/// - Nav*: Moves between nodes
/// - Insert*: Inserts nodes
/// - Add*: Inserts pieces
/// - *Mode: Toggles mode (viewing <-> editing)
#[derive(Debug, Clone, Copy)]
pub(super) enum Command<'a> {
    NavUp,
    NavDown,
    NavLeft,
    NavRight,
    NavIn,
    NavOut,
    InsertVar,
    EditMode,
    ViewMode,
    Run,
    NULL,
    TypeChar(&'a str),
}

const VIEW_KEYMAP: phf::Map<&'static str, Command> = phf_map! {
    "h" => Command::NavLeft,
    "j" => Command::NavDown,
    "k" => Command::NavUp,
    "l" => Command::NavRight,
    " " => Command::NavIn,
    "Backspace" => Command::NavOut,
    "Enter" => Command::EditMode,
    "r" => Command::Run,
};

const EDIT_KEYMAP: phf::Map<&'static str, Command> = phf_map! {
    "Escape" => Command::ViewMode,
    "v" => Command::InsertVar,
};

impl<'a> Command<'a> {
    pub(super) fn from(key: &'a str, mode: &ADMode) -> Cow<'a, Self> {
        match *mode {
            ADMode::VIEW => Cow::Borrowed(VIEW_KEYMAP.get(key).unwrap_or(&Command::NULL)),
            ADMode::EDIT(_) => Cow::Borrowed(EDIT_KEYMAP.get(key).unwrap_or(&Command::NULL)),
            ADMode::TYPE => Cow::Owned(Command::TypeChar(key)),
        }
    }
}
