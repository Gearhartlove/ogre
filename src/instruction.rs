use crate::room::{Player, get_room_name, Room, Rooms};
use crate::text::{LineStart, SayEvent};
use bevy::prelude::*;

#[derive(Component, Clone)]
/// Cardinal directions for player to travel towards.
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct InstructionEvent(pub InstructionEnum);

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum InstructionEnum {
    look,
    talk,
    moov_south,
    moov_north,
    moov_east,
    moov_west,
    remember,
    inventory,
    character,
    settings,
    loot,
    sleep,
    // util
    clear,
    err,
}

#[derive(Component)]
pub struct InstructionComponent(pub InstructionEnum);

#[derive(Bundle)]
pub struct InstructionBundle {
    pub moov_south: MoovSouth,
    pub moov_east: MoovEast,
    pub moov_west: MoovWest,
    pub moov_north: MoovNorth,
    pub look: Look,
}

impl Default for InstructionBundle {
    fn default() -> Self {
        Self {
            look: Look {
                text: "No description of room written",
            },
            moov_south: MoovSouth {
                room: None,
                text: "There is no path",
            },
            moov_east: MoovEast {
                room: None,
                text: "There is no path",
            },
            moov_west: MoovWest {
                room: None,
                text: "There is no path",
            },
            moov_north: MoovNorth {
                room: None,
                text: "There is no path",
            },
        }
    }
}

//###############################################################
// Instructions
//###############################################################

#[derive(Component)]
pub struct Look {
    pub text: &'static str,
}

impl Look {
    pub fn look(
        &self,
        mut commands: &mut Commands,
        curr_room: &Entity,
        mut say_evw: &mut EventWriter<SayEvent>,
    ) {
        // WRITE TO SCREEN
        say_evw.send(SayEvent(format!("\n{}", self.text.to_string())));
    }
}

#[derive(Component)]
pub struct MoovEast {
    pub room: Option<Room>,
    pub text: &'static str,
}

impl MoovEast {
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
            line_start.location = get_room_name(r); // change location name
            say_evw.send(SayEvent(format!("\n{}", self.text.to_string())));

            // MOVE PLAYER
            commands.entity(*curr_room).remove::<Player>();
            // add player to new room
            let next_room: &mut Entity = rooms.get_mut(r).unwrap();
            commands.entity(*next_room).insert(Player);
        }
    }
}

#[derive(Component)]
pub struct MoovWest {
    pub room: Option<Room>,
    pub text: &'static str,
}

impl MoovWest {
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
            say_evw.send(SayEvent(format!("\n{}", self.text.to_string())));
            line_start.location = get_room_name(r); // change location name

            // MOVE PLAYER
            commands.entity(*curr_room).remove::<Player>();
            // add player to new room
            let next_room: &mut Entity = rooms.get_mut(r).unwrap();
            commands.entity(*next_room).insert(Player);
        }
    }
}

#[derive(Component)]
pub struct MoovNorth {
    pub room: Option<Room>,
    pub text: &'static str,
}

impl MoovNorth {
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
            say_evw.send(SayEvent(format!("\n{}", self.text.to_string())));
            line_start.location = get_room_name(r); // change location name

            // MOVE PLAYER
            commands.entity(*curr_room).remove::<Player>();
            // add player to new room
            let next_room: &mut Entity = rooms.get_mut(r).unwrap();
            commands.entity(*next_room).insert(Player);
        }
    }
}

#[derive(Component)]
pub struct MoovSouth {
    pub room: Option<Room>,
    pub text: &'static str,
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
            say_evw.send(SayEvent(format!("\n{}", self.text.to_string())));
            line_start.location = get_room_name(r); // change location name

            // MOVE PLAYER
            commands.entity(*curr_room).remove::<Player>();
            // add player to new room
            let next_room: &mut Entity = rooms.get_mut(r).unwrap();
            commands.entity(*next_room).insert(Player);
        }
    }
}
