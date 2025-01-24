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
use std::time::Duration;

const APP_ICON: &[u8] = include_bytes!("../logo.png");
const REVEALED_CELL_CLASS: &str = "revealed-cell";
const LOST_CELL_CLASS: &str = "lost-cell";
const EMPTY_STRING: String = String::new();

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(NewGameAction, WindowActionGroup, "new-game");
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
relm4::new_stateless_action!(ExitAction, WindowActionGroup, "exit");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

pub struct App {
    game_state: GameState,
    mouse_tracker: MouseTracker,
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
    TrackMouse(f64, f64),
    LeftButtonPressed,
    LeftButtonReleased,
    RightButtonPressed,
    MiddleButtonPressed,
    MiddleButtonReleased,
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

                        #[name(mines_remaining_label)]
                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Start,
                            set_css_classes: &["mines_remaining_label", "seven-segment"],
                            #[watch]
                            set_label: &format!("{:03}", model.game_state.flags_remaining())
                        },

                        #[name(restart_button)]
                        gtk::Button {
                            set_halign: gtk::Align::Center,
                            set_size_request: (10, 10),
                            add_css_class: "restart_button",
                            #[watch]
                            set_label: &model.emoji_status(),
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
                            add_controller = gtk::EventControllerMotion {
                                connect_motion[sender]=> move |_, x, y| {
                                    sender.input(Msg::TrackMouse(x, y));
                                }
                            },
                            add_controller = gtk::GestureClick {
                                connect_pressed[sender] => move |_, _, _, _|{
                                    sender.input(Msg::LeftButtonPressed);
                                },
                                connect_released[sender] => move |_, _, _, _|{
                                    sender.input(Msg::LeftButtonReleased);
                                },
                            },
                            add_controller = gtk::GestureClick {
                                set_button: gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32,
                                connect_begin[sender] => move |_, _|{
                                    sender.input(Msg::RightButtonPressed);
                                },
                            },
                            add_controller = gtk::GestureClick {
                                set_button: gtk::gdk::ffi::GDK_BUTTON_MIDDLE as u32,
                                connect_pressed[sender] => move |_, _, _, _|{
                                    sender.input(Msg::MiddleButtonPressed);
                                },
                                connect_released[sender] => move |_, _, _, _|{
                                    sender.input(Msg::MiddleButtonReleased);
                                },
                            },
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
                section! {
                    "New" => NewGameAction,
                },
                section! {
                    "Beginner" => SetDifficultyBeginnerAction,
                    "Intermediate" => SetDifficultyIntermediateAction,
                    "Expert" => SetDifficultyExpertAction,
                    "Custom..." => SetDifficultyCustomAction,
                },
                section! {
                    "Exit" => ExitAction,
                },
            },
            "Help" {
                "About Rusty Minesweeper..." => AboutAction
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

        // let sender_clone = sender.clone();
        // gtk::glib::timeout_add_seconds_local(1, move || {
        //     sender_clone.input(Msg::Tick);
        //     ControlFlow::Continue
        // });

        // test fast tick to decrease the lag on difficulty change
        let sender_clone = sender.clone();
        gtk::glib::timeout_add_local(Duration::from_millis(4), move || {
            sender_clone.input(Msg::Tick);
            ControlFlow::Continue
        });

        let model = Self::new(game_state, cells);

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
            Msg::TrackMouse(x, y) => self.track_mouse(x, y),
            Msg::LeftButtonPressed => self.leftbutton_pressed(),
            Msg::LeftButtonReleased => self.leftbutton_released(),
            Msg::RightButtonPressed => self.rightbutton_pressed(),
            Msg::MiddleButtonPressed => self.middlebutton_pressed(),
            Msg::MiddleButtonReleased => self.middlebutton_released(),
        }
    }
}

impl App {
    fn new(game_state: GameState, mut cells: FactoryVecDeque<ButtonCell>) -> Self {
        let board_size = game_state.difficulty().board_size;

        let mut cells_guard = cells.guard();

        while cells_guard.len() < (board_size * board_size) {
            cells_guard.push_back(ButtonCell::new(board_size));
        }
        drop(cells_guard);

        Self {
            game_state,
            cells,
            mouse_tracker: MouseTracker::new(),
        }
    }

    fn handle_restart(&mut self) {
        self.game_state
            .restart()
            .expect("Failed to restart game. Bad difficulty?");

        self.cells.broadcast(ButtonMsg::Display(EMPTY_STRING));
        self.cells.broadcast(ButtonMsg::Reset);
    }

    fn handle_reveal(&mut self, index: usize) {
        let board_size = self.game_state.difficulty().board_size;
        let cell_pos = CellPosition::from_index(index, board_size);

        let reveal_result = self.game_state.reveal_cell(cell_pos);

        if let Ok(reveal_result) = reveal_result {
            self.reveal_cells(&reveal_result);
        }
    }

    fn reveal_cells(&mut self, reveal_result: &RevealResult) {
        let board_size = self.game_state.difficulty().board_size;

        for &revealed_pos in self.game_state.revealed_cells() {
            let revealed_index = revealed_pos.to_index(board_size);
            if let Ok(display) = self.game_state.display_cell(revealed_pos) {
                self.cells.send(revealed_index, ButtonMsg::Display(display));
                self.cells.send(
                    revealed_index,
                    ButtonMsg::AddCssClass(REVEALED_CELL_CLASS.to_string()),
                );
            }
        }
        self.game_state.clear_revealed_cells();

        match reveal_result {
            RevealResult::Continue => {
                if self.game_state.status().is_won() {
                    for flagged_pos in self.game_state.flagged_cells() {
                        let flag_index = flagged_pos.to_index(board_size);
                        self.cells
                            .send(flag_index, ButtonMsg::Display("🚩".to_string()));
                    }
                    self.game_state.clear_flagged_cells();
                }
            }
            RevealResult::GameOver(mine_pos) => self.cells.send(
                mine_pos.to_index(board_size),
                ButtonMsg::AddCssClass(LOST_CELL_CLASS.to_string()),
            ),
            RevealResult::CantReveal => (),
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
        self.game_state
            .change_difficulty(difficulty)
            .expect("Failed to change difficulty. Bad difficulty?");

        // Reset mouse tracker
        self.mouse_tracker = MouseTracker::new();

        let new_size = difficulty.board_size * difficulty.board_size;
        if self.cells.len() == new_size {
            self.cells.broadcast(ButtonMsg::Reset);
        } else {
            let mut cells_guard = self.cells.guard();
            cells_guard.clear();

            while cells_guard.len() < new_size {
                cells_guard.push_back(ButtonCell::new(difficulty.board_size));
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

        // Difficulty actions
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

        // New game action
        let sender_clone = sender.clone();
        group.add_action(RelmAction::<NewGameAction>::new_stateless(move |_| {
            sender_clone.input(Msg::Restart);
        }));

        // Exit action
        let window_clone = window.clone();
        group.add_action(RelmAction::<ExitAction>::new_stateless(move |_| {
            gtk::Window::close(&window_clone);
        }));

        group.add_action(RelmAction::<AboutAction>::new_stateless(move |_| {
            sender.input(Msg::ShowAbout);
        }));

        group.register_for_widget(window);
    }

    fn track_mouse(&mut self, x: f64, y: f64) {
        // Early return if game is over
        if self.game_state.status().is_over() {
            return;
        }

        let board_size = self.game_state.difficulty().board_size;

        // Check bounds and handle mouse exit
        if x < 0.0
            || y < 0.0
            || x > f64::from(self.cells.widget().width())
            || y > f64::from(self.cells.widget().height())
        {
            if let Some(old_cell_pos) = self.mouse_tracker.mouse_cell.take() {
                self.deactivate_cell(old_cell_pos, &MouseButton::Middle);
            }
            return;
        }

        // Calculate new cell position
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        let cell_width = f64::from(self.cells.widget().width()) / board_size as f64;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        let cell_height = f64::from(self.cells.widget().height()) / board_size as f64;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::cast_sign_loss)]
        let x = (x / cell_width).floor() as usize;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::cast_sign_loss)]
        let y = (y / cell_height).floor() as usize;
        let cell_pos = CellPosition::new(x, y);

        // Return early if mouse hasn't moved to a new cell
        if self.mouse_tracker.mouse_cell.as_ref() == Some(&cell_pos) {
            return;
        }

        // Deactivate old cell if it exists
        if let Some(old_cell_pos) = self.mouse_tracker.mouse_cell.take() {
            self.deactivate_cell(old_cell_pos, &MouseButton::Middle);
        }

        // Store the new cell position
        self.mouse_tracker.mouse_cell = Some(cell_pos);
        // println!("Mouse cell: {:?}", self.mouse_tracker.mouse_cell);

        // Activate cell if mouse is pressed and cell isn't flagged
        if self.mouse_tracker.is_pressed() {
            self.activate_cell(cell_pos);
        }
    }

    #[inline]
    fn leftbutton_pressed(&mut self) {
        self.mouse_tracker.lbutton_state = MouseState::Pressed;

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell {
            self.activate_cell(cell_pos);
        }
    }

    #[inline]
    fn leftbutton_released(&mut self) {
        self.mouse_tracker.lbutton_state = MouseState::Released;
        self.mouse_tracker.mbutton_state = MouseState::Released;

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell {
            self.deactivate_cell(cell_pos, &MouseButton::Middle);
            let index = cell_pos.to_index(self.game_state.difficulty().board_size);
            self.handle_reveal(index);
        }
    }

    #[inline]
    fn rightbutton_pressed(&mut self) {
        if let Some(cell_pos) = self.mouse_tracker.mouse_cell.take() {
            self.deactivate_cell(cell_pos, &MouseButton::Middle);
            let index = cell_pos.to_index(self.game_state.difficulty().board_size);
            self.handle_flag(index);
        }

        // GTK4: Handle the strange behavior of the left button not being released when right button is pressed
        self.mouse_tracker.lbutton_state = MouseState::Released;
        self.mouse_tracker.mbutton_state = MouseState::Released;
    }

    fn middlebutton_pressed(&mut self) {
        self.mouse_tracker.mbutton_state = MouseState::Pressed;

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell {
            self.activate_cell(cell_pos);
        }
    }

    fn middlebutton_released(&mut self) {
        self.mouse_tracker.mbutton_state = MouseState::Released;
        self.mouse_tracker.lbutton_state = MouseState::Released;

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell.take() {
            self.deactivate_cell(cell_pos, &MouseButton::Middle);

            if let Ok(chord_cells) = self.game_state.chording(cell_pos) {
                self.reveal_cells(&chord_cells);
            }
        }
    }

    #[inline]
    fn activate_cell(&self, cell_pos: CellPosition) {
        let board_size = self.game_state.difficulty().board_size;

        if let Ok(cell_content) = self.game_state.display_cell(cell_pos) {
            if cell_content == "🚩" {
                return;
            }
        }

        let index = cell_pos.to_index(board_size);
        self.cells.send(index, ButtonMsg::Activate);

        if self.mouse_tracker.mbutton_state == MouseState::Pressed {
            self.activate_adjacent_cells(cell_pos);
        }
    }

    #[inline]
    fn activate_adjacent_cells(&self, cell_pos: CellPosition) {
        for adj_pos in self.game_state.adjacent_positions(cell_pos) {
            if let Ok(cell_content) = self.game_state.display_cell(adj_pos) {
                if cell_content != "🚩" {
                    let adj_index = adj_pos.to_index(self.game_state.difficulty().board_size);
                    self.cells.send(adj_index, ButtonMsg::Activate);
                }
            }
        }
    }

    #[inline]
    fn deactivate_cell(&self, cell_pos: CellPosition, button: &MouseButton) {
        let index = cell_pos.to_index(self.game_state.difficulty().board_size);
        self.cells.send(index, ButtonMsg::Deactivate);

        if matches!(button, MouseButton::Middle) {
            for adj_pos in self.game_state.adjacent_positions(cell_pos) {
                let adj_index = adj_pos.to_index(self.game_state.difficulty().board_size);
                self.cells.send(adj_index, ButtonMsg::Deactivate);
            }
        }
    }

    fn emoji_status(&self) -> String {
        if self.mouse_tracker.is_pressed() && !self.game_state.status().is_over() {
            "😯".to_owned()
        } else {
            self.game_state.status().to_string()
        }
    }
}

struct MouseTracker {
    mouse_cell: Option<CellPosition>,
    lbutton_state: MouseState,
    mbutton_state: MouseState,
}

impl MouseTracker {
    const fn new() -> Self {
        Self {
            mouse_cell: None,
            lbutton_state: MouseState::Released,
            mbutton_state: MouseState::Released,
        }
    }

    #[must_use]
    #[inline]
    const fn is_pressed(&self) -> bool {
        (matches!(self.mbutton_state, MouseState::Pressed)
            || matches!(self.lbutton_state, MouseState::Pressed))
    }
}

#[derive(PartialEq, Eq)]
enum MouseState {
    Pressed,
    Released,
}

enum MouseButton {
    // Left,
    Middle,
    // Right,
}
