use super::{class::Pop, tick::PopTickResultAccumulated};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PopCollection {
    #[serde(default)]
    pops: HashMap<u64, Pop>,
    #[serde(default)]
    next_index: u64,
}

impl PopCollection {
    pub fn add_pop(&mut self, pop: Pop) {
        let index = self.next_index;
        self.next_index += 1;
        self.pops.insert(index, pop);
    }

    pub fn tick(&mut self, tps: u64) -> PopTickResultAccumulated {
        self.pops
            .par_iter_mut()
            .map(|(_, pop)| pop.tick(tps))
            .fold(
                PopTickResultAccumulated::default,
                |mut result_acc, tick_result| {
                    result_acc.total_food_eaten = tick_result.food_eaten;
                    result_acc
                },
            )
            .reduce(
                PopTickResultAccumulated::default,
                |mut result_acc, other_acc| {
                    result_acc.combine(other_acc);
                    result_acc
                },
            )
    }

    pub fn is_empty(&self) -> bool {
        self.pops.is_empty()
    }
}
