//! Stable IDs assigned explicitly by content registries, never by ECS or load order.
//!
//! Persist the raw numbers and never reuse an assigned number for different content.
//! Registry lookup/unknown-ID validation belongs to the consuming subsystem.

/// Stable block identity. Zero is reserved for air.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BlockId(u32);

impl BlockId {
    pub const AIR: Self = Self(0);

    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Stable item identity. An empty slot is `None`, not a reserved ItemId.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ItemId(u32);

impl ItemId {
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Stable player identity allocated by the authoritative simulation, not an ECS entity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PlayerId(u64);

impl PlayerId {
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }
}
