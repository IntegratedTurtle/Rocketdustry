use crate::components::{EnviromentBlock, HashSetFloat};
use bevy::prelude::*;
use std::{collections::HashMap, default};

#[derive(Resource, Debug)]
pub struct EnviromentEntities {
    pub map: HashMap<HashSetFloat, EnviromentBlock>,
}
impl Default for EnviromentEntities {
    fn default() -> Self {
        EnviromentEntities {
            map: HashMap::new(),
        }
    }
}
#[derive(Resource, Debug)]
pub struct StructureEntities {
    pub map: HashMap<HashSetFloat, Entity>,
}

impl Default for StructureEntities {
    fn default() -> Self {
        StructureEntities {
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
