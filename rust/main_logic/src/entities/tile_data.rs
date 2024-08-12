use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileData {
    pub is_buildable: bool,
    pub is_on_ground: bool,
}

#[derive(Debug, Default, Clone)]
pub struct TileDataMap {
    data: Arc<RwLock<HashMap<u64, TileData>>>,
}

impl TileDataMap {
    pub fn get_tile(&self, index: u64) -> Option<TileData> {
        self.data.read().unwrap().get(&index).copied()
    }

    pub fn get_buildable_tiles(&self, size: u64) -> Vec<bool> {
        let data = self.data.read().unwrap();
        let mut buildable_tiles = Vec::with_capacity(size as usize);
        for i in 0..size {
            let is_buildable = data
                .get(&i)
                .map_or(false, |tile_data| tile_data.is_buildable);
            buildable_tiles.push(is_buildable);
        }
        buildable_tiles
    }

    pub fn is_buildable(&self, index: u64) -> bool {
        self.data
            .read()
            .unwrap()
            .get(&index)
            .map(|tile| tile.is_buildable)
            .unwrap_or(false)
    }

    pub fn is_on_ground(&self, index: u64) -> bool {
        self.data
            .read()
            .unwrap()
            .get(&index)
            .map(|tile| tile.is_on_ground)
            .unwrap_or(false)
    }

    pub fn set_tile(&self, index: u64, tile_data: TileData) {
        self.data.write().unwrap().insert(index, tile_data);
    }

    pub fn set_buildable(&self, index: u64, is_buildable: bool) {
        let mut data = self.data.write().unwrap();
        data.entry(index).or_default().is_buildable = is_buildable;
    }

    pub fn set_on_ground(&self, index: u64, is_above_ground: bool) {
        let mut data = self.data.write().unwrap();
        data.entry(index).or_default().is_on_ground = is_above_ground;
    }
}

/*
impl Serialize for TileDataMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.read().unwrap().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TileDataMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = HashMap::deserialize(deserializer)?;
        Ok(TileDataMap {
            data: Arc::new(RwLock::new(data)),
        })
    }
}
*/
