use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_sprite3d::Sprite3dPlugin;
use components::MainCamera;
use enemy::EnemyPlugin;
use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use renderer::RenderPlugin;
pub mod components;
pub mod enemy;
pub mod environment;
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
    #[asset(texture_atlas(tile_size_x = 40., tile_size_y = 105.))]
    #[asset(texture_atlas(columns = 4, rows = 1))]
    #[asset(path = "torch_red.png")]
    torch_red: Handle<TextureAtlas>,
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
        .add_plugin(EnvironmentPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_system_set(SystemSet::on_enter(GameState::Ready).with_system(setup))
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_material = StandardMaterial {
        base_color: Color::PURPLE,
        metallic: 0.0,
        reflectance: 0.0,
        ..default()
    };

    let wall_material = StandardMaterial {
        base_color: Color::PURPLE,
        metallic: 0.0,
        reflectance: 0.0,
        ..default()
    };
    let mat_handle = materials.add(wall_material);
    let floor_mat_handle = materials.add(floor_material);
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: floor_mat_handle.clone(),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0 })),
        material: mat_handle.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
            rotation: Quat::from_rotation_x(1.570796),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0 })),
        material: mat_handle.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
            rotation: Quat::from_rotation_x(-1.570796),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0 })),
        material: mat_handle.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quat::from_rotation_z(1.570796),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0 })),
        material: mat_handle.clone(),
        transform: Transform {
            translation: Vec3 {
                x: -10.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quat::from_rotation_z(-1.570796),
            ..default()
        },
        ..Default::default()
    });

    // ==== LIGHTS ====
    // RED LIGHT
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(7.5, 0.5, 7.5),
            point_light: PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::RED,
                shadows_enabled: false,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(-7.5, 0.5, 7.5),
            point_light: PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::RED,
                shadows_enabled: false,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(-7.5, 0.5, -7.5),
            point_light: PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::RED,
                shadows_enabled: false,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(7.5, 0.5, -7.5),
            point_light: PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::RED,
                shadows_enabled: false,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(MainCamera);
}
