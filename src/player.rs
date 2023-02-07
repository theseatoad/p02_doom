use crate::components::Position;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Player;

const PLAYER_SPEED: f32 = 1.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    position: Position,
}

impl PlayerBundle {
    pub fn new(location: (f32, f32)) -> PlayerBundle {
        PlayerBundle {
            player: Player,
            position: Position {
                x: location.0,
                z: location.1,
            },
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set_to_stage(CoreStage::Update, SystemSet::new().with_system(movement));
    }
}

fn setup(mut commands: Commands) {
    // child camera to player
    commands.spawn(PlayerBundle::new((0.0, 0.0)));
}

fn movement(
    mut player_query: Query<&mut Position, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_pos = player_query
        .get_single_mut()
        .expect("Player not spawned in player movement!");
    if keyboard_input.pressed(KeyCode::W) {
        player_pos.z -= time.delta_seconds() * PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::A) {
        player_pos.x -= time.delta_seconds() * PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::S) {
        player_pos.z += time.delta_seconds() * PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player_pos.x += time.delta_seconds() * PLAYER_SPEED;
    }
}
