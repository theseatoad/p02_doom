use bevy::prelude::*;
#[derive(Component, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct MainCamera;
