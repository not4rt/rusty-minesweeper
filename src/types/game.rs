use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    Won,
    Lost,
    Started,
}

impl GameState {
    pub const fn is_over(&self) -> bool {
        matches!(self, Self::Won | Self::Lost)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Won => write!(f, "You Win!"),
            Self::Lost => write!(f, "You Lose!"),
            Self::Started => write!(f, "Game In Progress"),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::Started
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameDifficulty {
    pub board_size: usize,
    pub mines_count: usize,
}

impl GameDifficulty {
    pub const BEGINNER: Self = Self {
        board_size: 9,
        mines_count: 10,
    };
    pub const INTERMEDIATE: Self = Self {
        board_size: 16,
        mines_count: 50,
    };
    pub const EXPERT: Self = Self {
        board_size: 22,
        mines_count: 100,
    };
    pub const CUSTOM: Self = Self {
        board_size: 100,
        mines_count: 100,
    };
}

impl Default for GameDifficulty {
    fn default() -> Self {
        Self::BEGINNER
    }
}
