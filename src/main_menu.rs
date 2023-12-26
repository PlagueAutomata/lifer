use crate::game_state::GameState;
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, main_menu_ui.run_if(in_state(GameState::MainMenu)));
    }
}

fn main_menu_ui(
    mut contexts: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    egui::Area::new(egui::Id::from("#MAIN_MENU"))
        .anchor(egui::Align2::CENTER_CENTER, [0.0; 2])
        .show(contexts.ctx_mut(), |ui| {
            ui.scope(|ui| {
                use egui::FontFamily::Proportional;
                use egui::FontId;
                use egui::TextStyle::*;

                ui.style_mut().text_styles = [
                    (Body, FontId::new(24.0, Proportional)),
                    (Monospace, FontId::new(24.0, Proportional)),
                    (Button, FontId::new(24.0, Proportional)),
                ]
                .into();

                ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
                ui.spacing_mut().button_padding = egui::vec2(0.0, 32.0);
                ui.set_max_width(300.0);

                ui.vertical_centered_justified(|ui| {
                    if ui.button("New game").clicked() {
                        next_state.set(GameState::Playing);
                    }
                    if ui.button("Settings").clicked() {
                        //
                    }
                    if ui.button("Quit").clicked() {
                        app_exit_events.send(AppExit)
                    }
                });
            });
        });
}
