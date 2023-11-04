use bevy::prelude::*;

use crate::{components::HashSetFloat, structures::Facing};

///# Items
/// Here is the first place to add a new item
/// all items have to have entry in this enum
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ItemType {
    Nothing,
    Stone,
    Mud,
}

///# Get item direction
/// It has to be given from where the item is comming and TO where it is going
/// it then find from witch direction the item has come
pub fn get_item_direction(from: HashSetFloat, to: HashSetFloat) -> Facing {
    if from.x == to.x || from.y == to.y {
        if from.x > to.x {
            Facing::Left
        } else if from.x < to.x {
            Facing::Right
        } else if from.y > to.y {
            Facing::Down
        } else if from.y < to.y {
            Facing::Up
        } else {
            panic!(
                "Structure fromm {:?}, tries to send item to {:?}, which might be the same block",
                from, to
            );
        }
    } else {
        if from.y > to.y {
            Facing::Down
        } else if from.x > to.x {
            Facing::Left
        } else if to.x - from.x > to.y - from.y {
            Facing::Right
        } else {
            Facing::Up
        }
    }
}
