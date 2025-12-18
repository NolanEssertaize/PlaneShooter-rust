use bevy::prelude::*;
use crate::components::*;
use crate::resources::AimbotState;
use crate::constants::*;

pub fn toggle_aimbot(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<AimbotState>) {
    state.active = keyboard.pressed(KeyCode::Space);
}

pub fn player_shoot(
    mut commands: Commands,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    targets: Query<(&Transform, &Velocity), With<Target>>,
    state: Res<AimbotState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !mouse_btn.just_pressed(MouseButton::Left) { return; }

    let (camera, camera_transform) = camera_q.single().expect("Erreur critique: Pas de caméra principale !");
    let window = windows.single().expect("Erreur critique: Pas de fenêtre principale !");

    let Some(cursor_position) = window.cursor_position() else { return };

    let mut final_dir = Vec3::ZERO;
    let mut fired_with_aimbot = false;

    if state.active {
        if let Some((target_pos, target_vel)) = find_best_target(&targets, camera, camera_transform, cursor_position) {
            final_dir = calculate_ballistic_trajectory(target_pos, target_vel);
            fired_with_aimbot = true;
        }
    }

    if !fired_with_aimbot {
        let ray = camera.viewport_to_world(camera_transform, cursor_position).unwrap();
        final_dir = (ray.origin + ray.direction * 100.0 - SHOOTER_POS).normalize();
    }

    spawn_bullet(&mut commands, &mut meshes, &mut materials, final_dir, state.active);
}

pub fn draw_debug_gizmos(
    mut gizmos: Gizmos,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    targets: Query<(&Transform, &Velocity), With<Target>>,
    state: Res<AimbotState>,
) {
    let Ok((camera, camera_transform)) = camera_q.single() else { return };
    let Ok(window) = windows.single() else { return };
    
    if let Some(cursor_pos) = window.cursor_position() {
        if state.active {
            if let Some((target_pos, _)) = find_best_target(&targets, camera, camera_transform, cursor_pos) {
                gizmos.line(SHOOTER_POS, target_pos, Color::srgb(1.0, 0.0, 0.0));
            }
        } else if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            gizmos.line(ray.origin, ray.origin + ray.direction * 50.0, Color::srgb(1.0, 1.0, 1.0).with_alpha(0.1));
        }
    }
}


fn find_best_target(
    targets: &Query<(&Transform, &Velocity), With<Target>>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_pos: Vec2
) -> Option<(Vec3, Vec3)> {
    let mut best = None;
    let mut min_dist = AIMBOT_FOV;

    for (t_transform, t_vel) in targets {
        if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, t_transform.translation) {
            let dist = screen_pos.distance(cursor_pos);
            if dist < min_dist {
                min_dist = dist;
                best = Some((t_transform.translation, t_vel.0));
            }
        }
    }
    best
}

fn calculate_ballistic_trajectory(target_pos: Vec3, target_vel: Vec3) -> Vec3 {
    let dist = target_pos.distance(SHOOTER_POS);
    let t = dist / BULLET_SPEED;
    let future_pos = target_pos + (target_vel * t);
    let drop = 0.5 * GRAVITY * t.powi(2);
    
    let mut aim_point = future_pos;
    aim_point.y += drop;
    (aim_point - SHOOTER_POS).normalize()
}

fn spawn_bullet(
    commands: &mut Commands, 
    meshes: &mut Assets<Mesh>, 
    materials: &mut Assets<StandardMaterial>, 
    dir: Vec3, 
    is_aimbot: bool
) {
    let color = if is_aimbot { Color::srgb(1.0, 0.0, 0.0) } else { Color::srgb(0.0, 1.0, 0.0) };
    
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(color)),
        Transform::from_translation(SHOOTER_POS),
        Bullet { timer: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once) },
        Velocity(dir * BULLET_SPEED),
    ));
}