use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellContent {
    Mine,
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl CellContent {
    pub fn add_one(&mut self) {
        *self = match self {
            Self::Mine | Self::Eight => return,
            Self::Empty => Self::One,
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
        }
    }

    #[must_use]
    pub const fn as_number(self) -> Option<u8> {
        match self {
            Self::Mine => None,
            Self::Empty => Some(0),
            Self::One => Some(1),
            Self::Two => Some(2),
            Self::Three => Some(3),
            Self::Four => Some(4),
            Self::Five => Some(5),
            Self::Six => Some(6),
            Self::Seven => Some(7),
            Self::Eight => Some(8),
        }
    }
}

impl fmt::Debug for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mine => write!(f, "Mine"),
            Self::Empty => write!(f, "Blank"),
            content => write!(f, "{}", content.as_number().unwrap()),
        }
    }
}

impl fmt::Display for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mine => write!(f, "💣"),
            Self::Empty => write!(f, " "),
            content => write!(f, "{}", content.as_number().unwrap()),
        }
    }
}

impl Default for CellContent {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellPosition {
    pub x: usize,
    pub y: usize,
}

impl CellPosition {
    #[must_use]
    pub const fn from_index(index: usize, board_size: usize) -> Self {
        Self {
            x: index / board_size,
            y: index % board_size,
        }
    }

    #[must_use]
    pub const fn to_index(self, board_size: usize) -> usize {
        (self.x * board_size) + self.y
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Hidden
    }
}

#[derive(Debug, Default, Clone, Copy, Hash)]
pub struct Cell {
    pub content: CellContent,
    pub state: CellState,
    pub board_size: usize, // lol, this is the only way I found to set the row/column size dynamically on position method of relm4
}

impl Cell {
    #[must_use]
    pub const fn is_hidden(self) -> bool {
        matches!(self.state, CellState::Hidden)
    }
    #[must_use]
    pub const fn is_revealed(self) -> bool {
        matches!(self.state, CellState::Revealed)
    }
    #[must_use]
    pub const fn is_flagged(self) -> bool {
        matches!(self.state, CellState::Flagged)
    }
    #[must_use]
    pub const fn is_mine(self) -> bool {
        matches!(self.content, CellContent::Mine)
    }
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self.content, CellContent::Empty)
    }
    pub fn reveal(&mut self) {
        self.state = CellState::Revealed;
    }
    pub fn flag(&mut self) -> bool {
        if self.is_hidden() {
            self.state = CellState::Flagged;
            true
        } else {
            false
        }
    }
    pub fn unflag(&mut self) -> bool {
        if self.is_flagged() {
            self.state = CellState::Hidden;
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            CellState::Hidden => write!(f, ""),
            CellState::Revealed => write!(f, "{}", self.content),
            CellState::Flagged => write!(f, "🚩"),
        }
    }
}
