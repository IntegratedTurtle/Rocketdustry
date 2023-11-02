use crate::components::{EnviromentBlock, HashSetFloat};
use bevy::prelude::*;
use std::collections::HashMap;

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

#[derive(Resource)]
pub struct StoneRessource {
    pub value: u32,
}
impl Default for StoneRessource {
    fn default() -> Self {
        StoneRessource { value: 0 }
    }
}

#[derive(Resource)]
pub struct MudRessource {
    pub value: u32,
}
impl Default for MudRessource {
    fn default() -> Self {
        MudRessource { value: 0 }
    }
}
