use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::{nadk::display::Color565, physic::BoundingBox, renderer::mesh::QuadDir};

pub mod rendering {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const SCREEN_TILE_SUBDIVISION: usize = 4; // Minimum 2

    pub const MIN_FOV: f32 = 30.;
    pub const MAX_FOV: f32 = 110.;

    pub const FOV: f32 = 45.;

    pub const MAX_TRIANGLES: usize = 1500;

    pub const MAX_RENDER_DISTANCE: usize = 2; // You shouldn't go higher

    pub const BLURING_SCREEN_SUBDIVISION: usize = 5;
    pub const BLURING_RADIUS: isize = 2;

    pub const MAX_ENTITY_RENDER_DISTANCE: f32 = 10.;

    pub const ITEM_ENTITY_SPRITE_SIZE: f32 = 0.8;
}

pub mod color_palette {
    use crate::nadk::display::Color565;

    pub const MENU_OUTLINE_COLOR: Color565 = Color565::from_rgb888(150, 150, 150);
    pub const MENU_ELEMENT_BACKGROUND_COLOR: Color565 = Color565::from_rgb888(230, 230, 230);
    pub const MENU_ELEMENT_BACKGROUND_COLOR_HOVER: Color565 = Color565::from_rgb888(190, 190, 190);
    pub const MENU_TEXT_COLOR: Color565 = Color565::from_rgb888(0, 0, 0);
    pub const MENU_BACKGROUND_COLOR: Color565 = Color565::from_rgb888(255, 255, 255);

    pub const GAMEUI_SLOT_COLOR: Color565 = Color565::from_rgb888(80, 80, 80);
    pub const GAMEUI_SLOT_DEFAULT_OUTLINE_COLOR: Color565 = Color565::from_rgb888(120, 120, 120);
}

pub mod save_manager {
    pub const SETTINGS_FILENAME: &str = "settings.ncd"; // NCD = NumCraftData

    pub const WORLD_VERSION: u16 = 0; // Update the version at each world breaking update
}

pub mod world {
    pub const CHUNK_SIZE: usize = 8; // MAX 8
    pub const CHEST_SLOTS: usize = 27;

    pub const MAX_ITEM_MERGING_DISTANCE: f32 = 2.;
    pub const ITEM_MAGNET_FORCE: f32 = 10.;
    pub const MAX_PLAYER_ITEM_MAGNET_DISTANCE: f32 = 2.2;
}

pub mod player {
    use core::f32::consts::PI;

    pub const ROTATION_SPEED: f32 = PI / 3.0; // rad / sec
    pub const FLY_SPEED: f32 = 4.0;
    pub const WALK_FORCE: f32 = 20.0;
    pub const MAX_WALKING_VELOCITY: f32 = 4.;
    pub const JUMP_FORCE: f32 = 5.;

    pub const WOODEN_TOOL_SPEED: f32 = 2.;
    pub const STONE_TOOL_SPEED: f32 = 3.;
    pub const IRON_TOOL_SPEED: f32 = 4.5;
    pub const GOLD_TOOL_SPEED: f32 = 6.;
    pub const DIAMOND_TOOL_SPEED: f32 = 7.5;

    pub const MAX_HEALTH: f32 = 20.;
    pub const MAX_HUNGER: f32 = 20.;
    pub const HUNGER_DRAIN_PER_SECOND: f32 = 1. / 40.;
    pub const HEALTH_REGEN_PER_SECOND: f32 = 0.5;
    pub const HEALTH_REGEN_MIN_HUNGER: f32 = 12.;
    pub const STARVATION_DAMAGE_PER_SECOND: f32 = 0.5;
    pub const FALL_SAFE_HEIGHT: f32 = 3.;
    pub const FALL_DAMAGE_PER_BLOCK: f32 = 1.;
    pub const APPLE_HUNGER_RESTORE: f32 = 4.;
}

pub mod physic {
    use nalgebra::Vector3;

    pub const GRAVITY_FACTOR: f32 = 10.0;
    pub const MAX_FALLING_VELOCITY: f32 = 5.;
    pub const ON_FLOOR_FRICTION: f32 = 10.;

    pub const BLOCK_COLLISION_SCANNING_SIZE: Vector3<isize> = Vector3::new(2, 3, 2);
}

#[allow(unreachable_patterns)]
impl EntityType {
    pub fn get_bbox(&self) -> Option<BoundingBox> {
        match self {
            EntityType::Player => Some(BoundingBox {
                offset: Vector3::new(-0.4, -0.5, -0.4),
                size: Vector3::new(0.8, 1.8, 0.8),
            }),
            EntityType::Item => Some(BoundingBox {
                offset: Vector3::new(-0.2, -0.2, -0.2),
                size: Vector3::new(0.4, 0.4, 0.4),
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player = 0,
    Item = 1,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToolKind {
    Pickaxe,
    Axe,
    Shovel,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Sand = 4,
    Cobblestone = 5,
    Border = 6,
    Log = 7,
    Leaves = 8,
    Planks = 9,
    CraftingTable = 10,
    Bricks = 11,
    Chest = 12,
    CoalOre = 13,
    IronOre = 14,
    GoldOre = 15,
    DiamondOre = 16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ItemType {
    Air = 0,

    StoneBlock = 1,
    GrassBlock = 2,
    DirtBlock = 3,
    SandBlock = 4,
    CobblestoneBlock = 5,
    BorderBlock = 6,
    LogBlock = 7,
    LeavesBlock = 8,
    PlanksBlock = 9,

    CraftingTableBlock = 10,
    BricksBlock = 11,

    Stick = 12,
    WoodenPickaxe = 13,
    WoodenAxe = 14,
    WoodenShovel = 15,

    StonePickaxe = 16,
    StoneAxe = 17,
    StoneShovel = 18,

    ChestBlock = 19,

    Apple = 20,

    Coal = 21,
    RawIron = 22,
    RawGold = 23,
    IronIngot = 24,
    GoldIngot = 25,
    Diamond = 26,

    IronPickaxe = 27,
    IronAxe = 28,
    IronShovel = 29,
    GoldPickaxe = 30,
    GoldAxe = 31,
    GoldShovel = 32,
    DiamondPickaxe = 33,
    DiamondAxe = 34,
    DiamondShovel = 35,
}

impl ItemType {
    pub fn get_texture_id(&self) -> u8 {
        match *self {
            ItemType::Air => 0,

            ItemType::StoneBlock => 1,
            ItemType::GrassBlock => 2,
            ItemType::DirtBlock => 3, // 4 is the other texture of the grass block
            ItemType::SandBlock => 5,
            ItemType::CobblestoneBlock => 6,
            ItemType::BorderBlock => 7,
            ItemType::LogBlock => 8,
            ItemType::LeavesBlock => 9,
            ItemType::PlanksBlock => 10,
            ItemType::CraftingTableBlock => 12,
            ItemType::BricksBlock => 11,
            ItemType::Stick => 14,
            ItemType::WoodenPickaxe => 15,
            ItemType::WoodenAxe => 16,
            ItemType::WoodenShovel => 17,
            ItemType::StonePickaxe => 19,
            ItemType::StoneAxe => 20,
            ItemType::StoneShovel => 21,
            ItemType::ChestBlock => 18,
            ItemType::Apple => 22,
            ItemType::Coal => 27,
            ItemType::RawIron => 28,
            ItemType::RawGold => 29,
            ItemType::IronIngot => 30,
            ItemType::GoldIngot => 31,
            ItemType::Diamond => 32,
            ItemType::IronPickaxe => 33,
            ItemType::IronAxe => 34,
            ItemType::IronShovel => 35,
            ItemType::GoldPickaxe => 36,
            ItemType::GoldAxe => 37,
            ItemType::GoldShovel => 38,
            ItemType::DiamondPickaxe => 39,
            ItemType::DiamondAxe => 40,
            ItemType::DiamondShovel => 41,
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(ItemType::Air),

            1 => Some(ItemType::StoneBlock),
            2 => Some(ItemType::GrassBlock),
            3 => Some(ItemType::DirtBlock),
            4 => Some(ItemType::SandBlock),
            5 => Some(ItemType::CobblestoneBlock),
            6 => Some(ItemType::BorderBlock),
            7 => Some(ItemType::LogBlock),
            8 => Some(ItemType::LeavesBlock),
            9 => Some(ItemType::PlanksBlock),
            10 => Some(ItemType::CraftingTableBlock),
            11 => Some(ItemType::BricksBlock),
            12 => Some(ItemType::Stick),
            13 => Some(ItemType::WoodenPickaxe),
            14 => Some(ItemType::WoodenAxe),
            15 => Some(ItemType::WoodenShovel),
            16 => Some(ItemType::StonePickaxe),
            17 => Some(ItemType::StoneAxe),
            18 => Some(ItemType::StoneShovel),
            19 => Some(ItemType::ChestBlock),
            20 => Some(ItemType::Apple),
            21 => Some(ItemType::Coal),
            22 => Some(ItemType::RawIron),
            23 => Some(ItemType::RawGold),
            24 => Some(ItemType::IronIngot),
            25 => Some(ItemType::GoldIngot),
            26 => Some(ItemType::Diamond),
            27 => Some(ItemType::IronPickaxe),
            28 => Some(ItemType::IronAxe),
            29 => Some(ItemType::IronShovel),
            30 => Some(ItemType::GoldPickaxe),
            31 => Some(ItemType::GoldAxe),
            32 => Some(ItemType::GoldShovel),
            33 => Some(ItemType::DiamondPickaxe),
            34 => Some(ItemType::DiamondAxe),
            35 => Some(ItemType::DiamondShovel),
            _ => None,
        }
    }

    pub fn get_tool_kind(&self) -> Option<ToolKind> {
        match *self {
            ItemType::WoodenPickaxe => Some(ToolKind::Pickaxe),
            ItemType::WoodenAxe => Some(ToolKind::Axe),
            ItemType::WoodenShovel => Some(ToolKind::Shovel),
            ItemType::StonePickaxe => Some(ToolKind::Pickaxe),
            ItemType::IronPickaxe | ItemType::GoldPickaxe | ItemType::DiamondPickaxe => {
                Some(ToolKind::Pickaxe)
            }
            ItemType::IronAxe | ItemType::GoldAxe | ItemType::DiamondAxe => Some(ToolKind::Axe),
            ItemType::IronShovel | ItemType::GoldShovel | ItemType::DiamondShovel => {
                Some(ToolKind::Shovel)
            }
            ItemType::StoneAxe => Some(ToolKind::Axe),
            ItemType::StoneShovel => Some(ToolKind::Shovel),
            _ => None,
        }
    }

    /// The tool tier, compared with `BlockType::get_harvest_level`. Bare hands are tier 0
    pub fn get_tool_tier(&self) -> u8 {
        match *self {
            ItemType::WoodenPickaxe | ItemType::WoodenAxe | ItemType::WoodenShovel => 1,
            ItemType::StonePickaxe | ItemType::StoneAxe | ItemType::StoneShovel => 2,
            ItemType::IronPickaxe | ItemType::IronAxe | ItemType::IronShovel => 3,
            ItemType::GoldPickaxe | ItemType::GoldAxe | ItemType::GoldShovel => 2,
            ItemType::DiamondPickaxe | ItemType::DiamondAxe | ItemType::DiamondShovel => 4,
            _ => 0,
        }
    }

    /// How much faster this tool breaks the blocks it is made for
    pub fn get_tool_speed(&self) -> f32 {
        match *self {
            ItemType::WoodenPickaxe | ItemType::WoodenAxe | ItemType::WoodenShovel => {
                player::WOODEN_TOOL_SPEED
            }
            ItemType::StonePickaxe | ItemType::StoneAxe | ItemType::StoneShovel => {
                player::STONE_TOOL_SPEED
            }
            ItemType::IronPickaxe | ItemType::IronAxe | ItemType::IronShovel => {
                player::IRON_TOOL_SPEED
            }
            ItemType::GoldPickaxe | ItemType::GoldAxe | ItemType::GoldShovel => {
                player::GOLD_TOOL_SPEED
            }
            ItemType::DiamondPickaxe | ItemType::DiamondAxe | ItemType::DiamondShovel => {
                player::DIAMOND_TOOL_SPEED
            }
            _ => 1.,
        }
    }

    pub fn get_max_stack_amount(&self) -> u8 {
        match *self {
            ItemType::Air => 0,
            ItemType::StoneBlock => 64,
            ItemType::GrassBlock => 64,
            ItemType::DirtBlock => 64,
            ItemType::SandBlock => 64,
            ItemType::CobblestoneBlock => 64,
            ItemType::BorderBlock => 64,
            ItemType::LogBlock => 64,
            ItemType::LeavesBlock => 64,
            ItemType::PlanksBlock => 64,
            ItemType::CraftingTableBlock => 64,
            ItemType::BricksBlock => 64,
            ItemType::Stick => 64,
            ItemType::WoodenPickaxe => 1,
            ItemType::WoodenAxe => 1,
            ItemType::WoodenShovel => 1,
            ItemType::StonePickaxe => 1,
            ItemType::StoneAxe => 1,
            ItemType::StoneShovel => 1,
            ItemType::ChestBlock => 64,
            ItemType::Apple => 64,
            ItemType::Coal => 64,
            ItemType::RawIron => 64,
            ItemType::RawGold => 64,
            ItemType::IronIngot => 64,
            ItemType::GoldIngot => 64,
            ItemType::Diamond => 64,
            ItemType::IronPickaxe => 1,
            ItemType::IronAxe => 1,
            ItemType::IronShovel => 1,
            ItemType::GoldPickaxe => 1,
            ItemType::GoldAxe => 1,
            ItemType::GoldShovel => 1,
            ItemType::DiamondPickaxe => 1,
            ItemType::DiamondAxe => 1,
            ItemType::DiamondShovel => 1,
        }
    }

    pub fn get_matching_block_type(&self) -> Option<BlockType> {
        match self {
            ItemType::Air => None,
            ItemType::StoneBlock => Some(BlockType::Stone),
            ItemType::GrassBlock => Some(BlockType::Grass),
            ItemType::DirtBlock => Some(BlockType::Dirt),
            ItemType::SandBlock => Some(BlockType::Sand),
            ItemType::CobblestoneBlock => Some(BlockType::Cobblestone),
            ItemType::BorderBlock => Some(BlockType::Border),
            ItemType::LogBlock => Some(BlockType::Log),
            ItemType::LeavesBlock => Some(BlockType::Leaves),
            ItemType::PlanksBlock => Some(BlockType::Planks),
            ItemType::CraftingTableBlock => Some(BlockType::CraftingTable),
            ItemType::BricksBlock => Some(BlockType::Bricks),
            ItemType::Stick => None,
            ItemType::WoodenPickaxe => None,
            ItemType::WoodenAxe => None,
            ItemType::WoodenShovel => None,
            ItemType::StonePickaxe => None,
            ItemType::StoneAxe => None,
            ItemType::StoneShovel => None,
            ItemType::ChestBlock => Some(BlockType::Chest),
            ItemType::Apple => None,
            ItemType::Coal => None,
            ItemType::RawIron => None,
            ItemType::RawGold => None,
            ItemType::IronIngot => None,
            ItemType::GoldIngot => None,
            ItemType::Diamond => None,
            ItemType::IronPickaxe => None,
            ItemType::IronAxe => None,
            ItemType::IronShovel => None,
            ItemType::GoldPickaxe => None,
            ItemType::GoldAxe => None,
            ItemType::GoldShovel => None,
            ItemType::DiamondPickaxe => None,
            ItemType::DiamondAxe => None,
            ItemType::DiamondShovel => None,
        }
    }
}

impl BlockType {
    pub fn is_air(&self) -> bool {
        *self == BlockType::Air
    }

    pub fn get_texture_id(&self, dir: QuadDir) -> u8 {
        match *self {
            BlockType::Air => 0,
            BlockType::Stone => 1,
            BlockType::Grass => {
                if dir == QuadDir::Top {
                    2
                } else {
                    3
                }
            }
            BlockType::Dirt => 3,
            BlockType::Sand => 5,
            BlockType::Cobblestone => 6,
            BlockType::Border => 7,
            BlockType::Log => 8,
            BlockType::Leaves => 9,
            BlockType::Planks => 10,
            BlockType::CraftingTable => match dir {
                QuadDir::Top => 12,
                QuadDir::Bottom => 10,
                _ => 13,
            },
            BlockType::Bricks => 11,
            BlockType::CoalOre => 23,
            BlockType::IronOre => 24,
            BlockType::GoldOre => 25,
            BlockType::DiamondOre => 26,
            BlockType::Chest => match dir {
                QuadDir::Top | QuadDir::Bottom => 10,
                _ => 18,
            },
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(BlockType::Air),
            1 => Some(BlockType::Stone),
            2 => Some(BlockType::Grass),
            3 => Some(BlockType::Dirt),
            4 => Some(BlockType::Sand),
            5 => Some(BlockType::Cobblestone),
            6 => Some(BlockType::Border),
            7 => Some(BlockType::Log),
            8 => Some(BlockType::Leaves),
            9 => Some(BlockType::Planks),
            10 => Some(BlockType::CraftingTable),
            11 => Some(BlockType::Bricks),
            12 => Some(BlockType::Chest),
            13 => Some(BlockType::CoalOre),
            14 => Some(BlockType::IronOre),
            15 => Some(BlockType::GoldOre),
            16 => Some(BlockType::DiamondOre),
            _ => None,
        }
    }

    /// The tool tier needed to get the drop of this block. Below it, the block breaks with no drop
    pub const fn get_harvest_level(&self) -> u8 {
        match self {
            BlockType::CoalOre => 1,
            BlockType::IronOre => 2,
            BlockType::GoldOre | BlockType::DiamondOre => 3,
            _ => 0,
        }
    }

    pub const fn get_harvest_tool(&self) -> Option<ToolKind> {
        match self {
            BlockType::Stone
            | BlockType::Cobblestone
            | BlockType::CraftingTable
            | BlockType::Bricks
            | BlockType::CoalOre
            | BlockType::IronOre
            | BlockType::GoldOre
            | BlockType::DiamondOre => Some(ToolKind::Pickaxe),
            BlockType::Log | BlockType::Planks => Some(ToolKind::Axe),
            BlockType::Dirt | BlockType::Grass | BlockType::Sand => Some(ToolKind::Shovel),
            _ => None,
        }
    }

    pub const fn get_hardness(&self) -> f32 {
        match self {
            BlockType::Air => 0.,
            BlockType::Stone => 2.,
            BlockType::Grass => 1.2,
            BlockType::Dirt => 1.,
            BlockType::Sand => 1.,
            BlockType::Cobblestone => 2.2,
            BlockType::Border => -1.,
            BlockType::Log => 1.5,
            BlockType::Leaves => 0.3,
            BlockType::Planks => 1.2,
            BlockType::CraftingTable => 2.5,
            BlockType::Bricks => 2.,
            BlockType::Chest => 2.5,
            BlockType::CoalOre
            | BlockType::IronOre
            | BlockType::GoldOre
            | BlockType::DiamondOre => 3.,
        }
    }

    pub const fn get_dropped_item_type(&self) -> ItemType {
        match self {
            BlockType::Air => ItemType::Air,
            BlockType::Stone => ItemType::CobblestoneBlock,
            BlockType::Grass => ItemType::DirtBlock,
            BlockType::Dirt => ItemType::DirtBlock,
            BlockType::Sand => ItemType::SandBlock,
            BlockType::Cobblestone => ItemType::CobblestoneBlock,
            BlockType::Border => ItemType::BorderBlock,
            BlockType::Log => ItemType::LogBlock,
            BlockType::Leaves => ItemType::LeavesBlock,
            BlockType::Planks => ItemType::PlanksBlock,
            BlockType::CraftingTable => ItemType::CraftingTableBlock,
            BlockType::Bricks => ItemType::BricksBlock,
            BlockType::Chest => ItemType::ChestBlock,
            BlockType::CoalOre => ItemType::Coal,
            BlockType::IronOre => ItemType::RawIron,
            BlockType::GoldOre => ItemType::RawGold,
            BlockType::DiamondOre => ItemType::Diamond,
        }
    }
}

pub fn get_quad_color_from_texture_id(id: u8) -> Color565 {
    match id {
        1 => Color565::from_rgb888(160, 160, 160),
        2 => Color565::from_rgb888(21, 147, 0),
        3 => Color565::from_rgb888(120, 77, 49),
        5 => Color565::from_rgb888(208, 199, 6),
        6 => Color565::from_rgb888(178, 178, 178),
        7 => Color565::from_rgb888(19, 19, 19),
        8 => Color565::from_rgb888(79, 53, 30),
        9 => Color565::from_rgb888(36, 75, 37),
        10 => Color565::from_rgb888(152, 124, 61),
        11 => Color565::from_rgb888(170, 70, 50),
        12 => Color565::from_rgb888(154, 126, 62),
        13 => Color565::from_rgb888(120, 95, 50),
        14 => Color565::from_rgb888(120, 85, 45),
        15 | 16 | 17 => Color565::from_rgb888(150, 150, 150),
        18 => Color565::from_rgb888(120, 85, 45),
        22 => Color565::from_rgb888(200, 40, 40),
        23 => Color565::from_rgb888(120, 120, 120),
        24 => Color565::from_rgb888(200, 150, 110),
        25 => Color565::from_rgb888(230, 200, 70),
        26 => Color565::from_rgb888(90, 220, 230),
        27 => Color565::from_rgb888(30, 30, 30),
        28 => Color565::from_rgb888(190, 150, 120),
        29 => Color565::from_rgb888(230, 200, 80),
        30 => Color565::from_rgb888(200, 200, 200),
        31 => Color565::from_rgb888(240, 210, 90),
        32 => Color565::from_rgb888(120, 230, 240),
        33 => Color565::from_rgb888(200, 200, 200),
        34 => Color565::from_rgb888(200, 200, 200),
        35 => Color565::from_rgb888(200, 200, 200),
        36 => Color565::from_rgb888(240, 210, 90),
        37 => Color565::from_rgb888(240, 210, 90),
        38 => Color565::from_rgb888(240, 210, 90),
        39 => Color565::from_rgb888(120, 230, 240),
        40 => Color565::from_rgb888(120, 230, 240),
        41 => Color565::from_rgb888(120, 230, 240),
        19 | 20 | 21 => Color565::from_rgb888(110, 110, 110),

        _ => Color565::from_rgb888(0, 0, 0),
        // 255 is reserved for block outline
    }
}
