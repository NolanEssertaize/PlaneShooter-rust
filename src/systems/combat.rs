use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &mut Bullet)>,
) {
    for (entity, mut transform, mut velocity, mut bullet) in &mut query {
        velocity.0.y -= GRAVITY * time.delta_secs();
        transform.translation += velocity.0 * time.delta_secs();
        bullet.timer.tick(time.delta());

        if bullet.timer.is_finished() || transform.translation.y < -20.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    target_query: Query<&Transform, With<Target>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for target_transform in &target_query {
            if bullet_transform.translation.distance(target_transform.translation) < COLLISION_DIST {
                commands.entity(bullet_entity).despawn();
                spawn_impact_effect(&mut commands, &mut meshes, &mut materials, bullet_transform.translation);
                break;
            }
        }
    }
}

pub fn cleanup_impacts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ImpactEffect)>,
) {
    for (entity, mut effect) in &mut query {
        effect.timer.tick(time.delta());
        if effect.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_impact_effect(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
        Transform::from_translation(pos),
        ImpactEffect { timer: Timer::from_seconds(0.15, TimerMode::Once) },
    ));
}