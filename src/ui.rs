use bevy::prelude::*;

use crate::{GameState, ImageAssets};

#[derive(Component, Deref, DerefMut)]
pub struct GunAnimationTimer(pub Timer);
pub struct GunUIPlugin;
impl Plugin for GunUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Ready).with_system(animate_shooting))
            .add_system_set(SystemSet::on_enter(GameState::Ready).with_system(setup));
    }
}

fn animate_shooting(
    time: Res<Time>,
    mut query: Query<(&mut GunAnimationTimer, &mut UiImage)>,
    images: Res<ImageAssets>,
) {
    for (mut timer, mut image) in query.iter_mut() {
        if timer.elapsed_secs() == 0.0 {
            image.0 = images.pistol_shoot.clone().into();
        }
        timer.tick(time.delta());
    }
}

fn setup(mut commands: Commands, images: Res<ImageAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(400.0)),
                    ..default()
                },
                image: images.pistol_idle.clone().into(),
                ..default()
            });
        })
        .insert(GunAnimationTimer(Timer::from_seconds(0.5, TimerMode::Once)));
}
