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
}