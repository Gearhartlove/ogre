use bevy::prelude::*;

pub struct InstructionEvent(pub Instruction);


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
pub enum Instruction {
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

