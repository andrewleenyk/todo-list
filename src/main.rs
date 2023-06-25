mod todo_list;

use todo_list::ToDoList;
fn main() {
    let mut todo_list = ToDoList::new();

    loop {
        println!("===== Todo List =====");
        println!("Commands:");
        println!("  - add <description>");
        println!("  - complete <index>");
        println!("  - list");
        println!("  - remove <index>");
        println!("  - exit");
        println!("=====================");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();

        let parts: Vec<&str> = input.by_ref().take(2).collect();
        let command = parts[0];

        match command {
            "add" => {
                let description = parts[1..].join(" ");
                todo_list.add_task(description);
            }
            "complete" => {
                let index = parts[1].parse::<usize>().unwrap();
                todo_list.completed_task(index);
            }
            "list" => {
                todo_list.list_tasks();
            }
            "remove" => {
                let index = parts[1].parse::<usize>().unwrap();
                todo_list.remove_task(index);
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
