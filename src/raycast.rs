use crate::game_state::GameState;
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, plane_raycast.run_if(in_state(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app.add_systems(Startup, spawn_raycast_marker);
            app.add_systems(Update, update_raycast_marker.after(plane_raycast));
        }
    }
}

#[derive(Component)]
pub struct PlaneRaycast {
    pub plane_origin: Vec3,
    pub plane_normal: Vec3,
    pub result: Option<Vec3>,
}

impl PlaneRaycast {
    pub const Y: Self = Self {
        plane_origin: Vec3::ZERO,
        plane_normal: Vec3::Y,
        result: None,
    };
}

pub fn plane_raycast(
    windows: Query<&Window>,
    mut camera: Query<(Entity, &mut PlaneRaycast, &Camera)>,
    transform: TransformHelper,
) {
    let window = windows.single();
    let (entity, mut raycast, camera) = camera.single_mut();
    let camera_transform = transform.compute_global_transform(entity).unwrap();

    raycast.result = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(&camera_transform, cursor))
        .and_then(|ray| {
            ray.intersect_plane(raycast.plane_origin, raycast.plane_normal)
                .map(|distance| ray.get_point(distance))
        });
}

#[derive(Component)]
pub struct RaycastMarker;

fn spawn_raycast_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RaycastMarker,
        NotShadowCaster,
        NotShadowReceiver,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                unlit: true,
                ..default()
            }),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}

fn update_raycast_marker(
    raycast: Query<&PlaneRaycast>,
    mut marker: Query<(&mut Transform, &mut Visibility), With<RaycastMarker>>,
) {
    if let Ok((mut transform, mut visibility)) = marker.get_single_mut() {
        let point = raycast.get_single().ok().and_then(|raycast| raycast.result);
        *visibility = if let Some(point) = point {
            transform.translation = point;
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
