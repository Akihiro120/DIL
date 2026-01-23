use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub name: String,
    // pub desc: String,
}

impl Task {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            // desc: String::new(),
        }
    }
}
