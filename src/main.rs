use gtk::gdk;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::ControlFlow;
use gtk::prelude::*;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::prelude::FactoryVecDeque;
use relm4::{ComponentParts, RelmApp, RelmWidgetExt, SimpleComponent};
use std::time::Instant;

mod types;
use types::cell::{Cell, CellContent, CellOutput, CellPosition, CellState};
use types::game::{GameDifficulty, GameState};

const APP_ICON: &[u8] = include_bytes!("../logo.png");

type Board = Vec<Vec<Cell>>;

struct App {
    game_state: GameState,
    difficulty: GameDifficulty,
    board: Board,
    cells: FactoryVecDeque<Cell>,
    revealed_cells_count: usize,
    flagged_cells_count: usize,
    mined_cells: Vec<CellPosition>,
    start_time: Instant,
    elapsed_seconds: u64,
}

#[derive(Debug)]
enum Msg {
    Restart,
    Reveal(usize),
    Flag(usize),
    ChangeDifficulty(GameDifficulty),
    ShowAbout,
    Tick,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = Msg;
    type Output = ();
    type Init = GameDifficulty;

    view! {
        main_window = gtk::Window {
            set_title: Some("Rusty Minesweeper"),
            set_resizable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,

                        gtk::PopoverMenuBar::from_model(Some(&main_menu)) {
                    }
                },


                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_hexpand: true,
                        set_homogeneous: true,
                        set_spacing: 5,
                        set_margin_all: 5,

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Center,
                            add_css_class: "mines_remaining",
                            #[watch]
                            set_label: &format!("{:03}", model.difficulty.mines_count.saturating_sub(model.flagged_cells_count))
                        },

                        #[name(restart_button)]
                        gtk::Button {
                            set_hexpand: true,
                            set_halign: gtk::Align::Center,
                            set_size_request: (50,50),
                            add_css_class: "restart_button",
                            #[watch]
                            set_label: match &model.game_state {
                                GameState::Won => "😎",
                                GameState::Lost => "👺",
                                GameState::Started => "🙂",
                            },
                            connect_clicked => Msg::Restart,
                        },

                        #[name(timer_label)]
                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Center,
                            add_css_class: "time_remaining",
                            #[watch]
                            set_label: &format!("{:03}", model.elapsed_seconds)
                        },
                    },

                    #[local_ref]
                    cells_grid -> gtk::Grid {
                    }
                }
            }
        }
    }

    menu! {
        main_menu: {
            custom: "menubar",
            "Game" {
                "Difficulty" {
                    "Beginner" => SetDifficultyBeginner,
                    "Intermediate" => SetDifficultyIntermediate,
                    "Expert" => SetDifficultyExpert,
                    // "Custom" => SetDifficultyCustom,
                },
            },
            "Help" {
                "About" => ShowAbout
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cells =
            FactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), |msg| match msg {
                    CellOutput::Reveal(index) => Msg::Reveal(index),
                    CellOutput::Flag(index) => Msg::Flag(index),
                });

        let model = Self::new(init, cells);

        // Set up timer to update every second
        let sender_clone = sender.clone();
        gtk::glib::timeout_add_seconds_local(1, move || {
            sender_clone.input(Msg::Tick);
            ControlFlow::Continue
        });

        let cells_grid = model.cells.widget();
        let widgets = view_output!();

        // MenuBar
        let sender_clone = sender.clone();
        let action_setdifficultybeginner: RelmAction<SetDifficultyBeginner> = {
            RelmAction::new_stateless(move |_| {
                println!("Stateless action: SetDifficultyBeginner!");
                sender_clone.input(Msg::ChangeDifficulty(GameDifficulty::BEGINNER));
            })
        };
        let sender_clone = sender.clone();
        let action_setdifficultyintermediate: RelmAction<SetDifficultyIntermediate> = {
            RelmAction::new_stateless(move |_| {
                println!("Stateless action: SetDifficultyIntermediate!");
                sender_clone.input(Msg::ChangeDifficulty(GameDifficulty::INTERMEDIATE));
            })
        };
        let sender_clone = sender.clone();
        let action_setdifficultyexpert: RelmAction<SetDifficultyExpert> = {
            RelmAction::new_stateless(move |_| {
                println!("Stateless action: SetDifficultyExpert!");
                sender_clone.input(Msg::ChangeDifficulty(GameDifficulty::EXPERT));
            })
        };
        let sender_clone = sender;
        let action_showabout: RelmAction<ShowAbout> = {
            RelmAction::new_stateless(move |_| {
                println!("Stateless action: ShowAbout!");
                sender_clone.input(Msg::ShowAbout);
            })
        };
        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(action_setdifficultybeginner);
        group.add_action(action_setdifficultyintermediate);
        group.add_action(action_setdifficultyexpert);
        group.add_action(action_showabout);
        group.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Restart => {
                println!("Restart!");
                self.restart();
            }
            Msg::Reveal(index) => {
                let cell_pos = CellPosition::from_index(index, self.difficulty.board_size);
                println!(
                    "App::Reveal! index: {}, x:{}, y:{}, content: {}, state:{:?}",
                    index,
                    cell_pos.x,
                    cell_pos.y,
                    self.board[cell_pos.x][cell_pos.y].content,
                    self.board[cell_pos.x][cell_pos.y].state
                );

                if self.game_state.is_over() {
                    return;
                }

                if self.board[cell_pos.x][cell_pos.y].content.is_mine() {
                    self.set_game_over(GameState::Lost);
                    return;
                }

                self.reveal_area(cell_pos);

                if self.revealed_cells_count
                    == (self.difficulty.board_size * self.difficulty.board_size)
                        - self.difficulty.mines_count
                {
                    self.set_game_over(GameState::Won);
                }
            }
            Msg::Flag(index) => {
                let cell_pos = CellPosition::from_index(index, self.difficulty.board_size);
                println!("App::Flag: x:{}, y:{}", cell_pos.x, cell_pos.y);

                if self.game_state.is_over() {
                    return;
                }

                if self.board[cell_pos.x][cell_pos.y].state.is_revealed() {
                    return;
                }

                if !self.board[cell_pos.x][cell_pos.y].state.is_flagged()
                    && self.flagged_cells_count == self.difficulty.mines_count
                {
                    return;
                }

                self.flag_cell(cell_pos);
            }
            Msg::Tick => {
                if !self.game_state.is_over() {
                    self.elapsed_seconds = self.start_time.elapsed().as_secs();
                }
            }
            Msg::ChangeDifficulty(difficulty) => {
                println!("Change difficulty to {difficulty:?}!");
                self.difficulty = difficulty;
                self.restart();
            }
            Msg::ShowAbout => {
                // Show an "about" dialog
                println!("ShowAbout!");
                let dialog = gtk::AboutDialog::builder()
                    .program_name("Rusty Minesweeper")
                    .version("1.0")
                    .authors(vec!["not4rt".to_string()])
                    .comments("A Minesweeper clone written in Rust using GTK4 and Relm4")
                    .build();
                if let Ok(pixbuf) = Pixbuf::from_read(APP_ICON) {
                    let texture = gdk::Texture::for_pixbuf(&pixbuf);
                    dialog.set_logo(Some(&texture));
                }
                dialog.present();
            }
        }
    }
}

impl App {
    fn new(difficulty: GameDifficulty, mut cells: FactoryVecDeque<Cell>) -> Self {
        let (board, mined_cells) = generate_board(difficulty.board_size, difficulty.mines_count);

        for row in &board {
            for cell in row {
                cells.guard().push_back(*cell);
            }
        }

        Self {
            board,
            cells,
            difficulty,
            mined_cells,
            revealed_cells_count: 0,
            flagged_cells_count: 0,
            elapsed_seconds: 0,
            start_time: Instant::now(),
            game_state: GameState::default(),
        }
    }

    /// Change the state of cell at `cell_pos`, add to `self.revealed_cells_count` and update factory
    pub fn reveal_cell(&mut self, cell_pos: CellPosition) {
        self.board[cell_pos.x][cell_pos.y].state = CellState::Revealed;
        self.revealed_cells_count = self.revealed_cells_count.saturating_add(1);

        let mut cells_guard = self.cells.guard();
        cells_guard
            .get_mut(cell_pos.to_index(self.difficulty.board_size))
            .unwrap()
            .state = self.board[cell_pos.x][cell_pos.y].state;
    }

    pub fn reveal_area(&mut self, cell_pos: CellPosition) {
        if self.board[cell_pos.x][cell_pos.y].state.is_revealed()
            || self.board[cell_pos.x][cell_pos.y].state.is_flagged()
        {
            return;
        }

        self.reveal_cell(cell_pos);

        if self.board[cell_pos.x][cell_pos.y].content.is_blank() {
            for x in -1..=1 {
                for y in -1..=1 {
                    let new_x = cell_pos.x.checked_add_signed(x);
                    let new_y = cell_pos.y.checked_add_signed(y);

                    if new_x.is_none_or(|new_x| new_x >= self.difficulty.board_size)
                        || new_y.is_none_or(|new_y| new_y >= self.difficulty.board_size)
                    {
                        continue;
                    }
                    self.reveal_area(CellPosition {
                        x: new_x.unwrap(),
                        y: new_y.unwrap(),
                    });
                }
            }
        }
    }

    pub fn flag_cell(&mut self, cell_pos: CellPosition) {
        if self.board[cell_pos.x][cell_pos.y].state.is_flagged() {
            self.board[cell_pos.x][cell_pos.y].state = CellState::Hidden;
            self.flagged_cells_count = self.flagged_cells_count.saturating_sub(1);
        } else {
            self.board[cell_pos.x][cell_pos.y].state = CellState::Flagged;
            self.flagged_cells_count = self.flagged_cells_count.saturating_add(1);
        }

        let mut cells_guard = self.cells.guard();
        cells_guard
            .get_mut(cell_pos.to_index(self.difficulty.board_size))
            .unwrap()
            .state = self.board[cell_pos.x][cell_pos.y].state;
    }

    fn set_game_over(&mut self, state: GameState) {
        self.game_state = state;

        // Take ownership of mined_cells temporarily
        let mined_cells = std::mem::take(&mut self.mined_cells);

        // Reveal all unrevealed mines
        for cell_pos in &mined_cells {
            if self.game_state == GameState::Lost
                && !self.board[cell_pos.x][cell_pos.y].state.is_revealed()
            {
                self.reveal_cell(*cell_pos);
            } else if self.game_state == GameState::Won
                && !self.board[cell_pos.x][cell_pos.y].state.is_flagged()
            {
                self.flag_cell(*cell_pos);
            }
        }

        // Restore mined_cells
        self.mined_cells = mined_cells;
    }

    fn restart(&mut self) {
        self.flagged_cells_count = 0;
        self.revealed_cells_count = 0;
        self.game_state = GameState::Started;

        (self.board, self.mined_cells) =
            generate_board(self.difficulty.board_size, self.difficulty.mines_count);

        let mut cells_guard = self.cells.guard();

        // while cells_guard.len() > (self.difficulty.board_size * self.difficulty.board_size) {
        //     cells_guard.pop_back();
        // }
        //
        // for (row_index, row) in self.board.iter().enumerate() {
        //     for (column_index, cell) in row.iter().enumerate() {
        //         let cell_pos = CellPosition {
        //             x: row_index,
        //             y: column_index,
        //         };
        //
        //         if let Some(cell_reference) =
        //             cells_guard.get_mut(cell_pos.to_index(self.difficulty.board_size))
        //         {
        //             *cell_reference = *cell;
        //         } else {
        //             cells_guard.push_back(*cell);
        //         }
        //     }
        // }
        //
        // cells_guard.pop_front();
        // cells_guard.push_front(self.board[0][0]);

        if (self.difficulty.board_size * self.difficulty.board_size) == cells_guard.len() {
            for (row_index, row) in self.board.iter().enumerate() {
                for (column_index, cell) in row.iter().enumerate() {
                    let cell_pos = CellPosition {
                        x: row_index,
                        y: column_index,
                    };
                    *cells_guard
                        .get_mut(cell_pos.to_index(self.difficulty.board_size))
                        .unwrap() = *cell;
                }
            }
        } else {
            cells_guard.clear();
            for row in &self.board {
                for cell in row {
                    cells_guard.push_back(*cell);
                }
            }
        }

        self.elapsed_seconds = 0;
        self.start_time = Instant::now();
    }
}

fn generate_board(mut board_size: usize, mut mines_count: usize) -> (Board, Vec<CellPosition>) {
    // Validate inputs
    if board_size == 0 {
        println!("Invalid board_size, using 8 instead");
        board_size = 8;
    }

    let board_capacity = board_size * board_size;
    if mines_count >= board_capacity {
        println!("Invalid mines_count, using {} instead", board_capacity - 1);
        mines_count = board_capacity - 1;
    }

    // Initialize empty board
    let empty_cell = Cell {
        board_size,
        ..Default::default()
    };
    let mut board: Board = vec![vec![empty_cell; board_size]; board_size];
    let mut mines_placed = 0;
    let mut mined_cells: Vec<CellPosition> = Vec::with_capacity(mines_count);

    let mut rng = fastrand::Rng::new();

    while mines_placed < mines_count {
        let mine_pos = CellPosition {
            x: rng.usize(..board_size),
            y: rng.usize(..board_size),
        };

        if !board[mine_pos.x][mine_pos.y].content.is_mine() {
            // set mine
            board[mine_pos.x][mine_pos.y].content = CellContent::Mine;
            mined_cells.push(mine_pos);

            // Increment adjacent cells
            for x in -1..=1 {
                for y in -1..=1 {
                    let new_x = mine_pos.x.checked_add_signed(x);
                    let new_y = mine_pos.y.checked_add_signed(y);

                    if new_x.is_some_and(|new_x| new_x < board_size)
                        && new_y.is_some_and(|new_y| new_y < board_size)
                    {
                        board[new_x.unwrap()][new_y.unwrap()].content.add_one();
                    }
                }
            }

            mines_placed += 1;
        }
    }

    (board, mined_cells)
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(
    SetDifficultyBeginner,
    WindowActionGroup,
    "SetDifficultyBeginner"
);
relm4::new_stateless_action!(
    SetDifficultyIntermediate,
    WindowActionGroup,
    "SetDifficultyIntermediate"
);
relm4::new_stateless_action!(
    SetDifficultyExpert,
    WindowActionGroup,
    "SetDifficultyExpert"
);
relm4::new_stateless_action!(ShowAbout, WindowActionGroup, "ShowAbout");

fn main() {
    let app = RelmApp::new("not4rts.minesweeper");
    let difficulty = GameDifficulty::default();

    relm4::set_global_css(include_str!("css/style.css"));
    app.run::<App>(difficulty);
}
