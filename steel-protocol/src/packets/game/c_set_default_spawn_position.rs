use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_SET_DEFAULT_SPAWN_POSITION;
use steel_utils::types::{BlockPos, Identifier};

/// Sent by the server to set the player's compass target and default respawn position.
///
/// Vanilla: `ClientboundSetDefaultSpawnPositionPacket`
/// Contains a `LevelData.RespawnData` which wraps `GlobalPos` (dimension + pos) + yaw + pitch.
#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_SET_DEFAULT_SPAWN_POSITION)]
pub struct CSetDefaultSpawnPosition {
    /// The dimension resource key (e.g., "minecraft:overworld").
    pub dimension: Identifier,
    /// The spawn position (packed i64).
    pub pos: BlockPos,
    /// The yaw angle at the spawn point.
    pub yaw: f32,
    /// The pitch angle at the spawn point.
    pub pitch: f32,
}
