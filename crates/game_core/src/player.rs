//! Read model for UI/selection. Input submits simulation::GameCommand via CommandSink.
use crate::{coords::BlockPos, ids::PlayerId, inventory::SlotId};

/// Minimal block-address snapshot, not continuous motion/physics state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayerState {
    pub block_position: BlockPos,
    pub selected_slot: SlotId,
}

pub trait PlayerView {
    /// None means the player is unknown or not currently available.
    fn state(&self, player: PlayerId) -> Option<PlayerState>;
}
