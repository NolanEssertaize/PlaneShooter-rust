mod components;
mod constants;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::AimbotState;
use systems::{setup, player, enemy, combat};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AimbotState>()
        
        // Initialisation
        .add_systems(Startup, (
            setup::spawn_camera_and_light,
            setup::spawn_initial_enemies
        ))
        
        // Boucle de Jeu
        .add_systems(Update, (
            // Input & Joueur
            player::toggle_aimbot,
            player::player_shoot,
            player::draw_debug_gizmos,
            
            // IA & Environnement
            enemy::move_targets,
            
            // Physique & Combat
            combat::move_bullets,
            combat::handle_collisions,
            combat::cleanup_impacts,
        ))
        .run();
}