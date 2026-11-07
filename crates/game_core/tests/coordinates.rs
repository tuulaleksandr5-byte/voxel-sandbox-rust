use game_core::coords::{BlockPos, ChunkPos, LocalBlockPos, SectionPos};

#[test]
fn negative_axes_use_euclidean_section_and_local_coordinates() {
    let cases = [
        (BlockPos::new(0, 0, 0), (0, 0, 0), (0, 0, 0)),
        (BlockPos::new(15, 15, 15), (0, 0, 0), (15, 15, 15)),
        (BlockPos::new(-1, -1, -1), (-1, -1, -1), (15, 15, 15)),
        (BlockPos::new(-16, -16, -16), (-1, -1, -1), (0, 0, 0)),
        (BlockPos::new(-17, -17, -17), (-2, -2, -2), (15, 15, 15)),
        (BlockPos::new(-17, 320, 31), (-2, 20, 1), (15, 0, 15)),
    ];

    for (block, section, local) in cases {
        assert_eq!(block.chunk(), ChunkPos::new(section.0, section.2));
        assert_eq!(
            block.section(),
            SectionPos::new(section.0, section.1, section.2)
        );
        assert_eq!(
            block.local(),
            LocalBlockPos::new(local.0, local.1, local.2).unwrap()
        );
    }
}

#[test]
fn block_height_bounds_include_both_endpoints() {
    for (y, valid) in [
        (i32::MIN, false),
        (-65, false),
        (-64, true),
        (0, true),
        (319, true),
        (320, true),
        (321, false),
        (i32::MAX, false),
    ] {
        assert_eq!(BlockPos::new(i32::MIN, y, i32::MAX).is_in_world(), valid);
    }
}

#[test]
fn local_coordinates_reject_each_out_of_range_axis() {
    for (x, y, z) in [(16, 0, 0), (0, 16, 0), (0, 0, 16), (255, 255, 255)] {
        assert_eq!(LocalBlockPos::new(x, y, z), None);
    }
    let local = LocalBlockPos::new(15, 14, 13).unwrap();
    assert_eq!((local.x(), local.y(), local.z()), (15, 14, 13));
}

#[test]
fn local_index_places_x_first_then_z_then_y() {
    for ((x, y, z), index) in [
        ((0, 0, 0), 0),
        ((15, 0, 0), 15),
        ((0, 0, 1), 16),
        ((0, 1, 0), 256),
        ((1, 2, 3), 561),
        ((15, 15, 15), 4095),
    ] {
        let local = LocalBlockPos::new(x, y, z).unwrap();
        assert_eq!(local.index(), index);
        assert_eq!(LocalBlockPos::from_index(index), Some(local));
    }
}

#[test]
fn local_indices_form_a_bijection_over_all_4096_cells() {
    let mut seen = [false; 4096];
    for y in 0..16 {
        for z in 0..16 {
            for x in 0..16 {
                let local = LocalBlockPos::new(x, y, z).unwrap();
                let index = local.index();
                assert!(index < 4096);
                assert!(!seen[index]);
                seen[index] = true;
                assert_eq!(LocalBlockPos::from_index(index), Some(local));
            }
        }
    }
    assert!(seen.into_iter().all(|visited| visited));
    assert_eq!(LocalBlockPos::from_index(4096), None);
    assert_eq!(LocalBlockPos::from_index(usize::MAX), None);
}

#[test]
fn valid_blocks_reconstruct_at_negative_and_integer_extreme_coordinates() {
    for x in [i32::MIN, i32::MIN + 15, -17, -16, -1, 0, 15, 16, i32::MAX] {
        for z in [i32::MIN, -17, -1, 0, 16, i32::MAX] {
            for y in [-64, -17, -16, -1, 0, 15, 16, 319, 320] {
                let block = BlockPos::new(x, y, z);
                assert_eq!(block.section().block_at(block.local()), Some(block));
            }
        }
    }
    assert_eq!(
        BlockPos::new(i32::MIN, i32::MIN, i32::MAX).section(),
        SectionPos::new(-134_217_728, -134_217_728, 134_217_727)
    );
    assert_eq!(
        BlockPos::new(i32::MIN, i32::MAX, i32::MAX).local(),
        LocalBlockPos::new(0, 15, 15).unwrap()
    );
}

#[test]
fn section_reconstruction_rejects_overflow_and_invalid_world_heights() {
    let origin = LocalBlockPos::new(0, 0, 0).unwrap();
    let far_corner = LocalBlockPos::new(15, 0, 15).unwrap();
    assert_eq!(
        SectionPos::new(-134_217_728, -4, -134_217_728).block_at(origin),
        Some(BlockPos::new(i32::MIN, -64, i32::MIN))
    );
    assert_eq!(
        SectionPos::new(134_217_727, 20, 134_217_727).block_at(far_corner),
        Some(BlockPos::new(i32::MAX, 320, i32::MAX))
    );

    for section in [
        SectionPos::new(-134_217_729, 0, 0),
        SectionPos::new(134_217_728, 0, 0),
        SectionPos::new(0, 0, -134_217_729),
        SectionPos::new(0, 0, 134_217_728),
        SectionPos::new(i32::MIN, 0, i32::MAX),
        SectionPos::new(0, -5, 0),
        SectionPos::new(0, 21, 0),
        SectionPos::new(0, i32::MIN, 0),
        SectionPos::new(0, i32::MAX, 0),
    ] {
        assert!(!section.is_in_world(), "{section:?}");
        assert_eq!(section.block_at(origin), None, "{section:?}");
    }
}

#[test]
fn upper_section_exposes_only_world_height_320() {
    let lower = SectionPos::new(-1, -4, 1);
    let upper = SectionPos::new(-1, 20, 1);
    assert!(lower.is_in_world());
    assert!(upper.is_in_world());
    assert_eq!(
        lower.block_at(LocalBlockPos::new(15, 0, 15).unwrap()),
        Some(BlockPos::new(-1, -64, 31))
    );
    assert_eq!(
        upper.block_at(LocalBlockPos::new(15, 0, 15).unwrap()),
        Some(BlockPos::new(-1, 320, 31))
    );
    for y in 1..16 {
        assert_eq!(upper.block_at(LocalBlockPos::new(15, y, 15).unwrap()), None);
    }
}
