extern crate eva_motion_control;
use eva_motion_control::task::*;
use eva_motion_control::task::reader::ValueType;

// [0, 0, 0, 0, 0, 1, 1, 0]<=[0,0,0, y触发v2500.4,x触发v2500.3,y复位v2500.2,x复位v2500.1,灯v2500.0],  [65, 112, 0, 0]<=(x位置), [65, 80, 0, 0]<=(y位置), [65, 161, 153, 154]<=(速度)
// https://blog.csdn.net/abcdu1/article/details/75095781 浮点数标准

fn main() {
    let keys = vec![
        ("_",            ValueType::Bool),
        ("_",            ValueType::Bool),
        ("_",            ValueType::Bool),
        ("Y_TRIGGER",    ValueType::Bool),
        ("X_TRIGGER",    ValueType::Bool),
        ("Y_REST_STATE", ValueType::Bool),
        ("X_REST_STATE", ValueType::Bool),
        ("LIGHT",        ValueType::Bool),
        // f32
        ("X_POSITION",   ValueType::Float),
        ("Y_POSITION",   ValueType::Float),
        ("X_SPEED",      ValueType::Float),
        ("Y_SPEED",      ValueType::Float),
        // Bool
        ("_",            ValueType::Bool),
        ("_",            ValueType::Bool),
        ("_",            ValueType::Bool),
        ("_",            ValueType::Bool),
        ("X_FINISHED",   ValueType::Bool),
        ("X_DIRECTION",  ValueType::Bool),
        ("X_FINISHED",   ValueType::Bool),
        ("Y_DIRECTION",  ValueType::Bool),
        //Int
        ("X_ERROR_CODE",   ValueType::Int),
        ("Y_ERROR_CODE",   ValueType::Int),
        // Float
        ("X_CURRENT_POSITION",   ValueType::Float),
        ("X_CURRENT_SPEED",      ValueType::Float),
        ("Y_CURRENT_POSITION",   ValueType::Float),
        ("Y_CURRENT_SPEED",      ValueType::Float),
    ];

    // let mut task = Task::new("192.168.2.166:2000", keys);

    // task.invoke(
    //     vec![false,false,false,false,false,true,true,false],
    //     vec![0.0f32, 0.0f32, 0.0f32]);

    // std::thread::sleep_ms(10000);

    // task.invoke(
    //     vec![false,false,false,false,false,false,false,true],
    //     vec![0.0f32, 0.0f32, 0.0f32]);

    // std::thread::sleep_ms(10000);

    // task.invoke(
    //     vec![false,false,false,false,false,false,false,true],
    //     vec![100.0f32, 100.0f32, 70.0f32]);

    // std::thread::sleep_ms(10000);

    // task.invoke(
    //     vec![false,false,false,true,true,false,false,true],
    //     vec![100.0f32, 100.0f32, 70.0f32]);

    // std::thread::sleep_ms(10000);

    // task.invoke(
    //     vec![false,false,false,false,false,true,true,true],
    //     vec![0.0f32, 0.0f32, 0.0f32]);
}