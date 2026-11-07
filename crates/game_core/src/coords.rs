//! Integer world coordinates and the bounds shared by world storage.

/// Inclusive lower world height.
pub const WORLD_MIN_Y: i32 = -64;
/// Inclusive upper world height.
pub const WORLD_MAX_Y: i32 = 320;
/// Width, depth, and height of a section in blocks.
pub const SECTION_SIZE: i32 = 16;
/// Number of cells in a complete section.
pub const SECTION_VOLUME: usize = 4096;

/// A block position in world space.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn chunk(self) -> ChunkPos {
        ChunkPos::new(
            self.x.div_euclid(SECTION_SIZE),
            self.z.div_euclid(SECTION_SIZE),
        )
    }

    #[must_use]
    pub fn section(self) -> SectionPos {
        SectionPos::new(
            self.x.div_euclid(SECTION_SIZE),
            self.y.div_euclid(SECTION_SIZE),
            self.z.div_euclid(SECTION_SIZE),
        )
    }

    #[must_use]
    pub fn local(self) -> LocalBlockPos {
        LocalBlockPos {
            x: self.x.rem_euclid(SECTION_SIZE) as u8,
            y: self.y.rem_euclid(SECTION_SIZE) as u8,
            z: self.z.rem_euclid(SECTION_SIZE) as u8,
        }
    }

    #[must_use]
    pub fn is_in_world(self) -> bool {
        (WORLD_MIN_Y..=WORLD_MAX_Y).contains(&self.y)
    }
}

/// A horizontal 16 by 16 column of blocks.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    #[must_use]
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

/// A 16 by 16 by 16 section position, measured in sections.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SectionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl SectionPos {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Reconstructs a block, rejecting integer overflow and invalid world heights.
    #[must_use]
    pub fn block_at(self, local: LocalBlockPos) -> Option<BlockPos> {
        let block = BlockPos::new(
            self.x
                .checked_mul(SECTION_SIZE)?
                .checked_add(i32::from(local.x))?,
            self.y
                .checked_mul(SECTION_SIZE)?
                .checked_add(i32::from(local.y))?,
            self.z
                .checked_mul(SECTION_SIZE)?
                .checked_add(i32::from(local.z))?,
        );
        block.is_in_world().then_some(block)
    }

    /// Whether the horizontal span is representable and any height is in the world.
    #[must_use]
    pub fn is_in_world(self) -> bool {
        let size = i64::from(SECTION_SIZE);
        let x = i64::from(self.x) * size;
        let y = i64::from(self.y) * size;
        let z = i64::from(self.z) * size;
        let min = i64::from(i32::MIN);
        let max = i64::from(i32::MAX);
        x >= min
            && x + size - 1 <= max
            && z >= min
            && z + size - 1 <= max
            && y <= i64::from(WORLD_MAX_Y)
            && y + size > i64::from(WORLD_MIN_Y)
    }
}

/// A validated position within a section, with each axis in `0..16`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LocalBlockPos {
    x: u8,
    y: u8,
    z: u8,
}

impl LocalBlockPos {
    #[must_use]
    pub const fn new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < SECTION_SIZE as u8 && y < SECTION_SIZE as u8 && z < SECTION_SIZE as u8 {
            Some(Self { x, y, z })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn x(self) -> u8 {
        self.x
    }

    #[must_use]
    pub const fn y(self) -> u8 {
        self.y
    }

    #[must_use]
    pub const fn z(self) -> u8 {
        self.z
    }

    /// Linear cell index: X varies first, then Z, then Y.
    #[must_use]
    pub const fn index(self) -> usize {
        self.x as usize
            + SECTION_SIZE as usize * self.z as usize
            + (SECTION_SIZE * SECTION_SIZE) as usize * self.y as usize
    }

    #[must_use]
    pub const fn from_index(index: usize) -> Option<Self> {
        if index >= SECTION_VOLUME {
            return None;
        }
        let size = SECTION_SIZE as usize;
        Some(Self {
            x: (index % size) as u8,
            y: (index / (size * size)) as u8,
            z: (index / size % size) as u8,
        })
    }
}
