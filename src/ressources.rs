use bevy::prelude::*;

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
