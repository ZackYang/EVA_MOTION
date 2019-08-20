extern crate eva_motion_control;
use eva_motion_control::task::*;
use std::thread::sleep_ms;

// [0, 0, 1, 1, 0]<=[x复位,y复位,x触发,y触发,灯],  [65, 112, 0, 0]<=(x位置), [65, 161, 153, 154]<=(x速度), [65, 80, 0, 0]<=(y位置), [65, 161, 153, 154]<=(y速度)
// https://blog.csdn.net/abcdu1/article/details/75095781 浮点数标准
fn main() {
    let mut task = Task::new();
    // println!("{:X?}", table.to_bytes());
    task.add(MotionType::MoveTo(15.0, 13.0, 20.2));
    // task.add(MotionType::MoveTo(20.0, 40.0, 20.2));
    // task.add(MotionType::MoveTo(90.0, 20.0, 20.2));

    task.start().unwrap();
    let mut times = 1;
    while task.status == Status::Working {
        task.run().unwrap();
        println!("{}", times);
        sleep_ms(3000);
    }
}