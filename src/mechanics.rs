pub mod field;
pub mod house;
pub mod item;
pub mod market;

pub use self::{
    field::{Farm, Field, WorkNeedScorer},
    house::{Fatigue, FatigueScorer, House, Sleep},
    item::{Item, ItemAsset, ItemAssetLoader, ItemAssetLoaderError, ItemSpawnError},
    market::{Market, Sell, SellNeedScorer},
};

use crate::{game_state::GameState, loading::AssetCache};
use bevy::prelude::*;

pub const FIELD_COLOR: Color = Color::YELLOW;
pub const HOUSE_COLOR: Color = Color::BLUE;
pub const MARKET_COLOR: Color = Color::RED;

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            self::field::FieldPlugin,
            self::house::HousePlugin,
            self::item::ItemPlugin,
            self::market::MarketPlugin,
        ))
        .add_systems(OnEnter(GameState::Playing), spawn_scene)
        .add_systems(OnExit(GameState::Playing), despawn_scene);
    }
}

pub fn spawn_scene(
    mut cache: ResMut<AssetCache>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rotation = Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
    let model = meshes.add(shape::Circle::new(0.5).into());

    // farm field
    commands.spawn(Field).insert(PbrBundle {
        mesh: model.clone(),
        material: cache.get_material(&mut materials, FIELD_COLOR),
        transform: rotation.with_translation(Vec3::new(-5.0, 0.0, 0.0)),
        ..default()
    });

    // sleeping house
    commands.spawn(House).insert(PbrBundle {
        mesh: model.clone(),
        material: cache.get_material(&mut materials, HOUSE_COLOR),
        transform: rotation.with_translation(Vec3::new(5.0, 0.0, -5.0)),
        ..default()
    });

    // marketplace
    commands.spawn(Market).insert(PbrBundle {
        mesh: model,
        material: cache.get_material(&mut materials, MARKET_COLOR),
        transform: rotation.with_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..default()
    });
}

pub fn despawn_scene(
    mut commands: Commands,
    fields: Query<Entity, With<Field>>,
    houses: Query<Entity, With<House>>,
    markets: Query<Entity, With<Market>>,
) {
    for entity in &fields {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &houses {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &markets {
        commands.entity(entity).despawn_recursive();
    }
}
