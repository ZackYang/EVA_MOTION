use std::net::TcpStream;
use std::io::Write;
use std::collections::HashMap;
extern crate bitty;
use bitty::FromBits;

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Int(u8),
    Float(f32)
}

pub struct Msg {
    pub bools: Vec<bool>,
    pub floats: Vec<f32>,
    pub ints: Vec<u8>,
    conn: TcpStream,
    pub conditions: Vec<(&'static str, Value)>
}

impl Msg {
    pub fn new(stream: TcpStream) -> Msg {
        Msg {
            bools: vec![false; 8],
            floats: vec![0f32; 3],
            ints: vec![0u8; 0],
            conditions: Vec::<(&str, Value)>::new(),
            conn: stream
        }
    }

    pub fn load(&mut self, bools: Vec<bool>, floats: Vec<f32>) {
        self.bools = bools;
        self.floats = floats;
    }

    fn index(&self, key: &str) -> Result<usize, &'static str> {
        let pairs: HashMap<&str, i32> =
            [
                ("LIGHT",        7),
                ("X_REST_STATE", 6),
                ("Y_REST_STATE", 5),
                ("X_TRIGGER",    4),
                ("Y_TRIGGER",    3),
                // f32
                ("X_POSITION",   0),
                ("Y_POSITION",   1),
                ("SPEED",        2)
            ]
            .iter().cloned().collect();
        if pairs.contains_key(key) {
            Ok(pairs[key] as usize)
        } else {
            Err("Found nothing")
        }
    }

    pub fn set(&mut self, key: &'static str, value: Value) -> Result<(), &'static str> {
        let index = self.index(key)?;
        self.conditions.push((key, value.clone()));
        match value {
            Value::Bool(v) => { self.bools[index] = v },
            Value::Int(v) => { self.ints[index] = v },
            Value::Float(v) => { self.floats[index] = v }
        }
        Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        println!("{:?}", &self.bools);
        println!("{:?}", &self.floats);
        println!("======================================= 无害 =======================================");
        let mut bools = self.bools.clone();
        bools.reverse();
        let bools = u8::from_bits(&bools);
        let mut results = vec![bools];
        
        results.append(&mut self.ints.to_vec());

        for float in &self.floats {
            let mut result = float.to_bits().to_be_bytes().to_vec();
            results.append(&mut result);
        }
        results
    }

    pub fn send(&mut self) -> Result<(), &'static str> {
        match self.conn.write(&self.to_bytes()) {
            Ok(size) => {
                Ok(())
            },
            Err(e) => { Err("Cannot send data") }
        }
    }

}