use crate::task::Task;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

pub struct TaskStorage {
    pub tasks: Vec<Task>,
}

impl TaskStorage {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let mut path = config_dir().unwrap();
        path.push("DIL");
        path.push("tasks.json");
        let file = File::open(path);

        self.tasks = match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                serde_json::from_reader(reader)?
            }
            Err(_) => Vec::<Task>::new(),
        };

        println!("Loaded Tasks: {:?}", self.tasks);

        Ok(())
    }

    pub fn save(&self) -> std::io::Result<()> {
        Ok(())
    }
}
