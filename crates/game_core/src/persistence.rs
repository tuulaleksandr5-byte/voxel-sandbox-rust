//! Data-store boundary, not a file format, migration engine or complete world save.
use crate::{coords::SectionPos, section::Section};

/// A future save header. Persistence validates supported versions before loading.
/// Version numbers are assigned by the format/generator owners, not by core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldMetadata {
    pub schema_version: u32,
    pub world_seed: u64,
    pub generator_version: u32,
}

/// A store instance is scoped to one world. Implementations must load a section
/// with the requested position and preserve valid cells/IDs exactly.
/// Atomic writes, validation and serialization are implementation responsibilities.
pub trait SectionStore {
    type Error;
    /// None means no saved header; unsupported/corrupt headers are errors.
    fn load_metadata(&mut self) -> Result<Option<WorldMetadata>, Self::Error>;
    fn save_metadata(&mut self, metadata: &WorldMetadata) -> Result<(), Self::Error>;
    /// None means no saved section; corrupt or unsupported data is an error.
    fn load_section(&mut self, position: SectionPos) -> Result<Option<Section>, Self::Error>;
    fn save_section(&mut self, section: &Section) -> Result<(), Self::Error>;
}
