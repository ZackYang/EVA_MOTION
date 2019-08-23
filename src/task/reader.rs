extern crate bitty;
use bitty::AsBits;
use std::collections::HashMap;
use super::message::Value;

pub struct Reader {
    pub bools: Vec<bool>,
    pub floats: Vec<f32>
}

impl Reader {
    pub fn new(data: &mut Vec<u8>) -> Reader {
        let mut bools = Vec::<bool>::new();
        for i in 0..2 {
            bools.append(&mut data[i].as_bits());
        }
        let mut floats = Vec::<f32>::new();
        let (_, right) = data.split_at(bools.len());
        for chunk in right.chunks(4) {
            let mut arr = [0u8; 4];
            arr.copy_from_slice(chunk);
            let float = f32::from_bits(u32::from_be_bytes(arr));
            floats.push(float);
        }

        Reader { bools: bools, floats: floats }
    }

    fn index(&self, key: &str) -> Result<usize, &'static str> {
        let pairs: HashMap<&str, i32> =
            [
                ("X_REST_STATE", 6),
                ("Y_REST_STATE", 5),
                ("X_TRIGGER",    4),
                ("Y_TRIGGER",    3),
                ("LIGHT",        7),
                // f32
                ("X_POSITION",   0),
                ("Y_POSITION",   1),
                ("SPEED",        2)
            ]
            .iter().cloned().collect();
        if pairs.contains_key(key) {
            Ok(pairs[key] as usize)
        } else {
            println!("Cannot found key: {:?}", key);
            Err("Found nothing")
        }
    }

    pub fn check(&self, key: &str, value: Value) -> bool {
        let index = match self.index(key) {
            Ok(i) => { i },
            Err(_e) => { return false }
        };
        match value {
            Value::Bool(v) => { self.bools[index] == v },
            Value::Float(v) => { self.floats[index] == v }
        }
    }
}