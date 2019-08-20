mod table;
use table::Table;
use std::sync::Arc;

use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::{thread, time};
use std::sync::mpsc::channel;

static WRITE_TABLE_INDICES: [(&'static str, usize); 9] = [
    ("X_REST_STATE", 0usize),
    ("Y_REST_STATE", 1usize),
    ("X_TRIGGER",    2usize),
    ("Y_TRIGGER",    3usize),
    ("LIGHT",        4usize),
    // f32
    ("X_POSITION",   0usize),
    ("X_SPEED",      1usize),
    ("Y_POSITION",   2usize),
    ("Y_SPEED",      3usize)
];

static READ_TABLE_INDICES: [(&'static str, usize); 13] = [
    ("Y_REST_STATE", 0usize),
    ("X_ACTIVATION", 1usize),
    ("X_READY",      2usize),
    ("X_DIRECTION",  3usize),
    ("X_TRIGGER",    4usize),
    ("X_COMPLETED",  6usize),
    ("Y_TRIGGER",    14usize),
    ("Y_COMPLETED",  16usize),
    ("LIGHT",        20usize),
    // f32
    ("X_POSITION",   10usize),
    ("X_SPEED",      15usize),
    ("Y_POSITION",   20usize),
    ("Y_SPEED",      25usize)
];

pub struct DB {
    write_table: Table,
    read_table: Table,
    input_conn: Option<TcpStream>,
    output_conn: Option<TcpStream>
}

impl DB {
    pub fn new() -> DB {
        DB {
            write_table: Table::new(5, 4),
            read_table: Table::new(100, 100),
            output_conn: None,
            input_conn: None
        }
    }

    pub fn connect(&mut self) -> Result<(), &'static str> {
        match TcpStream::connect("localhost:3333") {
            Ok(stream) => {
                self.input_conn = Some(stream.try_clone().unwrap());
                self.output_conn = Some(stream.try_clone().unwrap());
                Ok(())
            },
            Err(_e) => {
                Err("Cannot connect")
            }
        }
    }

    pub fn disconnect(&mut self) {
        match &self.output_conn {
            Some(conn) => conn.shutdown(Shutdown::Both).unwrap(),
            None => {}
        }

        match &self.input_conn {
            Some(conn) => conn.shutdown(Shutdown::Both).unwrap(),
            None => {}
        }
    }

    pub fn write_local(&mut self, data: &[u8]) {
        self.read_table.from_bytes(data);
    }

    pub fn prepared_data(&self) -> Vec<u8> {
        self.write_table.to_bytes()
    }

    pub fn get_u8_by_key(&self, key: &str) -> u8 {
        let index = DB::find_read_table_index_by_key(key).unwrap();
        self.read_table.u8_range[index]
    }

    pub fn send(&self) -> Result<(), &'static str> {
        match &self.output_conn {
            Some(conn) => {
                let mut conn = conn;
                match conn.write(&self.prepared_data()) {
                    Ok(_) => Ok(()),
                    Err(_e) => Err("Sending error")
                }
            },
            None => {
                Err("No connection")
            }
        }
    }

    pub fn sync(&self) {
        match &self.input_conn {
            Some(conn) => {
                let mut conn = conn.try_clone().unwrap();
                conn.set_nonblocking(true).expect("set_nonblocking call failed");

                thread::spawn(move || {
                    let mut buffer = [0u8; 500];
                    match conn.read(&mut buffer) {
                        Ok(size) => {
                        
                        },
                        Err(e) => {

                        }
                    }
                });
            },
            None => {}
        }
    }

    pub fn find_write_table_index_by_key(key: &str) -> Result<usize, &'static str> {
        let mut result = Err("Cannot find key");
        for i in 0..WRITE_TABLE_INDICES.len() {
            let (key_word, index) = WRITE_TABLE_INDICES[i];
            if key_word == key {
                result = Ok(index);
            }
        }
        result
    }

    pub fn find_read_table_index_by_key(key: &str) -> Result<usize, &'static str> {
        let mut result = Err("Cannot find key");
        for i in 0..READ_TABLE_INDICES.len() {
            let (key_word, index) = WRITE_TABLE_INDICES[i];
            if key_word == key {
                result = Ok(index);
            }
        }
        result
    }

}

pub trait __SetAndGet<T, U> {
    fn set(&mut self, index: T, value: U);
}

pub trait __SetGetByKey<T, U> {
    fn set_by_key(&mut self, key: T, value: U) -> Result<(), &'static str>;
}

impl __SetAndGet<usize, u8> for DB {
    fn set(&mut self, index: usize, value: u8) {
        self.write_table.u8_range[index] = value;
    }
}

impl __SetAndGet<usize, f32> for DB {
    fn set(&mut self, index: usize, value: f32) {
        self.write_table.f32_range[index] = value;
    }
}

impl __SetGetByKey<&str, f32> for DB {
    fn set_by_key(&mut self, key: &str, value: f32) -> Result<(), &'static str> {
        let index = DB::find_write_table_index_by_key(key)?;
        self.write_table.f32_range[index] = value;
        Ok(())
    }
}

impl __SetGetByKey<&str, u8> for DB {
    fn set_by_key(&mut self, key: &str, value: u8) -> Result<(), &'static str> {
        let index = DB::find_write_table_index_by_key(key)?;
        self.write_table.u8_range[index] = value;
        Ok(())
    }
}