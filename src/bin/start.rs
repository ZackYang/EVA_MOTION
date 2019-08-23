extern crate eva_motion_control;
use eva_motion_control::task::*;
use eva_motion_control::task::message::Value;

// [0, 0, 0, 0, 0, 1, 1, 0]<=[0,0,0, y触发,x触发,y复位,x复位,灯],  [65, 112, 0, 0]<=(x位置), [65, 80, 0, 0]<=(y位置), [65, 161, 153, 154]<=(速度)
// https://blog.csdn.net/abcdu1/article/details/75095781 浮点数标准

fn main() {
    let mut task = Task::new("localhost:3333");

    task.add(MotionType::MoveTo(15.0, 13.0, 20.0));
    task.add(MotionType::MoveTo(20.0, 40.0, 20.0));
    task.add(MotionType::MoveTo(90.0, 20.0, 20.0));
    
    task.add(MotionType::GoOnIf(vec![("Y_TRIGGER", Value::Bool(true))]));

    task.start().unwrap();
    println!("Status: {:?}", task.status);
    while task.status == Status::Working {
        task.run().unwrap();
    }
}