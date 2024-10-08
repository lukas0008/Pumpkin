use pumpkin_macros::packet;
use serde::Serialize;

use crate::VarInt;

#[derive(Serialize)]
#[packet(0x58)]
pub struct CSetEntityMetadata {
    entity_id: VarInt,
    metadata: Metadata,
    end: u8,
}

impl CSetEntityMetadata {
    pub fn new(entity_id: VarInt, metadata: Metadata) -> Self {
        Self {
            entity_id,
            metadata,
            end: 255,
        }
    }
}

#[derive(Serialize)]
pub struct Metadata {
    index: u8,
    typ: VarInt,
    value: u8,
}

impl Metadata {
    pub fn new(index: u8, typ: VarInt, value: u8) -> Self {
        Self { index, typ, value }
    }
}
