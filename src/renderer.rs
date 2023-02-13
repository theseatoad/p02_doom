use crate::{components::MainCamera, player::Player, GameState};
use bevy::{prelude::*, window::CursorGrabMode};

const CAMERA_HEIGHT: f32 = 1.0;

#[derive(Component, Default, Debug)]
pub struct Billboard;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Ready)
                .with_system(update_camera_pos)
                .with_system(billboard_sprites),
        );
    }
}

fn update_camera_pos(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Player should be available for renderer");

    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("Camera should be available for renderer");

    camera_transform.translation = Vec3 {
        x: player_transform.translation.x,
        y: CAMERA_HEIGHT,
        z: player_transform.translation.z,
    };
    camera_transform.rotation = player_transform.rotation;
}

fn billboard_sprites(
    mut sprites_query: Query<&mut Transform, (With<Billboard>, Without<MainCamera>)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Billboard>)>,
) {
    let camera_transform = camera_query
        .get_single()
        .expect("Camera should be available for renderer");
    for mut sprite_transform in sprites_query.iter_mut() {
        let up = sprite_transform.up();
        let sprite_transform_y = sprite_transform.translation.y;
        sprite_transform.look_at(
            Vec3 {
                x: camera_transform.translation.x,
                y: sprite_transform_y,
                z: camera_transform.translation.z,
            },
            up,
        )
    }
}
