use bevy::prelude::*;
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
            // get the current line
            println!("{}", text.sections[curr_line.0].value);
            curr_line.0 += 1;
        }
    }
}