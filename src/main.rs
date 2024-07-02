use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Debug)]
struct TodoItem {
    description: String,
}

impl TodoItem {
    fn new(description: String) -> Self {
        TodoItem { description }
    }
}

struct TodoList {
    items: HashMap<u32, TodoItem>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_item(&mut self, description: String) {
        self.items.insert(self.next_id, TodoItem::new(description));
        self.next_id += 1;
    }

    fn list_items(&self) {
        for item in self.items.values() {
            println!("- {}", item.description);
        }
    }

    fn load_from_file(&mut self, _filename: &str) {
        // Removed for simplicity
    }

    fn save_to_file(&self, _filename: &str) {
        // Removed for simplicity
    }
}

fn main() -> std::io::Result<()> {
    let mut todo_list = TodoList::new();

    loop {
        let mut input = String::new();
        println!("--- TODO list ---");
        todo_list.list_items();
        println!("Enter command (add, list, quit):");
        std::io::stdin().read_line(&mut input)?;

        let command = input.trim();

        match command {
            "add" => {
                let mut new_input = String::new();
                println!("Enter task description:");
                std::io::stdin().read_line(&mut new_input)?;
                todo_list.add_item(new_input.trim().to_string());
            }
            "list" => todo_list.list_items(),
            "quit" => break,
            _ => println!("Invalid command"),
        }
    }

    Ok(())
}
