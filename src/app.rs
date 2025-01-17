use crate::components::button_cell::{ButtonCell, ButtonMsg, ButtonOutput};
use crate::game::state::GameState;
use crate::models::cell::CellPosition;
use crate::models::game::{GameDifficulty, GameStatus};
use gtk::gdk;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::ControlFlow;
use gtk::prelude::*;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::prelude::FactoryVecDeque;
use relm4::{ComponentParts, RelmWidgetExt, SimpleComponent};

const APP_ICON: &[u8] = include_bytes!("../logo.png");

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
                            set_size_request: (50,50),
                            add_css_class: "restart_button",
                            #[watch]
                            set_label: match model.game_state.status() {
                                GameStatus::Won => "😎",
                                GameStatus::Lost => "👺",
                                GameStatus::InProgress => "🙂",
                            },
                            connect_clicked => Msg::Restart,
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
                            set_size_request: (20*i32::try_from(model.game_state.difficulty().board_size).expect("This conversion should always succeed. (C03)"), 20*i32::try_from(model.game_state.difficulty().board_size).expect("This conversion should always succeed. (C03)")),
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
        let cells =
            FactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), |msg| match msg {
                    ButtonOutput::Reveal(index) => Msg::Reveal(index),
                    ButtonOutput::Flag(index) => Msg::Flag(index),
                });

        let model = Self::new(GameState::new(difficulty).unwrap(), cells);

        // Set up timer to update every second
        let sender_clone = sender.clone();
        gtk::glib::timeout_add_seconds_local(1, move || {
            sender_clone.input(Msg::Tick);
            ControlFlow::Continue
        });

        let cells_grid = model.cells.widget();
        let widgets = view_output!();

        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        // MenuBar
        let sender_clone = sender.clone();
        let action_set_beginner: RelmAction<SetDifficultyBeginnerAction> = {
            RelmAction::new_stateless(move |_| {
                sender_clone.input(Msg::ChangeDifficulty(GameDifficulty::BEGINNER));
            })
        };
        group.add_action(action_set_beginner);

        let sender_clone = sender.clone();
        let action_set_intermediate: RelmAction<SetDifficultyIntermediateAction> = {
            RelmAction::new_stateless(move |_| {
                sender_clone.input(Msg::ChangeDifficulty(GameDifficulty::INTERMEDIATE));
            })
        };
        group.add_action(action_set_intermediate);

        let sender_clone = sender.clone();
        let action_set_expert: RelmAction<SetDifficultyExpertAction> = {
            RelmAction::new_stateless(move |_| {
                sender_clone.input(Msg::ChangeDifficulty(GameDifficulty::EXPERT));
            })
        };
        group.add_action(action_set_expert);

        let sender_clone = sender.clone();
        let action_set_custom: RelmAction<SetDifficultyCustomAction> = {
            RelmAction::new_stateless(move |_| {
                sender_clone.input(Msg::ChangeDifficulty(GameDifficulty::CUSTOM));
            })
        };
        group.add_action(action_set_custom);
        let sender_clone = sender;
        let action_open_about: RelmAction<AboutAction> = {
            RelmAction::new_stateless(move |_| {
                sender_clone.input(Msg::ShowAbout);
            })
        };
        group.add_action(action_open_about);

        group.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Restart => {
                println!("Restart!");
                self.game_state
                    .restart()
                    .expect("Failed to restart game. Bad difficulty?");
                self.cells.broadcast(ButtonMsg::Display(String::new()));
                self.cells
                    .broadcast(ButtonMsg::SetCssClasses(vec!["square-button".to_string()]));
            }
            Msg::Reveal(index) => {
                let cell_pos =
                    CellPosition::from_index(index, self.game_state.difficulty().board_size);
                println!(
                    "App::Reveal! index: {}, x:{}, y:{}, display:{}",
                    index,
                    cell_pos.x,
                    cell_pos.y,
                    self.game_state
                        .display_cell(cell_pos)
                        .expect("Failed to display cell, position is invalid")
                );
                self.reveal(cell_pos);
            }
            Msg::Flag(index) => {
                let cell_pos =
                    CellPosition::from_index(index, self.game_state.difficulty().board_size);
                println!(
                    "App::Flag: x:{}, y:{}, display:{}",
                    cell_pos.x,
                    cell_pos.y,
                    self.game_state
                        .display_cell(cell_pos)
                        .expect("Failed to display cell, position is invalid")
                );

                self.game_state
                    .toggle_flag(cell_pos)
                    .expect("Failed to reveal cell. Bad position.");

                self.cells.send(
                    index,
                    ButtonMsg::Display(
                        self.game_state
                            .display_cell(cell_pos)
                            .expect("Failed to display cell"),
                    ),
                );
            }
            Msg::Tick => {
                self.game_state.tick();
            }
            Msg::ChangeDifficulty(difficulty) => {
                println!("Change difficulty to {difficulty:?}!");
                self.game_state.change_difficulty(difficulty);
                if self.cells.len() == (difficulty.board_size * difficulty.board_size) {
                    self.cells.broadcast(ButtonMsg::Display(String::new()));
                    self.cells
                        .broadcast(ButtonMsg::SetCssClasses(vec!["square-button".to_string()]));
                } else {
                    let mut cells_guard = self.cells.guard();
                    cells_guard.clear();
                    while cells_guard.len() < (difficulty.board_size * difficulty.board_size) {
                        cells_guard.push_front(ButtonCell::new(
                            difficulty.board_size,
                            vec!["square-button".to_string()],
                        ));
                    }
                    cells_guard.drop();
                }
            }
            Msg::ShowAbout => {
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
    fn new(game_state: GameState, mut cells: FactoryVecDeque<ButtonCell>) -> Self {
        let board_size = game_state.difficulty().board_size;
        let mut cells_guard = cells.guard();
        while cells_guard.len() < (board_size * board_size) {
            cells_guard.push_back(ButtonCell::new(
                board_size,
                vec!["square-button".to_string()],
            ));
        }
        cells_guard.drop();

        Self { game_state, cells }
    }

    fn reveal(&mut self, cell_pos: CellPosition) {
        if self.game_state.status() == &GameStatus::Lost {
            println!("You Lose!");
            return;
        }

        self.game_state
            .reveal_cell(cell_pos)
            .expect("Failed to reveal cell. Bad position.");

        for revealed_pos in self.game_state.revealed_cells() {
            let index = revealed_pos.to_index(self.game_state.difficulty().board_size);
            self.cells.send(
                index,
                ButtonMsg::Display(
                    self.game_state
                        .display_cell(*revealed_pos)
                        .expect("Failed to display cell"),
                ),
            );

            self.cells.send(
                index,
                ButtonMsg::AddCssClasses(vec!["revealed-cell".to_string()]),
            );
        }
        self.game_state.clear_revealed_cells();

        if self.game_state.status() == &GameStatus::Lost {
            let index = cell_pos.to_index(self.game_state.difficulty().board_size);
            self.cells.send(
                index,
                ButtonMsg::AddCssClasses(vec!["lost-cell".to_string()]),
            );
        } else if self.game_state.status() == &GameStatus::Won {
            for flagged_pos in self.game_state.flagged_cells() {
                let index = flagged_pos.to_index(self.game_state.difficulty().board_size);
                self.cells.send(index, ButtonMsg::Display("🚩".to_string()));
            }
        }
    }
}
