use serde::{Deserialize, Serialize};
use std::{env, fmt::Arguments, fs, io};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }
    pub fn add(&mut self, decription: String) {
        self.tasks.push(Task::new(decription));
    }

    pub fn delete(&mut self, id: usize) {
        if self.tasks.len() >= id {
            self.tasks.remove(id - 1);
        }
    }

    pub fn complete(&mut self, id: usize) {
        if self.tasks.len() >= id {
            self.tasks[id - 1].completed = true;
        }
    }

    pub fn uncomplete(&mut self, id: usize) {
        if self.tasks.len() >= id {
            self.tasks[id - 1].completed = false;
        }
    }

    pub fn get_all(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    pub fn flush(&mut self) {
        self.tasks = Vec::new();
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
    flush                   - Delete all tasks
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

fn display_list(tasklist: &TaskList) {
    const DONE: &str = "✅ ";
    const UNDO: &str = "🔳";
    println!("{:<4}{:>3}     {:<50}", "DONE", " ID", "DESCRIPTION");
    println!("--------------------------------");
    for (index, task) in tasklist.get_all().iter().enumerate() {
        println!(
            "{:<4}{:>3}  {}",
            if task.completed { DONE } else { UNDO },
            index + 1,
            task.description
        );
        //
    }
    println!("--------------------------------\n");
}

fn add_task(tasklist: &mut TaskList, args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Failed to add task, No description provided");
    }
    tasklist.add(args[2].to_string());
    display_list(tasklist);
}

fn delete_task(tasklist: &mut TaskList, args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Failed to delete task, No id provided");
    }
    match args[2].parse::<usize>() {
        Ok(id) => tasklist.delete(id),
        Err(_) => eprint!("Invalid ID provided"),
    }
    display_list(tasklist);
}

fn complete_task(tasklist: &mut TaskList, args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Failed to complete task, No id provided");
    }
    match args[2].parse::<usize>() {
        Ok(id) => tasklist.complete(id),
        Err(_) => eprint!("Invalid ID provided"),
    }
    display_list(tasklist);
}

fn uncomplete_task(tasklist: &mut TaskList, args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Failed to uncomplete task, No id provided");
    }
    match args[2].parse::<usize>() {
        Ok(id) => tasklist.uncomplete(id),
        Err(_) => eprint!("Invalid ID provided"),
    }
    display_list(tasklist);
}

fn flush_tasks(tasklist: &mut TaskList) {
    tasklist.flush();
}
fn main() {
    const FILENAME: &str = "todo-data.json";

    let mut tasklist = match load_data(FILENAME) {
        Ok(data) => data,
        Err(_) => TaskList::new(),
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a command, usage: cargo run -- command");
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "list" => display_list(&tasklist),
        "add" => add_task(&mut tasklist, args),
        "del" => delete_task(&mut tasklist, args),
        "done" => complete_task(&mut tasklist, args),
        "undo" => uncomplete_task(&mut tasklist, args),
        "flush" => flush_tasks(&mut tasklist),
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
