use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Clone, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks { tasks: Vec::new() }
    }

    pub fn load_tasks(&mut self, dir: String) -> Result<(), std::io::Error> {
        let path = dirs_next::config_dir().unwrap().join("DIL").join(dir);
        println!("searching {:?}", path);

        let file = File::open(path);
        match file {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents)?;

                println!("File contents: {}", contents);
                let tasks: Tasks = serde_json::from_str(&contents)?;
                self.tasks = tasks.tasks;
            }
            Err(_) => {
                println!("skipping, no tasks found");
            }
        }

        Ok(())
    }

    pub fn save_tasks(&self) -> Result<(), std::io::Error> {
        todo!()
    }
}
