use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

use crate::game::models::statistics::Statistics;

#[derive(Debug, Clone)]
pub struct StatisticsComponent {
    best_times: Statistics,
    is_visible: bool,
}

#[derive(Debug)]
pub enum StatisticsInput {
    Show(Statistics),
    ResetScores,
    Ok,
}

#[derive(Debug)]
pub enum StatisticsOutput {
    ResetScores,
}

#[relm4::component(pub)]
impl SimpleComponent for StatisticsComponent {
    type Init = bool;
    type Input = StatisticsInput;
    type Output = StatisticsOutput;

    view! {
        best_times_window = gtk::Window {
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
                    set_label: "Fastest Mine Sweepers",
                },
            },


            // General Box
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                // Labels Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_homogeneous: false,

                    // Beginner Box
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_hexpand: true,
                        set_width_request: 350,
                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Start,
                            set_valign: gtk::Align::Start,
                            set_label: "Beginner:",
                        },

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            #[watch]
                            set_label: &format!("{} seconds", &model.best_times.beginner().total_time.as_secs().to_string()),
                        },

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::End,
                            set_valign: gtk::Align::End,
                            set_label: "Anonymous", //&model.best_times.beginner().nickname,
                        },
                    },

                    // Intermediate Box
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_hexpand: true,
                        set_width_request: 350,
                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Start,
                            set_valign: gtk::Align::Start,
                            set_label: "Intermediate:",
                        },

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            #[watch]
                            set_label: &format!("{} seconds", &model.best_times.intermediate().total_time.as_secs().to_string()),
                        },

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::End,
                            set_valign: gtk::Align::End,
                            set_label: "Anonymous", //&model.best_times.intermediate().nickname,
                        },
                    },

                    // Expert Box
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_hexpand: true,
                        set_width_request: 350,
                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Start,
                            set_valign: gtk::Align::Start,
                            set_label: "Expert:",
                        },

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            #[watch]
                            set_label: &format!("{} seconds", &model.best_times.expert().total_time.as_secs().to_string()),
                        },

                        gtk::Label {
                            set_hexpand: true,
                            set_halign: gtk::Align::End,
                            set_valign: gtk::Align::End,
                            set_label: "Anonymous", //&model.best_times.expert().nickname,
                        },
                    }
                },

                // Buttons Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    set_margin_all: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_homogeneous: true,

                    gtk::Button {
                        set_label: "Reset Scores",
                        set_margin_top: 10,
                        connect_clicked => StatisticsInput::ResetScores,
                    },

                    gtk::Button {
                        set_label: "OK",
                        set_margin_top: 10,
                        connect_clicked => StatisticsInput::Ok,
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
            is_visible: init,
            best_times: Statistics::default(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            StatisticsInput::Show(current_best) => {
                self.best_times = current_best;

                self.is_visible = true;
            }
            StatisticsInput::ResetScores => {
                self.is_visible = false;

                let _ = sender.output(StatisticsOutput::ResetScores);
            }
            StatisticsInput::Ok => {
                self.is_visible = false;
            }
        }
    }
}
