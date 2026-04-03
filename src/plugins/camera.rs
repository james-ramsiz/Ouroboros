use bevy::prelude::*;
use crate::components::SnakeHead;

#[derive(Component)]
pub struct OrbitCamera {
    pub distance: f32,
    pub height:   f32,
    pub smoothing: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self { distance: 14.0, height: 5.0, smoothing: 6.0 }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, orbit_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 12.0, 18.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCamera::default(),
    ));
}

fn orbit_camera(
    time:    Res<Time>,
    head_q:  Query<&Transform, (With<SnakeHead>, Without<OrbitCamera>)>,
    mut cam_q: Query<(&OrbitCamera, &mut Transform)>,
) {
    let Ok(head_xf) = head_q.get_single() else { return };
    let Ok((cam, mut cam_xf)) = cam_q.get_single_mut() else { return };

    let behind = head_xf.back() * cam.distance + Vec3::Y * cam.height;
    let desired = head_xf.translation + behind;

    // Smooth follow
    cam_xf.translation = cam_xf.translation.lerp(
        desired,
        (cam.smoothing * time.delta_seconds()).min(1.0),
    );
    cam_xf.look_at(head_xf.translation, Vec3::Y);
}
