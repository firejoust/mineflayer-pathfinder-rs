use napi::bindgen_prelude::{Object, Buffer, Array};
use crate::chunk::{get_column_key, get_section_key};

// section: width, length, section height
const DIMENSIONS_F64: [f64; 3] = [16.0, 16.0, 16.0];
const DIMENSIONS_U32: [u32; 3] = [16, 16, 16];

#[napi(object)]
struct Section {
    pub data: Buffer
}

impl Section {
    pub fn get_block(&self, pos: [f64; 3]) -> u16 {
        let pos = self.pos_in_section(pos);
        let cursor = self.get_block_cursor(pos);
        self.read_u16_le(cursor)
    }

    fn pos_in_section(&self, pos: [f64; 3]) -> [u32; 3] {
        [
            pos[0].rem_euclid(DIMENSIONS_F64[0]) as u32,
            pos[1].rem_euclid(DIMENSIONS_F64[1]) as u32,
            pos[2].rem_euclid(DIMENSIONS_F64[2]) as u32
        ]
    }

    fn get_block_cursor(&self, pos: [u32; 3]) -> u32 {
        self.get_array_position(pos) * 2
    }

    fn get_array_position(&self, pos: [u32; 3]) -> u32 {
        pos[0] + DIMENSIONS_U32[0] * (pos[2] + DIMENSIONS_U32[1] * pos[1])
    }

    fn read_u16_le(&self, cursor: u32) -> u16 {
        let cursor = cursor as usize;
        u16::from(self.data[cursor]) + (u16::from(self.data[cursor + 1]) << 8)
    }
}

#[napi(object)]
struct Column {
    pub sections: Array
}

impl Column {
    fn get_section(&self, pos: [f64; 3]) -> Option<Section> {
        self.sections.get(
            get_section_key(pos, None)
        ).expect("Expected chunk section; got something else?")?
    }
}

pub struct Chunk {
    columns: Object
}

impl Chunk {
    pub fn new(columns: Object) -> Self {
        Self {
            columns
        }
    }

    fn get_column(&self, pos: [f64; 3]) -> Option<Column> {
        self.columns.get(
            get_column_key(pos)
        ).expect("Expected chunk column; got something else?")?
    }

    pub fn get_block(&self, pos: [f64; 3]) -> Option<u32> {
        let column = self.get_column(pos)?;
        let section = column.get_section(pos)?;
        Some(u32::from(section.get_block(pos)))
    }
}