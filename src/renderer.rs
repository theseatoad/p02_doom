use crate::{components::MainCamera, player::Player};
use bevy::prelude::*;

const CAMERA_HEIGHT: f32 = 1.0;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(update_camera_pos),
        );
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
