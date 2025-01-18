use crate::components::button_cell::{ButtonCell, ButtonMsg, ButtonOutput};
use crate::game::models::board::RevealResult;
use crate::game::models::cell::CellPosition;
use crate::game::models::game::GameDifficulty;
use crate::game::state::GameState;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::ControlFlow;
use gtk::{gdk, prelude::*};
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::prelude::FactoryVecDeque;
use relm4::{ComponentParts, RelmWidgetExt, SimpleComponent};
use std::rc::Rc;

const APP_ICON: &[u8] = include_bytes!("../logo.png");
const SQUARE_BUTTON_CLASS: &str = "square-button";
const REVEALED_CELL_CLASS: &str = "revealed-cell";
const LOST_CELL_CLASS: &str = "lost-cell";
const CELL_SIZE: i32 = 20;
const EMPTY_STRING: String = String::new();

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(
    SetDifficultyBeginnerAction,
    WindowActionGroup,
    "difficulty-beginner"
);
relm4::new_stateless_action!(
    SetDifficultyIntermediateAction,
    WindowActionGroup,
    "difficulty-intermediate"
);
relm4::new_stateless_action!(
    SetDifficultyExpertAction,
    WindowActionGroup,
    "difficulty-expert"
);
relm4::new_stateless_action!(
    SetDifficultyCustomAction,
    WindowActionGroup,
    "difficulty-custom"
);
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

pub struct App {
    game_state: GameState,
    cells: FactoryVecDeque<ButtonCell>,
}

#[derive(Debug)]
pub enum Msg {
    Restart,
    Reveal(usize),
    Flag(usize),
    ChangeDifficulty(GameDifficulty),
    ShowAbout,
    Tick,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Input = Msg;
    type Output = ();
    type Init = GameDifficulty;

    view! {
        main_window = gtk::Window {
            set_title: Some("Rusty Minesweeper"),
            set_resizable: false,

            gtk::Box {
                set_css_classes: &["main-box"],
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    gtk::PopoverMenuBar::from_model(Some(&main_menu)) {
                        set_css_classes: &["menu-bar"],
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_css_classes: &["inner-box"],
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_css_classes: &["top-box"],
                        set_hexpand: true,
                        set_homogeneous: true,

                        #[name(mines_remaining_label)]
                        gtk::Label {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["mines_remaining_label", "seven-segment"],
                            #[watch]
                            set_label: &format!("{:03}", model.game_state.flags_remaining())
                        },

                        #[name(restart_button)]
                        gtk::Button {
                            set_hexpand: true,
                            set_halign: gtk::Align::Center,
                            set_size_request: (50, 50),
                            add_css_class: "restart_button",
                            #[watch]
                            set_label: &model.game_state.status().to_string(),
                            connect_clicked[sender] => move |_| {
                                sender.input(Msg::Restart);
                            },
                        },

                        #[name(time_remaining_label)]
                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::End,
                            set_css_classes: &["time_remaining_label", "seven-segment"],
                            #[watch]
                            set_label: &format!("{:03}", model.game_state.elapsed_seconds())
                        },
                    },

                    gtk::Box {
                        set_css_classes: &["bottom-box"],
                        #[local_ref]
                        cells_grid -> gtk::Grid {
                            set_row_homogeneous: true,
                            set_column_homogeneous: true,
                            #[watch]
                            set_size_request: (CELL_SIZE * i32::try_from(model.game_state.difficulty().board_size).unwrap_or(1),
                                             CELL_SIZE * i32::try_from(model.game_state.difficulty().board_size).unwrap_or(1)),
                        }
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
                    "Beginner" => SetDifficultyBeginnerAction,
                    "Intermediate" => SetDifficultyIntermediateAction,
                    "Expert" => SetDifficultyExpertAction,
                    "Custom" => SetDifficultyCustomAction,
                },
            },
            "Help" {
                "About" => AboutAction
            },
        }
    }

    fn init(
        difficulty: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let game_state = GameState::new(difficulty).expect("Failed to create game state");

        let cells: FactoryVecDeque<ButtonCell> = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |msg| match msg {
                ButtonOutput::Reveal(index) => Msg::Reveal(index),
                ButtonOutput::Flag(index) => Msg::Flag(index),
            });

        let model = Self::new(game_state, cells);

        let sender_clone = sender.clone();
        gtk::glib::timeout_add_seconds_local(1, move || {
            sender_clone.input(Msg::Tick);
            ControlFlow::Continue
        });

        let cells_grid = model.cells.widget();
        let widgets = view_output!();

        Self::setup_actions(sender, &widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Restart => self.handle_restart(),
            Msg::Reveal(index) => self.handle_reveal(index),
            Msg::Flag(index) => self.handle_flag(index),
            Msg::Tick => self.game_state.tick(),
            Msg::ChangeDifficulty(difficulty) => self.handle_difficulty_change(difficulty),
            Msg::ShowAbout => Self::show_about_dialog(),
        }
    }
}

impl App {
    fn new(game_state: GameState, mut cells: FactoryVecDeque<ButtonCell>) -> Self {
        let board_size = game_state.difficulty().board_size;

        let mut cells_guard = cells.guard();

        while cells_guard.len() < (board_size * board_size) {
            cells_guard.push_back(ButtonCell::new(
                board_size,
                vec![SQUARE_BUTTON_CLASS.to_string()],
            ));
        }
        drop(cells_guard);

        Self { game_state, cells }
    }

    fn handle_restart(&mut self) {
        self.game_state
            .restart()
            .expect("Failed to restart game. Bad difficulty?");

        self.cells.broadcast(ButtonMsg::Display(EMPTY_STRING));
        self.cells.broadcast(ButtonMsg::SetCssClasses(vec![
            SQUARE_BUTTON_CLASS.to_string()
        ]));
    }

    fn handle_reveal(&mut self, index: usize) {
        let board_size = self.game_state.difficulty().board_size;
        let cell_pos = CellPosition::from_index(index, board_size);

        match self.game_state.reveal_cell(cell_pos) {
            Ok(RevealResult::CantReveal) | Err(_) => (),
            Ok(reveal_result) => {
                for &revealed_pos in self.game_state.revealed_cells() {
                    let revealed_index = revealed_pos.to_index(board_size);
                    if let Ok(display) = self.game_state.display_cell(revealed_pos) {
                        self.cells.send(revealed_index, ButtonMsg::Display(display));
                        self.cells.send(
                            revealed_index,
                            ButtonMsg::AddCssClasses(vec![REVEALED_CELL_CLASS.to_string()]),
                        );
                    }
                }
                self.game_state.clear_revealed_cells();

                if reveal_result == RevealResult::GameOver {
                    self.cells.send(
                        index,
                        ButtonMsg::AddCssClasses(vec![LOST_CELL_CLASS.to_string()]),
                    );
                } else if self.game_state.status().is_won() {
                    for flagged_pos in self.game_state.flagged_cells() {
                        let flag_index = flagged_pos.to_index(board_size);
                        self.cells
                            .send(flag_index, ButtonMsg::Display("🚩".to_string()));
                    }
                }
            }
        }
    }

    fn handle_flag(&mut self, index: usize) {
        let cell_pos = CellPosition::from_index(index, self.game_state.difficulty().board_size);
        if matches!(self.game_state.toggle_flag(cell_pos), Ok(true)) {
            if let Ok(display) = self.game_state.display_cell(cell_pos) {
                self.cells.send(index, ButtonMsg::Display(display));
            }
        }
    }

    fn handle_difficulty_change(&mut self, difficulty: GameDifficulty) {
        self.game_state.change_difficulty(difficulty);

        let new_size = difficulty.board_size * difficulty.board_size;
        if self.cells.len() == new_size {
            self.cells.broadcast(ButtonMsg::Display(EMPTY_STRING));
            self.cells.broadcast(ButtonMsg::SetCssClasses(vec![
                SQUARE_BUTTON_CLASS.to_string()
            ]));
        } else {
            let mut cells_guard = self.cells.guard();
            cells_guard.clear();

            while cells_guard.len() < new_size {
                cells_guard.push_back(ButtonCell::new(
                    difficulty.board_size,
                    vec![SQUARE_BUTTON_CLASS.to_string()],
                ));
            }
        }
    }

    fn show_about_dialog() {
        let dialog = gtk::AboutDialog::builder()
            .program_name("Rusty Minesweeper")
            .version("2.0")
            .authors(vec!["not4rt".to_string()])
            .comments("A Minesweeper clone written in Rust using GTK4 and Relm4")
            .build();

        if let Ok(pixbuf) = Pixbuf::from_read(APP_ICON) {
            let texture = gdk::Texture::for_pixbuf(&pixbuf);
            dialog.set_logo(Some(&texture));
        }

        dialog.present();
    }

    fn setup_actions(sender: relm4::ComponentSender<Self>, window: &gtk::Window) {
        let mut group = RelmActionGroup::<WindowActionGroup>::new();

        let sender = Rc::new(sender);

        macro_rules! add_difficulty_action {
            ($action:ty, $difficulty:expr) => {
                let sender = sender.clone();
                group.add_action(RelmAction::<$action>::new_stateless(move |_| {
                    sender.input(Msg::ChangeDifficulty($difficulty));
                }));
            };
        }

        add_difficulty_action!(SetDifficultyBeginnerAction, GameDifficulty::BEGINNER);
        add_difficulty_action!(
            SetDifficultyIntermediateAction,
            GameDifficulty::INTERMEDIATE
        );
        add_difficulty_action!(SetDifficultyExpertAction, GameDifficulty::EXPERT);
        add_difficulty_action!(SetDifficultyCustomAction, GameDifficulty::CUSTOM);

        let sender = sender;
        group.add_action(RelmAction::<AboutAction>::new_stateless(move |_| {
            sender.input(Msg::ShowAbout);
        }));

        group.register_for_widget(window);
    }
}
