use std::fmt;

pub enum GameStatus {
    Won,
    Lost,
    InProgress,
}

impl GameStatus {
    #[must_use]
    pub const fn is_over(&self) -> bool {
        matches!(self, Self::Won | Self::Lost)
    }

    #[must_use]
    pub const fn is_won(&self) -> bool {
        matches!(self, Self::Won)
    }

    #[must_use]
    pub const fn is_lost(&self) -> bool {
        matches!(self, Self::Lost)
    }
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Won => write!(f, "😎"),
            Self::Lost => write!(f, "👺"),
            Self::InProgress => write!(f, "🙂"),
        }
    }
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::InProgress
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        mines_count: 10,
    };
}

impl Default for GameDifficulty {
    fn default() -> Self {
        Self::BEGINNER
    }
}
