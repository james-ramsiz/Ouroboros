#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod components;
mod plugins;

use plugins::{
    camera::CameraPlugin,
    loop_detector::LoopPlugin,
    snake::OuroborosPlugin,
    ui::UiPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Ouroboros".into(),
                    resolution: (1280.0, 720.0).into(),
                    // Needed for WASM canvas targeting
                    canvas: Some("#bevy-canvas".to_owned()),
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                // Works for both native and WASM
                ..default()
            }),
    )
    .add_plugins((
        OuroborosPlugin,
        LoopPlugin,
        CameraPlugin,
        UiPlugin,
    ))
    .add_systems(Startup, setup_environment)
    .run();
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(40.0, 40.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.04, 0.08, 0.05),
            perceptual_roughness: 0.9,
            ..default()
        }),
        ..default()
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 10.0, 4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.1, 0.15, 0.1),
        brightness: 0.4,
    });
}
