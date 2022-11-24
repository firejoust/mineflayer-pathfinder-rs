#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Object;

// columns by key is "Object" (generic)

#[napi(object)]
struct Column {
  pub data: Vec<u32>
}

#[napi(object)]
pub struct World {
  pub columns: Object
}

#[napi(object)]
pub struct Pathfinder {
  pub world: World
}

#[napi]
impl Pathfinder {
  fn get_column_key(&self, x: i32, z: i32) -> String {
    format!("{},{}", x, z)
  }
  
  fn get_block(&self, pos: &[f64; 3]) {
    todo!()
  }
}

#[napi]
pub fn inject(mut bot: Object) {
  bot.set("pathfinder", Pathfinder {
    world: bot.get("world")
    .expect("'world' is undefined!")
    .expect("'world' is not a valid object!")
  }).unwrap() // 3: error
}