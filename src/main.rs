mod compiler;
mod gameflow;
mod instruction;
mod room;
mod state;
mod text;

use crate::compiler::CompilerPlugin;
use crate::gameflow::GameflowPlugin;
use crate::text::{ExecuteEvent, TextPlugin};
use bevy::ecs::query::WorldQuery;
use bevy::input::keyboard::KeyboardInput;
use bevy::text::Text2dBounds;
use bevy::utils::hashbrown::HashMap;
use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "OGRE".to_string(),
            width: 640.,
            height: 480.,
            position: WindowPosition::At(Vec2::new(5., 30.)),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(TextPlugin)
        .add_plugin(CompilerPlugin)
        .add_plugin(GameflowPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(3.4, 1.2, 0.).looking_at(Vec3::new(0., 1.2, 0.), Vec3::Y),
        ..default()
    });
    // tv
    commands.spawn_bundle(SceneBundle {
        scene: ass.load("3d/tv.gltf#Scene0"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 10.0, 0.0),
        ..default()
    });
}
