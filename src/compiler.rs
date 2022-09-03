use std::collections::VecDeque;
use bevy::prelude::*;
use crate::commands::Instruction;
use crate::text::{CurrentLine, ExecuteEvent, MainText};

pub struct CompilerPlugin;

impl Plugin for CompilerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::PostUpdate, listen_execute);
    }
}

fn listen_execute(
    mut exe_evr: EventReader<ExecuteEvent>,
    mut txt_query: Query<&mut Text, With<MainText>>,
    mut curr_line: ResMut<CurrentLine>,
) {
    for exe in exe_evr.iter() {
        if let Ok(text) = txt_query.get_single_mut() {
            let tokens = tokenize(&text.sections[curr_line.0].value);
            evaluate(tokens);
            dbg!(tokens);


            curr_line.0 += 1;
        }
    }
}

fn tokenize(s: &String) -> Vec<Instruction> {
    let mut tokens: Vec<Instruction> = Vec::new();
    let mut split: VecDeque<&str> = s.split(' ').collect();
    remove_prompt(&mut split);
    for command in split.iter() {
        match *command {
            "clear" => { tokens.push(Instruction::clear) },
            _ => { tokens.push(Instruction::err) }
        }
    }
    tokens
}

fn remove_prompt(split: &mut VecDeque<&str>) {
    split.pop_front();
    split.pop_front();
    split.pop_front();
}

fn evaluate(tokens: Vec<Instruction>) {
    for t in tokens.iter() {
        match t {
            Instruction::clear => {

            }
            Instruction::err => {}
        }
    }
}