extern crate serde_json;

mod config;
mod task;

fn main() {
    for each in task::build_tasks().unwrap() {
        println!("Task {} does {}", each.name, each.command);
    }
}

