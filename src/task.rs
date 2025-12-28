use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
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

    pub fn load_tasks(&mut self) -> Result<(), std::io::Error> {
        let path = dirs_next::config_dir()
            .unwrap()
            .join("DIL")
            .join("tasks.json");
        println!("searching {:?}", path);

        let file = File::open(path);
        match file {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents)?;

                self.tasks = serde_json::from_str(&contents)?;
            }
            Err(_) => {
                println!("skipping, no tasks found");
            }
        }

        Ok(())
    }

    pub fn save_tasks(&self) -> Result<(), std::io::Error> {
        let path = dirs_next::config_dir()
            .unwrap()
            .join("DIL")
            .join("tasks.json");

        println!("saving");
        let file = File::create(path);
        match file {
            Ok(file) => {
                let mut buf_writer = BufWriter::new(file);

                let contents = serde_json::to_string_pretty(&self.tasks)?;
                buf_writer.write_all(contents.as_bytes())?;
            }
            Err(_) => {
                println!("failed to create tasks.json");
            }
        }

        Ok(())
    }
}

impl Drop for Tasks {
    fn drop(&mut self) {
        match self.save_tasks() {
            Ok(_) => println!("successfully saved"),
            Err(_) => println!("there was a problem while saving"),
        }
    }
}
