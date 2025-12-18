use bevy::prelude::*;
use rand::Rng;
use crate::components::*;

pub fn spawn_target_at_random(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    mat: Handle<StandardMaterial>,
    rng: &mut impl Rng,
    start_in_middle: bool,
) {
    let x_min = if start_in_middle { -30.0 } else { 40.0 };
    let x_max = if start_in_middle { 40.0 } else { 60.0 };

    let x = rng.random_range(x_min..x_max);
    let y = rng.random_range(2.0..15.0);
    let z = rng.random_range(-40.0..-5.0);
    let vel_x = rng.random_range(-15.0..-5.0);

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat),
        Transform::from_xyz(x, y, z),
        Target,
        Velocity(Vec3::new(vel_x, rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0))),
    ));
}

pub fn move_targets(mut query: Query<(&mut Transform, &mut Velocity), With<Target>>, time: Res<Time>) {
    let mut rng = rand::rng();
    for (mut transform, mut velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();

        // Recyclage des avions
        if transform.translation.x < -50.0 {
            transform.translation.x = 50.0;
            transform.translation.y = rng.random_range(2.0..15.0);
            transform.translation.z = rng.random_range(-40.0..-5.0);
            velocity.0 = Vec3::new(
                rng.random_range(-15.0..-5.0),
                rng.random_range(-2.0..2.0),
                rng.random_range(-2.0..2.0),
            );
        }
    }
}