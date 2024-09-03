use super::state::ADMode;

enum Command {
    NavUp,
    NavDown,
    NavLeft,
    NavRight,
    NavIn,
    NavOut,
    Insert
}

impl Command {
    fn from(key: &str, mode: ADMode) -> Self {
        if mode == ADMode::VIEW {
            match key {
                "h" => Self::NavLeft,
                "j" => Self::NavUp,
                "k" => Self::NavDown,
                "l" => Self::NavRight,
                " " => Self::NavIn,
                "Backspace" => Self::NavOut,
                "Enter" => Self::Insert
                _ => todo!(),
            }
        } else {
            match key {
                _ => todo!(),
            }
        }
    }
}
