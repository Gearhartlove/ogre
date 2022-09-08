use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::Player;

pub struct GameflowPlugin;

impl Plugin for GameflowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Rooms::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_rooms)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player)
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

#[derive(Eq, Hash, PartialEq)]
pub enum Room {
    Sewer,
    Cellar
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
) {
    let start = &Room::Cellar;
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
        mut commands: Commands,
        curr_room: &Entity,
        mut rooms: ResMut<Rooms>,
        // Res<ScreenText> ?
    ) {
        // write to screen
        // remove player from current room
        if let Some(r) = &self.room {
            commands.entity(*curr_room).remove::<Player>();
            // add player to new room
            let next_room: &mut Entity = rooms.get_mut(r).unwrap();
            commands.entity(*next_room).insert(Player);
        }
    }
}

fn update_game(
    mut commands: Commands,
    mut instruction_event_reader: EventReader<InstructionEvent>,
    // Res<ScreenText> ?
    mut rooms: ResMut<Rooms>,
    mut instruction_query: Query<(&InstructionComponent, Entity, &MoovSouth), With<Player>>,
) {
    if let Ok((instruction, curr_room, moov_south)) = instruction_query.get_single_mut() {
        match instruction.0 {
            InstructionEnum::move_south => {
                moov_south.moov(commands, &curr_room, rooms);
            }
            _ => {}
        }
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

