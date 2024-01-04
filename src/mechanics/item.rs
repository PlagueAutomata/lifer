use bevy::prelude::*;

mod asset;
mod spawn;

pub use self::{
    asset::{ItemAsset, ItemAssetLoader, ItemAssetLoaderError},
    spawn::ItemSpawnError,
};

#[derive(Component)]
pub struct Item;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemAsset>()
            .init_asset_loader::<ItemAssetLoader>()
            .register_type::<Consumable>()
            .register_type::<ItemName>()
            .add_systems(SpawnScene, self::spawn::spawn_items_system);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Consumable {
    pub current: f32,
    pub maximum: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ItemName(pub String);
