extern crate eva_motion_control;
use eva_motion_control::task::*;
use eva_motion_control::task::message::Value;

// [0, 0, 0, 0, 0, 1, 1, 0]<=[0,0,0, y触发v2500.4,x触发v2500.3,y复位v2500.2,x复位v2500.1,灯v2500.0],  [65, 112, 0, 0]<=(x位置), [65, 80, 0, 0]<=(y位置), [65, 161, 153, 154]<=(速度)
// https://blog.csdn.net/abcdu1/article/details/75095781 浮点数标准

fn main() {
    let mut task = Task::new("192.168.2.166:2000");
    
    task.invoke(
        vec![false,false,false,false,false,false,false,false],
        vec![0.0f32, 0.0f32, 0.0f32]);
}
