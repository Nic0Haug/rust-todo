use serde::{Deserialize, Serialize};
use std::{env, fs, io};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct TaskList {
    max_id: usize,
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            max_id: 1,
            tasks: Vec::new(),
        }
    }
    pub fn add(&mut self, decription: String) {
        self.tasks.push(Task::new(self.max_id, decription));
        self.max_id += 1;
    }

    pub fn delete(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|x| x.id == id) {
            self.tasks.remove(index);
        }
    }

    pub fn complete(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|x| x.id == id) {
            self.tasks[index].completed = true;
        }
    }

    pub fn uncomplete(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|x| x.id == id) {
            self.tasks[index].completed = false;
        }
    }

    pub fn get_all(&self) -> Vec<Task> {
        self.tasks.clone()
    }
}

// fn add_task(task_list: TaskList, args: Vec<String>) {}

fn print_help() {
    const HELP_MESSAGE: &str = r#"
USAGE: 
cargo run -- <command> [options]

COMMANDS:
    list                    - List all tasks
    add <description>       - Add a task with a description
    del <id>                - Delete a task
    done <id>               - Mark task as done
    undo <id>               - Unmark task as done
    help                    - Show this help
    "#;

    println!("{}", HELP_MESSAGE);
}
/// Save data to filesystem
fn save_data(filename: &str, data: TaskList) -> io::Result<()> {
    let serialized_data = serde_json::to_string_pretty(&data)?;
    fs::write(filename, serialized_data)?;
    Ok(())
}

fn load_data(filename: &str) -> io::Result<TaskList> {
    let serialized_data = fs::read_to_string(filename)?;
    let deserialized_data: TaskList = serde_json::from_str(&serialized_data)?;

    Ok(deserialized_data)
}

fn main() {
    const FILENAME: &str = "todo-data.json";

    let mut tasklist = match load_data(FILENAME) {
        Ok(data) => data,
        Err(_) => TaskList::new(),
    };
    tasklist.add("Test1".to_string());
    tasklist.add("Test2".to_string());
    tasklist.add("Test3".to_string());

    tasklist.complete(1);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a command, usage: cargo run -- command");
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "list" => println!("List all tasks"),
        "add" => println!("Add a task"),
        "del" => println!("Delete a task"),
        "done" => println!("Un-complete a task"),
        "undo" => println!("Complete a task"),
        "help" => print_help(),
        _ => println!("Unknown command"),
    }

    if let Err(err) = save_data(FILENAME, tasklist) {
        eprintln!("Failed to save todo tasks to file! \nError: {}\n\n", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut task_list = TaskList::new();
        task_list.add("Test1".to_string());
        task_list.add("Test2".to_string());
        task_list.add("Test3".to_string());

        let all_tasks = task_list.get_all();
        assert_eq!(all_tasks[0].description, "Test1".to_string());
        assert_eq!(all_tasks[1].description, "Test2".to_string());
        assert_eq!(all_tasks[2].description, "Test3".to_string());
        assert_eq!(all_tasks.len(), 3);

        task_list.delete(2);
        assert_eq!(task_list.tasks[1].description, "Test3".to_string());
        assert_eq!(task_list.tasks.len(), 2);

        task_list.complete(1);
        assert!(task_list.tasks[0].completed);
    }
}
