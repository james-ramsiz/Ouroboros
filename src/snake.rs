use crate::materials::snake_ink::{SnakeInkExtension, SnakeInkMaterial, SnakeInkSettings};
use bevy::pbr::ExtendedMaterial;

// Register the material in your plugin's build():
app.add_plugins(MaterialPlugin::<SnakeInkMaterial>::default());

// In spawn_ouroboros, replace StandardMaterial with:
fn spawn_ouroboros(
    mut commands:   Commands,
    mut meshes:     ResMut<Assets<Mesh>>,
    mut ink_mats:   ResMut<Assets<SnakeInkMaterial>>,
) {
    let seg_mesh = meshes.add(Sphere::new(0.3).mesh().ico(3).unwrap());

    // One shared ink material for the whole snake
    let ink_mat = ink_mats.add(SnakeInkMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.9,
            metallic: 0.0,
            ..default()
        },
        extension: SnakeInkExtension {
            settings: SnakeInkSettings::default(),
        },
    });

    for i in 0..SEGMENT_COUNT {
        let angle = -(i as f32) * (TAU / SEGMENT_COUNT as f32);
        let pos = Vec3::new(angle.cos() * RING_RADIUS, 0.3, angle.sin() * RING_RADIUS);
        let scale = 1.0 - (i as f32 / SEGMENT_COUNT as f32) * 0.35;

        let mut entity = commands.spawn((
            MaterialMeshBundle {
                mesh: seg_mesh.clone(),
                material: ink_mat.clone(),
                transform: Transform::from_translation(pos)
                    .with_scale(Vec3::splat(scale)),
                ..default()
            },
            BodySegment { index: i, total: SEGMENT_COUNT },
        ));

        if i == 0 { entity.insert(SnakeHead::default()); }
        if i == SEGMENT_COUNT - 1 { entity.insert(SnakeTail); }
    }
}
