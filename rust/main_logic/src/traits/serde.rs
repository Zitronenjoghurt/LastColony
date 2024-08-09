use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub trait FromJsonString: Sized {
    fn from_json_string(json_string: &str) -> Self;
}

impl<T> FromJsonString for T
where
    T: DeserializeOwned,
{
    fn from_json_string(json_string: &str) -> Self {
        serde_json::from_str(json_string)
            .expect("Deserialization Error: Failed to deserialize data")
    }
}

pub trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl<T> ToJsonString for T
where
    T: Serialize,
{
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Serialization Error: Failed to serialize data")
    }
}
