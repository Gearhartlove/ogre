use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::instruction::{Instruction, Direction};
use crate::InstructionEvent;
use crate::text::{LineStart, SayEvent};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(CurrentState { state: Box::new(StateCellar) })
            .add_system(state::<StateCellar>)
            .add_system(state::<StateSewer>)
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_states)
            .add_startup_system_to_stage(StartupStage::Startup, spawn_player);
            // .add_startup_system_to_stage(CoreStage::Last, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut states: ResMut<States>,
    mut ls: ResMut<LineStart>,
) {
    let start = *states.get_mut("cellar").unwrap();
    commands.entity(start).insert(Player);
    ls.location = "cellar";
}

fn setup_states(
    mut commands: Commands,
) {
    let Cellar = commands.spawn()
        .insert(StateCellar) // spawn player
        .insert(Name::new("Cellar"))
        .id();

    let Sewer = commands.spawn()
        .insert(StateSewer)
        .insert(Name::new("Sewer"))
        .id();

    let mut map: HashMap<&str, Entity> = HashMap::new();
    map.insert("cellar", Cellar);
    map.insert("sewer", Sewer);

    commands.insert_resource(map);
}

type States = HashMap<&'static str, Entity>;

fn state<P: PlayerState + Component>(
    mut commands: Commands,
    mut execute_evr: EventReader<InstructionEvent>,
    mut say_writer: EventWriter<SayEvent>,
    mut curr_location_query: Query<(Entity, &P), With<Player>>,
    mut states: ResMut<States>,
    mut line_start: ResMut<LineStart>,
)
{
    for instruction in execute_evr.iter() {
        for (mut curr_location, player_state) in curr_location_query.iter_mut() {
            match instruction.0 {
                Instruction::move_north => {
                    player_state.handle_moov("north", &mut curr_location, &mut commands, &mut states, &mut line_start.location, &mut say_writer);
                }
                Instruction::move_south => {
                    player_state.handle_moov("south", &mut curr_location, &mut commands, &mut states, &mut line_start.location, &mut say_writer);
                }
                Instruction::move_west => {
                    player_state.handle_moov("west", &mut curr_location, &mut commands, &mut states, &mut line_start.location, &mut say_writer);
                }
                Instruction::move_east => {
                    player_state.handle_moov("east", &mut curr_location, &mut commands, &mut states, &mut line_start.location, &mut say_writer);
                }
                Instruction::err => { }
                _ => {}
            }
        }
        break;
    }
}

// #################################################################################################
// States
// #################################################################################################
pub trait PlayerState {
    fn name() -> &'static str;
    fn handle_moov(
        &self,
        direction: &'static str,
        old_pos: &mut Entity,
        commands: &mut Commands,
        mut states: &mut ResMut<States>,
        mut location: &mut &str,
        mut say_writer: &mut EventWriter<SayEvent>,
    ) {}
}

#[derive(Component)]
struct StateCellar;

impl PlayerState for StateCellar {
    fn name() -> &'static str { "cellar" }
    fn handle_moov(
        &self,
        direction: &'static str,
        old_pos: &mut Entity,
        commands: &mut Commands,
        mut states: &mut ResMut<States>,
        mut location: &mut &str,
        mut say_writer: &mut EventWriter<SayEvent>,
    ) {
        match direction {
            "south" => {
                commands.entity(*old_pos).remove::<Player>();
                let next = states.get_mut("sewer").unwrap();
                commands.entity(*next).insert(Player);
                *location = "sewer";

                say_writer.send(SayEvent("\nThe air is dark and damp around you.".to_string()));
            }
            _ => { unimplemented!() } // message saying there is no path
        }
    }
}

#[derive(Component)]
struct StateSewer;

impl PlayerState for StateSewer {
    fn name() -> &'static str { "sewer" }
}