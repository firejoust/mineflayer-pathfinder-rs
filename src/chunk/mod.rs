mod version;
mod common;

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

const LATEST_PROTOCOL: u32 = 759;

pub trait ChunkTrait {
    fn new(columns: Object, protocol_version: u32) -> Chunk;
    fn get_block(&self, pos: [f64; 3]) -> Option<u32>;
}

pub enum Chunk {
    Bountiful(bountiful::Chunk),
    _Combat(combat::Chunk),
    _Aquatic(aquatic::Chunk),
    _Pillage(pillage::Chunk),
    _Bees(bees::Chunk),
    _Nether(nether::Chunk),
    _Caves(caves::Chunk),
    _Cliffs(cliffs::Chunk)
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
            v if v <= LATEST_PROTOCOL => panic!("The latest chunk protocol has not yet been implemented"), // 1.19+ (will use the next available version)
            _ => panic!("Protocol version untested and therefore not supported")            // untested chunk implementation
        }
    }

    fn get_block(&self, pos: [f64; 3]) -> Option<u32> {
        match self {
            Chunk::Bountiful(chunk) => chunk.get_block(pos),
            Chunk::_Combat(_) => todo!(),
            Chunk::_Aquatic(_) => todo!(),
            Chunk::_Pillage(_) => todo!(),
            Chunk::_Bees(_) => todo!(),
            Chunk::_Nether(_) => todo!(),
            Chunk::_Caves(_) => todo!(),
            Chunk::_Cliffs(_) => todo!(),
        }
    }
}

pub fn get_column_key(pos: [f64; 3]) -> String {
    format!(
        "{},{}",
        (pos[0] as i32) >> 4,
        (pos[2] as i32) >> 4
    )
}

pub fn get_section_key(pos: [f64; 3], min_y: Option<f64>) -> Option<u32> {
    match min_y {
        Some(y) => u32::try_from((pos[1] - y) as i32 >> 4).ok(),
        None => u32::try_from((pos[1] as i32) >> 4).ok()
    }
}