use crate::{game_state::GameState, selectable::Selectable};
use crate::mechanics::*;
use bevy::prelude::*;
use big_brain::prelude::*;

mod inventory;
mod movement;

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
        app.add_plugins((BigBrainPlugin::new(PreUpdate),))
            .add_event::<SpawnCharacter>()
            .add_systems(PreUpdate, spawner_system)
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
    query: Query<(&CharacterController, &Children)>,
    material_query: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (ctrl, children) in &query {
        let child = children.first();
        let material_id = child.and_then(|&entity| material_query.get(entity).ok());
        let material = material_id.and_then(|id| materials.get_mut(id));
        if let Some(material) = material {
            material.base_color = ctrl.color;
        }
    }
}

#[derive(Event, Default)]
pub struct SpawnCharacter {
    pub transform: Transform,
    pub model: CharacterModel,
}

pub struct CharacterModel {
    pub height: f32,
    pub radius: f32,
    pub face_height: f32,
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

pub fn spawner_system(
    mut commands: Commands,
    mut events: ResMut<Events<SpawnCharacter>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for SpawnCharacter { transform, model } in events.drain() {
        let mut entity = commands.spawn((
            SpatialBundle {
                transform,
                ..default()
            },
            Selectable {},
            CharacterController {
                speed: 5.0,
                color: DEFAULT_COLOR,
                is_sleeping: false,
            },
            Fatigue {
                current: 0.0,
                change: 8.0,
            },
            Inventory {
                money: 0,
                items: 0.0,
            },
            Thinker::first_to_score(0.6)
                .when(
                    FatigueScorer::default(),
                    Sequence::step((FindAndMove::<House>::new(0.1), Sleep::new(10.0, 30.0))),
                )
                .when(
                    WorkNeedScorer,
                    Sequence::step((FindAndMove::<Field>::new(0.1), Farm::new(10.0, 30.0))),
                )
                .when(
                    SellNeedScorer,
                    Sequence::step((FindAndMove::<Market>::new(0.1), Sell)),
                ),
        ));

        entity.with_children(|builder| {
            let capsule = shape::Capsule {
                depth: model.height,
                radius: model.radius,
                ..default()
            };

            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(capsule)),
                material: materials.add(Color::YELLOW.into()),
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    model.height / 2.0 + model.radius,
                    0.0,
                )),
                ..default()
            });

            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    model.radius * 2.0,
                    model.face_height,
                    model.radius,
                ))),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    model.height - model.face_height / 2.0 + model.radius,
                    -model.radius,
                )),
                ..default()
            });
        });
    }
}
