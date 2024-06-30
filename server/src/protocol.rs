use bevy_eventwork::NetworkMessage;
use serde::{Deserialize, Serialize};

// SpawnMessage

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpawnMessage {
    pub player_name: String,
}

impl NetworkMessage for SpawnMessage {
    const NAME: &'static str = "balls_of_steel:SpawnMessage";
}

// DespawnMessage

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DespawnMessage;

impl NetworkMessage for DespawnMessage {
    const NAME: &'static str = "balls_of_steel:DespawnMessage";
}

// ChatMessage

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub message: String,
}

impl NetworkMessage for ChatMessage {
    const NAME: &'static str = "balls_of_steel:ChatMessage";
}
