extern crate serde_json;

use std::result::Result;
use serde_json::{Value, Error};

use config;

pub struct Task {
    pub name: String,
    pub command: String,
}

pub fn build_tasks() -> Result<Vec<Task>, Error> {
    let task_file = config::get_config_file();
    let v: Value = serde_json::from_str(&task_file)?;
    let keys = v.as_object().unwrap().keys();
    let mut tasks = Vec::new();
    for k in keys {
        tasks.push(Task {
            name: k.to_owned(),
            command: v.get(k.to_owned()).unwrap().to_string()
        });
    };
    Ok(tasks)
}