use std::{collections::HashSet, sync::Arc};

use dashmap::DashMap;

#[derive(Debug, Default, Clone)]
pub struct HousingRegistrySlot {
    pub limit: u32,
    pub pop_indices: HashSet<u64>,
}

impl HousingRegistrySlot {
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            ..Default::default()
        }
    }

    pub fn exceeded_limit(&self) -> bool {
        self.pop_indices.len() > self.limit as usize
    }

    pub fn at_limit(&self) -> bool {
        self.pop_indices.len() >= self.limit as usize
    }

    pub fn has_pop(&self, index: &u64) -> bool {
        self.pop_indices.contains(index)
    }

    pub fn add_pop(&mut self, index: u64) -> bool {
        if self.at_limit() || self.has_pop(&index) {
            return false;
        }

        self.pop_indices.insert(index);
        true
    }

    pub fn remove_pop(&mut self, index: u64) -> bool {
        if !self.has_pop(&index) {
            false
        } else {
            self.pop_indices.remove(&index);
            true
        }
    }

    pub fn adjust_limit(&mut self, new_limit: u32) -> HashSet<u64> {
        self.limit = new_limit;
        self.remove_overflowing_pops()
    }

    pub fn remove_overflowing_pops(&mut self) -> HashSet<u64> {
        if !self.exceeded_limit() {
            return HashSet::new();
        }

        let overflow_count = self.pop_indices.len() - self.limit as usize;
        let mut removed = HashSet::with_capacity(overflow_count);

        while self.pop_indices.len() > self.limit as usize {
            if let Some(&pop) = self.pop_indices.iter().next() {
                self.pop_indices.remove(&pop);
                removed.insert(pop);
            }
        }

        removed
    }
}

#[derive(Debug, Default, Clone)]
pub struct HousingRegistry {
    slots: Arc<DashMap<u64, HousingRegistrySlot>>,
}

impl HousingRegistry {
    pub fn register_house(&mut self, object_index: u64, limit: u32) {
        self.slots
            .entry(object_index)
            .or_insert(HousingRegistrySlot::new(limit));
    }

    pub fn has_house(&self, object_index: u64) -> bool {
        self.slots.contains_key(&object_index)
    }

    pub fn has_pop_in_house(&self, object_index: u64, pop_index: u64) -> bool {
        if let Some(slot) = self.slots.get(&object_index) {
            slot.has_pop(&pop_index)
        } else {
            false
        }
    }

    /// Tries to register a pop in a given housing, returns true if successful,
    /// returns false if the housing limit was reached
    pub fn register_pop(&mut self, object_index: u64, pop_index: u64) -> bool {
        if let Some(mut slot) = self.slots.get_mut(&object_index) {
            slot.add_pop(pop_index)
        } else {
            false
        }
    }

    /// Will return a vector of pop indices that potentially became homeless
    pub fn adjust_limit(&mut self, object_index: u64, new_limit: u32) -> HashSet<u64> {
        if let Some(mut slot) = self.slots.get_mut(&object_index) {
            slot.adjust_limit(new_limit)
        } else {
            HashSet::new()
        }
    }

    /// Will return a vector of pop indices that are now homeless
    /// (if the given object_index was registered)
    pub fn remove_house(&mut self, object_index: u64) -> HashSet<u64> {
        self.slots
            .remove(&object_index)
            .map(|(_, slot)| slot.pop_indices)
            .unwrap_or_default()
    }

    /// Remove a pop index in a given house, returns true if the pop was removed,
    /// returns false if the pop was not found
    pub fn remove_pop(&mut self, object_index: u64, pop_index: u64) -> bool {
        if let Some(mut slot) = self.slots.get_mut(&object_index) {
            slot.remove_pop(pop_index)
        } else {
            false
        }
    }

    /// Replaces the house at the given index and returns the homeless pops
    /// or registers the house if the given index is still free.
    pub fn replace_or_register_house(&mut self, object_index: u64, limit: u32) -> HashSet<u64> {
        if self.slots.contains_key(&object_index) {
            let homeless_indices = self.remove_house(object_index);
            self.register_house(object_index, limit);
            homeless_indices
        } else {
            self.register_house(object_index, limit);
            HashSet::new()
        }
    }

    pub fn get_pops_in_house(&self, object_index: u64) -> HashSet<u64> {
        if let Some(slot) = self.slots.get(&object_index) {
            slot.pop_indices.clone()
        } else {
            HashSet::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HousingRegistry;

    #[test]
    fn test_register_house() {
        let mut registry = HousingRegistry::default();
        registry.register_house(1, 5);
        registry.register_house(2, 3);
        assert!(registry.slots.contains_key(&1));
        assert!(registry.slots.contains_key(&2));
        assert!(!registry.slots.contains_key(&3));
        assert!(registry.has_house(1));
        assert!(registry.has_house(2));
        assert!(!registry.has_house(3));
        assert_eq!(registry.slots.entry(1).or_default().limit, 5);
        assert_eq!(registry.slots.entry(2).or_default().limit, 3);
    }

    #[test]
    fn test_register_pop() {
        let mut registry = HousingRegistry::default();
        assert!(!registry.register_pop(1, 1));
        registry.register_house(1, 2);
        assert!(registry.register_pop(1, 1));
        assert!(!registry.register_pop(1, 1));
        assert!(registry.register_pop(1, 2));
        assert!(!registry.register_pop(1, 3));
        assert!(registry
            .slots
            .get(&1)
            .map_or(false, |slot| slot.pop_indices.contains(&1)));
        assert!(registry
            .slots
            .get(&1)
            .map_or(false, |slot| slot.pop_indices.contains(&2)));
        assert!(!registry
            .slots
            .get(&1)
            .map_or(false, |slot| slot.pop_indices.contains(&3)));
        assert!(registry.has_pop_in_house(1, 1));
        assert!(registry.has_pop_in_house(1, 2));
        assert!(!registry.has_pop_in_house(1, 3));
    }

    #[test]
    fn test_adjust_limit() {
        let mut registry = HousingRegistry::default();
        registry.register_house(1, 5);
        registry.register_pop(1, 1);
        registry.register_pop(1, 2);
        registry.register_pop(1, 3);
        registry.register_pop(1, 4);
        registry.register_pop(1, 5);
        let homeless_pops = registry.adjust_limit(1, 3);
        assert_eq!(homeless_pops.len(), 2);
        assert_eq!(
            registry
                .slots
                .get(&1)
                .map_or(0, |slot| slot.pop_indices.len()),
            3
        );
    }

    #[test]
    fn test_remove_house() {
        let mut registry = HousingRegistry::default();
        registry.register_house(1, 5);
        registry.register_pop(1, 1);
        registry.register_pop(1, 2);
        registry.register_pop(1, 3);
        registry.register_pop(1, 4);
        registry.register_pop(1, 5);
        let homeless_pops = registry.remove_house(1);
        assert_eq!(homeless_pops.len(), 5);
        assert!(!registry.has_house(1));
    }

    #[test]
    fn test_remove_pop() {
        let mut registry = HousingRegistry::default();
        registry.register_house(1, 5);
        registry.register_pop(1, 1);
        registry.register_pop(1, 2);
        registry.register_pop(1, 3);
        registry.register_pop(1, 4);
        registry.register_pop(1, 5);
        assert!(registry.has_pop_in_house(1, 3));
        assert!(registry.remove_pop(1, 3));
        assert!(!registry.remove_pop(1, 3));
        assert!(!registry.has_pop_in_house(1, 3));
    }

    #[test]
    fn test_replace_or_register_house() {
        let mut registry = HousingRegistry::default();
        let homeless_pops = registry.replace_or_register_house(1, 3);
        assert!(homeless_pops.is_empty());
        registry.register_pop(1, 1);
        registry.register_pop(1, 2);
        registry.register_pop(1, 3);
        let homeless_pops = registry.replace_or_register_house(1, 5);
        assert_eq!(homeless_pops.len(), 3);
    }
}
