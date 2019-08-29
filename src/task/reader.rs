extern crate bitty;
use bitty::AsBits;
use std::collections::HashMap;
use super::message::Value;

pub struct Reader {
    pub bools: Vec<bool>,
    pub ints: Vec<u8>,
    pub floats: Vec<f32>
}

impl Reader {
    pub fn new(data: &mut Vec<u8>) -> Reader {
        let mut bools = Vec::<bool>::new();
        bools.append(&mut data[0].as_bits());
        bools.append(&mut data[14].as_bits());

        let mut ints = Vec::<u8>::new();

        ints.append(&mut data[18]);
        ints.append(&mut data[19]);

        let mut floats = Vec::<f32>::new();
        let indexes = vec![1usize, 5, 9, 13, 20, 24, 28, 32];

        for i in indexes {
            let arr = data.get(i..(i+4)).unwrap();
            let float = f32::from_bits(u32::from_be_bytes(arr));
            floats.push(float);
        }

        Reader { bools: bools, floats: floats, ints: ints }
    }

    fn index(&self, key: &str) -> Result<usize, &'static str> {
        let pairs: HashMap<&str, i32> =
            [
                ("X_REST_STATE", 6),
                ("Y_REST_STATE", 5),
                ("X_TRIGGER",    4),
                ("Y_TRIGGER",    3),
                ("LIGHT",        7),
                // int
                ("X_ERROR_CODE", 0),
                ("Y_ERROR_CODE", 1),
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
            Value::Int(v) => { self.ints[index] == v }
        }
    }
}