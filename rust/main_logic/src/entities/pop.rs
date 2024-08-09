use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(GodotClass, Serialize, Deserialize)]
#[class(no_init)]
struct Pop {
    name: String,
}

#[godot_api]
impl Pop {
    #[func]
    fn from_json(json_string: String) -> Gd<Self> {
        let object: Pop = serde_json::from_str(&json_string)
            .expect("Deserialization Error: Failed to deserialize Pop data");
        Gd::from_object(object)
    }
}
