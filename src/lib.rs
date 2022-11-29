#[macro_use]
extern crate napi_derive;

use chunk::{ChunkTrait, Chunk};
use napi::bindgen_prelude::Object;

mod chunk;

#[napi(object)]
pub struct Async {
  pub columns: Object
}

#[napi(object)]
pub struct World {
  #[napi(js_name="async")]
  pub _async: Async
}

#[napi]
pub struct Plugin {
  chunk: Chunk
}

#[napi]
impl Plugin {
  #[napi(constructor)]
  pub fn new(columns: Object, protocol_version: u32) -> Self {
    Self {
      chunk: Chunk::new(columns, protocol_version)
    }
  }

  #[napi(js_name="getBlock")]
  pub fn get_block(&self, x: f64, y: f64, z: f64) -> u32 {
    match self.chunk.get_block([x, y, z]) {
      Some(v) => v,
      None => 0
    }
  }
}

#[napi]
pub fn inject(mut bot: Object) {
  let world: World = bot.get("world").unwrap().unwrap();
  let protocol_version = bot.get("protocolVersion").unwrap().unwrap();
  bot.set("pathfinder", Plugin::new(
    world._async.columns,
    protocol_version
  )).unwrap();
}