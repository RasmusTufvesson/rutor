use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Packet {
    Joined,
    Left,
    OnJoin(u32, bool), // id, other player exists
}

impl Packet {
    pub fn to_slice(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        bincode::deserialize(slice).unwrap()
    }
}