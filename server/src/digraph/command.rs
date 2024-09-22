use super::state::ADMode;
use phf::phf_map;

#[derive(Debug)]
pub(super) enum Command {
    NavUp,
    NavDown,
    NavLeft,
    NavRight,
    NavIn,
    NavOut,
    InsertMode,
    ViewMode,
    Run,
    InsertVar,
    NULL,
}

const VIEW_KEYMAP: phf::Map<&'static str, Command> = phf_map! {
    "h" => Command::NavLeft,
    "j" => Command::NavDown,
    "k" => Command::NavUp,
    "l" => Command::NavRight,
    " " => Command::NavIn,
    "Backspace" => Command::NavOut,
    "Enter" => Command::InsertMode,
    "r" => Command::Run,
};

const EDIT_KEYMAP: phf::Map<&'static str, Command> = phf_map! {
    "Escape" => Command::ViewMode,
    "v" => Command::InsertVar,
};

impl Command {
    pub(super) fn from<'a>(key: &'a str, mode: &ADMode) -> &'a Self {
        if *mode == ADMode::VIEW {
            VIEW_KEYMAP.get(key).unwrap_or(&Command::NULL)
        } else {
            EDIT_KEYMAP.get(key).unwrap_or(&Command::NULL)
        }
    }
}
