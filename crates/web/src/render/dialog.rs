use egui::{Align2, Modifiers, RichText, Ui, Vec2};

use crate::LocalApp;

impl LocalApp {
    pub fn show_error(&mut self, title: &str, err: anyhow::Error) {
        self.dialog_title = title.to_string();
        self.dialog_msg = err.to_string();
        self.dialog_visible = true;
    }

    pub fn show_dialog(&mut self, title: &str, msg: &str) {
        self.dialog_title = title.to_string();
        self.dialog_msg = msg.to_string();
        self.dialog_visible = true;
    }

    pub fn render_dialog(&mut self, ui: &Ui) {
        if self.dialog_visible {
            if ui.input_mut(|i| {
                i.consume_key(Modifiers::NONE, egui::Key::Enter)
                    | i.consume_key(Modifiers::NONE, egui::Key::Escape)
            }) {
                self.dialog_visible = false;
            }

            // Render any dialog box
            let mut should_close = false;
            egui::Window::new(
                RichText::new(&self.dialog_title).strong(),
            )
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .collapsible(false)
            .movable(false)
            .resizable(false)
            .open(&mut self.dialog_visible)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(&self.dialog_msg),
                    );

                    if ui.button(RichText::new("Close").strong()).clicked() {
                        should_close = true;
                    }
                });
            });
            if should_close {
                self.dialog_visible = false;
            }
        }
    }
}