use egui::{Align2, Color32, Key, TextEdit, Vec2, Widget};

use crate::{state::local_app::FocusOn, LocalApp};

use super::Mode;

fn is_ok(app: &LocalApp) -> bool {
    is_username_ok(app) && is_password_ok(app)
}

fn is_username_ok(app: &LocalApp) -> bool {
    app.username.len() >= 5
}

fn is_password_ok(app: &LocalApp) -> bool {
    app.password.len() >= 5
}

impl LocalApp {
    fn login(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        if is_ok(self) {
            self.login_bank(ui, frame);
        } else {
            self.show_dialog(ui, "Invalid", "Inputs are invalid, check the fields");
        }
    }

    pub fn render_login(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.try_finish(ui, |app| {
            app.mode = Mode::Summary;
        });

        egui::Window::new("Login")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(250.0, 200.0))
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.add_space(5.0);

                let mut enter_pressed = ui.ctx().input_mut(|input| input.key_pressed(Key::Enter));

                let mut focus_password = false;

                ui.horizontal(|ui| {
                    ui.label("Bank: ");

                    let res = egui::ComboBox::from_id_source("select_login")
                        .selected_text(format!("{}", self.username))
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(100.0);

                            for (bank, _) in self.banks.iter() {
                                ui.selectable_value(
                                    &mut self.username,
                                    bank.clone(),
                                    format!("{}", bank),
                                );
                            }
                        });

                    if matches!(self.focus_on, Some(FocusOn::Username)) {
                        res.response.request_focus();
                        self.focus_on.take();
                        self.save_state(frame);
                    }

                    if res.response.changed() || res.response.clicked() {
                        focus_password = true;
                    }

                    if enter_pressed && res.response.lost_focus() {
                        enter_pressed = false;
                        focus_password = true;
                    }
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Password: ");

                    let is_ok = is_password_ok(self);
                    let res = TextEdit::singleline(&mut self.password)
                        .password(true)
                        .text_color(if is_ok {
                            Color32::GREEN
                        } else {
                            Color32::LIGHT_RED
                        })
                        .ui(ui);

                    if focus_password {
                        res.request_focus();
                    }

                    if enter_pressed && res.lost_focus() {
                        enter_pressed = false;
                        self.login(ui, frame);
                    }
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if ui
                        .add_sized(Vec2::new(100.0, 20.0), egui::Button::new("Login"))
                        .clicked()
                    {
                        self.login(ui, frame);
                    }
                    if ui
                        .add_sized(Vec2::new(100.0, 20.0), egui::Button::new("Register"))
                        .clicked()
                    {
                        self.username = Default::default();
                        self.password = Default::default();
                        self.confirm_password = Default::default();
                        self.mode = Mode::NewAccount;
                        self.focus_on.replace(FocusOn::Username);
                    }
                });
            });
    }
}
