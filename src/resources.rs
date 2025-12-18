use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AimbotState {
    pub active: bool,
}