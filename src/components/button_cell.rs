use gtk::prelude::{GridExt, WidgetExt};
use relm4::{
    factory::positions::GridPosition,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

use crate::game::models::cell::CellPosition;

const CELL_SIZE: i32 = 20;
const FLAG_ICON: &[u8] = include_bytes!("../../assets/flag.png");

pub struct ButtonCell {
    pub label: String,
    pub css_classes: Vec<String>,
    pub position: CellPosition,
}

impl ButtonCell {
    #[must_use]
    pub fn new(pos: CellPosition) -> Self {
        Self {
            label: String::new(),
            css_classes: vec!["square-button".to_string()],
            position: pos,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ButtonMsg {
    Display(String),
    Reset,
    AddCssClass(String),
    Activate,
    Deactivate,
}

impl relm4::factory::Position<GridPosition, DynamicIndex> for ButtonCell {
    fn position(&self, _index: &DynamicIndex) -> GridPosition {
        let x = self.position.x;
        let y = self.position.y;

        GridPosition {
            column: i32::try_from(x).expect("This conversion should always succeed. (C02)"),
            row: i32::try_from(y).expect("This conversion should always succeed. (C01)"),
            width: 1,
            height: 1,
        }
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for ButtonCell {
    type Init = Self;
    type Input = ButtonMsg;
    type Output = ();
    type CommandOutput = ();
    type Widgets = ButtonCellWidgets;
    type ParentWidget = gtk::Grid;
    type Index = DynamicIndex;

    view! {
        #[root]
        gtk::Box {
            set_can_focus: false,
            set_hexpand: false,
            set_vexpand: false,
            set_width_request: CELL_SIZE,
            set_height_request: CELL_SIZE,
            set_can_target: false,

            gtk::Label {
                set_can_focus: false,
                set_hexpand: true,
                set_vexpand: true,
                set_width_request: CELL_SIZE,
                set_height_request: CELL_SIZE,
                set_can_target: false,

                #[watch]
                set_css_classes: &self.css_classes.iter().map(std::string::String::as_str).collect::<Vec<&str>>(),
                #[watch]
                set_label: &self.label,
                #[watch]
                set_visible: self.label != "🚩",

            },
            #[name(test_grid)]
             gtk::Grid {
                #[watch]
                set_visible: self.label == "🚩",
                set_hexpand: true,
                set_vexpand: true,
                #[watch]
                set_css_classes: &self.css_classes.iter().map(std::string::String::as_str).collect::<Vec<&str>>(),
            }
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        init
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &gtk::Widget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let button = view_output!();

        let picture = gtk::Image::from_paintable(Some(&gtk::gdk::Texture::for_pixbuf(
            &gtk::gdk_pixbuf::Pixbuf::from_read(FLAG_ICON).expect("Failed to create pixbuf"),
        )));
        picture.set_hexpand(true);
        picture.set_vexpand(true);
        picture.set_halign(gtk::Align::Center);
        picture.set_valign(gtk::Align::Center);

        button.test_grid.attach(&picture, 0, 0, 1, 1);

        button
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) -> Self::CommandOutput {
        match msg {
            ButtonMsg::Display(label) => {
                match label.as_str() {
                    "1" => self.css_classes.push("number-one".to_string()),
                    "2" => self.css_classes.push("number-two".to_string()),
                    "3" => self.css_classes.push("number-three".to_string()),
                    "4" => self.css_classes.push("number-four".to_string()),
                    "5" => self.css_classes.push("number-five".to_string()),
                    "6" => self.css_classes.push("number-six".to_string()),
                    "7" => self.css_classes.push("number-seven".to_string()),
                    "8" => self.css_classes.push("number-eight".to_string()),
                    _ => {}
                }
                self.label = label;
            }
            ButtonMsg::Reset => {
                self.label = String::new();
                self.css_classes = vec!["square-button".to_string()];
            }
            ButtonMsg::AddCssClass(class) => self.css_classes.push(class),
            ButtonMsg::Activate => self.css_classes.push("active".to_string()),
            ButtonMsg::Deactivate => self.css_classes.retain(|c| c != "active"),
        }
    }
}
