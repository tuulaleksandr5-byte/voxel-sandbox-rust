//! Rendering boundary; concrete Bevy meshes/materials stay in game_render.
use crate::{coords::SectionPos, section::Section, world::WorldRead};

pub trait SectionRenderer {
    type Error;
    /// Rebuild a section using a coherent snapshot and read-only neighbor data.
    /// Implementors decide how to handle missing neighbors and schedule work;
    /// they must not change authoritative world state.
    fn update_section(
        &mut self,
        section: &Section,
        neighbors: &dyn WorldRead,
    ) -> Result<(), Self::Error>;
    fn remove_section(&mut self, position: SectionPos) -> Result<(), Self::Error>;
}
