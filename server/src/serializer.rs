use bevy_eventwork::{error::NetworkError, NetworkSerializer};

#[derive(Default)]
pub struct JsonSerializer;

impl NetworkSerializer for JsonSerializer {
    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, bevy_eventwork::error::NetworkError>
    where
        T: serde::Serialize {
        serde_json::to_vec(value).map_err(|_| NetworkError::Serialization)
    }

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, bevy_eventwork::error::NetworkError>
    where
        T: serde::de::Deserialize<'a> {
        serde_json::from_slice(bytes).map_err(|_| NetworkError::Serialization)
    }
}