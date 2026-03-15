use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_SET_EXPERIENCE;

/// Sent by the server to set the player's experience bar, level, and total points.
///
/// Vanilla: `ClientboundSetExperiencePacket`
/// Wire order: float progress, VarInt level, VarInt total.
#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_SET_EXPERIENCE)]
pub struct CSetExperience {
    /// Experience bar progress, between 0.0 and 1.0.
    pub experience_progress: f32,
    /// The player's experience level.
    #[write(as = VarInt)]
    pub experience_level: i32,
    /// The player's total experience points.
    #[write(as = VarInt)]
    pub total_experience: i32,
}
