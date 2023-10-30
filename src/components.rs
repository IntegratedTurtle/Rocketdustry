use bevy::prelude::*;
use ordered_float::{self, OrderedFloat};
use std::collections::HashSet;
use umath::FF32;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct HashSetFloat {
    pub x: FF32,
    pub y: FF32,
}

#[derive(PartialEq, Debug)]
pub enum Block {
    Gras,
    Iron,
    Stone,
    Mud,
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

#[derive(Component)]
pub struct TestValue {
    pub value: u32,
}
