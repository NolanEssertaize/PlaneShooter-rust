use bevy::prelude::*;

pub const GRAVITY: f32 = 9.81;
pub const BULLET_SPEED: f32 = 80.0;
pub const BULLET_LIFETIME: f32 = 5.0;
pub const SHOOTER_POS: Vec3 = Vec3::new(0.0, 5.0, 20.0);
pub const AIMBOT_FOV: f32 = 300.0;
pub const COLLISION_DIST: f32 = 1.3;