use std::collections::HashMap;
use std::io::{self, BufReader, Write};
use std::process;

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
        for (id, item) in self.items.iter() {
            println!("  - [{}] {}", id, item.description);
        }
    }

    // Simplified load and save using serde (optional)
    #[cfg(feature = "serde")]
    fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let file = std::fs::File::open(filename)?;
        let reader = BufReader::new(file);
        *self = serde_json::from_reader(reader)?;
        Ok(())
    }

    #[cfg(feature = "serde")]
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let file = std::fs::File::create(filename)?;
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer(&mut writer, self)?;
        writer.flush()?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut todo_list = TodoList::new();

    loop {
        println!("--- TODO list ---");
        todo_list.list_items();

        println!("Enter command (add, list, quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim();
        match command {
            "add" => {
                println!("Enter task description:");
                let mut new_input = String::new();
                io::stdin().read_line(&mut new_input)?;
                todo_list.add_item(new_input.trim().to_string());
            }
            "list" => todo_list.list_items(),
            "quit" => break,
            _ => println!("Invalid command"),
        }
    }

    println!("Exiting...");
    Ok(())
}
