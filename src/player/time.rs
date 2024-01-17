use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn time_ui(mut time: ResMut<Time<Virtual>>, mut contexts: EguiContexts) {
    egui::Area::new("#TIME_HUD")
        .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.scope(|ui| {
                egui::Frame::side_top_panel(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let pause = egui::Button::new("\u{23F8}");
                        let speed_1 = egui::Button::new("x1");
                        let speed_2 = egui::Button::new("x2");
                        let speed_3 = egui::Button::new("x3");

                        let speed = time.effective_speed();

                        if ui.add_enabled(!time.is_paused(), pause).clicked() {
                            time.pause();
                        }
                        if ui.add_enabled(speed != 1.0, speed_1).clicked() {
                            time.set_relative_speed(1.0);
                            time.unpause();
                        }
                        if ui.add_enabled(speed != 2.0, speed_2).clicked() {
                            time.set_relative_speed(2.0);
                            time.unpause();
                        }
                        if ui.add_enabled(speed != 3.0, speed_3).clicked() {
                            time.set_relative_speed(3.0);
                            time.unpause();
                        }
                    });
                });
            });
        });
}
