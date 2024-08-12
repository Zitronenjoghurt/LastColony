use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HousingBehavior {
    pub capacity: u32,
}
