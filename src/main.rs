mod text;
mod compiler;
mod instruction;
mod state;
mod gameflow;

use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};
use bevy::ecs::query::WorldQuery;
use bevy::input::keyboard::KeyboardInput;
use bevy::text::Text2dBounds;
use crate::compiler::CompilerPlugin;
use crate::instruction::{Instruction, InstructionEvent};
use crate::state::{Player, PlayerState, StatePlugin};
use crate::text::{ExecuteEvent, TextPlugin};
use bevy::utils::hashbrown::HashMap;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(
            WindowDescriptor {
                title: "OGRE".to_string(),
                width: 640.,
                height: 480.,
                position: WindowPosition::At(Vec2::new(5., 30.)),
                ..default()
            }
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(TextPlugin)
        .add_plugin(CompilerPlugin)
        .add_plugin(StatePlugin)
        .add_startup_system(setup)
        .add_system(gameflow)
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

fn gameflow<P: PlayerState + Component>(
    mut commands: Commands,
    mut execute_evr: EventReader<InstructionEvent>,
    mut player_query: Query<(Entity, &P), With<Player>>,
    mut states: ResMut<HashMap<&'static str, Entity>>,
)
{
    for instruction in execute_evr.iter() {
        println!("got it");
        for (mut player_ent, player_state) in player_query.iter_mut() {
            match instruction.0 {
                Instruction::move_north => {
                    player_state.handle_moov("north", &mut player_ent, &mut commands, &mut states);
                }
                Instruction::move_south => {
                    println!("moving south");
                    player_state.handle_moov("south", &mut player_ent, &mut commands, &mut states);
                }
                Instruction::move_west => {
                    player_state.handle_moov("west", &mut player_ent, &mut commands, &mut states);
                }
                Instruction::move_east => {
                    player_state.handle_moov("east", &mut player_ent, &mut commands, &mut states);
                }
                _ => {}
            }
        }
    }
}