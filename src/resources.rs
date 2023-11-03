use crate::components::{EnvironmentBlock, HashSetFloat};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Debug)]
pub struct EnvironmentEntities {
    pub map: HashMap<HashSetFloat, EnvironmentBlock>,
}
impl Default for EnvironmentEntities {
    fn default() -> Self {
        EnvironmentEntities {
            map: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct StoneResource {
    pub value: u32,
}
impl Default for StoneResource {
    fn default() -> Self {
        StoneResource { value: 0 }
    }
}

#[derive(Resource)]
pub struct MudResource {
    pub value: u32,
}
impl Default for MudResource {
    fn default() -> Self {
        MudResource { value: 0 }
    }
}
