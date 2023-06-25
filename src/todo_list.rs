// task structure
pub struct Task {
    description: String,
    completed: bool,
}

pub struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    pub fn new() -> ToDoList {
        ToDoList {tasks: Vec::new()}
    }

    pub fn add_task(&mut self, description: String) {
        self.tasks.push(Task {description, completed: false});
    }

    pub fn completed_task(&mut self, index: usize) {
        //i want to retrieve the taks by the index,
        // then i want to change the compleyted bool to true
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
        }
    }

    pub fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}: {} {}", index, task.description, task.completed);
        }
    }
    
    pub fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }
    

}