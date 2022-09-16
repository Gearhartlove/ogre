use crate::instruction::Direction;
use crate::instruction::{InstructionComponent, InstructionEnum, InstructionEvent};
use crate::text::{CurrentLine, ExecuteEvent, MainText};
use crate::room::Player;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct CompilerPlugin;

impl Plugin for CompilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstructionEvent>()
            .add_system_to_stage(CoreStage::PostUpdate, handle_user_input);
    }
}

fn handle_user_input(
    mut commands: Commands,
    mut exe_evr: EventReader<ExecuteEvent>,
    mut txt_query: Query<&mut Text, With<MainText>>,
    mut curr_room: Query<Entity, With<Player>>,
    mut instruction_evw: EventWriter<InstructionEvent>,
    mut curr_line: Res<CurrentLine>,
) {
    for exe in exe_evr.iter() {
        if let Ok(text) = txt_query.get_single_mut() {
            if let Ok(room) = curr_room.get_single_mut() {
                let tokens = tokenize(&text.sections[curr_line.0].value);
                for instruction in tokens {
                    dbg!(instruction.clone());
                    commands
                        .entity(room)
                        .insert(InstructionComponent(instruction));
                }
            }
        }
    }
}

fn tokenize(s: &String) -> Vec<InstructionEnum> {
    let mut tokens: Vec<InstructionEnum> = Vec::new();
    let mut split: VecDeque<&str> = s.split(' ').collect();
    remove_prompt(&mut split);
    dbg!(split.clone());
    let mut split = split.into_iter().peekable();
    loop {
        match split.next() {
            None => break,
            Some(next) => match next {
                // TODO: define possible additional charater info
                "clear" => tokens.push(InstructionEnum::clear),
                "sleep" => tokens.push(InstructionEnum::sleep),
                "loot" => tokens.push(InstructionEnum::loot),
                "settings" => tokens.push(InstructionEnum::settings),
                "character" => tokens.push(InstructionEnum::character),
                "inventory" => tokens.push(InstructionEnum::inventory),
                "remember" => tokens.push(InstructionEnum::remember),
                "talk" => tokens.push(InstructionEnum::talk),
                "look" => tokens.push(InstructionEnum::look),
                "move" => {
                    if let Some(direction) = split.peek() {
                        if *direction == "south" {
                            split.next();
                            tokens.push(InstructionEnum::moov_south)
                        }

                        if let Some(direction) = split.peek() {
                            if *direction == "north" {
                                split.next();
                                tokens.push(InstructionEnum::moov_north)
                            }
                        }
                        if let Some(direction) = split.peek() {
                            if *direction == "east" {
                                split.next();
                                tokens.push(InstructionEnum::moov_east)
                            }
                        }
                        if let Some(direction) = split.peek() {
                            if *direction == "west" {
                                split.next();
                                tokens.push(InstructionEnum::moov_west)
                            }
                        }
                    }
                }
                _ => tokens.push(InstructionEnum::err),
            },
        }
    }
    tokens
}

fn remove_prompt(split: &mut VecDeque<&str>) {
    split.pop_front();
    split.pop_front();
    split.pop_front();
}
