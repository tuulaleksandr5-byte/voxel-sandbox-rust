//! Bounded block storage for one section.

use crate::coords::{LocalBlockPos, SECTION_SIZE, SECTION_VOLUME, SectionPos, WORLD_MAX_Y};
use crate::ids::BlockId;

/// An owned section whose public accessors enforce the world height bounds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section {
    position: SectionPos,
    blocks: [BlockId; SECTION_VOLUME],
}

impl Section {
    /// Fills the valid cells, leaving cells above the world height inaccessible.
    #[must_use]
    pub fn new(position: SectionPos, fill: BlockId) -> Option<Self> {
        if !position.is_in_world() {
            return None;
        }

        let mut blocks = [BlockId::AIR; SECTION_VOLUME];
        // The lower world bound is section-aligned; only the upper section is partial.
        let valid_layers = (WORLD_MAX_Y - position.y * SECTION_SIZE + 1).min(SECTION_SIZE);
        let valid_cells = valid_layers as usize * (SECTION_SIZE * SECTION_SIZE) as usize;
        blocks[..valid_cells].fill(fill);
        Some(Self { position, blocks })
    }

    #[must_use]
    pub const fn position(&self) -> SectionPos {
        self.position
    }

    /// Returns no block for cells above the inclusive upper world height.
    #[must_use]
    pub fn get(&self, local: LocalBlockPos) -> Option<BlockId> {
        self.position.block_at(local)?;
        Some(self.blocks[local.index()])
    }

    /// Replaces a valid cell and returns its previous block; invalid cells are untouched.
    pub fn set(&mut self, local: LocalBlockPos, block: BlockId) -> Option<BlockId> {
        self.position.block_at(local)?;
        Some(std::mem::replace(&mut self.blocks[local.index()], block))
    }
}
