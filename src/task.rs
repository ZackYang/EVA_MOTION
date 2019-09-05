// Y_REST_STATE
// X_ACTIVATION
// X_READY
// X_DIRECTION
// X_TRIGGER
// X_COMPLETED
// Y_TRIGGER
// Y_COMPLETED
// LIGHT
// ============ f32 ==============
// X_POSITION
// X_SPEED
// Y_POSITION
// Y_SPEED
extern crate reqwest;
pub mod message;
pub mod reader;
use reader::Reader;
use reader::ValueType;
use message::{Msg, Value};
use std::net::{TcpStream, Shutdown};
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};
use std::io::Read;
use std::process::Command;
use std::time::Instant;


#[derive(Clone)]
pub enum MotionType {
    MoveTo(f32, f32, f32),
    InitCamera,
    StopCamera,
    Capture,
    GoOnIf(Vec<(&'static str, Value)>),
    Stop,
    Reset,
    Light(bool)
}

pub trait AddImage {
    fn add_image(&mut self, image: Vec<u8>, filename: String);
    fn change_status(&mut self, status: String);
}

#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    Pending,
    Working,
    Fail,
    Done
}

pub struct Task<T> {
    actions: Vec<MotionType>,
    pub status: Status,
    current_step: usize,
    conn: TcpStream,
    reader: Reader,
    image_receiver: T,
    current_image: usize
}

lazy_static! {
    static ref REMOTE_STATES: Mutex<Vec<u8>> = Mutex::new(vec![0u8; 500]);
}

impl<T: AddImage> Task<T> {
    pub fn new(can_add_image: T, uuid: &'static str, addr: &str, reader_keys: Vec<(&'static str, ValueType)>) -> Task<T> {
        let conn = TcpStream::connect(addr).unwrap();
        conn.set_nonblocking(true).unwrap();
        let mut input_conn = conn.try_clone().unwrap();

        let receiver = can_add_image;

        thread::spawn(move || {
            let mut buffer = [0u8;36];
            // let mut md5 = 
            loop {
                match input_conn.read(&mut buffer) {
                    Ok(size) => {
                        if size > 0 {
                            match REMOTE_STATES.lock() {
                                Ok(mut states) => {
                                    // println!("{:?}", buffer.to_vec());
                                    *states = buffer.to_vec();
                                },
                                Err(_) => {}
                            };
                        }
                    },
                    Err(_e) => {}
                }
            }
        });

        let task = Task {
            actions: Vec::new(),
            status: Status::Pending,
            current_step: 0,
            conn: conn,
            reader: Reader::new(reader_keys),
            image_receiver: receiver,
            current_image: 0
        };
        task
    }

    pub fn add(&mut self, motion_type: MotionType) {
        self.actions.push(motion_type)
    }

    pub fn start(&mut self) -> Result<(), &str> {
        if self.status == Status::Pending && self. current_step == 0 {
            self.status = Status::Working;
            Ok(())
        } else {
            Err("Task is not pending")
        }
    }

    pub fn run(&mut self) -> Result<(), &str> {
        self.stop_camera();
        if self.status == Status::Fail {
            return Err("Task Failed");
        }

        if self.status == Status::Done {
            return Err("Task is Done");
        }

        let actions = self.actions.clone(); 
        match &actions[self.current_step] {
            MotionType::MoveTo(x, y, speed) => {
                self.move_to(*x, *y, *speed)?;
            },
            MotionType::GoOnIf(conditions) => {
                self.go_on_if(conditions.to_vec());
            },
            MotionType::InitCamera => {
                self.init_camera();
            },
            MotionType::StopCamera => {
                self.stop_camera();
            },
            MotionType::Capture => {
                self.capture();
            },
            MotionType::Stop => {},
            MotionType::Reset => {
                self.reset()?;
            },
            MotionType::Light(on) => {
                self.light(*on)?;
            }
        }
        self.current_step += 1;
        if self.current_step == self.actions.len() {
            self.status = Status::Done;
            self.conn.shutdown(Shutdown::Both).unwrap();
        }
        std::thread::sleep_ms(100);
        Ok(())
    }

    pub fn invoke(&mut self, bools: Vec<bool>, floats: Vec<f32>) {
        if bools.len() != 8 {
            panic!("Arguments should contain 8 bools");
        }

        if floats.len() != 3 {
            panic!("Arguments should contain 3 floats");
        }
        let mut msg = Msg::new(self.conn.try_clone().unwrap());
        msg.load(bools, floats);
        msg.send().unwrap();
        std::thread::sleep_ms(10000);
    }

    fn move_to(&mut self, x: f32, y: f32, speed: f32) -> Result<(), &'static str> {
        let mut msg = Msg::new(self.conn.try_clone().unwrap());
        msg.set("X_POSITION", Value::Float(x))?;
        msg.set("Y_POSITION", Value::Float(y))?;
        msg.set("LIGHT", Value::Bool(true))?;
        msg.set("SPEED", Value::Float(speed))?;
        msg.send()?;
        self.go_on_if(msg.conditions.clone());
        msg.set("X_TRIGGER", Value::Bool(true))?;
        msg.set("Y_TRIGGER", Value::Bool(true))?;
        msg.send()?;
        self.go_on_if(msg.conditions.clone());
        Ok(())
    }

    fn light(&mut self, on: bool) -> Result<(), &'static str> {
        let mut msg = Msg::new(self.conn.try_clone().unwrap());
        msg.set("LIGHT", Value::Bool(on))?;
        msg.send()?;
        self.go_on_if(msg.conditions.clone());
        Ok(())
    }

    fn reset(&mut self) -> Result<(), &'static str> {
        let mut msg = Msg::new(self.conn.try_clone().unwrap());
        msg.set("X_REST_STATE", Value::Bool(true))?;
        msg.set("Y_REST_STATE", Value::Bool(true))?;
        msg.send()?;
        std::thread::sleep_ms(100);
        self.go_on_if(vec![
            ("X_FINISHED", Value::Bool(true)),
            ("Y_FINISHED", Value::Bool(true))
        ]);
        Ok(())
    }

    fn go_on_if(&mut self, conditions: Vec<(&str, Value)>) {
        let mut done = false;
        let start = Instant::now();
        while !done {
            match REMOTE_STATES.lock() {
                Ok(states) => {
                    // let reader = Reader::new(&mut states.clone());
                    self.reader.load(&mut states.clone());
                    let mut result = true;
                    for (key, value) in &conditions {
                        result &= self.reader.check(key, value.clone());
                        println!("Key: {:?}, Value: {:?}, Check Result: {:?}", key, value, result);
                    }
                    if result { done = true }
                },
                Err(_e) => {}
            }
            if start.elapsed().as_secs() > 10 {
                self.status = Status::Fail;
                self.image_receiver.change_status("Arguments confirmation timeout".to_string());
                done = true
            }
        }
    }

    fn init_camera(&mut self) {
        match Command::new("eva_camera").arg("").output() {
            Ok(r) => {},
            Err(e) => {
                self.status = Status::Fail;
                self.image_receiver.change_status("Cannot start camera".to_string());
                return;
            } 
        }
        std::thread::sleep_ms(10000);
    }

    fn capture(&mut self) {
        // camera port 9990
        let mut try_get_image = reqwest::get("http://localhost:9990/capture");
        let mut resp = match try_get_image {
            Ok(r) => { r },
            Err(e) => {
                self.status = Status::Fail;
                self.image_receiver.change_status("Cannot connect to camera".to_string());
                return;
            } 
        };

        if resp.status().is_success() {
            println!("success!");
            let mut buf: Vec<u8> = vec![];
            resp.copy_to(&mut buf).unwrap();
            self.image_receiver.add_image(buf, format!("{}.png", self.current_image).to_string());
            self.current_image += 1;
        }
    }

    fn stop_camera(&mut self) {
        let pid = Command::new("cat")
            .arg("/tmp/eva_camera.pid")
            .output()
            .expect("Failed to execute command");

        let pid_str = format!("{}", String::from_utf8_lossy(&pid.stdout));

        let output = Command::new("kill")
                    .arg("-9")
                    .arg(pid_str)
                    .output()
                    .expect("Failed to execute command");
    }
}
