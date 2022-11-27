mod version;

use napi::bindgen_prelude::Object;

// prismarine-chunk
use version::{
    bountiful, // 1.8
    combat,    // 1.9
    aquatic,   // 1.13
    pillage,   // 1.14
    bees,      // 1.15
    nether,    // 1.16
    caves,     // 1.17
    cliffs     // 1.18
};

trait ChunkTrait {
    fn new(columns: Object, protocol_version: u32) -> Chunk;
}

pub enum Chunk {
    Bountiful(bountiful::Chunk),
    Combat(combat::Chunk),
    Aquatic(aquatic::Chunk),
    Pillage(pillage::Chunk),
    Bees(bees::Chunk),
    Nether(nether::Chunk),
    Caves(caves::Chunk),
    Cliffs(cliffs::Chunk)
}

impl ChunkTrait for Chunk {
    fn new(columns: Object, protocol_version: u32) -> Self {
        match protocol_version {
            v if v <= 47  => Chunk::Bountiful(bountiful::Chunk::new(columns)),         // 1.8
            v if v <= 110 => panic!("Protocol version '{}' not supported (1.9+)", v),  // 1.9
            v if v <= 404 => panic!("Protocol version '{}' not supported (1.13+)", v), // 1.13
            v if v <= 498 => panic!("Protocol version '{}' not supported (1.14+)", v), // 1.14
            v if v <= 578 => panic!("Protocol version '{}' not supported (1.15+)", v), // 1.15
            v if v <= 754 => panic!("Protocol version '{}' not supported (1.16+)", v), // 1.16
            v if v <= 756 => panic!("Protocol version '{}' not supported (1.17+)", v), // 1.17
            v if v <= 758 => panic!("Protocol version '{}' not supported (1.18+)", v), // 1.18
            v => panic!("Protocol version '{}' not supported", v)                      // untested chunk implementation
        }
    }
}

fn test() {
    let chunk = Chunk::new(todo!(), 3);
}