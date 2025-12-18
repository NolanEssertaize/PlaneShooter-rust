use bevy::prelude::*;
use crate::constants::*;
use crate::systems::enemy::spawn_target_at_random;

pub fn spawn_camera_and_light(mut commands: Commands) {
    // Caméra
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(SHOOTER_POS).looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
    ));

    // Lumière
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0),
    ));
}

pub fn spawn_initial_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(1.5, 0.5, 2.0));
    let mat = materials.add(Color::srgb(0.9, 0.1, 0.1));
    let mut rng = rand::rng();

    for _ in 0..20 {
        spawn_target_at_random(&mut commands, mesh.clone(), mat.clone(), &mut rng, true);
    }
}