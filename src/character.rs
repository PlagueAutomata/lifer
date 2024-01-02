use crate::{game_state::GameState, selectable::Selectable};
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCharacter>()
            .add_systems(PreUpdate, spawner_system)
            .add_systems(
                Update,
                (read_target_position, move_character)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct CharacterData {
    pub faction: i32,
}

#[derive(Component)]
pub struct CharacterController {
    pub target: Option<Vec3>,
    pub velocity: Vec3,
    pub speed: f32,
}

#[derive(Component)]
pub struct MovementTarget(pub Entity);

pub fn read_target_position(
    transforms: TransformHelper,
    mut query: Query<(&mut CharacterController, &MovementTarget)>,
) {
    for (mut ctrl, &MovementTarget(target)) in query.iter_mut() {
        ctrl.target = transforms
            .compute_global_transform(target)
            .map(|transform| transform.translation())
            .ok();
    }
}

pub fn move_character(
    time: Res<Time>,
    mut query: Query<(&mut CharacterController, &mut Transform), Without<Parent>>,
) {
    for (mut ctrl, mut transform) in query.iter_mut() {
        ctrl.velocity = match ctrl.target {
            Some(target) => (target - transform.translation).normalize_or_zero() * ctrl.speed,
            None => Vec3::ZERO,
        };
        transform.translation += ctrl.velocity * time.delta_seconds();

        if ctrl.velocity.length() > 0.0 {
            // TODO: remove pitch/roll
            *transform = transform.looking_to(ctrl.velocity, Vec3::Y);
        }
    }
}
#[derive(Event, Default)]
pub struct SpawnCharacter {
    pub transform: Transform,
    pub target: Option<Entity>,
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
    for SpawnCharacter {
        transform,
        target,
        model,
    } in events.drain()
    {
        let mut entity = commands.spawn((
            SpatialBundle {
                transform,
                ..default()
            },
            CharacterController {
                target: None,
                velocity: Vec3::ZERO,
                speed: 3.0,
            },
            CharacterData { faction: -1 },
            Selectable {},
        ));

        if let Some(target) = target {
            entity.insert(MovementTarget(target));
        }

        entity.with_children(|builder| {
            let capsule = shape::Capsule {
                depth: model.height,
                radius: model.radius,
                ..default()
            };

            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(capsule)),
                material: materials.add(Color::YELLOW.into()),
                transform: Transform::from_translation(Vec3::new(0.0, model.height / 2.0, 0.0)),
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
                    model.height - model.face_height / 2.0,
                    -model.radius,
                )),
                ..default()
            });
        });
    }
}
