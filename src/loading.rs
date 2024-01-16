use crate::{
    character::{CharacterModel, ModelCacheEntry},
    game_state::GameState,
    mechanics::ItemAsset,
};
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_egui::{egui, EguiContexts};
use iyes_progress::{Progress, ProgressCounter, ProgressPlugin, ProgressSystem};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(GameState::Loading).continue_to(GameState::MainMenu))
            .add_loading_state(
                LoadingState::new(GameState::Loading).load_collection::<ItemDatabase>(),
            )
            .add_systems(
                Update,
                (track_fake_long_task::<5>.track_progress(), print_progress)
                    .chain()
                    .run_if(in_state(GameState::Loading))
                    .after(LoadingStateSet(GameState::Loading)),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct ItemDatabase {
    #[asset(path = "items/money.item.ron")]
    pub money: Handle<ItemAsset>,

    #[asset(path = "items/raw_food.item.ron")]
    pub raw_food: Handle<ItemAsset>,

    #[asset(path = "items/potion.item.ron")]
    pub potion: Handle<ItemAsset>,
}

fn track_fake_long_task<const TOTAL: u32>(time: Res<Time>) -> Progress {
    let progress = Progress {
        done: time.elapsed_seconds() as u32,
        total: TOTAL,
    };
    if progress.done >= progress.total {
        info!("Long task is completed");
    }
    progress
}

fn print_progress(mut contexts: EguiContexts, counter: Res<ProgressCounter>) {
    let progress = counter.progress();

    egui::Area::new("#LOADING_SCREEN")
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
                    (Heading, FontId::new(34.0, Proportional)),
                ]
                .into();

                ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
                ui.spacing_mut().button_padding = egui::vec2(0.0, 32.0);
                ui.set_max_width(300.0);

                ui.vertical_centered_justified(|ui| {
                    ui.heading("loading...");

                    let bar = egui::ProgressBar::new(progress.done as f32 / progress.total as f32);

                    ui.add(bar);
                });
            });
        });
}

#[derive(Clone, Copy, PartialEq)]
struct HashedColor(Color);

impl std::cmp::Eq for HashedColor {}

impl std::hash::Hash for HashedColor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.r().to_bits().hash(state);
        self.0.g().to_bits().hash(state);
        self.0.b().to_bits().hash(state);
        self.0.a().to_bits().hash(state);
    }
}

#[derive(Resource, Default)]
pub struct AssetCache {
    model: HashMap<CharacterModel, ModelCacheEntry>,
    material: HashMap<HashedColor, Handle<StandardMaterial>>,
}

impl AssetCache {
    pub fn get_model(
        &mut self,
        meshes: &mut Assets<Mesh>,
        model: CharacterModel,
    ) -> ModelCacheEntry {
        self.model
            .entry(model)
            .or_insert_with(|| {
                let capsule = shape::Capsule {
                    depth: model.height,
                    radius: model.radius,

                    latitudes: 4,
                    longitudes: 6,
                    ..default()
                };
                let capsule = meshes.add(Mesh::from(capsule));
                let cube = meshes.add(Mesh::from(shape::Box::new(
                    model.radius * 2.0,
                    model.face_height,
                    model.radius,
                )));

                ModelCacheEntry { capsule, cube }
            })
            .clone()
    }

    pub fn get_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        color: Color,
    ) -> Handle<StandardMaterial> {
        self.material
            .entry(HashedColor(color))
            .or_insert_with(|| {
                materials.add(StandardMaterial {
                    base_color: color,
                    perceptual_roughness: 1.0,
                    ..default()
                })
            })
            .clone()
    }
}
