use std::fmt;

use super::board::BoardSize;

pub enum GameStatus {
    Won,
    Lost,
    InProgress,
    New,
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

    #[must_use]
    pub const fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    #[must_use]
    pub const fn is_new(&self) -> bool {
        matches!(self, Self::New)
    }
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Won => write!(f, "😎"),
            Self::Lost => write!(f, "👺"),
            Self::InProgress | Self::New => write!(f, "🙂"),
        }
    }
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::InProgress
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameDifficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom(BoardSize, usize),
}

impl GameDifficulty {
    #[must_use]
    pub const fn board_size(&self) -> BoardSize {
        match self {
            Self::Beginner => BoardSize {
                width: 9,
                height: 9,
            },
            Self::Intermediate => BoardSize {
                width: 16,
                height: 16,
            },
            Self::Expert => BoardSize {
                width: 30,
                height: 16,
            },
            Self::Custom(board_size, _) => *board_size,
        }
    }
    #[must_use]
    pub const fn mines_count(&self) -> usize {
        match self {
            Self::Beginner => 10,
            Self::Intermediate => 40,
            Self::Expert => 99,
            Self::Custom(_, mines_count) => *mines_count,
        }
    }

    pub const fn set_width(&mut self, width: usize) {
        *self = Self::Custom(
            BoardSize {
                width,
                height: self.board_size().height,
            },
            self.mines_count(),
        );
    }

    pub const fn set_height(&mut self, height: usize) {
        *self = Self::Custom(
            BoardSize {
                width: self.board_size().width,
                height,
            },
            self.mines_count(),
        );
    }

    pub const fn set_mines_count(&mut self, mines_count: usize) {
        *self = Self::Custom(
            BoardSize {
                width: self.board_size().width,
                height: self.board_size().height,
            },
            mines_count,
        );
    }

    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_, _))
    }
}

impl Default for GameDifficulty {
    fn default() -> Self {
        Self::Beginner
    }
}
