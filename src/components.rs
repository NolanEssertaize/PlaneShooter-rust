use bevy::prelude::*;

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Bullet {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ImpactEffect {
    pub timer: Timer,
}