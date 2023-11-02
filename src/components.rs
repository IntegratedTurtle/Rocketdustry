use crate::mapsetup::Block;
use crate::structures::StructureType;
use bevy::prelude::*;
use std::collections::HashSet;
use umath::FF32;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct HashSetFloat {
    pub x: FF32,
    pub y: FF32,
}

#[derive(Component, Debug, PartialEq)]
pub struct EnviromentBlock {
    pub location: HashSetFloat,
    pub block: Block,
    pub neighbour: HashSet<HashSetFloat>,
}

impl EnviromentBlock {
    pub fn get_neighbours(x: FF32, y: FF32, x_max: FF32, y_max: FF32) -> HashSet<HashSetFloat> {
        let mut outputvec = HashSet::with_capacity(4);

        if x > 0.0 {
            outputvec.insert(HashSetFloat { x: x - 1.0, y });
        }
        if y > 0.0 {
            outputvec.insert(HashSetFloat { x, y: y - 1.0 });
        }
        if x_max > x + 1.0 {
            outputvec.insert(HashSetFloat { x: x + 1.0, y });
        }
        if y_max > y + 1.0 {
            outputvec.insert(HashSetFloat { x, y: y + 1.0 });
        }
        return outputvec;
    }
}

#[derive(Component, Debug, PartialEq)]
pub struct Structure {
    pub location: HashSetFloat,
    pub structure: StructureType,
    pub neighbour: HashSet<HashSetFloat>,
    pub enviroment_block_under: Vec<Block>,
}

impl Structure {
    pub fn get_neighbours(
        x: FF32,
        y: FF32,
        x_max: FF32,
        y_max: FF32,
        size: u8,
    ) -> HashSet<HashSetFloat> {
        let mut outputvec = HashSet::new();

        if x > 0.0 {
            for i in 0..size {
                outputvec.insert(HashSetFloat {
                    x: x - 1.0,
                    y: y + i as f32,
                });
            }
        }
        if y > 0.0 - size as f32 {
            for i in 0..size {
                outputvec.insert(HashSetFloat {
                    x: x + i as f32,
                    y: y - size as f32,
                });
            }
        }
        if x_max > x + size as f32 {
            for i in 0..size {
                outputvec.insert(HashSetFloat {
                    x: x + size as f32,
                    y: y + i as f32,
                });
            }
        }
        if y_max > y + 1.0 {
            for i in 0..size {
                outputvec.insert(HashSetFloat {
                    x: x + i as f32,
                    y: y + 1.0,
                });
            }
        }
        return outputvec;
    }
}

#[derive(Component)]
pub struct TestValue {
    pub value: u32,
}
