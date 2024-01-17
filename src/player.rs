use crate::{game_state::GameState, raycast::PlaneRaycast};
use bevy::prelude::*;

pub mod inventory;
pub mod time;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentlySelected>()
            .init_resource::<InputConfig>()
            .add_systems(
                Update,
                (update_player_camera, object_select)
                    .before(crate::raycast::plane_raycast)
                    .run_if(in_state(GameState::Playing)),
            );

        app.add_systems(
            Update,
            (self::inventory::inventory_ui, self::time::time_ui)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraController;

#[derive(Component)]
pub struct Selectable;

#[derive(Resource)]
pub struct CurrentlySelected {
    pub selected: Entity,
}

impl Default for CurrentlySelected {
    fn default() -> Self {
        Self {
            selected: Entity::PLACEHOLDER,
        }
    }
}

pub fn object_select(
    mut query: Query<&PlaneRaycast, With<Camera>>,
    selectables: Query<(&GlobalTransform, &Selectable, Entity)>,
    mbtn: Res<Input<MouseButton>>,
    mut sel: ResMut<CurrentlySelected>,
) {
    if !mbtn.just_pressed(MouseButton::Left) {
        return;
    }

    let raycast = query.single_mut();
    let Some(pos) = raycast.result else { return };

    let control_distance = 2.0;

    for p in selectables.iter() {
        if (p.0.translation() - pos).length() < control_distance {
            sel.selected = p.2;
            println!("Selected entity with id = {}", p.2.index());
        }
    }
}

#[derive(Resource)]
pub struct InputConfig {
    pub movement_speed: f32,
    pub rotation_speed: f32,

    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,

    pub rotate_left: KeyCode,
    pub rotate_right: KeyCode,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            movement_speed: 20.0,
            rotation_speed: 3.0,

            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,

            rotate_left: KeyCode::Q,
            rotate_right: KeyCode::E,
        }
    }
}

pub fn update_player_camera(
    input: Res<InputConfig>,
    mut query: Query<&mut Transform, With<CameraController>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time<Real>>,
) {
    let mut transform = query.single_mut();

    let mut diff = Vec3::new(0.0, 0.0, 0.0);

    if keyboard.pressed(input.move_left) {
        diff.x += 1.0;
    }
    if keyboard.pressed(input.move_right) {
        diff.x -= 1.0
    }
    if keyboard.pressed(input.move_forward) {
        diff.z += 1.0;
    }
    if keyboard.pressed(input.move_backward) {
        diff.z -= 1.0;
    }

    let base_forward = transform.forward();

    let forward = Vec3::new(base_forward.x, 0.0, base_forward.z);
    let right = Vec3::new(base_forward.z, 0.0, -base_forward.x);

    let movement = (right * diff.x + forward * diff.z).normalize_or_zero();

    transform.translation += movement * time.delta_seconds() * input.movement_speed;

    let mut angle = 0.0;
    if keyboard.pressed(input.rotate_right) {
        angle += 1.0;
    }
    if keyboard.pressed(input.rotate_left) {
        angle -= 1.0;
    }
    transform.rotate_y(angle * time.delta_seconds() * input.rotation_speed);
}
