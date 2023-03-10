use crate::GameState;
/// Inspiration from https://github.com/sburris0/bevy_flycam/
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
#[derive(Component, Default, Debug)]
pub struct Player;

const PLAYER_SPEED: f32 = 5.0;
const PLAYER_ROTATION_SPEED: f32 = 0.001;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    y_rot: f32,
}

#[derive(Component)]
struct Shooting();

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    transform: Transform,
}

impl PlayerBundle {
    pub fn new(location: (f32, f32)) -> PlayerBundle {
        PlayerBundle {
            player: Player,
            transform: Transform {
                translation: Vec3 {
                    x: location.0,
                    y: 0.0,
                    z: location.1,
                },
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::on_update(GameState::Ready)
                    .with_system(player_movement)
                    .with_system(mouse_look)
                    .with_system(cursor_grab),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Ready)
                    .with_system(setup)
                    .with_system(initial_grab_cursor),
            );
    }
}

fn setup(mut commands: Commands) {
    // child camera to player
    commands.spawn(PlayerBundle::new((0.0, 0.0)));
}

fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player_query
        .get_single_mut()
        .expect("Player not spawned in 'player_movement'!");

    let mut velocity = Vec3::ZERO;
    let local_z = player_transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);
    if keyboard_input.pressed(KeyCode::W) {
        velocity += forward;
    }
    if keyboard_input.pressed(KeyCode::A) {
        velocity -= right;
    }
    if keyboard_input.pressed(KeyCode::S) {
        velocity -= forward;
    }
    if keyboard_input.pressed(KeyCode::D) {
        velocity += right;
    }
    velocity = velocity.normalize_or_zero();
    player_transform.translation += velocity * time.delta_seconds() * PLAYER_SPEED
}

fn mouse_look(
    motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    mut player_query: Query<&mut Transform, With<Player>>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        if window.cursor_grab_mode() != CursorGrabMode::None {
            let mut player_transform = player_query
                .get_single_mut()
                .expect("Player not spawned in player 'player_look'!");
            let mut delta_state = state.as_mut();
            for ev in delta_state.reader_motion.iter(&motion) {
                let window_scale = window.height().min(window.width());
                delta_state.y_rot -=
                    (PLAYER_ROTATION_SPEED * ev.delta.x * window_scale).to_radians();
                // Order is important to prevent unintended roll
                player_transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.y_rot);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
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
