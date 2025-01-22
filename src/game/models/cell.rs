use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub const fn as_number(self) -> u8 {
        match self {
            Self::Mine => 99,
            Self::Empty => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
        }
    }
}

impl fmt::Debug for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mine => write!(f, "Mine"),
            Self::Empty => write!(f, "Blank"),
            content => write!(f, "{}", content.as_number()),
        }
    }
}

impl fmt::Display for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mine => write!(f, "💣"),
            Self::Empty => write!(f, " "),
            content => write!(f, "{}", content.as_number()),
        }
    }
}

impl Default for CellContent {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellPosition {
    pub x: usize,
    pub y: usize,
}

impl CellPosition {
    #[must_use]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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

impl From<(usize, usize)> for CellPosition {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Default, Clone, Copy)]
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
