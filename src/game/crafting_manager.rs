use crate::{
    constants::ItemType,
    inventory::{Inventory, ItemStack},
};

struct Craft {
    pattern: [[ItemType; 3]; 3],
    strict_shape: bool,
    result: ItemStack,
}

impl Craft {
    pub const fn new(data: &'static [u8]) -> Self {
        let mut pattern = [const { [ItemType::Air; 3] }; 3];

        // 9 bytes : pattern
        let mut index = 0;

        // We can't use a for loop because it is not available in const context
        while index < 9 {
            pattern[index % 3][index / 3] =
                ItemType::get_from_id(data[index]).expect("Invalid item id in craft.");
            index += 1;
        }

        // 1 byte : strict_shape
        let strict_shape = data[9] == 1;

        // 1 byte : result
        let result_type = ItemType::get_from_id(data[10]).expect("Invalid item id in craft.");

        // 1 byte : result amount
        let result_amount = data[11];

        let result = ItemStack::new(result_type, result_amount, false);

        Craft {
            pattern,
            strict_shape,
            result,
        }
    }

    pub fn matches(&self, grid: [[ItemType; 3]; 3]) -> bool {
        if self.strict_shape {
            // Get the tiniest possible rectangle in the grid

            // Get the top left corner x
            let mut x1 = 0;
            'scan: while x1 < 3 {
                for i in 0..3 {
                    if grid[x1][i] != ItemType::Air {
                        break 'scan;
                    }
                }
                x1 += 1;
            }
            if x1 == 3 {
                return false;
            }

            // Get the top left corner y
            let mut y1 = 0;
            'scan: while y1 < 3 {
                for i in 0..3 {
                    if grid[i][y1] != ItemType::Air {
                        break 'scan;
                    }
                }
                y1 += 1;
            }

            if y1 == 3 {
                return false;
            }

            // Get the bottom right corner x
            let mut x2 = 2;
            'scan: while x2 > x1 {
                for i in 0..3 {
                    if grid[x2][i] != ItemType::Air {
                        break 'scan;
                    }
                }
                x2 -= 1;
            }

            // Get the bottom right corner x
            let mut y2 = 2;
            'scan: while y2 > y1 {
                for i in 0..3 {
                    if grid[i][y2] != ItemType::Air {
                        break 'scan;
                    }
                }
                y2 -= 1;
            }

            let width = x2 - x1;
            let height = y2 - y1;

            // Check if it matches the shape
            for x in 0..3 {
                for y in 0..3 {
                    if x <= width && y <= height {
                        // Check if the pattern is right
                        if self.pattern[x][y] != grid[x1 + x][y1 + y] {
                            return false;
                        }
                    } else {
                        // Check if there is no items outside of the pattern
                        if self.pattern[x][y] != ItemType::Air {
                            return false;
                        }
                    }
                }
            }

            true
        } else {
            // The max size is 3*3
            let mut items_craft: heapless::Vec<ItemType, 9> = heapless::Vec::new();
            let mut items_grid: heapless::Vec<ItemType, 9> = heapless::Vec::new();

            for x in 0..3 {
                for y in 0..3 {
                    let item = self.pattern[x][y];
                    // 3 * 3 = 9 so if it fails, mathematics are broken
                    items_craft.push(item).unwrap();
                    let item = grid[x][y];
                    items_grid.push(item).unwrap();
                }
            }

            // Check if the items are the same
            // These loop and sub loops do 162 iterations but can be optimised. TODO
            for item in items_grid.iter() {
                let count_grid = items_grid.iter().filter(|v| *v == item).count();
                let count_craft = items_craft.iter().filter(|v| *v == item).count();

                if count_grid != count_craft {
                    return false;
                }
            }

            true
        }
    }
}

const CRAFTS: [Craft; 23] = [
    Craft::new(include_bytes!("../../target/crafts/planks.bin")),
    Craft::new(include_bytes!("../../target/crafts/stone.bin")),
    Craft::new(include_bytes!("../../target/crafts/grass.bin")),
    Craft::new(include_bytes!("../../target/crafts/crafting_table.bin")),
    Craft::new(include_bytes!("../../target/crafts/bricks.bin")),
    Craft::new(include_bytes!("../../target/crafts/stick.bin")),
    Craft::new(include_bytes!("../../target/crafts/wooden_pickaxe.bin")),
    Craft::new(include_bytes!("../../target/crafts/wooden_axe.bin")),
    Craft::new(include_bytes!("../../target/crafts/wooden_shovel.bin")),
    Craft::new(include_bytes!("../../target/crafts/stone_pickaxe.bin")),
    Craft::new(include_bytes!("../../target/crafts/stone_axe.bin")),
    Craft::new(include_bytes!("../../target/crafts/stone_shovel.bin")),
    Craft::new(include_bytes!("../../target/crafts/chest.bin")),
    Craft::new(include_bytes!("../../target/crafts/iron_pickaxe.bin")),
    Craft::new(include_bytes!("../../target/crafts/iron_axe.bin")),
    Craft::new(include_bytes!("../../target/crafts/iron_shovel.bin")),
    Craft::new(include_bytes!("../../target/crafts/gold_pickaxe.bin")),
    Craft::new(include_bytes!("../../target/crafts/gold_axe.bin")),
    Craft::new(include_bytes!("../../target/crafts/gold_shovel.bin")),
    Craft::new(include_bytes!("../../target/crafts/diamond_pickaxe.bin")),
    Craft::new(include_bytes!("../../target/crafts/diamond_axe.bin")),
    Craft::new(include_bytes!("../../target/crafts/diamond_shovel.bin")),
    Craft::new(include_bytes!("../../target/crafts/furnace.bin")),
];

pub struct CraftingManager {
    pub crafting_inventory_2x2: Inventory,
    pub crafting_inventory_3x3: Inventory,
    valid_craft_2x2: bool,
    valid_craft_3x3: bool,
}

impl CraftingManager {
    pub fn new() -> Self {
        let crafting_inventory_2x2 = Inventory::new(5);
        let crafting_inventory_3x3 = Inventory::new(10);
        CraftingManager {
            crafting_inventory_2x2,
            crafting_inventory_3x3,
            valid_craft_2x2: false,
            valid_craft_3x3: false,
        }
    }

    /// Update the result slot of a square crafting grid of `size` x `size` slots.
    /// The result slot is the slot just after the grid.
    fn update_grid(inventory: &mut Inventory, valid_craft: &mut bool, size: usize) {
        let result_slot = size * size;

        // Remove the recipies if the player picked up the item
        if *valid_craft
            && inventory.get_item_type_at_slot_index(result_slot).unwrap() == ItemType::Air
        {
            for i in 0..result_slot {
                inventory.take_one(i);
            }
        }

        let mut grid = [[ItemType::Air; 3]; 3];

        // The inventory slots indexes must be from 0 to result_slot - 1
        for x in 0..size {
            for y in 0..size {
                // If it fails, a cosmic particle just hit the calculators Ram! Incredible!
                grid[x][y] = inventory.get_item_type_at_slot_index(x + y * size).unwrap();
            }
        }

        // Check for all crafts to match with our grid
        if inventory.modified {
            let mut found_craft = false;
            for craft in CRAFTS {
                if craft.matches(grid) {
                    inventory.replace_slot_item_stack(result_slot, craft.result);
                    *valid_craft = true;
                    found_craft = true;
                    break;
                }
            }
            if !found_craft {
                inventory.replace_slot_item_stack(result_slot, ItemStack::void());
                *valid_craft = false;
            }
        }
    }

    pub fn update_2x2(&mut self) {
        Self::update_grid(
            &mut self.crafting_inventory_2x2,
            &mut self.valid_craft_2x2,
            2,
        );
    }

    pub fn update_3x3(&mut self) {
        Self::update_grid(
            &mut self.crafting_inventory_3x3,
            &mut self.valid_craft_3x3,
            3,
        );
    }

    /// Empty the 3x3 grid and forget its last craft. Used when the crafting table is closed.
    pub fn clear_3x3(&mut self) {
        self.crafting_inventory_3x3.fill(ItemStack::void());
        self.valid_craft_3x3 = false;
    }
}
