use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::{thread};
use std::sync::mpsc::{channel, Receiver};

static WRITE_TABLE_INDICES: [(&'static str, usize); 8] = [
    ("X_REST_STATE", 0usize),
    ("Y_REST_STATE", 1usize),
    ("X_TRIGGER",    2usize),
    ("Y_TRIGGER",    3usize),
    ("LIGHT",        4usize),
    // f32
    ("X_POSITION",   0usize),
    ("Y_POSITION",   1usize),
    ("SPEED",        2usize)
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
    table: Table,
    conn: TcpStream,
    receiver: std::sync::mpsc::Receiver<std::vec::Vec<u8>>
}

impl DB {
    pub fn new() -> DB {
        let conn = DB::connect().unwrap();
        let receiver = DB::sync(conn.try_clone().unwrap());
        DB {
            table: Table::new(100, 100),
            conn: conn,
            receiver: receiver
        }
    }

    pub fn connect() -> Result<TcpStream, &'static str> {
        match TcpStream::connect("localhost:3333") {
            Ok(stream) => {
                Ok(stream)
            },
            Err(_e) => {
                Err("Cannot connect")
            }
        }
    }

    pub fn disconnect(&mut self) {
        self.conn.shutdown(Shutdown::Both).unwrap()
    }

    pub fn write_local(&mut self, data: &[u8]) {
        self.read_table.from_bytes(data);
    }

    pub fn prepared_data(&self) -> Vec<u8> {
        self.write_table.to_bytes()
    }

    pub fn get_u8_by_key(&mut self, key: &str) -> bool {
        self.update_read_table();
        let index = DB::find_read_table_index_by_key(key).unwrap();
        self.read_table.bool_range[index]
    }

    pub fn get_f32_by_key(&mut self, key: &str) -> f32 {
        self.update_read_table();
        let index = DB::find_read_table_index_by_key(key).unwrap();
        self.read_table.f32_range[index]
    }

    pub fn update_read_table(&mut self) {
        match self.receiver.try_recv() {
            Ok(data) => {
                self.write_local(&data);
            },
            Err(e) => {}
        }
    }

    pub fn send(&mut self) -> Result<(), &'static str> {
        match self.conn.write(&self.prepared_data()) {
            Ok(_) => Ok(()),
            Err(_e) => Err("Sending error")
        }
    }

    pub fn sync(conn: TcpStream) -> Receiver<Vec<u8>> {
        let (sender, receiver) = channel::<Vec<u8>>();
        let mut conn = conn.try_clone().unwrap();
        thread::spawn(move || {
            let mut buffer = [0u8; 500];
            match conn.read(&mut buffer) {
                Ok(size) => {
                    if size > 0 {
                        println!("Received data: {:?}", buffer.to_vec());
                        sender.send(buffer.to_vec()).expect("cannot send");
                    }
                },
                Err(e) => {

                }
            }
        });
        receiver
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
