use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::Player;
use crate::text::{LineStart, SayEvent};

pub struct GameflowPlugin;

impl Plugin for GameflowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Rooms::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_rooms)
            .add_startup_system_to_stage(StartupStage::Startup, spawn_player)
            .add_system(update_game);
    }
}

pub struct InstructionEvent(pub InstructionEnum);

#[derive(Bundle)]
pub struct InstructionBundle {
    moov_south: MoovSouth,
}

impl Default for InstructionBundle {
    fn default() -> Self {
        Self {
            moov_south: MoovSouth {
                room: None,
                text: "There is no path"
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Room {
    Sewer,
    Cellar,
}

pub fn get_room_name(room: &Room) -> &'static str {
    return match room {
        &Room::Cellar => { "cellar" },
        &Room::Sewer => { "sewer" },
    }
}

type Rooms = HashMap<Room, Entity>;

fn setup_rooms(
    mut commands: Commands,
    mut rooms: ResMut<Rooms>,
) {
    let cellar: Entity = commands.spawn()
        .insert_bundle(InstructionBundle {
            moov_south: MoovSouth {
                room: Some(Room::Sewer),
                text: "You move from the cellar to the sewer going south"
            }
        })
        .insert(Name::new("Cellar"))
        .id();

    let sewer: Entity = commands.spawn()
        .insert_bundle(InstructionBundle::default())
        .insert(Name::new("Sewer"))
        .id();

    rooms.insert(Room::Cellar, cellar);
    rooms.insert(Room::Sewer, sewer);
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

#[derive(Component)]
pub struct MoovSouth {
    room: Option<Room>,
    text: &'static str,
}

impl MoovSouth {
    pub fn moov(
        &self,
        mut commands: &mut Commands,
        curr_room: &Entity,
        mut rooms: ResMut<Rooms>,
        mut say_evw: &mut EventWriter<SayEvent>,
        mut line_start: &mut ResMut<LineStart>,
    ) {
        // remove player from current room
        if let Some(r) = &self.room {
            /// WRITE TO SCREEN
            say_evw.send(SayEvent(format!("\n{}",self.text.to_string())));
            line_start.location = get_room_name(r); // change location name
            dbg!(r);
            dbg!(get_room_name(r));
            say_evw.send(SayEvent(format!("\n{}@ogre {} {}", line_start.user, line_start.location, line_start.prompt)));

            // MOVE PLAYER
            commands.entity(*curr_room).remove::<Player>();
            // add player to new room
            let next_room: &mut Entity = rooms.get_mut(r).unwrap();
            commands.entity(*next_room).insert(Player);
        }
    }
}

fn update_game(
    mut commands: Commands,
    mut rooms: ResMut<Rooms>,
    mut instruction_query: Query<(&InstructionComponent, Entity, &MoovSouth), With<Player>>,
    mut say_evw: EventWriter<SayEvent>,
    mut line_start: ResMut<LineStart>,
) {
    if let Ok((instruction, curr_room, moov_south)) = instruction_query.get_single_mut() {
        match instruction.0 {
            InstructionEnum::move_south => {
                moov_south.moov(&mut commands, &curr_room, rooms, &mut say_evw, &mut line_start);
            }
            _ => {}
        }
    // remove instruction component from the room
    commands.entity(curr_room).remove::<InstructionComponent>();
    }
}

#[derive(Component, Clone)]
/// Cardinal directions for player to travel towards.
pub enum Direction {
    North,
    South,
    East,
    West
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum InstructionEnum {
    look,
    talk,
    move_south,
    move_north,
    move_east,
    move_west,
    remember,
    inventory,
    character,
    settings,
    loot,
    sleep,
    // util
    clear,
    err
}

#[derive(Component)]
pub struct InstructionComponent(pub InstructionEnum);

