/*use std::hint::black_box;

use fltk::{
    app::{event_inside, event_x},
    browser::Browser,
    enums::{Cursor, Event},
    prelude::{BrowserExt, WidgetBase, WidgetExt},
    widget_extends,
};

pub struct Table {
    inner: Browser,
    last_cursor: Cursor,
    dragged_column: Option<u32>,
    column_widths: Vec<u32>,
}

impl Table {
    pub fn new() -> Self {
        let inner = Browser::default();
        let last_cursor = Cursor::Default;
        let dragged_column = None;
        let column_widths = Vec::new();
        Self {
            inner,
            last_cursor,
            dragged_column,
            column_widths,
        }
    }

    pub fn change_cursor(&mut self, new_cursor: Cursor) {
        if new_cursor == self.last_cursor {
            return;
        }
        if let Some(mut window) = self.inner.window() {
            window.set_cursor(new_cursor);
        }
    }

    pub fn which_column_near_mouse(&self) -> Option<u32> {
        let x = self.inner.x();
        let y = self.inner.y();
        let w = self.inner.w();
        let h = self.inner.h();
        if !event_inside(x, y, w, h) {
            return None;
        }
        let mouse_x = event_x() + self.inner.hposition();
        let mut column_x = x as u32;
        for i in 0..self.column_widths.len() {
            column_x += self.column_widths[i];
            let difference = mouse_x - column_x as i32;
            if (-4..=4).contains(&difference) {
                return Some(i as u32);
            }
        }
        None
    }

    pub fn recalculate_hscrollbar(&mut self) {
        let size = self.inner.text_size();
        self.inner.set_text_size(size + 1);
        self.inner.set_text_size(size);
        self.inner.redraw();
    }

    fn handle(&mut self, event: Event) -> bool {
        let mut ret = false;
        match event {
            Event::Enter => ret = true,
            Event::Move => {
                if self.which_column_near_mouse().is_some() {
                    self.change_cursor(Cursor::WE);
                } else {
                    self.change_cursor(Cursor::Default)
                }
                ret = true;
            }
            Event::Push => {
                if let Some(column) = self.which_column_near_mouse() {
                    self.dragged_column = Some(column);
                    self.change_cursor(Cursor::Default);
                    return true;
                }
            }
            Event::Drag => {
                if let Some(dragged_column) = self.dragged_column {
                    let mouse_x = event_x() + self.hposition();
                    let mut new_width = mouse_x - self.x();
                    for i in 0..dragged_column {
                        new_width -= self.column_widths[i as usize] as i32;
                    }
                    if new_width > 0 {
                        self.column_widths[dragged_column as usize] = new_width as u32;
                        if self.column_widths[dragged_column as usize] < 2 {
                            self.column_widths[dragged_column as usize] = 2;
                        }
                        self.recalculate_hscrollbar();
                        self.inner.redraw();
                    }
                    return true;
                }
            }
            Event::Leave | Event::Released => {
                self.dragged_column = None;
                self.change_cursor(Cursor::Default);
                if event == Event::Released {
                    return true;
                }
                ret = true;
            }
            _ => (),
        }
        if self.inner.handle_event(event) {
            true
        } else {
            ret
        }
    }

    fn draw(&self) {
        self.inner.draw();
    }
}

widget_extends!(Table, Browser, inner);
*/
