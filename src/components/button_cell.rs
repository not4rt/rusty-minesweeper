use gtk::prelude::WidgetExt;
use relm4::{
    factory::positions::GridPosition,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

const CELL_SIZE: i32 = 20;

pub struct ButtonCell {
    pub label: String,
    pub css_classes: Vec<String>,
    pub grid_size: usize,
}

impl ButtonCell {
    #[must_use]
    pub fn new(grid_size: usize) -> Self {
        Self {
            label: String::new(),
            css_classes: vec!["square-button".to_string()],
            grid_size,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ButtonMsg {
    Display(String),
    ChangeGridSize(usize),
    Reset,
    AddCssClass(String),
    Activate,
    Deactivate,
}

#[derive(Debug)]
pub enum ButtonOutput {
    Reveal(usize),
    Flag(usize),
}

impl relm4::factory::Position<GridPosition, DynamicIndex> for ButtonCell {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index();
        let x = index % self.grid_size;
        let y = index / self.grid_size;

        GridPosition {
            row: i32::try_from(x).expect("This conversion should always succeed. (C01)"),
            column: i32::try_from(y).expect("This conversion should always succeed. (C02)"),
            width: 1,
            height: 1,
        }
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for ButtonCell {
    type Init = Self;
    type Input = ButtonMsg;
    type Output = ButtonOutput;
    type CommandOutput = ();
    type Widgets = ButtonCellWidgets;
    type ParentWidget = gtk::Grid;
    type Index = DynamicIndex;

    view! {
        #[root]
        gtk::Label {
            set_can_focus: false,
            set_hexpand: true,
            set_vexpand: true,
            set_width_request: CELL_SIZE,
            set_height_request: CELL_SIZE,
            #[watch]
            set_css_classes: &self.css_classes.iter().map(std::string::String::as_str).collect::<Vec<&str>>(),

            set_can_target: false,

            #[watch]
            set_label: &self.label,
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
            ButtonMsg::ChangeGridSize(size) => self.grid_size = size,
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
