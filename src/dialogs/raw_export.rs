use std::sync::Arc;

use fltk::{
    frame::Frame,
    group::{Flex, FlexType},
    input::Input,
    misc::InputChoice,
    prelude::{WidgetBase, WidgetExt},
    window,
};
use fltk_grid::Grid;

pub struct RawExportDialog {
    volumes: InputChoice,
    output: Input,
}

impl RawExportDialog {
    pub fn default() -> Self {
        let mut window = window::Window::default()
            .with_size(600, 300)
            .with_label("Export raw volume");
        let mut container = Flex::default_fill();
        container.set_type(FlexType::Column);

        let mut grid = Grid::default_fill();
        grid.set_gap(5, 5);
        grid.set_margin(5, 5, 5, 5);
        grid.set_layout(7, 6);

        let mut selection_label = Frame::default().with_label("Volume/process to output");
        let mut selection_input = InputChoice::default();
        grid.set_widget(&mut selection_label, 0, 0..0);

        RawExportDialog {
            volumes: todo!(),
            output: todo!(),
        }
    }
}
