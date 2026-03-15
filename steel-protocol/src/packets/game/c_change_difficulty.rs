use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_CHANGE_DIFFICULTY;

/// Sent by the server to inform the client of the current difficulty.
///
/// Vanilla: `ClientboundChangeDifficultyPacket`
#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_CHANGE_DIFFICULTY)]
pub struct CChangeDifficulty {
    /// 0 = Peaceful, 1 = Easy, 2 = Normal, 3 = Hard.
    #[write(as = VarInt)]
    pub difficulty: i32,
    /// Whether the difficulty is locked.
    pub locked: bool,
}
