use crate::{components::MainCamera, player::Player, GameState};
use bevy::{prelude::*, window::CursorGrabMode};

const CAMERA_HEIGHT: f32 = 1.0;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Ready).with_system(update_camera_pos));
    }
}

fn update_camera_pos(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Player should be avaible for renderer");

    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("Camera should be avaible for renderer");

    camera_transform.translation = Vec3 {
        x: player_transform.translation.x,
        y: CAMERA_HEIGHT,
        z: player_transform.translation.z,
    };
    camera_transform.rotation = player_transform.rotation;
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);
        }
        _ => {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}
