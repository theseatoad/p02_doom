use bevy::prelude::*;
use components::MainCamera;
use player::PlayerPlugin;
use renderer::RenderPlugin;
use resources::WINDOWSIZE;
pub mod components;
pub mod player;
pub mod renderer;
pub mod resources;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "p02_doom".to_string(),
                        width: WINDOWSIZE.0,
                        height: WINDOWSIZE.1,
                        ..default()
                    },
                    ..default()
                })
                .build()
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RenderPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.at_start())
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
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(MainCamera);
}
