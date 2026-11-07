//! World access shared by simulation, player queries and meshing.
use crate::{coords::BlockPos, ids::BlockId, simulation::BlockChanged};
use std::fmt;

/// Missing data must not silently be interpreted as air.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldAccessError {
    OutOfBounds,
    SectionUnavailable,
}
impl fmt::Display for WorldAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::OutOfBounds => "block is outside the world",
            Self::SectionUnavailable => "section is not available",
        })
    }
}
impl std::error::Error for WorldAccessError {}

/// Read-only access, including neighboring sections needed by the renderer.
pub trait WorldRead {
    fn block(&self, position: BlockPos) -> Result<BlockId, WorldAccessError>;
}

/// Low-level authoritative mutation; gameplay authorization belongs to simulation.
pub trait WorldWrite: WorldRead {
    /// Errors leave the world unchanged. Return None for an unchanged block,
    /// otherwise return the actual old/new values after applying the edit.
    /// Bounds are checked before availability; Y outside -64..=320 is OutOfBounds.
    fn set_block(
        &mut self,
        position: BlockPos,
        block: BlockId,
    ) -> Result<Option<BlockChanged>, WorldAccessError>;
}
