use bevy::core_pipeline::bloom;
use bevy::prelude::*;
use rand::Rng;

use crate::components::Structure;
use crate::mapsetup::Block;
use crate::structures::random_item;
use crate::structures::StructureCreateEvent;
use crate::structures::StructureType;
use std::collections::HashMap;

pub const SIZE: u8 = 2;
pub const SPRITENAME: &str = "sprites/Drill.png";
pub const PIXLEVALUE: [u8; 3] = [56, 56, 56];
const STRUCTURETYPE: StructureType = StructureType::Drill;

const DRILLSPEED_STONE: f32 = 0.3;
const DRILLSPEED_MUD: f32 = 0.5;

pub struct DrillPlugin;

#[derive(Component)]
struct Drill {}

impl Plugin for DrillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (make_drill, drill));
    }
}
// Gets all Struct, that are no specific Block and if the are an drill, add the Tag, so I can be found easier
fn make_drill(
    mut commands: Commands,
    structure_query: Query<(&Structure, Entity)>,
    mut structure_create_event: EventReader<StructureCreateEvent>,
) {
    for _ev in &mut structure_create_event {
        for (structure, entity) in structure_query.iter() {
            if structure.structure == STRUCTURETYPE {
                commands.entity(entity).insert(Drill {});
            }
        }
    }
}

#[rustfmt::skip]
fn drill(drills_query: Query<&Structure, With<Drill>>, time: Res<Time>) {
    for drill in drills_query.iter() {
        // Get the most used Block
        let mut enviroment_blocks = HashMap::new();
        for env_block in drill.enviroment_block_under.iter() {
            if !enviroment_blocks.contains_key(env_block) {
                enviroment_blocks.insert(env_block, 1);
            } else {
                if let Some(value) = enviroment_blocks.get_mut(env_block) {
                    *value += 1; // Increment the value by one
                }
            }
        }
        // get which Block is with how mutch the most prominent
        let (max_key, max_value) = enviroment_blocks
            .iter()
            .max_by_key(|&(_, value)| value)
            .map(|(key, value)| (key, *value))
            .unwrap_or((&&Block::Nothing, 0));
        // If the block is not mineable, by the drill, it should break out of the loop
        for _ in 0..max_value {
            match max_key {
                Block::Stone => if random_item(DRILLSPEED_STONE * time.delta_seconds()) {println!("STONE")},
                Block::Mud => if random_item(DRILLSPEED_MUD * time.delta_seconds()) {println!("MUD")},
                _ => break,
            }
            
        }
    }
}
