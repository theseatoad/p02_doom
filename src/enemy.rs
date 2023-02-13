use bevy::prelude::*;
use bevy_sprite3d::{AtlasSprite3d, AtlasSprite3dComponent, Sprite3dParams};

use crate::{renderer::Billboard, GameState, ImageAssets};
#[derive(Component, Default, Debug)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Ready).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_update(GameState::Ready).with_system(animate_sprite));
    }
}

fn spawn_enemy(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands
        .spawn(
            AtlasSprite3d {
                atlas: images.imp.clone(),

                pixels_per_metre: 60.,
                partial_alpha: true,
                unlit: true,

                index: 3,

                transform: Transform::from_xyz(7.0, 0.5, 7.),
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

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut AtlasSprite3dComponent)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % sprite.atlas.len();
        }
    }
}
