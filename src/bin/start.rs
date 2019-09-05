extern crate eva_motion_control;
use eva_motion_control::task::*;
use eva_motion_control::task::reader::ValueType;

// [0, 0, 0, 0, 0, 1, 1, 0]<=[0,0,0, y触发v2500.4,x触发v2500.3,y复位v2500.2,x复位v2500.1,灯v2500.0],  [65, 112, 0, 0]<=(x位置), [65, 80, 0, 0]<=(y位置), [65, 161, 153, 154]<=(速度)
// https://blog.csdn.net/abcdu1/article/details/75095781 浮点数标准

fn main() {

    
    let mut task = Task::new("localhost:3333", keys);

    task.add(MotionType::InitCamera);
    task.add(MotionType::Capture);
    task.add(MotionType::StopCamera);
    task.add(MotionType::Reset);
    task.add(MotionType::Light(true));
    task.add(MotionType::MoveTo(15.0, 13.0, 20.0));
    task.add(MotionType::Reset);

    task.add(MotionType::GoOnIf(vec![("Y_TRIGGER", Value::Bool(true))]));

    task.start().unwrap();
    println!("Status: {:?}", task.status);
    while task.status == Status::Working {
        task.run().unwrap();
        std::thread::sleep_ms(20000);
    }
}
