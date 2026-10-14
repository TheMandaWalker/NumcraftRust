use core::any::Any;

use serde::{Deserialize, Serialize};

use crate::constants::world::{
    CHEST_SLOTS, CHUNK_SIZE, ITEM_MAGNET_FORCE, MAX_ITEM_MERGING_DISTANCE,
    MAX_PLAYER_ITEM_MAGNET_DISTANCE,
};
use crate::constants::{BlockType, EntityType, ItemType, furnace::SMELT_TIME};
use crate::entity::Entity;
use crate::entity::item::ItemEntityCustomData;
use crate::inventory::{Inventory, ItemStack};
use crate::nadk::random::get_random_in_range;
use crate::world::chunk_manager::ChunksManager;
use crate::world::world_generator::WorldGenerator;

calc_use!(alloc::boxed::Box);
calc_use!(alloc::vec::Vec);
calc_use!(alloc::vec);

use nalgebra::Vector3;

pub mod chunk;
pub mod chunk_manager;
mod structures;
pub mod world_generator;

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct World {
    pub chunks_manager: ChunksManager,
    chests: Vec<Chest>,
    furnaces: Vec<Furnace>,
    loaded_entities: Vec<Entity>,
    next_available_entity_id: usize,
    world_generator: WorldGenerator,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chest {
    pub pos: (isize, isize, isize),
    pub inventory: Inventory,
}

/// A furnace. Slot 0 is the input, slot 1 the fuel and slot 2 the output
#[derive(Serialize, Deserialize, Clone)]
pub struct Furnace {
    pub pos: (isize, isize, isize),
    pub inventory: Inventory,
    pub burn_time_left: f32,
    pub cook_progress: f32,
}

const FURNACE_INPUT_SLOT: usize = 0;
const FURNACE_FUEL_SLOT: usize = 1;
const FURNACE_OUTPUT_SLOT: usize = 2;

impl Furnace {
    fn new(pos: (isize, isize, isize)) -> Self {
        Furnace {
            pos,
            inventory: Inventory::new(3),
            burn_time_left: 0.,
            cook_progress: 0.,
        }
    }

    /// Whether the output slot can take one more of this item
    fn output_accepts(&self, item_type: ItemType) -> bool {
        match self
            .inventory
            .get_item_stack_at_slot_index(FURNACE_OUTPUT_SLOT)
        {
            Some(stack) if stack.get_item_type() == ItemType::Air => true,
            Some(stack) => {
                stack.get_item_type() == item_type
                    && stack.get_amount() < item_type.get_max_stack_amount()
            }
            None => false,
        }
    }

    fn update(&mut self, delta_time: f32) {
        let input = self
            .inventory
            .get_item_type_at_slot_index(FURNACE_INPUT_SLOT)
            .unwrap_or(ItemType::Air);
        let smelted = input.get_smelted_item();
        let can_smelt = smelted.is_some_and(|item| self.output_accepts(item));

        // When the last fuel is burnt out, burn the next one
        if self.burn_time_left <= 0. && can_smelt {
            let fuel = self
                .inventory
                .get_item_type_at_slot_index(FURNACE_FUEL_SLOT)
                .unwrap_or(ItemType::Air);
            if fuel.get_fuel_time() > 0. {
                self.inventory.take_one(FURNACE_FUEL_SLOT);
                self.burn_time_left = fuel.get_fuel_time();
            }
        }

        let burning = self.burn_time_left > 0.;
        if burning {
            self.burn_time_left = (self.burn_time_left - delta_time).max(0.);
        }

        if can_smelt && burning {
            self.cook_progress += delta_time;
        } else if !can_smelt {
            // The progress starts again if the input changes
            self.cook_progress = 0.;
        }

        if self.cook_progress >= SMELT_TIME {
            self.cook_progress = 0.;
            if let Some(item) = smelted {
                self.inventory.take_one(FURNACE_INPUT_SLOT);
                let amount = match self
                    .inventory
                    .get_item_stack_at_slot_index(FURNACE_OUTPUT_SLOT)
                {
                    Some(stack) if stack.get_item_type() == item => stack.get_amount() + 1,
                    _ => 1,
                };
                self.inventory.replace_slot_item_stack(
                    FURNACE_OUTPUT_SLOT,
                    ItemStack::new(item, amount, false),
                );
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the current world. Contains all the chunks
impl World {
    pub fn new() -> Self {
        let mut world = World {
            chunks_manager: ChunksManager::new(),
            chests: Vec::new(),
            furnaces: Vec::new(),
            loaded_entities: vec![Entity::new(0, EntityType::Player, None)], // The player entity is always loaded and id 0
            next_available_entity_id: 1,
            world_generator: WorldGenerator::new(),
        };

        world
    }

    pub fn get_player_entity_mut(&mut self) -> &mut Entity {
        &mut self.loaded_entities[0]
    }

    pub fn get_player_entity(&self) -> &Entity {
        &self.loaded_entities[0]
    }

    pub fn load_area(
        &mut self,
        x_start: isize,
        x_stop: isize,
        y_start: isize,
        y_stop: isize,
        z_start: isize,
        z_stop: isize,
    ) {
        for x in x_start..x_stop {
            for y in y_start..y_stop {
                for z in z_start..z_stop {
                    self.chunks_manager.add_chunk(Vector3::new(x, y, z));
                }
            }
        }

        for x in x_start..x_stop {
            for y in y_start..y_stop {
                for z in z_start..z_stop {
                    self.world_generator
                        .generate_chunk(&mut self.chunks_manager, Vector3::new(x, y, z));
                }
            }
        }
    }

    pub fn update_entities(&mut self, delta_time: f32) {
        // Check for item merging and player magnet
        'first_loop: for i in 0..self.loaded_entities.len() {
            if self.loaded_entities[i].get_type() == EntityType::Item {
                // Ignore items with custom data = None, they will be removed after
                if self.loaded_entities[i].custom_data.is_none() {
                    continue;
                }

                // Get the item_data from the first item
                let first_item_data = ItemEntityCustomData::get_item_data(&self.loaded_entities[i])
                    .expect("Item Entity must have ItemData as custom data.");
                let first_item_stack = first_item_data.item_stack.clone();

                let max_stack = first_item_stack.get_item_type().get_max_stack_amount();

                for j in 0..self.loaded_entities.len() {
                    if i != j
                        && self.loaded_entities[j].custom_data.is_some() // Ignore items with custom data = None, they will be removed after
                        && self.loaded_entities[j].get_type() == EntityType::Item
                        && self.loaded_entities[i]
                            .pos
                            .metric_distance(&self.loaded_entities[j].pos)
                            <= MAX_ITEM_MERGING_DISTANCE
                    {
                        // Check if the items can merge
                        let second_item_data =
                            ItemEntityCustomData::get_item_data(&self.loaded_entities[j])
                                .expect("Item Entity must have ItemData as custom data.");
                        let second_item_stack = second_item_data.item_stack.clone();

                        if second_item_stack.get_item_type() != first_item_stack.get_item_type() {
                            continue;
                        }

                        if let Some(first_bbox) = self.loaded_entities[i].get_bbox()
                            && let Some(second_bbox) = self.loaded_entities[j].get_bbox()
                            && first_bbox.is_coliding(&second_bbox)
                        {
                            if first_item_stack.get_amount() == max_stack
                                || second_item_stack.get_amount() == max_stack
                            {
                                continue;
                            }

                            let total =
                                first_item_stack.get_amount() + second_item_stack.get_amount();
                            if total <= max_stack {
                                // Merge the two items together and request the deletion of the second one
                                self.loaded_entities[i].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            total,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[j].custom_data = None; // Yes, this should be illegal but it can also be a feature.
                                self.loaded_entities[i].velocity = Vector3::zeros();
                                continue 'first_loop;
                            } else {
                                self.loaded_entities[i].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            max_stack,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[j].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            total - max_stack,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[i].velocity = Vector3::zeros();
                                self.loaded_entities[j].velocity = Vector3::zeros();
                            }
                            continue;
                        }

                        // Calculate the direction to the other item
                        let direction =
                            (self.loaded_entities[j].pos - self.loaded_entities[i].pos).normalize();

                        self.loaded_entities[i].velocity +=
                            direction * ITEM_MAGNET_FORCE * delta_time;

                        // Limit the magnet speed
                        /*if self.loaded_entities[i].velocity.norm() > ITEM_MAGNET_SPEED {
                            self.loaded_entities[i].velocity =
                                self.loaded_entities[i].velocity.normalize()
                                    * ITEM_MAGNET_SPEED
                                    * delta_time;
                        }*/
                    }
                }
            }
        }

        // Player item magnet
        for i in 0..self.loaded_entities.len() {
            let distance = self.loaded_entities[i]
                .pos
                .metric_distance(&self.get_player_entity().pos);
            if self.loaded_entities[i].get_type() == EntityType::Item
                && self.loaded_entities[i].custom_data.is_some()
                && distance < MAX_PLAYER_ITEM_MAGNET_DISTANCE
            {
                let direction =
                    (self.get_player_entity().pos - self.loaded_entities[i].pos).normalize();

                self.loaded_entities[i].velocity += direction * ITEM_MAGNET_FORCE * delta_time;
            }
        }

        // Remove illegal items
        self.loaded_entities.retain(|entity| {
            entity.get_type() != EntityType::Item
                || (entity.get_type() == EntityType::Item && !entity.custom_data.is_none())
        });
    }

    /// Set the world generation seed
    pub fn set_seed(&mut self, seed: i32) {
        self.world_generator.set_seed(seed);
    }

    pub fn get_all_entities_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.loaded_entities
    }
    pub fn get_all_entities(&self) -> &Vec<Entity> {
        &self.loaded_entities
    }

    pub fn get_entity_by_id_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.loaded_entities
            .iter_mut()
            .find(|entity| entity.get_id() == id)
    }

    pub fn get_entity_by_id(&self, id: usize) -> Option<&Entity> {
        self.loaded_entities
            .iter()
            .find(|entity| entity.get_id() == id)
    }

    pub fn spawn_entity(&mut self, mut entity: Entity, pos: Vector3<f32>) {
        entity.pos = pos;
        self.loaded_entities.push(entity);
    }

    pub fn get_new_entity_id(&mut self) -> usize {
        let id = self.next_available_entity_id;
        self.next_available_entity_id += 1;
        id
    }

    pub fn clear_entities(&mut self) {
        if self.loaded_entities.len() > 1 {
            for _ in 1..self.loaded_entities.len() {
                self.loaded_entities.remove(1);
            }
        }
    }

    pub fn spawn_entity_auto(
        &mut self,
        entity_type: EntityType,
        pos: Vector3<f32>,
        custom_data: Option<Box<dyn Any>>,
    ) {
        let id = self.get_new_entity_id();
        self.spawn_entity(Entity::new(id, entity_type, custom_data), pos);
    }

    pub fn spawn_item_entity(&mut self, pos: Vector3<f32>, item_stack: ItemStack) {
        self.spawn_entity_auto(
            EntityType::Item,
            pos,
            Some(Box::new(ItemEntityCustomData { item_stack })),
        );
    }

    /// Replace a block. Its drop is spawned only if `can_drop` is true
    pub fn replace_block_and_drop_item(
        &mut self,
        pos: Vector3<isize>,
        block_type: BlockType,
        can_drop: bool,
    ) {
        if let Some(current_block) = self.chunks_manager.get_block_in_world(pos) {
            if matches!(current_block, BlockType::Chest | BlockType::Furnace) {
                self.drop_container_contents(pos);
            }

            // Leaves sometimes drop an apple, 1 time out of 10
            if current_block == BlockType::Leaves && get_random_in_range(0, 10) == 0 {
                self.spawn_item_entity(
                    pos.map(|v| v as f32 + 0.5),
                    ItemStack::new(ItemType::Apple, 1, false),
                );
            }

            let drop_type = current_block.get_dropped_item_type();
            if drop_type != ItemType::Air && can_drop {
                self.chunks_manager.set_block_in_world(pos, block_type);
                self.spawn_item_entity(
                    pos.map(|v| v as f32 + 0.5),
                    ItemStack::new(drop_type, 1, false),
                );
            }
        }
    }

    /// The inventory of the chest at this position. The chest is created if it does not exist yet
    pub fn get_chest_inventory_mut(&mut self, pos: Vector3<isize>) -> &mut Inventory {
        let key = (pos.x, pos.y, pos.z);
        let index = match self.chests.iter().position(|chest| chest.pos == key) {
            Some(index) => index,
            None => {
                self.chests.push(Chest {
                    pos: key,
                    inventory: Inventory::new(CHEST_SLOTS),
                });
                self.chests.len() - 1
            }
        };

        &mut self.chests[index].inventory
    }

    pub fn get_chests(&self) -> &Vec<Chest> {
        &self.chests
    }

    pub fn set_chests(&mut self, chests: Vec<Chest>) {
        self.chests = chests;
    }

    /// The furnace at this position. It is created if it does not exist yet
    pub fn get_furnace_mut(&mut self, pos: Vector3<isize>) -> &mut Furnace {
        let key = (pos.x, pos.y, pos.z);
        let index = match self.furnaces.iter().position(|furnace| furnace.pos == key) {
            Some(index) => index,
            None => {
                self.furnaces.push(Furnace::new(key));
                self.furnaces.len() - 1
            }
        };

        &mut self.furnaces[index]
    }

    pub fn update_furnaces(&mut self, delta_time: f32) {
        for furnace in self.furnaces.iter_mut() {
            furnace.update(delta_time);
        }
    }

    pub fn get_furnaces(&self) -> &Vec<Furnace> {
        &self.furnaces
    }

    pub fn set_furnaces(&mut self, furnaces: Vec<Furnace>) {
        self.furnaces = furnaces;
    }

    /// Remove the chest or furnace at this position and drop its items in the world
    pub fn drop_container_contents(&mut self, pos: Vector3<isize>) {
        let key = (pos.x, pos.y, pos.z);
        let inventory = if let Some(index) = self.chests.iter().position(|chest| chest.pos == key) {
            Some(self.chests.remove(index).inventory)
        } else if let Some(index) = self.furnaces.iter().position(|furnace| furnace.pos == key) {
            Some(self.furnaces.remove(index).inventory)
        } else {
            None
        };

        if let Some(inventory) = inventory {
            for item_stack in inventory.get_all_slots() {
                if item_stack.get_item_type() != ItemType::Air {
                    self.spawn_item_entity(pos.map(|v| v as f32 + 0.5), item_stack.clone());
                }
            }
        }
    }

    pub fn remove_entity(&mut self, id: usize) -> bool {
        for i in 0..self.loaded_entities.len() {
            if self.loaded_entities[i].get_id() == id {
                self.loaded_entities.remove(i);
                return true;
            }
        }
        false
    }

    pub fn get_highest_block(&self, x: isize, z: isize) -> isize {
        let mut highest_chunk_y = self.chunks_manager.chunks[0].get_pos().y;
        for chunk in self.chunks_manager.chunks.iter() {
            if chunk.get_pos().y > highest_chunk_y {
                highest_chunk_y = chunk.get_pos().y;
            }
        }
        let max_block = (highest_chunk_y + 1) * CHUNK_SIZE_I;
        let mut highest_block = max_block - 1;

        for y in (0..max_block).rev() {
            if !self
                .chunks_manager
                .get_block_in_world(Vector3::new(x, y, z))
                .is_some_and(|b| b.is_air())
            {
                highest_block = y + 1;
                break;
            }
        }
        highest_block
    }

    /// Clear all the chunks and entities
    pub fn clear(&mut self) {
        self.chunks_manager.clear();
        self.clear_entities();
    }
}
