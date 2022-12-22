use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motion {
    pub name: String,
    pub state: MotionState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionState {
    pub presence: bool,
}
