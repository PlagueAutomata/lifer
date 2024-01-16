use crate::loading::AssetCache;
use crate::mechanics::*;
use crate::{game_state::GameState, loading::ItemDatabase};
use bevy::prelude::*;
use big_brain::prelude::*;

mod inventory;
mod movement;

use self::item::ContainerBundle;
pub use self::{
    inventory::Inventory,
    movement::{CachedFinder, FindAndMove},
};

pub const DEFAULT_COLOR: Color = Color::BLACK;
pub const SLEEP_COLOR: Color = Color::BLUE;
pub const FARM_COLOR: Color = Color::YELLOW;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BigBrainPlugin::new(PreUpdate),
            self::inventory::InventoryPlugin,
        ))
        .init_resource::<AssetCache>()
        .add_event::<SpawnCharacter>()
        .add_systems(
            PreUpdate,
            spawner_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            sync_character_color.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct CharacterController {
    pub speed: f32,
    pub color: Color,
    pub is_sleeping: bool,
}

pub fn sync_character_color(
    mut cache: ResMut<AssetCache>,
    query: Query<(&CharacterController, &Children)>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (ctrl, children) in &query {
        let mut handle = children
            .first()
            .and_then(|&entity| material_query.get_mut(entity).ok());

        if let Some(dst) = handle.as_deref_mut() {
            *dst = cache.get_material(&mut materials, ctrl.color);
        }
    }
}

#[derive(Event, Default)]
pub struct SpawnCharacter {
    pub player: bool,
    pub transform: Transform,
    pub model: CharacterModel,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CharacterModel {
    pub height: f32,
    pub radius: f32,
    pub face_height: f32,
}

impl std::cmp::Eq for CharacterModel {}

impl std::hash::Hash for CharacterModel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.height.to_bits().hash(state);
        self.radius.to_bits().hash(state);
        self.face_height.to_bits().hash(state);
    }
}

impl Default for CharacterModel {
    fn default() -> Self {
        Self {
            height: 1.75,
            radius: 0.5,
            face_height: 0.5,
        }
    }
}

#[derive(Component, Clone, ActionSpawn)]
pub struct Idle;

#[derive(Clone)]
pub struct ModelCacheEntry {
    pub capsule: Handle<Mesh>,
    pub cube: Handle<Mesh>,
}

pub fn spawner_system(
    mut cache: ResMut<AssetCache>,
    mut commands: Commands,
    mut events: ResMut<Events<SpawnCharacter>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    items: Res<ItemDatabase>,
) {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    let mut rng = SmallRng::from_entropy();

    for SpawnCharacter {
        player,
        transform,
        model,
    } in events.drain()
    {
        let mut container_entity = commands.spawn(ContainerBundle::default());
        let container = container_entity.id();

        if player {
            container_entity.with_children(|builder| {
                let sphere = shape::Icosphere {
                    radius: 0.3,
                    subdivisions: 3,
                };
                builder.spawn((
                    items.potion.clone(),
                    PbrBundle {
                        mesh: meshes.add(sphere.try_into().unwrap()),
                        material: cache.get_material(&mut materials, Color::RED),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        ..default()
                    },
                ));
            });
        }

        let mut entity = commands.spawn((
            SpatialBundle {
                transform,
                ..default()
            },
            CharacterController {
                speed: 5.0,
                color: DEFAULT_COLOR,
                is_sleeping: false,
            },
            Fatigue {
                current: rng.gen_range(0.0..=100.0),
                change: 8.0,
            },
            Inventory { container },
            create_thinker(),
        ));

        if player {
            entity.insert(crate::player::Player);
        }

        entity.with_children(|builder| {
            let ModelCacheEntry { capsule, cube } = cache.get_model(&mut meshes, model);

            builder.spawn(PbrBundle {
                mesh: capsule.clone(),
                material: cache.get_material(&mut materials, Color::YELLOW),
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    model.height / 2.0 + model.radius,
                    0.0,
                )),
                ..default()
            });

            builder.spawn(PbrBundle {
                mesh: cube.clone(),
                material: cache.get_material(&mut materials, Color::RED),
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    model.height - model.face_height / 2.0 + model.radius,
                    -model.radius,
                )),
                ..default()
            });
        });

        entity.add_child(container);
    }
}

fn create_thinker() -> ThinkerBuilder {
    Thinker::highest()
        .when(
            FatigueScorer::default(),
            Sequence::step((FindAndMove::<House>::new(0.1), Sleep::new(10.0, 30.0))),
        )
        .when(
            WorkNeedScorer,
            Sequence::step((FindAndMove::<Field>::new(0.1), Farm::new(30.0))),
        )
        .when(
            SellNeedScorer,
            Sequence::step((FindAndMove::<Market>::new(0.1), Sell)),
        )
        .otherwise(Idle)
}
