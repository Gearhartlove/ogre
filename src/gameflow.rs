use crate::instruction::*;
use crate::room::{get_room_name, setup_rooms, Room, Rooms, Player};
use crate::text::{LineStart, SayEvent};
use bevy::prelude::*;

pub struct GameflowPlugin;

impl Plugin for GameflowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rooms::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_rooms)
            .add_startup_system_to_stage(StartupStage::Startup, spawn_player)
            .add_system(update_game);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut rooms: ResMut<Rooms>,
    mut line_start: ResMut<LineStart>,
) {
    let start = &Room::Cellar;

    line_start.location = get_room_name(start);

    let room = rooms.get_mut(start).unwrap();
    commands.entity(*room).insert(Player);
}

fn update_game(
    mut commands: Commands,
    mut say_evw: EventWriter<SayEvent>,
    mut rooms: ResMut<Rooms>,
    mut line_start: ResMut<LineStart>,
    mut instruction_query: Query<(&InstructionComponent, Entity, &MoovSouth, &MoovNorth, &MoovEast, &MoovWest, &Look), With<Player>>,
) {
    if let Ok((instruction, curr_room, moov_south, moov_north, moov_east, moov_west, look)) = instruction_query.get_single_mut() {
        match instruction.0 {
            InstructionEnum::moov_south => {
                moov_south.moov(
                    &mut commands,
                    &curr_room,
                    rooms,
                    &mut say_evw,
                    &mut line_start,
                );
            },
            InstructionEnum::moov_north => {
                moov_north.moov(
                    &mut commands,
                    &curr_room,
                    rooms,
                    &mut say_evw,
                    &mut line_start,
                );
            },
            InstructionEnum::moov_east => {
                moov_east.moov(
                    &mut commands,
                    &curr_room,
                    rooms,
                    &mut say_evw,
                    &mut line_start,
                );
            },
            InstructionEnum::moov_west => {
                moov_west.moov(
                    &mut commands,
                    &curr_room,
                    rooms,
                    &mut say_evw,
                    &mut line_start,
                );
            },
            InstructionEnum::look => {
                look.description(&mut say_evw);
            }
            _ => { panic!("Instruction not added to update_game in gameflow.rs")}
        }
        // remove instruction component from the room
        commands.entity(curr_room).remove::<InstructionComponent>();
    }
}
