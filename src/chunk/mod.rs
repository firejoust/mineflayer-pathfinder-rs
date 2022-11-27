mod version;

use napi::bindgen_prelude::Object;
// prismarine-chunk
use version::{
    bountiful, // 1.8
    combat,    // 1.9
    color,     // 1.12
    aquatic,   // 1.13
    pillage,   // 1.14
    bees,      // 1.15
    nether,    // 1.16
    caves      // 1.17
};

pub trait ChunkTrait {
    fn new(columns: Object, protocol_version: u32) -> Chunk;
}

enum Chunk {
    Bountiful(bountiful::ChunkColumn),
    Combat(combat::ChunkColumn),
    Color(color::ChunkColumn),
    Aquatic(aquatic::ChunkColumn),
    Pillage(pillage::ChunkColumn),
    Bees(bees::ChunkColumn),
    Nether(nether::ChunkColumn),
    Caves(caves::ChunkColumn)
}

impl ChunkTrait for Chunk {
    fn new(columns: Object, protocol_version: u32) -> Self {
        match protocol_version {
            v if v <= 47  => Chunk::Bountiful (bountiful::ChunkColumn::new()),
            v if v <= 110 => Chunk::Combat    (combat::ChunkColumn::new()),
            v if v <= 210 => Chunk::Color     (color::ChunkColumn::new()),
            v if v <= 404 => Chunk::Aquatic   (aquatic::ChunkColumn::new()),
            v if v <= 498 => Chunk::Pillage   (pillage::ChunkColumn::new()),
            v if v <= 578 => Chunk::Bees      (bees::ChunkColumn::new()),
            v if v <= 754 => Chunk::Nether    (nether::ChunkColumn::new()),
            v if v <= 759 => Chunk::Caves     (caves::ChunkColumn::new()),
            v => panic!("Protocol version '{}' not supported", v)
        }
    }
}

fn test() {
    let chunk = Chunk::new(todo!(), 3);
}