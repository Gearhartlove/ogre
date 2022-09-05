use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::instruction::{Instruction, Direction};
use crate::InstructionEvent;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(CurrentState { state: Box::new(StateCellar) })
            .add_system(state::<StateCellar>)
            .add_system(state::<StateSewer>)
            .add_startup_system(setup_states);
    }
}

#[derive(Component)]
pub struct Player;

// pub struct CurrentState {
//     pub state: Box<dyn PlayerState + Send + Sync>,
// }

fn setup_states(
    mut commands: Commands,
) {
    let Cellar = commands.spawn()
        .insert(StateCellar)
        .insert(Player)
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

    // let state_store = CurrentState {
    //     current_state: Box::new(StateCellar),
    //     //map
    // };

    //commands.insert_resource(state_store);
}

fn state<P: PlayerState + Component>(
    mut commands: Commands,
    mut execute_evr: EventReader<InstructionEvent>,
    mut player_query: Query<(Entity, &P), With<Player>>,
    mut states: ResMut<HashMap<&'static str, Entity>>,
)
{
    for instruction in execute_evr.iter() {
        println!("got it");
        for (mut player_ent, player_state) in player_query.iter_mut() {
            match instruction.0 {
                Instruction::move_north => {
                    player_state.handle_moov("north", &mut player_ent, &mut commands, &mut states);
                }
                Instruction::move_south => {
                    player_state.handle_moov("south", &mut player_ent, &mut commands, &mut states);
                }
                Instruction::move_west => {
                    player_state.handle_moov("west", &mut player_ent, &mut commands, &mut states);
                }
                Instruction::move_east => {
                    player_state.handle_moov("east", &mut player_ent, &mut commands, &mut states);
                }
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
    fn handle_moov(&self, direction: &'static str, old_pos: &mut Entity, commands: &mut Commands, mut states: &mut ResMut<HashMap<&'static str, Entity>>) {}
}

#[derive(Component)]
struct StateCellar;

impl PlayerState for StateCellar {
    fn handle_moov(&self, direction: &'static str, old_pos: &mut Entity, commands: &mut Commands, mut states: &mut ResMut<HashMap<&'static str, Entity>>) {
        let move_player = |commands: &mut Commands| {
            // commands.entity()
        };

        match direction{
            "south" => {
                println!("going to sewer");
                commands.entity(*old_pos).remove::<Player>();
                let next = states.get_mut("sewer").unwrap();
                commands.entity(*next).insert(Player);
            }
            _ => { unimplemented!() } // message saying there is no path
        }

    }
}


#[derive(Component)]
struct StateSewer;

impl PlayerState for StateSewer {}