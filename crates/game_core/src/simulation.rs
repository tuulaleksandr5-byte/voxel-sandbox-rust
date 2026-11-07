//! Engine-neutral requests and authoritative facts. No simulation is implemented here.
use crate::{
    coords::BlockPos,
    ids::{BlockId, PlayerId},
    inventory::{ItemStack, SlotId},
};
use std::num::NonZeroU32;

/// Client intent, NOT permission to perform an operation.
/// The simulation validates identity, bounds, reach, content and inventory rules.
///
/// Consumers must leave room for future commands:
/// ```compile_fail
/// use game_core::simulation::GameCommand;
/// fn handle(command: GameCommand) {
///     match command {
///         GameCommand::BreakBlock { .. } => (),
///         GameCommand::PlaceBlock { .. } => (),
///         GameCommand::SelectSlot { .. } => (),
///         GameCommand::MoveStack { .. } => (),
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GameCommand {
    BreakBlock {
        player: PlayerId,
        position: BlockPos,
    },
    PlaceBlock {
        player: PlayerId,
        position: BlockPos,
        block: BlockId,
    },
    SelectSlot {
        player: PlayerId,
        slot: SlotId,
    },
    /// Move within this player's inventory; requires authoritative validation.
    MoveStack {
        player: PlayerId,
        from: SlotId,
        to: SlotId,
        quantity: NonZeroU32,
    },
}

/// A completed block mutation, emitted only when previous != current.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockChanged {
    pub position: BlockPos,
    pub previous: BlockId,
    pub current: BlockId,
}

/// Confirmed results, published by authoritative simulation after state changes.
/// Future variants do not require downstream exhaustive-match changes.
///
/// ```compile_fail
/// use game_core::simulation::GameEvent;
/// fn handle(event: GameEvent) {
///     match event {
///         GameEvent::BlockChanged(_) => (),
///         GameEvent::InventoryChanged { .. } => (),
///         GameEvent::SelectedSlotChanged { .. } => (),
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GameEvent {
    BlockChanged(BlockChanged),
    InventoryChanged {
        player: PlayerId,
        slot: SlotId,
        stack: Option<ItemStack>,
    },
    SelectedSlotChanged {
        player: PlayerId,
        slot: SlotId,
    },
}

/// Boundary for player input, UI or a future network adapter.
pub trait CommandSink {
    type Error;
    /// Accept for processing. Ok does not imply the simulation accepted the action.
    fn submit(&mut self, command: GameCommand) -> Result<(), Self::Error>;
}
