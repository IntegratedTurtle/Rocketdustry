use crate::components::HashSetFloat;
use crate::components::Structure;
use crate::mapsetup::TEXTURESIZE;
use crate::resources::EnviromentEntities;
use crate::resources::StructureEntities;
use rand::Rng;

use crate::drill;

use crate::mapsetup::Block;
use crate::MapAsPng;
use bevy::prelude::*;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use umath::FF32;

///# Structure create event
/// When ever a structure gets created this event is send
/// it is used to
/// 1. Add the Component in its file
#[derive(Event)]
pub struct StructureCreateEvent {
    structure_type: StructureType,
}

///# Structure
/// Here is the first place if you want to define a structure
#[derive(PartialEq, Debug, Clone)]
pub enum StructureType {
    Core,
    Conveyor,
    Drill,
    Nothing,
    Checker,
}

impl StructureType {
    /// Defines How big a block ist, where sizexsize is the Size
    fn size(&self) -> u8 {
        return match self {
            Self::Core => 2,
            Self::Conveyor => 1,
            Self::Drill => drill::SIZE,
            Self::Nothing => 1,
            Self::Checker => 1,
        };
    }
    /// Definer here which colors in the map the blocks should have
    fn pixel_to_block(pixel: [u8; 3]) -> Self {
        return match pixel {
            [0, 0, 0] => Self::Nothing,
            [255, 200, 0] => Self::Core,
            drill::PIXLEVALUE => Self::Drill,
            [100, 100, 100] => Self::Conveyor,
            _ => Self::Checker,
        };
    }
    fn texture_string(&self) -> String {
        return match self {
            Self::Nothing => "sprites/Nothing.png".to_string(),
            Self::Checker => "sprites/Checker.png".to_string(),
            Self::Core => "sprites/Core.png".to_string(),
            Self::Conveyor => "sprites/Conveyer.png".to_string(),
            Self::Drill => drill::SPRITENAME.to_string(),
        };
    }
}

fn create_empty_image(dimension: (u32, u32)) -> DynamicImage {
    let height = dimension.0;
    let width = dimension.1;
    let img = ImageBuffer::from_fn(width, height, |_, _| {
        Rgba([0, 0, 0, 0]) // Create an RGBA pixel with all zeros (fully transparent)
    });

    DynamicImage::ImageRgba8(img)
}

///# StructuresAsPng
/// Saves the current Structures as image and it dimensions so it can easily be reused
#[derive(Resource, Clone)]
pub struct StructuresAsPng {
    image: DynamicImage,
    dimension: (u32, u32),
}

impl Default for StructuresAsPng {
    fn default() -> Self {
        // While there is no loading screen that determents the structures on which on will play on
        // the game will choose structures.png in the root
        let image = match image::open("./structures.png") {
            Ok(file) => file,
            Err(_) => create_empty_image(image::open("./map.png").unwrap().dimensions()),
        };
        let dimension = image.dimensions();
        StructuresAsPng {
            // Since image crates and bevys coordinate systems have mirrored x x acis, we have to mirror this again
            image: image.clone().flipv(),
            dimension: dimension,
        }
    }
}

/// This function uses the DynamicImage saved in image, and gets the r,g,b values from its positions
impl StructuresAsPng {
    fn coordinates_to_pixel_without_alpha(&self, x: u32, y: u32) -> [u8; 3] {
        let pixels = self.image.get_pixel(x, y);
        [pixels[0], pixels[1], pixels[2]]
    }
}

/// For ease of use, the program does always counts the top left part of the Structure as its base
/// Since the origins are in the middle this function has to calculate the real position, from the theoretical one
fn reallocation_with_size(location: &HashSetFloat, size: u8) -> (f32, f32) {
    (
        (*location.x + (0.5 * (size - 1) as f32)) * TEXTURESIZE, // - (0.5 * (size - 1) as f32)
        (*location.y + (0.5 * (size - 1) as f32)) * TEXTURESIZE, // + (0.5 * (size - 1) as f32)
    )
}

///# Spawn Struct
/// Spawns a Structure
pub fn spawn_structure(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    strucute_entities: &mut ResMut<StructureEntities>,
    location: HashSetFloat,
    enviromentblock: Vec<Block>,
    structure_type: StructureType,
    map_size: (u32, u32),
    structure_create_event: &mut EventWriter<StructureCreateEvent>,
) {
    let reallocation = reallocation_with_size(&location, structure_type.size());
    let structure_id = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(reallocation.0, reallocation.1, 0.5),
                texture: asset_server.load(structure_type.texture_string()),
                ..default()
            },
            Structure {
                location: location,
                structure: structure_type.clone(),
                neighbour: Structure::get_neighbours(
                    location.x,
                    location.y,
                    unsafe { FF32::new(map_size.0 as f32) },
                    unsafe { FF32::new(map_size.1 as f32) },
                    structure_type.size(),
                ),
                enviroment_block_under: enviromentblock,
            },
        ))
        .id();
    strucute_entities.map.insert(location, structure_id);
    structure_create_event.send(StructureCreateEvent { structure_type });
}

///# Get Enviroment block from xy
/// Creates a map of all coordinates that are underneath the block
/// Then with the coordinates it gets the Block types from the EnviromentEntities
/// If the Block is just 1x1, we don't need to create such maps and can just take the value
pub fn get_environment_block_from_xy(
    x: usize,
    y: usize,
    size: u8,
    environmentblock_entities: &Res<EnviromentEntities>,
) -> Vec<Block> {
    return if size > 1 {
        (x..=x + size as usize - 1)
            .flat_map(|x_val| (y..=y + size as usize - 1).map(move |y_val| (x_val, y_val)))
            .map(|(x_i, y_i)| {
                match environmentblock_entities.map.get(&HashSetFloat {
                    x: unsafe { FF32::new(x_i as f32) },
                    y: unsafe { FF32::new(y_i as f32) },
                }) {
                    Some(block) => block.block,
                    None => Block::Nothing,
                }
            })
            .collect()
    } else {
        vec![match environmentblock_entities.map.get(&HashSetFloat {
            x: unsafe { FF32::new(x as f32) },
            y: unsafe { FF32::new(y as f32) },
        }) {
            Some(block) => block.block,
            None => Block::Nothing,
        }]
    };
}
///# Get most Block
/// Gets the Vec and removes all non prominent Blocks, so the drill does only know which Blocks are under it
pub fn get_most_block(mut blocks: Vec<Block>) -> Vec<Block> {
    if blocks.is_empty() {
        return Vec::new(); // Handle the case of an empty vector.
    }

    let mut counts = std::collections::HashMap::new();

    // Count the occurrences of each Block in the vector.
    for &block in &blocks {
        *counts.entry(block).or_insert(0) += 1;
    }

    // Find the most prominent Block and its count.
    let (most_prominent, _) = counts.iter().max_by_key(|&(_, count)| count).unwrap();

    // Use iterator filter to keep only the most prominent Block.
    blocks.retain(|&block| block == *most_prominent);

    blocks
}

///# Spawn Structures from Map
/// If there is a structures.png given, it is possible to spawn structures at the beginning of the game, of the game
pub fn spawn_structures_from_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_as_png: Res<StructuresAsPng>,
    enviromentblock_entities: Res<EnviromentEntities>,
    mut strucute_entities: ResMut<StructureEntities>,
    mut structure_create_event: EventWriter<StructureCreateEvent>,
) {
    let x_max: usize = map_as_png.dimension.0 as usize;
    let y_max: usize = map_as_png.dimension.1 as usize;
    for x in 0..x_max {
        for y in 0..y_max {
            let structure_type = StructureType::pixel_to_block(
                map_as_png.coordinates_to_pixel_without_alpha(x as u32, y as u32),
            );
            if structure_type != StructureType::Nothing {
                spawn_structure(
                    &mut commands,
                    &asset_server,
                    &mut strucute_entities,
                    HashSetFloat {
                        x: unsafe { FF32::new(x as f32) },
                        y: unsafe { FF32::new(y as f32) },
                    },
                    get_environment_block_from_xy(
                        x,
                        y,
                        structure_type.size(),
                        &enviromentblock_entities,
                    ),
                    structure_type,
                    map_as_png.dimension,
                    &mut structure_create_event,
                )
            }
        }
    }
}

///# Random item
/// Takes in a properbility value and returns a bool
/// Gets used for determening if ressource gets spawned
pub fn random_item(probability: f32) -> bool {
    let mut rng = rand::thread_rng();
    let random_value: f32 = rng.gen();

    random_value < probability
}
