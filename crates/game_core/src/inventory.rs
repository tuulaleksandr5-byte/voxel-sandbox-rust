//! Inventory data boundaries; stacking limits and transactions belong to game_items.
use crate::ids::ItemId;
use std::{fmt, num::NonZeroU32};

/// Zero-based slot index, scoped to a particular inventory.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SlotId(pub u16);

/// A nonempty stack. Item-specific maximum counts are validated by game_items.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ItemStack {
    item: ItemId,
    count: NonZeroU32,
}

impl ItemStack {
    #[must_use]
    pub const fn new(item: ItemId, count: u32) -> Option<Self> {
        match NonZeroU32::new(count) {
            Some(count) => Some(Self { item, count }),
            None => None,
        }
    }

    #[must_use]
    pub const fn item(self) -> ItemId {
        self.item
    }

    #[must_use]
    pub const fn count(self) -> NonZeroU32 {
        self.count
    }
}

/// A slot index is outside this inventory's capacity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SlotOutOfBounds;

impl fmt::Display for SlotOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("slot index is outside the inventory")
    }
}
impl std::error::Error for SlotOutOfBounds {}

/// Read a single inventory. Valid indices are `0..slot_count()`.
/// `Ok(None)` means an empty slot; an invalid index is an error.
pub trait InventoryRead {
    fn slot_count(&self) -> u16;
    fn stack(&self, slot: SlotId) -> Result<Option<ItemStack>, SlotOutOfBounds>;
}

/// Authoritative inventory mutation, not a client-facing permission boundary.
pub trait InventoryWrite: InventoryRead {
    /// Validate the slot before writing and return the old stack. Callers validate
    /// item-specific limits and gameplay rules before invoking this primitive.
    /// An error must leave the inventory unchanged. Cross-slot/world transactions
    /// are the simulation's responsibility.
    fn set_stack(
        &mut self,
        slot: SlotId,
        stack: Option<ItemStack>,
    ) -> Result<Option<ItemStack>, SlotOutOfBounds>;
}
