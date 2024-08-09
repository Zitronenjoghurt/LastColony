use super::pop::Pop;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PopCollection {
    #[serde(default)]
    pops: HashMap<u64, Pop>,
    #[serde(default)]
    last_index: u64,
}

impl PopCollection {
    pub fn add_pawn(&mut self, pop: Pop) {
        let index = self.last_index;
        self.last_index += 1;
        self.pops.insert(index, pop);
    }
}
