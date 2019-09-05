extern crate bitty;
use bitty::{AsBits, FromBits};
use std::collections::HashMap;
use super::message::Value;

#[derive(Clone)]
pub enum ValueType {
    Bool,
    Int,
    Float
}

pub struct Reader {
    pub bools: Vec<bool>,
    pub ints: Vec<u8>,
    pub floats: Vec<f32>,
    pub indexes: HashMap<&'static str, (usize, usize, ValueType)>,
    pub keys: Vec<&'static str>
}

impl Reader {
    pub fn new(data: Vec<(&'static str, ValueType)>) -> Reader {
        let pairs: HashMap<&'static str, (usize, usize, ValueType)> = HashMap::new();
        let mut reader = Reader {
            bools:  Vec::<bool>::new(),
            floats: Vec::<f32>::new(),
            ints:   Vec::<u8>::new(),
            indexes: pairs,
            keys:  Vec::<&'static str>::new()
        };

        let mut current_index = 0usize;
        let mut bool_index = 0usize;
        let mut int_index = 0usize;
        let mut float_index = 0usize;

        for (key, value_type) in data {
            let mut new_key = key;
            let tmp_key = format!("KEY_{:?}", current_index).clone();
            if new_key == "_" {
                new_key = Box::leak(tmp_key.into_boxed_str());
            }
            reader.keys.push(new_key);
            match value_type {
                ValueType::Float => {
                    reader.indexes.insert(new_key, (current_index, float_index, value_type.clone()));
                    current_index += 32;
                    float_index += 1;
                },
                ValueType::Int => {
                    reader.indexes.insert(new_key, (current_index, int_index, value_type.clone()));
                    current_index += 8;
                    int_index += 1;
                },
                ValueType::Bool => {
                    reader.indexes.insert(new_key, (current_index, bool_index, value_type.clone()));
                    current_index += 1;
                    bool_index += 1
                },
            }
        }

        reader
    }

    pub fn load(&mut self, data: &mut Vec<u8>) {
        self.bools  = Vec::<bool>::new();
        self.floats = Vec::<f32>::new();
        self.ints   = Vec::<u8>::new();

        let mut bits = Vec::<bool>::new();
        for int in data {
            let mut value = int.as_bits();
            value.reverse();
            bits.append(&mut value);
        }

        for key in self.keys.clone() {
            let (index, _, value_type) = self.indexes.get(key).unwrap();
            let index = *index;
            match value_type {
                ValueType::Float => {
                    let float_bits = bits.get(index..index+32).unwrap();
                    let mut float_u8s = Vec::<u8>::new();
                    for chunk in float_bits.to_vec().chunks(8) {
                        let mut _bits = chunk.to_vec();
                        _bits.reverse();
                        float_u8s.push(u8::from_bits(&_bits));
                    };
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(&float_u8s);
                    let float = f32::from_bits(u32::from_be_bytes(arr));
                    self.floats.push(float);
                },
                ValueType::Int => {
                    let int_bits = bits.get(index..index+8).unwrap();
                    let mut _bits = int_bits.to_vec();
                    _bits.reverse();
                    let int = u8::from_bits(&_bits);
                    self.ints.push(int);
                },
                ValueType::Bool => {
                    self.bools.push(bits[index]);
                },
            }
        }

        println!("{:?}", self.bools);
    }

    fn index(&self, key: &str) -> Result<usize, &'static str> {
        if self.indexes.contains_key(key) {
            Ok(self.indexes[key].1 as usize)
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

#[cfg(test)]
mod tests {
    use super::ValueType;
    use super::Reader;
    use super::super::message::Value;

    #[test]
    fn test_index() {
        let keys = vec![
            ("_",              ValueType::Bool),
            ("SHOULD_BE_B1",   ValueType::Bool),
            ("SHOULD_BE_B2",   ValueType::Bool),
            ("SHOULD_BE_F0",   ValueType::Float),
            ("SHOULD_BE_I0",   ValueType::Int),
            ("_",              ValueType::Bool),
            ("SHOULD_BE_B4",   ValueType::Bool),

        ];

        let reader = Reader::new(keys);

        assert_eq!(reader.index("SHOULD_BE_B1").unwrap(), 1);
        assert_eq!(reader.index("SHOULD_BE_B2").unwrap(), 2);
        assert_eq!(reader.index("SHOULD_BE_F0").unwrap(), 0);
        assert_eq!(reader.index("SHOULD_BE_I0").unwrap(), 0);
        assert_eq!(reader.index("SHOULD_BE_B4").unwrap(), 4);
    }

    #[test]
    fn test_load() {
        let keys = vec![
            ("_",              ValueType::Bool),
            ("_",              ValueType::Bool),
            ("_",              ValueType::Bool),
            ("_",              ValueType::Bool),
            ("D",              ValueType::Bool),
            ("C",              ValueType::Bool),
            ("B",              ValueType::Bool),
            ("A",              ValueType::Bool),
            ("FLOAT",          ValueType::Float),
            ("INT",            ValueType::Int)
        ];

        let mut reader = Reader::new(keys);

        let mut data = vec![2u8];
        let mut float_bytes = (0.15625_f32).to_bits().to_be_bytes().to_vec();
        data.append(&mut float_bytes);
        let mut int_bytes = (214u8).to_be_bytes().to_vec();
        data.append(&mut int_bytes);

        reader.load(&mut data);
        assert_eq!(reader.check("A",        Value::Bool(false)), true);
        assert_eq!(reader.check("B",        Value::Bool(true)), true);
        assert_eq!(reader.check("C",        Value::Bool(false)), true);
        assert_eq!(reader.check("FLOAT",    Value::Float(0.15625)), true);
        assert_eq!(reader.check("INT",      Value::Int(214)), true);
    }
}