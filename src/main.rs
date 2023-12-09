#[derive(Clone, Debug)]
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

fn main() {
    let tasklist = TaskList::new();
    // Parse arguments
    // Switch argument
    // 1. Show tasks
    // 2. Add task
    // 3. complete task
    // 4. Delete task
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
