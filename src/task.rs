mod db;
use db::DB;
use db::__SetGetByKey;
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

#[derive(Clone)]
pub enum MotionType {
    MoveTo(f32, f32, f32),
    Capture,
    GoOnIf(Vec<(String, u8)>),
    Stop,
    Reset
}

#[derive(PartialEq)]
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
    pub db: DB
}

impl Task {
    pub fn new() -> Task {
        let mut task = Task { actions: Vec::new(), status: Status::Pending, current_step: 0, db: DB::new() };
        task.db.connect().unwrap();
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
            MotionType::GoOnIf(_conditions) => {},
            MotionType::Capture => {},
            MotionType::Stop => {},
            MotionType::Reset => {}
        }
        self.current_step += 1;
        if self.current_step == self.actions.len() {
            self.status = Status::Done;
            self.db.disconnect();
        }
        Ok(())
    }

    fn move_to(&mut self, x: f32, y: f32, speed: f32) -> Result<(), &'static str> {
        self.db.set_by_key("X_POSITION", x)?;
        self.db.set_by_key("Y_POSITION", y)?;
        self.db.set_by_key("X_SPEED", speed)?;
        self.db.set_by_key("Y_SPEED", speed)?;
        self.db.send()?;
        self.db.set_by_key("X_TRIGGER", 1u8)?;
        self.db.set_by_key("Y_TRIGGER", 1u8)?;
        self.db.send()?;
        // self.db.set_by_key("X_TRIGGER", 0u8)?;
        // self.db.set_by_key("Y_TRIGGER", 0u8)?;
        Ok(())
    }

    fn go_on_if(&mut self, conditions: Vec<(String, u8)>) {
        for (key, string_value) in conditions {
            self.db.get_u8_by_key(key.as_str());
        }
    }   
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn send_data() {
        let task = Task::new();
        // println!("{:X?}", table.to_bytes());
        let result = task.db.send().unwrap();
        assert_eq!(result, ());
    }
}