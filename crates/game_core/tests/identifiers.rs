use game_core::ids::{BlockId, ItemId, PlayerId};
use std::collections::HashMap;

#[test]
fn content_ids_round_trip_without_truncation_or_registration_order() {
    let mut blocks = HashMap::new();
    for raw in [u32::MAX, 7, 0, 65_536] {
        blocks.insert(BlockId::new(raw), raw);
        assert_eq!(ItemId::new(raw).raw(), raw);
    }
    assert_eq!(blocks[&BlockId::AIR], 0);
    assert_eq!(blocks[&BlockId::new(u32::MAX)], u32::MAX);
    assert_eq!(blocks[&BlockId::new(65_536)], 65_536);
    for (id, raw) in blocks {
        assert_eq!(id.raw(), raw);
    }
    assert_eq!(PlayerId::new(u64::MAX).raw(), u64::MAX);
}
