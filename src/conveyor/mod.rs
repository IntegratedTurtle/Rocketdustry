use bevy::ecs::query;
use bevy::prelude::*;

use crate::components::Structure;
use crate::mapsetup::Block;
use crate::structures::random_item;
use crate::structures::AdditionalInformation;
use crate::structures::StructureCreateEvent;
use crate::structures::StructureType;
use crate::structures::WhereFacing;
use std::collections::HashMap;

pub const SIZE: u8 = 1;
pub const SPRITENAME: &str = "sprites/Conveyor.png";
pub const PIXLEVALUE: [u8; 3] = [100, 100, 100];
const STRUCTURETYPE: StructureType = StructureType::Conveyor;

use crate::items::ItemType;
use crate::structures::Facing;
use crate::structures::InputAble;

pub struct ConveyorPlugin;

#[derive(Component)]
struct Conveyor {
    place_0: ItemType,
    place_1: ItemType,
    place_2: ItemType,
    place_3: ItemType,
}

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (make_conveyor, print_items));
    }
}

fn make_conveyor(
    mut commands: Commands,
    mut structure_query: Query<(&mut Transform, &Structure, Entity, &AdditionalInformation)>,
    mut structure_create_event: EventReader<StructureCreateEvent>,
) {
    for _ev in &mut structure_create_event {
        for (mut transform, structure, entity, additional_info) in structure_query.iter_mut() {
            if structure.structure == STRUCTURETYPE {
                let facing = additional_information_to_facing(additional_info.value);
                commands.entity(entity).insert(Conveyor {
                    place_0: ItemType::Nothing,
                    place_1: ItemType::Nothing,
                    place_2: ItemType::Nothing,
                    place_3: ItemType::Nothing,
                });
                rotate_after_facing(additional_info.value, &mut transform);
                commands.entity(entity).insert(InputAble {
                    item: ItemType::Nothing,
                    inputable: get_input_facing(facing),
                });
                commands.entity(entity).insert(WhereFacing { facing });
            }
        }
    }
}

fn rotate_after_facing(value: u8, transform: &mut Transform) {
    let facing = additional_information_to_facing(value);
    match facing {
        Facing::Down => transform.rotate_z(0.0),
        Facing::Up => transform.look_to(
            Vec3::ZERO,
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        ),
        Facing::Left => transform.look_to(
            Vec3::ZERO,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        Facing::Right => transform.look_to(
            Vec3::ZERO,
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
    }
}

fn additional_information_to_facing(value: u8) -> Facing {
    return match value {
        255 => Facing::Down,
        254 => Facing::Up,
        253 => Facing::Left,
        252 => Facing::Right,
        // Default is down
        _ => Facing::Down,
    };
}

fn get_input_facing(facing: Facing) -> Vec<Facing> {
    match facing {
        Facing::Up => vec![Facing::Left, Facing::Right, Facing::Up],
        Facing::Down => vec![Facing::Left, Facing::Right, Facing::Down],
        Facing::Left => vec![Facing::Up, Facing::Left, Facing::Down],
        Facing::Right => vec![Facing::Right, Facing::Up, Facing::Down],
    }
}

fn print_items(query_input: Query<&InputAble, With<Conveyor>>) {
    for inputable in query_input.iter() {
        println!("item = {:?}", inputable.item);
    }
}
