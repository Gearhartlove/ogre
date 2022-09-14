use crate::instruction::*;
use crate::text::LineStart;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Room {
    Sewer,
    Cellar,
    Storage,
    Stairway,
}

pub fn get_room_name(room: &Room) -> &'static str {
    return match room {
        &Room::Cellar => "cellar",
        &Room::Sewer => "sewer",
        &Room::Storage => "storage",
        &Room::Stairway => "stairway",
    };
}

pub type Rooms = HashMap<Room, Entity>;

#[derive(Component)]
pub struct Player;

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

pub fn setup_rooms(mut commands: Commands, mut rooms: ResMut<Rooms>) {
    let cellar: Entity = commands
        .spawn()
        .insert_bundle(InstructionBundle {
            look: Look {
                text: "A hard coble curls your toes as you look around the dark room.",
            },
            moov_south: MoovSouth {
                room: Some(Room::Sewer),
                text: "You move from the cellar to the sewer going south",
            },
            moov_west: MoovWest {
                room: Some(Room::Storage),
                text: "Moving west to storage",
            },
            ..default()
        })
        .insert(Name::new("Cellar"))
        .id();

    let sewer: Entity = commands
        .spawn()
        .insert_bundle(InstructionBundle {
            look: Look {
                text: "The air is putrid and trash litters the wet cramped area around you.",
            },
            moov_north: MoovNorth {
                room: Some(Room::Cellar),
                text: "Moving North to cellar",
            },
            moov_west: MoovWest {
                room: Some(Room::Stairway),
                text: "Moving West to stairway",
            },
            ..default()
        })
        .insert(Name::new("Sewer"))
        .id();

    let storage: Entity = commands
        .spawn()
        .insert_bundle(InstructionBundle {
            look: Look {
                text: "description for storage",
            },
            moov_east: MoovEast {
                room: Some(Room::Cellar),
                text: "Moving east to Cellar",
            },
            moov_south: MoovSouth {
                room: Some(Room::Stairway),
                text: "Moving south to Stairway",
            },
            ..default()
        })
        .insert(Name::new("Storage"))
        .id();

    let stairway: Entity = commands
        .spawn()
        .insert_bundle(InstructionBundle {
            look: Look {
                text: "description for stairway",
            },
            moov_east: MoovEast {
                room: Some(Room::Sewer),
                text: "Moving east to Sewer",
            },
            moov_north: MoovNorth {
                room: Some(Room::Storage),
                text: "Moving south to Storage",
            },
            ..default()
        })
        .insert(Name::new("Stairway"))
        .id();

    rooms.insert(Room::Cellar, cellar);
    rooms.insert(Room::Sewer, sewer);
    rooms.insert(Room::Storage, storage);
    rooms.insert(Room::Stairway, stairway);
}
