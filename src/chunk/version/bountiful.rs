use napi::bindgen_prelude::Object;

pub struct Chunk {
    columns: Object
}

impl Chunk {
    pub fn new(columns: Object) -> Self {
        Self {
            columns
        }
    }

    pub fn get_block(&self, pos: &[f64; 3]) -> Option<u32> {
        todo!()
    }
}