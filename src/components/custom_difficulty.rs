use gtk::prelude::{
    BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt, GtkWindowExt, OrientableExt,
    WidgetExt,
};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

use crate::game::models::game::GameDifficulty;

#[derive(Debug, Clone)]
pub struct CustomDifficultyComponent {
    height: gtk::EntryBuffer,
    width: gtk::EntryBuffer,
    mines: gtk::EntryBuffer,
    is_visible: bool,
}

#[derive(Debug)]
pub enum CustomDifficultyInput {
    Show(GameDifficulty),
    SetCustomBoard(GameDifficulty),
    Cancel,
}

#[derive(Debug)]
pub enum CustomDifficultyOutput {
    SetCustomBoard(GameDifficulty),
}

#[relm4::component(pub)]
impl SimpleComponent for CustomDifficultyComponent {
    type Init = bool;
    type Input = CustomDifficultyInput;
    type Output = CustomDifficultyOutput;

    view! {
        custom_difficulty = gtk::Window {
            set_modal: true,
            #[watch]
            set_visible: model.is_visible,
            set_width_request: 50,
            set_height_request: 100,

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &gtk::Label {
                    add_css_class: "title",
                    set_label: "Custom Board",
                },
            },


            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_homogeneous: false,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        gtk::Label {
                            set_width_request: 75,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_label: "Height:",
                        },

                        gtk::Entry {
                            set_max_width_chars: 5,
                            set_buffer: &model.height,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_text: "9",
                            set_max_length: 5,
                        },
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        gtk::Label {
                            set_width_request: 75,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_label: "Width:",
                        },

                        gtk::Entry {
                            set_max_width_chars: 5,
                            set_buffer: &model.width,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_text: "9",
                            set_max_length: 5,
                        },
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        gtk::Label {
                            set_width_request: 75,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_label: "Mines:",
                        },

                        gtk::Entry {
                            set_max_width_chars: 5,
                            set_buffer: &model.mines,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_text: "10",
                            set_max_length: 5,
                        },
                    }
                },


                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_homogeneous: false,

                    gtk::Button {
                        set_label: "OK",
                        connect_clicked[model, sender] => move |_| {
                            let height = model.height.text().parse().unwrap();
                            let width = model.width.text().parse().unwrap();
                            let mines_count = model.mines.text().parse().unwrap();
                            sender.input(CustomDifficultyInput::SetCustomBoard(GameDifficulty::Custom((width, height).into(), mines_count)));
                        },
                    },

                    gtk::Button {
                        set_label: "Cancel",
                        set_margin_top: 40,
                        connect_clicked => CustomDifficultyInput::Cancel,
                    },
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            height: gtk::EntryBuffer::default(),
            width: gtk::EntryBuffer::default(),
            mines: gtk::EntryBuffer::default(),
            is_visible: init,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            CustomDifficultyInput::Show(current_difficulty) => {
                self.width
                    .set_text(current_difficulty.board_size().width.to_string());
                self.height
                    .set_text(current_difficulty.board_size().height.to_string());
                self.mines
                    .set_text(current_difficulty.mines_count().to_string());

                self.is_visible = true;
            }
            CustomDifficultyInput::SetCustomBoard(mut difficulty) => {
                self.is_visible = false;

                // Minimum board size and mines count is the same as the beginner difficulty
                if let GameDifficulty::Custom(board_size, mines_count) = difficulty {
                    if board_size.width < GameDifficulty::Beginner.board_size().width {
                        difficulty.set_width(GameDifficulty::Beginner.board_size().width);
                    }
                    if board_size.height < GameDifficulty::Beginner.board_size().height {
                        difficulty.set_height(GameDifficulty::Beginner.board_size().height);
                    }
                    if mines_count < GameDifficulty::Beginner.mines_count() {
                        difficulty.set_mines_count(GameDifficulty::Beginner.mines_count());
                    }

                    // Maximum board size is 30x24 and maximum mines count is (x-1)(y-1)
                    if board_size.width > 30 {
                        difficulty.set_width(30);
                    }
                    if board_size.height > 24 {
                        difficulty.set_height(24);
                    }

                    let max_mines_count =
                        (difficulty.board_size().width - 1) * (difficulty.board_size().height - 1);
                    if mines_count > max_mines_count {
                        difficulty.set_mines_count(max_mines_count);
                    }
                }

                let _ = sender.output(CustomDifficultyOutput::SetCustomBoard(difficulty));
            }
            CustomDifficultyInput::Cancel => {
                self.is_visible = false;
            }
        }
    }
}
