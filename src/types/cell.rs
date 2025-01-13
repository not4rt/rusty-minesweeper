use gtk::prelude::{ButtonExt, GestureExt, GestureSingleExt, WidgetExt};
use relm4::factory::positions::GridPosition;
use relm4::prelude::{DynamicIndex, FactoryComponent, FactorySender};
use relm4::RelmWidgetExt;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellContent {
    Mine,
    Blank,
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
            Self::Blank => Self::One,
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
        }
    }

    pub const fn is_mine(self) -> bool {
        matches!(self, Self::Mine)
    }

    pub const fn is_blank(self) -> bool {
        matches!(self, Self::Blank)
    }

    pub const fn as_number(self) -> Option<u8> {
        match self {
            Self::Mine => None,
            Self::Blank => Some(0),
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
            Self::Blank => write!(f, "Blank"),
            content => write!(f, "{}", content.as_number().unwrap()),
        }
    }
}

impl fmt::Display for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mine => write!(f, "💣"),
            Self::Blank => write!(f, "-"),
            content => write!(f, "{}", content.as_number().unwrap()),
        }
    }
}

impl Default for CellContent {
    fn default() -> Self {
        Self::Blank
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellPosition {
    pub x: usize,
    pub y: usize,
}

impl CellPosition {
    pub const fn from_index(index: usize, board_size: usize) -> Self {
        Self {
            x: index / board_size,
            y: index % board_size,
        }
    }

    pub const fn to_index(self, board_size: usize) -> usize {
        (self.x * board_size) + self.y
    }
}

#[derive(Debug, Clone, Copy)]
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

impl CellState {
    pub const fn is_hidden(self) -> bool {
        matches!(self, Self::Hidden)
    }
    pub const fn is_revealed(self) -> bool {
        matches!(self, Self::Revealed)
    }
    pub const fn is_flagged(self) -> bool {
        matches!(self, Self::Flagged)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub content: CellContent,
    pub state: CellState,
    pub board_size: usize, // lol, this is the only way I found to set the row/column size dynamically on position method of relm4
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            CellState::Hidden => write!(f, " "),
            CellState::Revealed => write!(f, "{}", self.content),
            CellState::Flagged => write!(f, "🚩"),
        }
    }
}

#[derive(Debug)]
pub enum CellOutput {
    Reveal(usize),
    Flag(usize),
}

impl relm4::factory::Position<GridPosition, DynamicIndex> for Cell {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index();
        let x = (index % self.board_size) * 50;
        let y = (index / self.board_size) * 50;

        GridPosition {
            row: i32::try_from(x).expect("This conversion should always succeed. (C01)"),
            column: i32::try_from(y).expect("This conversion should always succeed. (C02)"),
            width: 50,
            height: 50,
        }
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for Cell {
    type Init = Self;
    type Input = ();
    type Output = CellOutput;
    type CommandOutput = ();
    type Widgets = CellWidgets;
    type ParentWidget = gtk::Grid;
    type Index = DynamicIndex;

    view! {
        #[root]
        gtk::Button {
            set_width_request: 50,
            set_margin_all: 1,
            #[watch]
            set_label: &self.to_string(),
            connect_clicked[sender, index] => move |_|{
                sender.output(CellOutput::Reveal(index.current_index())).unwrap();
            },
            add_controller = gtk::GestureClick {
                set_button: gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32,
                connect_pressed[sender, index] => move |gesture, _, _, _|{
                    sender.output(CellOutput::Flag(index.current_index())).unwrap();
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        init
    }

    fn init_widgets(
        &mut self,
        index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &gtk::Widget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let button = view_output!();

        button
    }
}
