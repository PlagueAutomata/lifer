use crate::character::SpawnCharacter;
use crate::loading::AssetCache;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    window::PresentMode,
};

pub mod character;
pub mod game_state;
pub mod loading;
pub mod main_menu;
pub mod mechanics;
pub mod player;
pub mod raycast;
pub mod splash_screen;

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
        bevy_egui::EguiPlugin,
        crate::splash_screen::SplashScreenPlugin,
        crate::loading::LoadingPlugin,
        crate::player::PlayerPlugin,
        crate::main_menu::MainMenuPlugin,
        crate::mechanics::MechanicsPlugin,
        crate::character::CharacterPlugin,
        crate::raycast::RaycastPlugin,
    ));

    app.add_systems(Startup, setup_camera);
    app.add_systems(OnEnter(crate::game_state::GameState::Playing), init_scene);

    app.add_systems(
        PreUpdate,
        absorb_egui_inputs
            .after(bevy_egui::systems::process_input_system)
            .before(bevy_egui::EguiSet::BeginFrame),
    );

    app.run();
}

// see: https://github.com/mvlabat/bevy_egui/issues/47
fn absorb_egui_inputs(
    mut contexts: bevy_egui::EguiContexts,
    mut mouse: ResMut<Input<MouseButton>>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();
    if ctx.wants_pointer_input() || ctx.is_pointer_over_area() {
        let modifiers = [
            KeyCode::SuperLeft,
            KeyCode::SuperRight,
            KeyCode::ControlLeft,
            KeyCode::ControlRight,
            KeyCode::AltLeft,
            KeyCode::AltRight,
            KeyCode::ShiftLeft,
            KeyCode::ShiftRight,
        ];

        let pressed = modifiers.map(|key| keyboard.pressed(key).then_some(key));

        mouse.reset_all();
        keyboard.reset_all();

        for key in pressed.into_iter().flatten() {
            keyboard.press(key);
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(-2.5, 0.0, 0.0)),
            crate::player::CameraController,
        ))
        .with_children(|builder| {
            builder.spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 15.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                crate::raycast::PlaneRaycast::Y,
            ));
        });
}

fn init_scene(
    mut cache: ResMut<AssetCache>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawner: EventWriter<SpawnCharacter>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(14.0).into()),
        material: cache.get_material(&mut materials, Color::DARK_GREEN),
        transform: Transform {
            translation: Vec3::new(0.0, -0.01, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ..default()
        },
        ..default()
    });

    {
        // some characters

        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};
        let mut rng = SmallRng::from_entropy();

        for _ in 0..100 {
            let x = rng.gen_range(-20.0..=20.0);
            let z = rng.gen_range(-20.0..=20.0);

            spawner.send(SpawnCharacter {
                transform: Transform::from_translation(Vec3::new(x, 0.0, z)),
                ..default()
            });
        }
    }

    // spawn player
    spawner.send(SpawnCharacter {
        player: true,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    commands.insert_resource(DirectionalLightShadowMap { size: 2048 });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },

        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.1,
            // first_cascade_far_bound: 5.0,
            first_cascade_far_bound: 4.0,
            // maximum_distance: 1000.0,
            maximum_distance: 150.0,
            overlap_proportion: 0.2,
        }
        .build(),
        ..default()
    });
}
