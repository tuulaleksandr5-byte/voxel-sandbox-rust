//! Consumer-side implementations prove the modules can integrate using core alone.
use game_core::{
    coords::{BlockPos, LocalBlockPos, SectionPos},
    ids::{BlockId, PlayerId},
    inventory::{InventoryRead, InventoryWrite, ItemStack, SlotId, SlotOutOfBounds},
    persistence::{SectionStore, WorldMetadata},
    player::{PlayerState, PlayerView},
    render::SectionRenderer,
    section::Section,
    simulation::{BlockChanged, CommandSink, GameCommand, GameEvent},
    world::{WorldAccessError, WorldRead, WorldWrite},
};
use std::convert::Infallible;

struct World(Section);
impl WorldRead for World {
    fn block(&self, position: BlockPos) -> Result<BlockId, WorldAccessError> {
        if !position.is_in_world() {
            return Err(WorldAccessError::OutOfBounds);
        }
        if position.section() != self.0.position() {
            return Err(WorldAccessError::SectionUnavailable);
        }
        self.0
            .get(position.local())
            .ok_or(WorldAccessError::OutOfBounds)
    }
}
impl WorldWrite for World {
    fn set_block(
        &mut self,
        position: BlockPos,
        block: BlockId,
    ) -> Result<Option<BlockChanged>, WorldAccessError> {
        let previous = self.block(position)?;
        if previous == block {
            return Ok(None);
        }
        self.0.set(position.local(), block).unwrap();
        Ok(Some(BlockChanged {
            position,
            previous,
            current: block,
        }))
    }
}

#[derive(Default)]
struct Storage {
    metadata: Option<WorldMetadata>,
    section: Option<Section>,
}
impl SectionStore for Storage {
    type Error = Infallible;
    fn load_metadata(&mut self) -> Result<Option<WorldMetadata>, Self::Error> {
        Ok(self.metadata)
    }
    fn save_metadata(&mut self, metadata: &WorldMetadata) -> Result<(), Self::Error> {
        self.metadata = Some(*metadata);
        Ok(())
    }
    fn load_section(&mut self, position: SectionPos) -> Result<Option<Section>, Self::Error> {
        Ok(self
            .section
            .as_ref()
            .filter(|s| s.position() == position)
            .cloned())
    }
    fn save_section(&mut self, section: &Section) -> Result<(), Self::Error> {
        self.section = Some(section.clone());
        Ok(())
    }
}

// Rendering can choose any backend without introducing it into game_core.
struct Renderer;
impl SectionRenderer for Renderer {
    type Error = Infallible;
    fn update_section(&mut self, _: &Section, _: &dyn WorldRead) -> Result<(), Self::Error> {
        Ok(())
    }
    fn remove_section(&mut self, _: SectionPos) -> Result<(), Self::Error> {
        Ok(())
    }
}
struct Player;
impl PlayerView for Player {
    fn state(&self, _: PlayerId) -> Option<PlayerState> {
        None
    }
}
impl CommandSink for Player {
    type Error = Infallible;
    fn submit(&mut self, _: GameCommand) -> Result<(), Self::Error> {
        Ok(())
    }
}
struct Inventory;
impl InventoryRead for Inventory {
    fn slot_count(&self) -> u16 {
        0
    }
    fn stack(&self, _: SlotId) -> Result<Option<ItemStack>, SlotOutOfBounds> {
        Err(SlotOutOfBounds)
    }
}
impl InventoryWrite for Inventory {
    fn set_stack(
        &mut self,
        _: SlotId,
        _: Option<ItemStack>,
    ) -> Result<Option<ItemStack>, SlotOutOfBounds> {
        Err(SlotOutOfBounds)
    }
}

#[test]
fn negative_world_edit_survives_the_public_snapshot_boundary() {
    let position = BlockPos::new(-1, 320, -17);
    let mut world = World(Section::new(position.section(), BlockId::AIR).unwrap());
    let command = GameCommand::PlaceBlock {
        player: PlayerId::new(1),
        position,
        block: BlockId::new(8),
    };
    let event = match command {
        GameCommand::PlaceBlock {
            position, block, ..
        } => GameEvent::BlockChanged(world.set_block(position, block).unwrap().unwrap()),
        _ => panic!("expected place intent"),
    };
    match event {
        GameEvent::BlockChanged(change) => {
            assert_eq!(change.position, BlockPos::new(-1, 320, -17));
            assert_eq!(change.previous, BlockId::AIR);
            assert_eq!(change.current, BlockId::new(8));
        }
        _ => panic!("expected authoritative block change"),
    }
    let metadata = WorldMetadata {
        schema_version: 1,
        world_seed: u64::MAX,
        generator_version: 7,
    };
    let mut store = Storage::default();
    store.save_metadata(&metadata).unwrap();
    store.save_section(&world.0).unwrap();
    let loaded = store
        .load_section(SectionPos::new(-1, 20, -2))
        .unwrap()
        .unwrap();
    assert_eq!(
        loaded.get(LocalBlockPos::new(15, 0, 15).unwrap()),
        Some(BlockId::new(8))
    );
    assert_eq!(store.load_metadata().unwrap(), Some(metadata));
    assert_eq!(
        world.block(BlockPos::new(-1, 321, -17)),
        Err(WorldAccessError::OutOfBounds)
    );
    assert_eq!(
        world.block(BlockPos::new(0, 320, 0)),
        Err(WorldAccessError::SectionUnavailable)
    );
    // Re-applying the same value must not invent a second mutation event.
    assert_eq!(world.set_block(position, BlockId::new(8)).unwrap(), None);
    Renderer.update_section(&loaded, &world).unwrap();
    Renderer.remove_section(loaded.position()).unwrap();
    assert!(Player.state(PlayerId::new(1)).is_none());
    assert_eq!(Inventory.slot_count(), 0);
}
