use super::state::{ADMode, Expecting};
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
    InsertIf,
    InsertFor,
    InsertWhile,
    InsertReturn,
    InsertBreak,
    InsertContinue,
    InsertImport,
    InsertOutput,
    InsertPending,
    AddVarName,
    AddNum,
    AddText,
    AddTrue,
    AddFalse,
    AddNothing,
    AddCall,
    AddList,
    ChainAdd,
    ChainSub,
    ChainMul,
    ChainDiv,
    ChainMod,
    ChainEq,
    ChainGt,
    ChainLt,
    ChainGe,
    ChainLe,
    ChainAnd,
    ChainOr,
    ChainNot,
    ChainIn,
    ChainDot,
    ChainIdx,
    EditMode,
    ViewMode,
    Escape,
    CommitChunk,
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

const TOKEN_KEYMAP: phf::Map<&'static str, Command> = phf_map! {
    "v" => Command::InsertVar,
    "i" => Command::InsertIf,
    "f" => Command::InsertFor,
    "w" => Command::InsertWhile,
    "r" => Command::InsertReturn,
    "b" => Command::InsertBreak,
    "c" => Command::InsertContinue,
    "g" => Command::InsertImport,
    "o" => Command::InsertOutput,
    "p" => Command::InsertPending,
    "Escape" => Command::ViewMode,
};

const VAL_KEYMAP: phf::Map<&'static str, Command> = phf_map! {
    "v" => Command::AddVarName,
    "n" => Command::AddNum,
    "s" => Command::AddText,
    "t" => Command::AddTrue,
    "f" => Command::AddFalse,
    " " => Command::AddNothing,
    "c" => Command::AddCall,
    "l" => Command::AddList,
    "Escape" => Command::Escape,
};

const OP_KEYMAP: phf::Map<&'static str, Command> = phf_map! {
    "p" => Command::ChainAdd,
    "m" => Command::ChainSub,
    "t" => Command::ChainMul,
    "d" => Command::ChainDiv,
    "r" => Command::ChainMod,
    "e" => Command::ChainEq,
    "g" => Command::ChainGt,
    "l" => Command::ChainLt,
    "x" => Command::ChainGe,
    "s" => Command::ChainLe,
    "a" => Command::ChainAnd,
    "o" => Command::ChainOr,
    "n" => Command::ChainNot,
    "i" => Command::ChainIn,
    "c" => Command::ChainDot,
    "b" => Command::ChainIdx,
    "Escape" => Command::Escape,
    "Enter" => Command::CommitChunk,
};

impl<'a> Command<'a> {
    pub(super) fn from(key: &'a str, mode: &ADMode) -> Cow<'a, Self> {
        match *mode {
            ADMode::VIEW => Cow::Borrowed(VIEW_KEYMAP.get(key).unwrap_or(&Command::NULL)),
            ADMode::EDIT(Expecting::Token) => {
                Cow::Borrowed(TOKEN_KEYMAP.get(key).unwrap_or(&Command::NULL))
            }
            ADMode::EDIT(Expecting::Value) => {
                Cow::Borrowed(VAL_KEYMAP.get(key).unwrap_or(&Command::NULL))
            }
            ADMode::EDIT(Expecting::Op) => {
                Cow::Borrowed(OP_KEYMAP.get(key).unwrap_or(&Command::NULL))
            }
            ADMode::TYPE => Cow::Owned(Command::TypeChar(key)),
        }
    }
}
