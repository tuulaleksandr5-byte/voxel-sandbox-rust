use game_core::coords::{LocalBlockPos, SectionPos};
use game_core::ids::BlockId;
use game_core::section::Section;

#[test]
fn section_fills_all_4096_cells_with_the_requested_block() {
    let position = SectionPos::new(-1, -4, 2);
    let fill = BlockId::new(7);
    let section = Section::new(position, fill).unwrap();
    assert_eq!(section.position(), position);
    for index in 0..4096 {
        assert_eq!(
            section.get(LocalBlockPos::from_index(index).unwrap()),
            Some(fill)
        );
    }
}

#[test]
fn writes_return_the_previous_value_without_affecting_other_cells() {
    let mut section = Section::new(SectionPos::new(0, 0, 0), BlockId::AIR).unwrap();
    let first = LocalBlockPos::new(0, 0, 0).unwrap();
    let last = LocalBlockPos::new(15, 15, 15).unwrap();
    assert_eq!(section.set(first, BlockId::new(1)), Some(BlockId::AIR));
    assert_eq!(section.set(last, BlockId::new(2)), Some(BlockId::AIR));
    assert_eq!(section.set(first, BlockId::new(3)), Some(BlockId::new(1)));
    assert_eq!(section.get(first), Some(BlockId::new(3)));
    assert_eq!(section.get(last), Some(BlockId::new(2)));
    assert_eq!(
        section.get(LocalBlockPos::new(1, 0, 0).unwrap()),
        Some(BlockId::AIR)
    );
    assert_eq!(
        section.get(LocalBlockPos::new(0, 1, 0).unwrap()),
        Some(BlockId::AIR)
    );
    assert_eq!(
        section.get(LocalBlockPos::new(0, 0, 1).unwrap()),
        Some(BlockId::AIR)
    );
}

#[test]
fn sections_and_their_clones_own_independent_blocks() {
    let position = SectionPos::new(-2, 3, 4);
    let original = Section::new(position, BlockId::new(5)).unwrap();
    let mut cloned = original.clone();
    let separate = Section::new(SectionPos::new(3, 3, 4), BlockId::new(5)).unwrap();
    assert_eq!(original, cloned);
    let local = LocalBlockPos::new(7, 8, 9).unwrap();
    assert_eq!(cloned.set(local, BlockId::new(6)), Some(BlockId::new(5)));
    assert_eq!(original.get(local), Some(BlockId::new(5)));
    assert_eq!(separate.get(local), Some(BlockId::new(5)));
    assert_eq!(cloned.get(local), Some(BlockId::new(6)));
    assert_ne!(original, cloned);
}

#[test]
fn upper_section_rejects_every_cell_above_world_height_320() {
    let mut section = Section::new(SectionPos::new(0, 20, 0), BlockId::new(7)).unwrap();
    for index in 0..4096 {
        let local = LocalBlockPos::from_index(index).unwrap();
        if local.y() == 0 {
            assert_eq!(section.get(local), Some(BlockId::new(7)));
            assert_eq!(section.set(local, BlockId::new(8)), Some(BlockId::new(7)));
            assert_eq!(section.get(local), Some(BlockId::new(8)));
        } else {
            assert_eq!(section.get(local), None);
            assert_eq!(section.set(local, BlockId::new(9)), None);
            assert_eq!(section.get(local), None);
        }
    }
    for z in 0..16 {
        for x in 0..16 {
            assert_eq!(
                section.get(LocalBlockPos::new(x, 0, z).unwrap()),
                Some(BlockId::new(8))
            );
        }
    }
}

#[test]
fn section_creation_rejects_out_of_world_and_unrepresentable_positions() {
    for position in [
        SectionPos::new(0, -5, 0),
        SectionPos::new(0, 21, 0),
        SectionPos::new(i32::MIN, 0, 0),
        SectionPos::new(i32::MAX, 0, 0),
        SectionPos::new(0, 0, i32::MIN),
        SectionPos::new(0, 0, i32::MAX),
        SectionPos::new(0, i32::MIN, 0),
        SectionPos::new(0, i32::MAX, 0),
    ] {
        assert_eq!(Section::new(position, BlockId::AIR), None, "{position:?}");
    }
    assert!(Section::new(SectionPos::new(-134_217_728, -4, 134_217_727), BlockId::AIR).is_some());
    assert!(Section::new(SectionPos::new(134_217_727, 20, -134_217_728), BlockId::AIR).is_some());
}
