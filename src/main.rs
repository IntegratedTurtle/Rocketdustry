mod camera;
mod components;
pub mod drill;
mod ingameui;
mod mapsetup;
mod player;
mod resources;
mod structures;
use bevy::{prelude::*, utils::petgraph::csr::Neighbors, window::PrimaryWindow};
use camera::{move_camera, zoom_out_camera, CameraScale, CameraView};
use components::{EnviromentBlock, HashSetFloat, TestValue};
use drill::DrillPlugin;
use mapsetup::MapAsPng;
use player::PlayerSpawnInfo;
use resources::EnviromentEntities;
use resources::MudResource;
use resources::StoneResource;
use resources::StructureEntities;
use std::collections::HashSet;
use structures::StructureCreateEvent;
use structures::StructuresAsPng;
use umath::FF32;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DrillPlugin))
        .add_event::<GetNeighbours>()
        .init_resource::<CameraView>()
        .init_resource::<CameraScale>()
        .init_resource::<MapAsPng>()
        .init_resource::<StructuresAsPng>()
        .init_resource::<PlayerSpawnInfo>()
        .init_resource::<StoneResource>()
        .init_resource::<MudResource>()
        .init_resource::<EnviromentEntities>()
        .init_resource::<StructureEntities>()
        .add_event::<StructureCreateEvent>()
        .add_systems(
            Startup,
            (
                mapsetup::spawn_blocks,
                camera::spawn_camera,
                get_neighbours,
                player::spawn_player,
                ingameui::spawn_ingame_ui,
                structures::spawn_structures_from_map.after(mapsetup::spawn_blocks),
            ),
        )
        .add_systems(
            Update,
            (
                get_neighbours,
                player::player_movement,
                player::camera_follow_player.after(player::player_movement),
                ingameui::update_stone_ui,
                ingameui::update_mud_ui,
                // find_all_structures,
                // player::player_sprite_rotate,
            ),
        )
        .run()
}

#[derive(Event)]
pub struct GetNeighbours(u32);

// ! How to send Information
// ! There might be entity id's, this will have to be investigated
pub fn get_neighbours(
    // value_query: Query<&mut TestValue,With<EnvironmentBlock>>,
    block_query: Query<&EnviromentBlock>,
    mut neighbour_query: Query<(&EnviromentBlock, &mut TestValue)>,
    mut event_reader: EventReader<GetNeighbours>,
) {
    for ev in event_reader.iter() {
        for block in block_query.iter() {
            for (pot_neigh, mut test_value) in neighbour_query.iter_mut() {
                if block.neighbour.contains(&pot_neigh.location) {
                    test_value.value += 3;
                }
            }
        }
    }
}

fn find_all_structures(
    structure_entities: Res<resources::StructureEntities>,
    entitiy_query: Query<&Transform>,
) {
    println!(
        "{:?}",
        match structure_entities.map.get(&HashSetFloat {
            x: unsafe { FF32::new(4.0) },
            y: unsafe { FF32::new(2.0) }
        }) {
            Some(entity) => entitiy_query.get(*entity),
            None => panic!("WAAAAAAAAA"),
        }
    );
}

// pub fn add_values(
//     // value_query: Query<&TestValue>,
//     mut other_value_query: Query<&mut TestValue>,
// ) {
//     for value in other_value_query.iter_combinations_mut() {
//         for mut other_value in other_value_query.iter_mut() {
//             other_value.value += value.value;
//         }
//     }
// }

pub fn print_values(test_value: Query<&TestValue>) {
    for value in test_value.iter() {
        println!("{}", value.value);
    }
}
