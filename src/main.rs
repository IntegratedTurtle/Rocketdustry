mod components;
use bevy::{prelude::*, utils::petgraph::csr::Neighbors, window::PrimaryWindow};
use components::{EnviromentBlock, HashSetFloat, TestValue};
use std::collections::HashSet;
use umath::FF32;

pub const TEXTURESIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GetNeighbours>()
        .add_systems(Startup, (spawn_blocks, spawn_camera, get_neighbours))
        .add_systems(Update, (get_neighbours, print_values, add_values))
        .run()
}

#[derive(Event)]
pub struct GetNeighbours(u32);

pub fn spawn_blocks(
    mut get_neigbour_event: EventWriter<GetNeighbours>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let x_max: usize = 3;
    let y_max: usize = 3;
    for x in 0..x_max {
        for y in 0..y_max {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        window.width() / 2.0 + (x as f32 * TEXTURESIZE),
                        window.height() / 2.0 + (y as f32 * TEXTURESIZE),
                        0.0,
                    ),
                    texture: asset_server.load("sprites/Gras.png"),
                    ..default()
                },
                EnviromentBlock {
                    location: HashSetFloat {
                        x: unsafe { FF32::new(x as f32) },
                        y: unsafe { FF32::new(y as f32) },
                    },
                    block: components::Block::Gras,
                    neighbour: EnviromentBlock::get_neighbours(
                        unsafe { FF32::new(x as f32) },
                        unsafe { FF32::new(y as f32) },
                        unsafe { FF32::new(x_max as f32) },
                        unsafe { FF32::new(y_max as f32) },
                    ),
                },
                TestValue { value: 3 },
            ));
        }
    }
    get_neigbour_event.send(GetNeighbours(45));
}
// ! How to send Information
pub fn get_neighbours(
    // value_query: Query<&mut TestValue,With<EnviromentBlock>>,
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

pub fn add_values(
    // value_query: Query<&TestValue>,
    mut other_value_query: Query<&mut TestValue>,
) {
    for value in other_value_query.iter_combinations_mut() {
        for mut other_value in other_value_query.iter_mut() {
            other_value.value += value.value;
        }
    }
}

pub fn print_values(test_value: Query<&TestValue>) {
    for value in test_value.iter() {
        println!("{}", value.value);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
