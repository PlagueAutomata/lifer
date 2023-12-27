use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

use bevy::input::mouse::*;

mod game_state;
mod main_menu;
mod raycast;
mod splash_screen;

#[derive(Component)]
struct PlayerCamera {
    control_speed: f32,
    rot_speed: f32,
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player_camera);
    }
}

fn main() {
    let mut app = App::new();

    app.add_state::<crate::game_state::GameState>();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Lifer"),
                resolution: (1280.0, 720.0).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }),
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
        PlayerInputPlugin,
        bevy_egui::EguiPlugin,
        crate::splash_screen::SplashScreenPlugin,
        crate::main_menu::MainMenuPlugin,
        crate::raycast::RaycastPlugin,
    ));

    app.add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCamera {
            control_speed: 10.0,
            rot_speed: 5.0,
        },
        crate::raycast::PlaneRaycast::Y,
    ));
}

fn update_player_camera(
    mut query: Query<(&mut Transform, &PlayerCamera)>,
    keyboard: Res<Input<KeyCode>>,
    _mouse_button: Res<Input<MouseButton>>,
    time: Res<Time>,
    _mouse_movement: EventReader<MouseMotion>,
    _mouse_wheel: EventReader<MouseWheel>,
) {
    let (mut transform, cam) = query.single_mut();
    let mut diff = Vec3::new(0.0, 0.0, 0.0);
    if keyboard.pressed(KeyCode::D) {
        diff.x += 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        diff.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::W) {
        diff.z += 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        diff.z -= 1.0;
    }
    let fwd = -Vec3::new(transform.local_z().x, 0.0, transform.local_z().z);
    let right = Vec3::new(transform.local_z().z, 0.0, -transform.local_z().x);
    diff = right * diff.x + fwd * diff.z;
    transform.translation += diff.normalize_or_zero() * time.delta_seconds() * cam.control_speed;

    let mut rot = 0.0;
    if keyboard.pressed(KeyCode::E) {
        rot -= 1.0;
    }
    if keyboard.pressed(KeyCode::Q) {
        rot += 1.0;
    }
    transform.rotate_y(rot * time.delta_seconds() * cam.rot_speed);
}
