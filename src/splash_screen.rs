use crate::game_state::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_enter)
            .add_systems(Update, splash_ui.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), splash_exit);
    }
}

#[derive(Resource)]
struct SplashState {
    timer: Timer,
    logo: egui::TextureId,
}

fn splash_enter(mut commands: Commands, mut contexts: EguiContexts, assets: Res<AssetServer>) {
    commands.insert_resource(SplashState {
        // bevy.png just for now
        logo: contexts.add_image(assets.load("bevy.png")),
        timer: Timer::from_seconds(2.0, TimerMode::Once),
    });
}

fn splash_exit(mut commands: Commands) {
    // also free splash screen image
    commands.remove_resource::<SplashState>();
}

fn splash_ui(
    time: Res<Time>,
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut state: ResMut<SplashState>,
) {
    if state.timer.tick(time.delta()).finished() {
        game_state.set(GameState::Loading);
        return;
    }

    egui::Area::new("#SPLASH_SCREEN")
        .anchor(egui::Align2::CENTER_CENTER, [0.0; 2])
        .show(contexts.ctx_mut(), |ui| {
            let max = 128.0;
            let base = 120.0;
            let delta = max - base;

            let (_, rect) = ui.allocate_space(egui::vec2(base, base));
            let amount = delta * state.timer.percent();
            let scaled_size = [delta + amount; 2];

            let source = egui::load::SizedTexture::new(state.logo, scaled_size);
            let image = egui::widgets::Image::new(source);

            image.paint_at(ui, rect.expand(amount));
        });
}
