use gtk::prelude::{GestureExt, GestureSingleExt, WidgetExt};
use relm4::{
    factory::positions::GridPosition,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender, RelmWidgetExt,
};

const CELL_SIZE: i32 = 20;

#[derive(Debug, Default)]
pub struct ButtonCell {
    pub label: String,
    pub css_classes: Vec<String>,
    pub grid_size: usize,
}

impl ButtonCell {
    #[must_use]
    pub const fn new(grid_size: usize, css_classes: Vec<String>) -> Self {
        Self {
            label: String::new(),
            css_classes,
            grid_size,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ButtonMsg {
    Display(String),
    ChangeGridSize(usize),
    SetCssClasses(Vec<String>),
    AddCssClasses(Vec<String>),
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
            #[watch]
            set_class_active: ("number-one", &self.label == "1"),
            #[watch]
            set_class_active: ("number-two", &self.label == "2"),
            #[watch]
            set_class_active: ("number-three", &self.label == "3"),
            #[watch]
            set_class_active: ("number-four", &self.label == "4"),
            #[watch]
            set_class_active: ("number-five", &self.label == "5"),
            #[watch]
            set_class_active: ("number-six", &self.label == "6"),
            #[watch]
            set_class_active: ("number-seven", &self.label == "7"),
            #[watch]
            set_class_active: ("number-eight", &self.label == "8"),

            #[watch]
            set_label: &self.label,
            add_controller = gtk::GestureClick {
                set_button: gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32,
                set_state: gtk::EventSequenceState::Denied,
                connect_begin[sender, index] => move |_, _|{
                    sender.output(ButtonOutput::Flag(index.current_index())).unwrap();
                },
            },
            add_controller = gtk::GestureClick {
                set_button: gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32,
                connect_released[sender, index] => move |gesture, _, _, _|{
                    sender.output(ButtonOutput::Reveal(index.current_index())).unwrap();
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

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) -> Self::CommandOutput {
        match msg {
            ButtonMsg::Display(label) => {
                self.label = label;
            }
            ButtonMsg::ChangeGridSize(size) => {
                self.grid_size = size;
            }
            ButtonMsg::SetCssClasses(vec) => {
                self.css_classes = vec;
            }
            ButtonMsg::AddCssClasses(vec) => {
                self.css_classes.extend(vec);
            }
        }
    }
}
