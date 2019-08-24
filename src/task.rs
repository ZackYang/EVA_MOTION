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
use message::{Msg, Value};
use std::net::{TcpStream, Shutdown};
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};
use std::io::Read;
use std::process::Command;


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

#[derive(PartialEq, Debug)]
pub enum Status {
    Pending,
    Working,
    Fail,
    Done
}

pub struct Task {
    actions: Vec<MotionType>,
    pub status: Status,
    current_step: usize,
    conn: TcpStream
}

lazy_static! {
    static ref REMOTE_STATES: Mutex<Vec<u8>> = Mutex::new(vec![0u8; 500]);
}

impl Task {
    pub fn new(addr: &str) -> Task {
        let conn = TcpStream::connect(addr).unwrap();
        conn.set_nonblocking(true).unwrap();
        let mut input_conn = conn.try_clone().unwrap();
        thread::spawn(move || {
            let mut buffer = [0u8;500];
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
            conn: conn
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
            self.conn.shutdown(Shutdown::Both);
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
        // self.go_on_if(msg.conditions.clone());
        msg.set("X_TRIGGER", Value::Bool(true))?;
        msg.set("Y_TRIGGER", Value::Bool(true))?;
        msg.send()?;
        // self.go_on_if(msg.conditions.clone());
        Ok(())
    }

    fn light(&mut self, on: bool) -> Result<(), &'static str> {
        let mut msg = Msg::new(self.conn.try_clone().unwrap());
        msg.set("LIGHT", Value::Bool(on))?;
        msg.send()?;
        // self.go_on_if(msg.conditions.clone());
        Ok(())
    }

    fn reset(&mut self) -> Result<(), &'static str> {
        let mut msg = Msg::new(self.conn.try_clone().unwrap());
        msg.set("X_REST_STATE", Value::Bool(true))?;
        msg.set("Y_REST_STATE", Value::Bool(true))?;
        msg.set("LIGHT", Value::Bool(true))?;
        msg.send()?;
        Ok(())
    }

    fn go_on_if(&mut self, conditions: Vec<(&str, Value)>) {
        let mut done = false;
        while !done {
            match REMOTE_STATES.lock() {
                Ok(states) => {
                    let reader = Reader::new(&mut states.clone());
                    let mut result = false;
                    for (key, value) in &conditions {
                        result &= reader.check(key, value.clone());
                    }
                    if result { done = true }
                },
                Err(_e) => {

                }
            }
        }
    }

    fn init_camera(&mut self) {
        let output = Command::new("./eva_camera")
                            .arg("")
                            .output()
                            .expect("Failed to execute command");
    }

    fn capture(&mut self) {
        // camera port 9990
        let mut resp = reqwest::get("http://localhost:9990/capture").unwrap();
        if resp.status().is_success() {
            println!("success!");
            let mut file = std::fs::File::create("file.jpg").unwrap();
            resp.copy_to(&mut file).unwrap();
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
