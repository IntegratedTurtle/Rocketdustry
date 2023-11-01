use crate::components::{EnviromentBlock, HashSetFloat};
use bevy::prelude::*;
use image::{DynamicImage, GenericImageView};
use std::collections::HashSet;
use umath::FF32;

/// ## Texture Size
/// Defines how big the smalles texture is, so an ingame 1x1 block
/// a 2x2 Block would be 4*Texturesize
pub const TEXTURESIZE: f32 = 64.0;

/// # Block
/// These are the different types of Groundblock there are.
/// Ground Interactiv Blocks will use these to determen what to do
/// ## Creating new Blocks
/// 1. Add the new Block to this Enum
/// 2. Define a pixle for it and add it to the pixle_to_block funktion
/// 3. Define a texture and add it to the texture_string funktion
#[derive(PartialEq, Debug)]
pub enum Block {
    Gras,
    Iron,
    Stone,
    Mud,
    Nothing,
    Checker,
}
impl Block {
    fn pixel_to_block(pixle: [u8; 3]) -> Self {
        return match pixle {
            [0, 0, 0] => Self::Nothing,
            [0, 80, 40] => Self::Gras,
            [80, 40, 0] => Self::Mud,
            [100, 100, 100] => Self::Stone,
            _ => Self::Checker,
        };
    }
    fn texture_string(&self) -> String {
        return match self {
            Self::Gras => "sprites/Gras.png".to_string(),
            Self::Mud => "sprites/Mud.png".to_string(),
            Self::Stone => "sprites/Stone.png".to_string(),
            // This has to be renamed
            Self::Nothing => "sprites/Wall.png".to_string(),
            Self::Checker => "sprites/Checker.png".to_string(),
            _ => "sprites/Checker.png".to_string(),
        };
    }
}

///# MapAsPng
/// Saves the current map as image and it dimensions so it can easily be reused
#[derive(Resource, Clone)]
pub struct MapAsPng {
    image: DynamicImage,
    pub dimension: (u32, u32),
}

impl Default for MapAsPng {
    // While there is no loading screen that determents the map on which on will play on
    // the game will chouse map.png in the root
    fn default() -> Self {
        let image = image::open("./map.png").expect("Could not find map.png in root");
        let dimension = image.dimensions();
        MapAsPng {
            dimension,
            // Since image 0/0 starts left top and bevies bottom left we have to flip
            image: image.clone().flipv(),
        }
    }
}
/// This funktion uses the DynamicImage saved in image, and gets the r,g,b values from its positions
impl MapAsPng {
    fn coordinates_to_pixel_without_alpha(&self, x: u32, y: u32) -> [u8; 3] {
        let pixels = self.image.get_pixel(x, y);
        [pixels[0], pixels[1], pixels[2]]
    }
}

/// # Spawn_blocks
/// This is currently the way to spawn the map at the beginning of the game
/// It requires MapAsPng to be loaded with an image when executed and will
/// create the map from this picture

/// ## block_type
/// is the block the currently created block is type of
/// 1. gets the r,g,b values with coordingates from the picture
/// 2. maps these r,g,b values to an Block
/// 3. loads the texture of an Block into the texture of the SpriteBundle

pub fn spawn_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_as_png: Res<MapAsPng>,
) {
    let x_max: usize = map_as_png.dimension.0 as usize;
    let y_max: usize = map_as_png.dimension.1 as usize;
    for x in 0..x_max {
        for y in 0..y_max {
            let block_type = Block::pixel_to_block(
                map_as_png.coordinates_to_pixel_without_alpha(x as u32, y as u32),
            );
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        x as f32 * TEXTURESIZE,
                        y as f32 * TEXTURESIZE,
                        0.0,
                    ),
                    /// Load the texture of an Block into the texture
                    texture: asset_server.load(&block_type.texture_string()),
                    ..default()
                },
                EnviromentBlock {
                    location: HashSetFloat {
                        x: unsafe { FF32::new(x as f32) },
                        y: unsafe { FF32::new(y as f32) },
                    },
                    block: block_type,
                    neighbour: EnviromentBlock::get_neighbours(
                        unsafe { FF32::new(x as f32) },
                        unsafe { FF32::new(y as f32) },
                        unsafe { FF32::new(x_max as f32) },
                        unsafe { FF32::new(y_max as f32) },
                    ),
                },
            ));
        }
    }
}
