use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_sprite3d::Sprite3dPlugin;
use components::MainCamera;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use renderer::RenderPlugin;
pub mod components;
pub mod enemy;
pub mod player;
pub mod renderer;
pub mod resources;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState {
    Loading,
    Ready,
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(texture_atlas(tile_size_x = 44., tile_size_y = 60.))]
    #[asset(texture_atlas(columns = 7, rows = 1))]
    #[asset(path = "imp.png")]
    imp: Handle<TextureAtlas>,
}

// Defines the amount of time that should elapse between each physics step.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "p02_doom".to_string(),
                        ..default()
                    },
                    ..default()
                })
                .build()
                .set(ImagePlugin::default_nearest()),
        )
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Ready)
                .with_collection::<ImageAssets>(),
        )
        .add_state(GameState::Loading)
        .add_plugin(Sprite3dPlugin)
        // .add_plugin(WorldInspectorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(EnemyPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_system_set(SystemSet::on_enter(GameState::Ready).with_system(setup))
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(MainCamera);
}
