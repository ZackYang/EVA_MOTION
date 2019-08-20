extern crate eva_motion_control;
use eva_motion_control::task::*;
use std::thread::sleep_ms;

fn main() {
    let mut task = Task::new();
    // println!("{:X?}", table.to_bytes());
    task.add(MotionType::MoveTo(15.0, 13.0, 20.2));
    task.add(MotionType::MoveTo(20.0, 40.0, 20.2));
    task.add(MotionType::MoveTo(90.0, 20.0, 20.2));

    task.start().unwrap();
    let mut times = 1;
    while task.status == Status::Working {
        task.run().unwrap();
        println!("{}", times);
        sleep_ms(3000);
    }
}