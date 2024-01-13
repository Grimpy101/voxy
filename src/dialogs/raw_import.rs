use std::{borrow::BorrowMut, rc::Rc, sync::Mutex};

use fltk::{
    app,
    button::Button,
    dialog,
    enums::Align,
    frame::Frame,
    group::{Flex, FlexType, Tabs},
    input::Input,
    misc::Spinner,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt},
    text::{TextBuffer, TextEditor, WrapMode},
    window,
};
use fltk_grid::Grid;

use crate::volumes::{raw::RawVolume, Volume};

pub struct RawImportDialog {
    file_input: Input,
    dimensions: [Spinner; 3],
    name: Input,
    description: TextBuffer,
    semantic_type: Input,
    volume_size: [Spinner; 3],
    voxel_size: [Spinner; 3],
    is_ok: Rc<Mutex<bool>>,
}

impl RawImportDialog {
    pub fn default() -> Self {
        let mut window = window::Window::default()
            .with_size(600, 300)
            .with_label("Add raw volume");

        let mut container = Flex::default_fill();
        container.set_type(FlexType::Column);

        let mut tabs = Tabs::default_fill();

        let basic = Flex::default_fill().with_label("Basic").row();

        let mut basic_grid = Grid::default_fill();
        basic_grid.set_gap(5, 5);
        basic_grid.set_margin(5, 5, 5, 5);
        basic_grid.set_layout(7, 6);

        let mut file_label = Frame::default()
            .with_label("Volume file:")
            .with_align(Align::Left | Align::Inside);
        let mut file_input = Input::default();
        let mut file_button = Button::default().with_label("Select");
        basic_grid.set_widget(&mut file_label, 0, 0);
        basic_grid.set_widget(&mut file_input, 0, 1..5);
        basic_grid.set_widget(&mut file_button, 0, 5);

        let mut dimensions_label = Frame::default()
            .with_label("Dimensions:")
            .with_align(Align::Left | Align::Inside);
        let mut dimensions_x_input = Spinner::default();
        dimensions_x_input.set_range(1.0, f64::MAX);
        let mut dimensions_y_input = Spinner::default();
        dimensions_y_input.set_range(1.0, f64::MAX);
        let mut dimensions_z_input = Spinner::default();
        dimensions_z_input.set_range(1.0, f64::MAX);
        basic_grid.set_widget(&mut dimensions_label, 1, 0);
        basic_grid.set_widget(&mut dimensions_x_input, 1, 1);
        basic_grid.set_widget(&mut dimensions_y_input, 1, 2);
        basic_grid.set_widget(&mut dimensions_z_input, 1, 3);

        basic_grid.end();

        basic.end();

        let advanced = Flex::default_fill().with_label("Advanced").row();

        let mut advanced_grid = Grid::default_fill();
        advanced_grid.set_gap(5, 5);
        advanced_grid.set_margin(5, 5, 5, 5);
        advanced_grid.set_layout(7, 8);

        let mut name_label = Frame::default()
            .with_label("Name:")
            .with_align(Align::Left | Align::Inside);
        let mut name_input = Input::default();
        advanced_grid.set_widget(&mut name_label, 0, 0..3);
        advanced_grid.set_widget(&mut name_input, 0, 3..8);

        let mut semantic_type_label = Frame::default()
            .with_label("Semantic type:")
            .with_align(Align::Left | Align::Inside);
        let mut semantic_type_input = Input::default();
        advanced_grid.set_widget(&mut semantic_type_label, 1, 0..3);
        advanced_grid.set_widget(&mut semantic_type_input, 1, 3..8);

        let mut volume_size_label = Frame::default()
            .with_label("Physical volume size (mm):")
            .with_align(Align::Left | Align::Inside);
        let mut volume_size_x_input = Spinner::default();
        volume_size_x_input.set_range(0.0, f64::MAX);
        volume_size_x_input.set_value(0.0);
        let mut volume_size_y_input = Spinner::default();
        volume_size_y_input.set_range(0.0, f64::MAX);
        volume_size_y_input.set_value(0.0);
        let mut volume_size_z_input = Spinner::default();
        volume_size_z_input.set_range(0.0, f64::MAX);
        volume_size_z_input.set_value(0.0);
        advanced_grid.set_widget(&mut volume_size_label, 2, 0..3);
        advanced_grid.set_widget(&mut volume_size_x_input, 2, 3);
        advanced_grid.set_widget(&mut volume_size_y_input, 2, 4);
        advanced_grid.set_widget(&mut volume_size_z_input, 2, 5);

        let mut voxel_size_label = Frame::default()
            .with_label("Physical voxel size (mm):")
            .with_align(Align::Left | Align::Inside);
        let mut voxel_size_x_input = Spinner::default();
        voxel_size_x_input.set_range(0.0, f64::MAX);
        voxel_size_x_input.set_value(0.0);
        let mut voxel_size_y_input = Spinner::default();
        voxel_size_y_input.set_range(0.0, f64::MAX);
        voxel_size_y_input.set_value(0.0);
        let mut voxel_size_z_input = Spinner::default();
        voxel_size_z_input.set_range(0.0, f64::MAX);
        voxel_size_z_input.set_value(0.0);
        advanced_grid.set_widget(&mut voxel_size_label, 3, 0..3);
        advanced_grid.set_widget(&mut voxel_size_x_input, 3, 3);
        advanced_grid.set_widget(&mut voxel_size_y_input, 3, 4);
        advanced_grid.set_widget(&mut voxel_size_z_input, 3, 5);

        let mut description_label = Frame::default()
            .with_label("Description:")
            .with_align(Align::Left | Align::Inside);
        let description_buffer = TextBuffer::default();
        let mut description_text = TextEditor::default();
        description_text.set_buffer(description_buffer.clone());
        description_text.wrap_mode(WrapMode::AtBounds, 5);

        advanced_grid.set_widget(&mut description_label, 4, 0..2);
        advanced_grid.set_widget(&mut description_text, 5..7, 0..8);

        advanced_grid.end();

        advanced.end();

        tabs.end();
        tabs.auto_layout();

        let mut buttons_container = Flex::default_fill()
            .row()
            .with_align(Align::Inside | Align::Right);
        buttons_container.set_margin(5);

        Frame::default();
        let mut cancel_button = Button::default().with_label("Cancel");
        let mut ok_button = Button::default().with_label("Ok");
        buttons_container.fixed(&cancel_button, 100);
        buttons_container.fixed(&ok_button, 100);

        buttons_container.end();
        container.fixed(&buttons_container, 40);
        container.end();

        window.end();
        window.make_modal(true);
        window.show();

        file_button.set_callback({
            let mut file_input = file_input.clone();
            move |_| {
                let file = dialog::file_chooser("Choose volume file", "*", ".", false);
                if let Some(filepath) = file {
                    file_input.set_value(&filepath);
                }
            }
        });

        cancel_button.set_callback({
            let mut window = window.clone();
            move |_| {
                window.hide();
            }
        });
        let is_ok = Rc::new(Mutex::new(false));
        ok_button.set_callback({
            let mut window = window.clone();
            let file_input = file_input.clone();
            let mut is_ok = is_ok.clone();
            move |_| {
                let file = file_input.value();
                if file.is_empty() {
                        dialog::message_default("Cannot proceed because volume file is not selected.\n\nPlease enter valid volume file.");
                } else {
                    let mut ok = is_ok.borrow_mut().lock().unwrap();
                    *ok = true;
                    window.hide();
                }
            }
        });

        while window.shown() {
            app::wait();
        }

        Self {
            file_input,
            dimensions: [dimensions_x_input, dimensions_y_input, dimensions_z_input],
            name: name_input,
            description: description_buffer,
            semantic_type: semantic_type_input,
            volume_size: [
                volume_size_x_input,
                volume_size_y_input,
                volume_size_z_input,
            ],
            voxel_size: [voxel_size_x_input, voxel_size_y_input, voxel_size_z_input],
            is_ok,
        }
    }

    pub fn value(&self) -> Option<Volume> {
        if !*self.is_ok.lock().unwrap() {
            return None;
        }
        let file = self.file_input.value();
        let size_x = self.dimensions[0].value();
        let size_y = self.dimensions[1].value();
        let size_z = self.dimensions[2].value();
        if file.is_empty() {
            None
        } else {
            let name = if !self.name.value().is_empty() {
                Some(self.name.value())
            } else {
                None
            };
            let description = if !self.description.text().is_empty() {
                Some(self.description.text())
            } else {
                None
            };
            let semantic_type = if !self.semantic_type.value().is_empty() {
                Some(self.semantic_type.value())
            } else {
                None
            };
            let volume_size = if self.volume_size[0].value() == 0.0
                || self.volume_size[1].value() == 0.0
                || self.volume_size[2].value() == 0.0
            {
                None
            } else {
                Some([
                    self.volume_size[0].value(),
                    self.volume_size[1].value(),
                    self.volume_size[2].value(),
                ])
            };
            let voxel_size = if self.voxel_size[0].value() == 0.0
                || self.voxel_size[1].value() == 0.0
                || self.voxel_size[2].value() == 0.0
            {
                None
            } else {
                Some([
                    self.voxel_size[0].value(),
                    self.voxel_size[1].value(),
                    self.voxel_size[2].value(),
                ])
            };
            Some(Volume::create_raw(RawVolume {
                filepath: file,
                dimensions: [size_x as u32, size_y as u32, size_z as u32],
                name,
                description,
                semantic_type,
                volume_size,
                voxel_size,
            }))
        }
    }
}
