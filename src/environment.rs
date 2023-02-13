use bevy::prelude::*;
use bevy_sprite3d::{AtlasSprite3d, AtlasSprite3dComponent, Sprite3dParams};

use crate::{enemy::AnimationTimer, renderer::Billboard, GameState, ImageAssets};
#[derive(Component, Default, Debug)]
pub struct Enemy;

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Ready).with_system(spawn_lights));
    }
}

fn spawn_lights(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    // Spawn red lights in all four corners
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.torch_red.clone(),

                pixels_per_metre: 105.,
                partial_alpha: true,
                unlit: true,

                index: 3,

                transform: Transform::from_xyz(7.5, 1.0, 7.5).with_scale(Vec3::splat(2.0)),
                pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(AnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert(Billboard);
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.torch_red.clone(),

                pixels_per_metre: 105.,
                partial_alpha: true,
                unlit: true,

                index: 3,

                transform: Transform::from_xyz(-7.5, 1.0, 7.5).with_scale(Vec3::splat(2.0)),
                pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(AnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert(Billboard);
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.torch_red.clone(),

                pixels_per_metre: 105.,
                partial_alpha: true,
                unlit: true,

                index: 3,

                transform: Transform::from_xyz(-7.5, 1.0, -7.5).with_scale(Vec3::splat(2.0)),
                pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(AnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert(Billboard);
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.torch_red.clone(),

                pixels_per_metre: 105.,
                partial_alpha: true,
                unlit: true,

                index: 3,

                transform: Transform::from_xyz(7.5, 1.0, -7.5).with_scale(Vec3::splat(2.0)),
                pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(AnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert(Billboard);
}
