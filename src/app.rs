use crate::components::board_cell::{BoardCell, BoardCellMsg};
use crate::components::custom_difficulty::{
    CustomDifficultyComponent, CustomDifficultyInput, CustomDifficultyOutput,
};
use crate::components::statistics::{StatisticsComponent, StatisticsInput, StatisticsOutput};
use crate::game::models::board::RevealResult;
use crate::game::models::cell::CellPosition;
use crate::game::models::game::GameDifficulty;
use crate::game::models::statistics::Statistics;
use crate::game::state::GameState;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::ControlFlow;
use gtk::{gdk, prelude::*};
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::prelude::FactoryVecDeque;
use relm4::{
    Component, ComponentController, ComponentParts, Controller, RelmWidgetExt, SimpleComponent,
};
use std::rc::Rc;
use std::time::Duration;

const APP_ICON: &[u8] = include_bytes!("../assets/logo.png");
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
relm4::new_stateless_action!(ShowBestAction, WindowActionGroup, "action-best-times");
relm4::new_stateless_action!(ExitAction, WindowActionGroup, "exit");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

pub struct App {
    game_state: GameState,
    mouse_tracker: MouseTracker,
    cells: FactoryVecDeque<BoardCell>,
    custom_difficulty_dialog: Controller<CustomDifficultyComponent>,
    difficulty_tracker: Rc<std::cell::Cell<GameDifficulty>>,
    statistics_dialog: Controller<StatisticsComponent>,
    statistics_tracker: Rc<std::cell::Cell<Statistics>>,
}

#[derive(Debug)]
pub enum Msg {
    Restart,
    ChangeDifficulty(GameDifficulty),
    ResetStats,
    ShowAbout,
    Tick,
    TrackMouse(f64, f64),
    LeftButtonPressed,
    LeftButtonReleased,
    RightButtonPressed,
    RightButtonReleased,
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
            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &gtk::Label {
                    add_css_class: "title",
                    set_label: "Minesweeper",
                },
            },


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
                            #[watch]
                            set_label: &model.emoji_status(),
                            connect_clicked[sender] => move |_| {
                                sender.input(Msg::Restart);
                            },
                            set_css_classes: &["restart_button"],
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
                                connect_unpaired_release[sender] => move |_, _, _, _, _|{
                                    sender.input(Msg::LeftButtonReleased);
                                },
                            },
                            add_controller = gtk::GestureClick {
                                set_button: gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32,
                                connect_pressed[sender] => move |_, _, _, _|{
                                    sender.input(Msg::RightButtonPressed);
                                },
                                connect_released[sender] => move |_, _, _, _|{
                                    sender.input(Msg::RightButtonReleased);
                                },
                                connect_unpaired_release[sender] => move |_, _, _, _, _|{
                                    sender.input(Msg::RightButtonReleased);
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
                    "Best Times..." => ShowBestAction,
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

        let cells: FactoryVecDeque<BoardCell> =
            FactoryVecDeque::builder().launch_default().detach();

        let custom_difficulty_dialog = CustomDifficultyComponent::builder()
            .transient_for(&root)
            .launch(false) // Hide the dialog initially
            .forward(
                sender.input_sender(),
                |msg: CustomDifficultyOutput| match msg {
                    CustomDifficultyOutput::SetCustomBoard(difficulty) => {
                        Msg::ChangeDifficulty(difficulty)
                    }
                },
            );
        let stats_dialog = StatisticsComponent::builder()
            .transient_for(&root)
            .launch(false) // Hide the dialog initially
            .forward(sender.input_sender(), |msg: StatisticsOutput| match msg {
                StatisticsOutput::ResetScores => Msg::ResetStats,
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

        let model = Self::new(game_state, cells, custom_difficulty_dialog, stats_dialog);

        let cells_grid = model.cells.widget();

        let widgets = view_output!();
        model.setup_actions(sender, &widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Restart => self.handle_restart(),
            Msg::Tick => self.game_state.tick(),
            Msg::ChangeDifficulty(difficulty) => self.handle_difficulty_change(difficulty),
            Msg::ShowAbout => Self::show_about_dialog(),
            Msg::TrackMouse(x, y) => self.track_mouse(x, y),
            Msg::LeftButtonPressed => self.leftbutton_pressed(),
            Msg::LeftButtonReleased => self.leftbutton_released(),
            Msg::RightButtonPressed => self.rightbutton_pressed(),
            Msg::RightButtonReleased => self.rightbutton_released(),
            Msg::MiddleButtonPressed => self.middlebutton_pressed(),
            Msg::MiddleButtonReleased => self.middlebutton_released(),
            Msg::ResetStats => self.reset_stats(),
        }
    }
}

impl App {
    fn new(
        game_state: GameState,
        mut cells: FactoryVecDeque<BoardCell>,
        custom_difficulty_dialog: Controller<CustomDifficultyComponent>,
        stats_dialog: Controller<StatisticsComponent>,
    ) -> Self {
        let board_size = game_state.difficulty().board_size();

        let mut cells_guard = cells.guard();

        for x in 0..board_size.width {
            for y in 0..board_size.height {
                cells_guard.push_back(BoardCell::new(CellPosition::new(x, y)));
            }
        }
        cells_guard.drop();

        let difficulty_tracker = Rc::new(std::cell::Cell::new(*game_state.difficulty()));
        let stats_tracker = Rc::new(std::cell::Cell::new(*game_state.statistics()));

        Self {
            game_state,
            cells,
            mouse_tracker: MouseTracker::new(),
            custom_difficulty_dialog,
            difficulty_tracker,
            statistics_dialog: stats_dialog,
            statistics_tracker: stats_tracker,
        }
    }

    fn handle_restart(&mut self) {
        self.game_state
            .restart()
            .expect("Failed to restart game. Bad difficulty?");

        self.cells.broadcast(BoardCellMsg::Display(EMPTY_STRING));
        self.cells.broadcast(BoardCellMsg::Reset);
    }

    fn handle_reveal(&mut self, cell_pos: CellPosition) {
        let reveal_result = self.game_state.reveal_cell(cell_pos);

        if let Ok(reveal_result) = reveal_result {
            self.reveal_cells(&reveal_result);
        }
    }

    fn reveal_cells(&mut self, reveal_result: &RevealResult) {
        let board_size = self.game_state.difficulty().board_size();

        for &revealed_pos in self.game_state.revealed_cells() {
            let revealed_index = revealed_pos.to_index(board_size);
            if let Ok(display) = self.game_state.display_cell(revealed_pos) {
                self.cells
                    .send(revealed_index, BoardCellMsg::Display(display));
                self.cells.send(
                    revealed_index,
                    BoardCellMsg::AddCssClass(REVEALED_CELL_CLASS.to_string()),
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
                            .send(flag_index, BoardCellMsg::Display("🚩".to_string()));
                    }
                    self.game_state.clear_flagged_cells();
                }
            }
            RevealResult::GameOver(mine_pos) => self.cells.send(
                mine_pos.to_index(board_size),
                BoardCellMsg::AddCssClass(LOST_CELL_CLASS.to_string()),
            ),
            RevealResult::CantReveal => (),
        }

        self.statistics_tracker.set(*self.game_state.statistics());
    }

    fn handle_flag(&mut self, cell_pos: CellPosition) {
        if matches!(self.game_state.toggle_flag(cell_pos), Ok(true)) {
            if let Ok(display) = self.game_state.display_cell(cell_pos) {
                self.cells.send(
                    cell_pos.to_index(self.game_state.difficulty().board_size()),
                    BoardCellMsg::Display(display),
                );
            }
        }
    }

    fn handle_difficulty_change(&mut self, difficulty: GameDifficulty) {
        self.game_state
            .change_difficulty(difficulty)
            .expect("Failed to change difficulty. Bad difficulty?");

        self.difficulty_tracker.set(difficulty);

        // Reset mouse tracker
        self.mouse_tracker = MouseTracker::new();

        let new_size = difficulty.board_size().width * difficulty.board_size().height;
        if self.cells.len() == new_size {
            self.cells.broadcast(BoardCellMsg::Reset);
        } else {
            let mut cells_guard = self.cells.guard();
            cells_guard.clear();

            for x in 0..difficulty.board_size().width {
                for y in 0..difficulty.board_size().height {
                    cells_guard.push_back(BoardCell::new(CellPosition::new(x, y)));
                }
            }
        }
    }

    fn show_about_dialog() {
        let dialog = gtk::AboutDialog::builder()
            .program_name("Rusty Minesweeper")
            .version("1.0")
            .comments("A Minesweeper clone written in Rust using GTK4 and Relm4")
            .build();

        if let Ok(pixbuf) = Pixbuf::from_read(APP_ICON) {
            let texture = gdk::Texture::for_pixbuf(&pixbuf);
            dialog.set_logo(Some(&texture));
        }

        dialog.present();
    }

    fn setup_actions(&self, sender: relm4::ComponentSender<Self>, window: &gtk::Window) {
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

        add_difficulty_action!(SetDifficultyBeginnerAction, GameDifficulty::Beginner);
        add_difficulty_action!(
            SetDifficultyIntermediateAction,
            GameDifficulty::Intermediate
        );
        add_difficulty_action!(SetDifficultyExpertAction, GameDifficulty::Expert);

        let dialog_sender_clone = self.custom_difficulty_dialog.sender().clone();
        let difficulty_clone = self.difficulty_tracker.clone();
        group.add_action(RelmAction::<SetDifficultyCustomAction>::new_stateless(
            move |_| {
                let _ =
                    dialog_sender_clone.send(CustomDifficultyInput::Show(difficulty_clone.get()));
            },
        ));

        let dialog_sender_clone = self.statistics_dialog.sender().clone();
        let stats_clone = self.statistics_tracker.clone();
        group.add_action(RelmAction::<ShowBestAction>::new_stateless(move |_| {
            let _ = dialog_sender_clone.send(StatisticsInput::Show(stats_clone.get()));
        }));

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

        let board_size = self.game_state.difficulty().board_size();

        // Check bounds and handle mouse exit
        if x < 0.0
            || y < 0.0
            || x > f64::from(self.cells.widget().width())
            || y > f64::from(self.cells.widget().height())
        {
            if let Some(old_cell_pos) = self.mouse_tracker.mouse_cell.take() {
                self.deactivate_cell(old_cell_pos);
            }
            return;
        }

        // Calculate new cell position
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        let cell_width = f64::from(self.cells.widget().width()) / board_size.width as f64;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        let cell_height = f64::from(self.cells.widget().height()) / board_size.height as f64;
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

        let old_cell_pos = self.mouse_tracker.mouse_cell.take();

        // Store the new cell position
        self.mouse_tracker.mouse_cell = Some(cell_pos);
        // println!("Mouse cell: {:?}", self.mouse_tracker.mouse_cell);

        // Activate cell if mouse is pressed and cell isn't flagged
        if self.mouse_tracker.is_pressed() {
            // Deactivate old cell if it exists
            if let Some(old_cell_pos) = old_cell_pos {
                self.deactivate_cell(old_cell_pos);
            }

            self.activate_cell(cell_pos);
        }
    }

    #[inline]
    fn leftbutton_pressed(&mut self) {
        // println!("Left button pressed");
        self.mouse_tracker.lbutton_state = MouseState::Pressed;

        if self.mouse_tracker.rbutton_state == MouseState::Pressed {
            self.mouse_tracker.chording = true;
        }

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell {
            self.activate_cell(cell_pos);
        }
    }

    #[inline]
    fn leftbutton_released(&mut self) {
        // println!("Left button released");
        if self.mouse_tracker.lbutton_state == MouseState::Released {
            // println!("Left button already released");
            return;
        }
        self.mouse_tracker.lbutton_state = MouseState::Released;
        self.mouse_tracker.mbutton_state = MouseState::Released;
        self.mouse_tracker.rbutton_state = MouseState::Released;

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell {
            self.deactivate_cell(cell_pos);

            if self.mouse_tracker.chording {
                if let Ok(chord_cells) = self.game_state.chording(cell_pos) {
                    self.reveal_cells(&chord_cells);
                }
            } else {
                self.handle_reveal(cell_pos);
            }

            self.mouse_tracker.chording = false;
        }
    }

    #[inline]
    fn rightbutton_pressed(&mut self) {
        // println!("Right button pressed");
        self.mouse_tracker.rbutton_state = MouseState::Pressed;

        if self.mouse_tracker.lbutton_state == MouseState::Pressed {
            self.mouse_tracker.chording = true;

            if let Some(cell_pos) = self.mouse_tracker.mouse_cell {
                self.activate_cell(cell_pos);
            }
        } else if let Some(cell_pos) = self.mouse_tracker.mouse_cell.take() {
            self.handle_flag(cell_pos);
        }

        // // GTK4: Handle the strange behavior of the left button not being released when right button is pressed
        // self.mouse_tracker.lbutton_state = MouseState::Released;
        // self.mouse_tracker.mbutton_state = MouseState::Released;
    }

    fn rightbutton_released(&mut self) {
        // println!("Right button released");
        if self.mouse_tracker.rbutton_state == MouseState::Released {
            // println!("Right button already released");
            return;
        }
        self.mouse_tracker.rbutton_state = MouseState::Released;
        self.mouse_tracker.mbutton_state = MouseState::Released;
        self.mouse_tracker.lbutton_state = MouseState::Released;

        if self.mouse_tracker.chording {
            if let Some(cell_pos) = self.mouse_tracker.mouse_cell.take() {
                self.deactivate_cell(cell_pos);

                if let Ok(chord_cells) = self.game_state.chording(cell_pos) {
                    self.reveal_cells(&chord_cells);
                }

                self.mouse_tracker.chording = false;
            }
        }
    }

    fn middlebutton_pressed(&mut self) {
        // println!("Middle button pressed");
        self.mouse_tracker.mbutton_state = MouseState::Pressed;
        self.mouse_tracker.chording = true;

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell {
            self.activate_cell(cell_pos);
        }
    }

    fn middlebutton_released(&mut self) {
        // println!("Middle button released");
        self.mouse_tracker.mbutton_state = MouseState::Released;
        self.mouse_tracker.rbutton_state = MouseState::Released;
        self.mouse_tracker.lbutton_state = MouseState::Released;

        if let Some(cell_pos) = self.mouse_tracker.mouse_cell.take() {
            self.deactivate_cell(cell_pos);

            if let Ok(chord_cells) = self.game_state.chording(cell_pos) {
                self.reveal_cells(&chord_cells);
            }

            self.mouse_tracker.chording = false;
        }
    }

    #[inline]
    fn activate_cell(&self, cell_pos: CellPosition) {
        let board_size = self.game_state.difficulty().board_size();

        if let Ok(cell_content) = self.game_state.display_cell(cell_pos) {
            if cell_content == "🚩" {
                return;
            }
        }

        let index = cell_pos.to_index(board_size);
        self.cells.send(index, BoardCellMsg::Activate);

        if self.mouse_tracker.chording {
            self.activate_adjacent_cells(cell_pos);
        }
    }

    #[inline]
    fn activate_adjacent_cells(&self, cell_pos: CellPosition) {
        for adj_pos in self.game_state.adjacent_positions(cell_pos) {
            if let Ok(cell_content) = self.game_state.display_cell(adj_pos) {
                if cell_content != "🚩" {
                    let adj_index = adj_pos.to_index(self.game_state.difficulty().board_size());
                    self.cells.send(adj_index, BoardCellMsg::Activate);
                }
            }
        }
    }

    #[inline]
    fn deactivate_cell(&self, cell_pos: CellPosition) {
        let index = cell_pos.to_index(self.game_state.difficulty().board_size());
        self.cells.send(index, BoardCellMsg::Deactivate);

        if self.mouse_tracker.chording {
            for adj_pos in self.game_state.adjacent_positions(cell_pos) {
                let adj_index = adj_pos.to_index(self.game_state.difficulty().board_size());
                self.cells.send(adj_index, BoardCellMsg::Deactivate);
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

    fn reset_stats(&mut self) {
        self.game_state.reset_stats();
        self.statistics_tracker.set(*self.game_state.statistics());
    }
}

struct MouseTracker {
    mouse_cell: Option<CellPosition>,
    lbutton_state: MouseState,
    mbutton_state: MouseState,
    rbutton_state: MouseState,
    chording: bool,
}

impl MouseTracker {
    const fn new() -> Self {
        Self {
            mouse_cell: None,
            lbutton_state: MouseState::Released,
            mbutton_state: MouseState::Released,
            rbutton_state: MouseState::Released,
            chording: false,
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
