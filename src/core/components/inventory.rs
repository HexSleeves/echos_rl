use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an item in the inventory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
pub struct InventoryItem {
    /// Unique identifier for the item type
    pub item_id: String,
    /// Display name of the item
    pub name: String,
    /// Number of this item in the stack
    pub quantity: u32,
    /// Maximum stack size for this item type
    pub max_stack: u32,
    /// Weight per individual item
    pub weight: f32,
    /// Item description
    pub description: String,
}

impl InventoryItem {
    /// Create a new inventory item
    pub fn new(
        item_id: String,
        name: String,
        quantity: u32,
        max_stack: u32,
        weight: f32,
        description: String,
    ) -> Self {
        Self { item_id, name, quantity: quantity.min(max_stack), max_stack, weight, description }
    }

    /// Get the total weight of this item stack
    pub fn total_weight(&self) -> f32 { self.weight * self.quantity as f32 }

    /// Check if this item can stack with another item
    pub fn can_stack_with(&self, other: &InventoryItem) -> bool {
        self.item_id == other.item_id && self.quantity < self.max_stack
    }

    /// Try to add quantity to this stack, returns the amount that couldn't be added
    pub fn add_quantity(&mut self, amount: u32) -> u32 {
        let can_add = (self.max_stack - self.quantity).min(amount);
        self.quantity += can_add;
        amount - can_add
    }

    /// Remove quantity from this stack, returns the amount actually removed
    pub fn remove_quantity(&mut self, amount: u32) -> u32 {
        let can_remove = self.quantity.min(amount);
        self.quantity -= can_remove;
        can_remove
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool { self.quantity == 0 }

    /// Check if the stack is full
    pub fn is_full(&self) -> bool { self.quantity >= self.max_stack }

    /// Split this stack into two, returning the new stack with the specified quantity
    pub fn split(&mut self, amount: u32) -> Option<InventoryItem> {
        if amount >= self.quantity {
            return None;
        }

        let split_amount = amount.min(self.quantity);
        self.quantity -= split_amount;

        Some(InventoryItem {
            item_id: self.item_id.clone(),
            name: self.name.clone(),
            quantity: split_amount,
            max_stack: self.max_stack,
            weight: self.weight,
            description: self.description.clone(),
        })
    }
}

/// Inventory component for entities that can carry items
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Inventory {
    /// Items stored in the inventory, indexed by slot
    pub items: HashMap<usize, InventoryItem>,
    /// Maximum number of item slots
    pub max_slots: usize,
    /// Maximum weight capacity
    pub max_weight: f32,
    /// Current total weight
    pub current_weight: f32,
}

impl Inventory {
    /// Create a new inventory with specified capacity
    pub fn new(max_slots: usize, max_weight: f32) -> Self {
        Self { items: HashMap::new(), max_slots, max_weight, current_weight: 0.0 }
    }

    /// Get the number of used slots
    pub fn used_slots(&self) -> usize { self.items.len() }

    /// Get the number of free slots
    pub fn free_slots(&self) -> usize { self.max_slots - self.used_slots() }

    /// Check if the inventory is full (no free slots)
    pub fn is_full(&self) -> bool { self.used_slots() >= self.max_slots }

    /// Check if the inventory is empty
    pub fn is_empty(&self) -> bool { self.items.is_empty() }

    /// Get the weight capacity percentage (0.0 to 1.0)
    pub fn weight_percentage(&self) -> f32 {
        if self.max_weight <= 0.0 { 0.0 } else { (self.current_weight / self.max_weight).clamp(0.0, 1.0) }
    }

    /// Check if adding an item would exceed weight capacity
    pub fn would_exceed_weight(&self, item: &InventoryItem) -> bool {
        self.current_weight + item.total_weight() > self.max_weight
    }

    /// Find the first empty slot
    pub fn find_empty_slot(&self) -> Option<usize> {
        (0..self.max_slots).find(|&slot| !self.items.contains_key(&slot))
    }

    /// Find a slot with a stackable item of the same type
    pub fn find_stackable_slot(&self, item: &InventoryItem) -> Option<usize> {
        for (&slot, existing_item) in &self.items {
            if existing_item.can_stack_with(item) {
                return Some(slot);
            }
        }
        None
    }

    /// Try to add an item to the inventory, returns the amount that couldn't be added
    pub fn add_item(&mut self, item: InventoryItem) -> Result<u32, InventoryError> {
        if item.quantity == 0 {
            return Ok(0);
        }

        // Check weight capacity
        if self.would_exceed_weight(&item) {
            return Err(InventoryError::ExceedsWeightLimit);
        }

        let mut remaining = item.quantity;

        // First, try to stack with existing items
        if let Some(slot) = self.find_stackable_slot(&item)
            && let Some(existing_item) = self.items.get_mut(&slot)
        {
            let old_weight = existing_item.total_weight();
            remaining = existing_item.add_quantity(remaining);
            let new_weight = existing_item.total_weight();
            self.current_weight += new_weight - old_weight;
        }

        // If there's still remaining quantity, try to add to empty slots
        while remaining > 0 {
            if let Some(slot) = self.find_empty_slot() {
                let stack_size = remaining.min(item.max_stack);
                let new_item = InventoryItem {
                    item_id: item.item_id.clone(),
                    name: item.name.clone(),
                    quantity: stack_size,
                    max_stack: item.max_stack,
                    weight: item.weight,
                    description: item.description.clone(),
                };

                self.current_weight += new_item.total_weight();
                self.items.insert(slot, new_item);
                remaining -= stack_size;
            } else {
                // No more slots available
                break;
            }
        }

        Ok(remaining)
    }

    /// Remove an item from a specific slot
    pub fn remove_item(
        &mut self,
        slot: usize,
        quantity: u32,
    ) -> Result<Option<InventoryItem>, InventoryError> {
        if let Some(item) = self.items.get_mut(&slot) {
            let old_weight = item.total_weight();
            let removed_quantity = item.remove_quantity(quantity);
            let new_weight = item.total_weight();
            self.current_weight -= old_weight - new_weight;

            if item.is_empty() {
                let removed_item = self.items.remove(&slot);
                Ok(removed_item)
            } else {
                Ok(Some(InventoryItem {
                    item_id: item.item_id.clone(),
                    name: item.name.clone(),
                    quantity: removed_quantity,
                    max_stack: item.max_stack,
                    weight: item.weight,
                    description: item.description.clone(),
                }))
            }
        } else {
            Err(InventoryError::SlotEmpty)
        }
    }

    /// Get an item from a specific slot (read-only)
    pub fn get_item(&self, slot: usize) -> Option<&InventoryItem> { self.items.get(&slot) }

    /// Move an item from one slot to another
    pub fn move_item(&mut self, from_slot: usize, to_slot: usize) -> Result<(), InventoryError> {
        if from_slot == to_slot {
            return Ok(());
        }

        if to_slot >= self.max_slots {
            return Err(InventoryError::InvalidSlot);
        }

        if let Some(from_item) = self.items.remove(&from_slot) {
            if let Some(to_item) = self.items.get_mut(&to_slot) {
                // Try to stack items
                if to_item.can_stack_with(&from_item) {
                    let old_weight = to_item.total_weight();
                    let remaining = to_item.add_quantity(from_item.quantity);
                    let new_weight = to_item.total_weight();
                    self.current_weight += new_weight - old_weight;

                    if remaining > 0 {
                        // Put the remaining back in the original slot
                        let mut remaining_item = from_item;
                        remaining_item.quantity = remaining;
                        self.items.insert(from_slot, remaining_item);
                    }
                } else {
                    // Swap items
                    let to_item = self.items.remove(&to_slot).unwrap();
                    self.items.insert(from_slot, to_item);
                    self.items.insert(to_slot, from_item);
                }
            } else {
                // Move to empty slot
                self.items.insert(to_slot, from_item);
            }
            Ok(())
        } else {
            Err(InventoryError::SlotEmpty)
        }
    }

    /// Count the total quantity of a specific item type
    pub fn count_item(&self, item_id: &str) -> u32 {
        self.items.values().filter(|item| item.item_id == item_id).map(|item| item.quantity).sum()
    }

    /// Check if the inventory contains at least the specified quantity of an item
    pub fn has_item(&self, item_id: &str, quantity: u32) -> bool { self.count_item(item_id) >= quantity }

    /// Clear all items from the inventory
    pub fn clear(&mut self) {
        self.items.clear();
        self.current_weight = 0.0;
    }

    /// Get all items as a vector (for iteration)
    pub fn get_all_items(&self) -> Vec<(usize, &InventoryItem)> {
        let mut items: Vec<_> = self.items.iter().map(|(&slot, item)| (slot, item)).collect();
        items.sort_by_key(|(slot, _)| *slot);
        items
    }

    /// Recalculate the current weight (useful for fixing inconsistencies)
    pub fn recalculate_weight(&mut self) {
        self.current_weight = self.items.values().map(|item| item.total_weight()).sum();
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(20, 100.0) // Default: 20 slots, 100.0 weight capacity
    }
}

/// Errors that can occur during inventory operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InventoryError {
    /// The inventory is full (no free slots)
    InventoryFull,
    /// Adding the item would exceed weight capacity
    ExceedsWeightLimit,
    /// The specified slot is empty
    SlotEmpty,
    /// The specified slot is invalid (out of bounds)
    InvalidSlot,
    /// The item cannot be stacked
    CannotStack,
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::InventoryFull => write!(f, "Inventory is full"),
            InventoryError::ExceedsWeightLimit => write!(f, "Would exceed weight limit"),
            InventoryError::SlotEmpty => write!(f, "Slot is empty"),
            InventoryError::InvalidSlot => write!(f, "Invalid slot"),
            InventoryError::CannotStack => write!(f, "Items cannot be stacked"),
        }
    }
}

impl std::error::Error for InventoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_item(id: &str, quantity: u32) -> InventoryItem {
        InventoryItem::new(
            id.to_string(),
            format!("Test {id}"),
            quantity,
            10,  // max stack
            1.0, // weight
            format!("Description for {id}"),
        )
    }

    #[test]
    fn test_inventory_creation() {
        let inventory = Inventory::new(10, 50.0);
        assert_eq!(inventory.max_slots, 10);
        assert_eq!(inventory.max_weight, 50.0);
        assert_eq!(inventory.current_weight, 0.0);
        assert!(inventory.is_empty());
        assert!(!inventory.is_full());
    }

    #[test]
    fn test_add_item() {
        let mut inventory = Inventory::new(5, 50.0);
        let item = create_test_item("sword", 1);

        let remaining = inventory.add_item(item).unwrap();
        assert_eq!(remaining, 0);
        assert_eq!(inventory.used_slots(), 1);
        assert_eq!(inventory.current_weight, 1.0);
    }

    #[test]
    fn test_item_stacking() {
        let mut inventory = Inventory::new(5, 50.0);
        let item1 = create_test_item("potion", 5);
        let item2 = create_test_item("potion", 3);

        inventory.add_item(item1).unwrap();
        inventory.add_item(item2).unwrap();

        assert_eq!(inventory.used_slots(), 1);
        assert_eq!(inventory.count_item("potion"), 8);
    }

    #[test]
    fn test_weight_limit() {
        let mut inventory = Inventory::new(10, 5.0);
        let heavy_item = InventoryItem::new(
            "boulder".to_string(),
            "Heavy Boulder".to_string(),
            1,
            1,
            10.0, // Exceeds weight limit
            "Very heavy".to_string(),
        );

        let result = inventory.add_item(heavy_item);
        assert!(matches!(result, Err(InventoryError::ExceedsWeightLimit)));
    }

    #[test]
    fn test_remove_item() {
        let mut inventory = Inventory::new(5, 50.0);
        let item = create_test_item("sword", 1);

        inventory.add_item(item).unwrap();
        let removed = inventory.remove_item(0, 1).unwrap();

        assert!(removed.is_some());
        assert!(inventory.is_empty());
        assert_eq!(inventory.current_weight, 0.0);
    }

    #[test]
    fn test_move_item() {
        let mut inventory = Inventory::new(5, 50.0);
        let item = create_test_item("sword", 1);

        inventory.add_item(item).unwrap();
        inventory.move_item(0, 2).unwrap();

        assert!(inventory.get_item(0).is_none());
        assert!(inventory.get_item(2).is_some());
    }

    #[test]
    fn test_item_operations() {
        let mut item = create_test_item("potion", 5);

        // Test adding quantity
        let remaining = item.add_quantity(3);
        assert_eq!(item.quantity, 8);
        assert_eq!(remaining, 0);

        // Test adding more than max stack
        let remaining = item.add_quantity(5);
        assert_eq!(item.quantity, 10); // Capped at max_stack
        assert_eq!(remaining, 3);

        // Test removing quantity
        let removed = item.remove_quantity(3);
        assert_eq!(item.quantity, 7);
        assert_eq!(removed, 3);

        // Test splitting
        let split_item = item.split(2).unwrap();
        assert_eq!(item.quantity, 5);
        assert_eq!(split_item.quantity, 2);
    }

    #[test]
    fn test_inventory_queries() {
        let mut inventory = Inventory::new(5, 50.0);
        let item1 = create_test_item("sword", 1);
        let item2 = create_test_item("potion", 5);

        inventory.add_item(item1).unwrap();
        inventory.add_item(item2).unwrap();

        assert!(inventory.has_item("sword", 1));
        assert!(inventory.has_item("potion", 5));
        assert!(!inventory.has_item("potion", 6));
        assert_eq!(inventory.count_item("sword"), 1);
        assert_eq!(inventory.count_item("potion"), 5);
        assert_eq!(inventory.count_item("nonexistent"), 0);
    }
}
