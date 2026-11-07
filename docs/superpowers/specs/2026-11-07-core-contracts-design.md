# Core contracts — Issue #1

## Goal and boundaries
Freeze a small, Bevy-independent public API for parallel world, render, player,
inventory and persistence work. Implement IDs, coordinates and bounded section
data, not a world manager, generation, graphics, gameplay, networking or Issue #22.
Keep Bevy =0.20.1 and the existing workspace unchanged; add no dependencies.
The user's implementation request and supplied Issue #1 define this scope.

## Approach
Extend `game_core` with focused modules. Numeric newtypes preserve explicit content
IDs independently of ECS entities or registration order; a future registry must
assign and preserve these numbers. String registries and Bevy ECS wrappers would
introduce policy/dependencies unnecessary for these contracts.
Use `i32` global coordinates, Euclidean division and checked reconstruction.
Chunks are horizontal 16x16 columns, sections are 16x16x16. Y=-64..320 is inclusive;
section Y=20 has only its bottom layer accessible. Local indexing is X-fastest,
then Z, then Y. Invalid local/global reconstruction returns None, never wraps.
A small Section value is the transferable data boundary, not a chunk manager.

## Contracts
- `ids`: BlockId(u32), ItemId(u32), PlayerId(u64), explicit raw conversion;
  block zero is AIR, item emptiness is Option<ItemStack>.
- `coords`, `section`: BlockPos, ChunkPos, SectionPos, validated LocalBlockPos,
  owned Section with no mutable access to invalid cells.
- `simulation`: non-exhaustive GameCommand/GameEvent, BlockChanged, CommandSink.
  Commands express intent; only authoritative simulation may publish facts.
- `world`: WorldRead/WorldWrite with explicit out-of-bounds and unloaded errors.
- `render`: SectionRenderer consumes section data without owning simulation.
- `player`: PlayerView/PlayerState; player input writes to CommandSink.
- `inventory`: SlotId, nonzero ItemStack, InventoryRead/InventoryWrite.
- `persistence`: WorldMetadata (schema, seed, generator version), SectionStore.
  File formats, migrations and runtime implementations belong to future crates.

## Validation and integration
Write tests before implementation, observe missing-API/behavior failures, then
implement. Cover negative coordinates, integer extremes, inclusive Y limits,
partial top sections, local index bijection, stable IDs and nonzero stacks.
Compile examples demonstrate command consumption and independent trait implementors.
Run all four requested workspace checks; CI already runs the same commands.
Document the API and independent work streams; review before opening a PR into
main from feature/core-contracts. Never merge without user permission.
