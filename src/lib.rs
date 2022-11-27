#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Object;

mod chunk;

#[napi(object)]
struct BitArray {
  pub data: Vec<u32>
}

#[napi(object)]
pub struct Async {
  pub columns: Object
}

#[napi(object)]
pub struct World {
  #[napi(js_name="async")]
  pub _async: Async
}

#[napi(object)]
pub struct Options {
  #[napi(js_name="protocolVersion")]
  pub protocol_version: u32
}

#[napi(object)]
pub struct Pathfinder {
  pub world: World,
  pub options: Options
}

impl Pathfinder {
  fn get_column_key(&self, x: i32, z: i32) -> String {
    format!("{},{}", x, z)
  }
  
  fn get_block(&self, pos: &[f64; 3]) {
    let column: Object = self.world._async.columns.get(
      self.get_column_key(
        (pos[0] as i32) / 0x10,
        (pos[2] as i32) / 0x10
      )
    )
    .expect("Chunk hasn't loaded yet!") // this could be undefined if the chunk hasn't loaded.
    .expect("Chunk is not a valid object!");

  }
}

#[napi]
pub fn inject(mut bot: Object) {
  bot.set("pathfinder", Pathfinder {
    world: bot.get("world")
    .expect("'world' is undefined!")
    .expect("'world' is not a valid object!"),

    options: bot.get("options")
    .expect("'options' is undefined!")
    .expect("'options' is not a valid object!")
  }).unwrap() // unable to set object properties
}