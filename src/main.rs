use std::io;
use std::io::Write;

struct Todo {
    items: Vec<(String, String)>,
}

impl Todo {
    fn new() -> Todo {
        Todo { items: Vec::new() }
    }

    fn add_item(&mut self, item: String, project: String) {
        self.items.push((item, project));
    }

    fn view_items(&self) {
        if self.items.is_empty() {
            println!("No todo items yet!");
        } else {
            println!("** Todo Items **");
            println!("-----------------");
            for (i, (item, project)) in self.items.iter().enumerate() {
                println!("| {:>2} | {} - {} |", i + 1, item, project);
            }
            println!("-----------------");
        }
    }

    fn delete_item(&mut self, index: usize) {
        if index > 0 && index <= self.items.len() {
            self.items.remove(index - 1);
            println!("Item deleted successfully!");
        } else {
            println!("Invalid index");
        }
    }
}

fn main() {
    let mut todo = Todo::new();

    loop {
        println!("\n** Todo App **");
        println!("-----------------");
        println!("1. **Add** a Todo Item");
        println!("2. **View** Todo Items");
        println!("3. **Delete** a Todo Item");
        println!("4. **Exit**");
        println!("-----------------");

        print!("Choose an option: ");
        io::stdout().flush().expect("Can't flush stdout");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("Invalid choice. Please try again!");
                continue;
            }
        };

        match choice {
            1 => {
                let mut item = String::new();
                print!("Enter a todo item: ");
                io::stdout().flush().expect("Can't flush stdout");
                io::stdin().read_line(&mut item)
                    .expect("Failed to read line");
                let mut project = String::new();
                print!("Enter the project: ");
                io::stdout().flush().expect("Can't flush stdout");
                io::stdin().read_line(&mut project)
                    .expect("Failed to read line");
                todo.add_item(item.trim().to_string(), project.trim().to_string());
                println!("Item added successfully!");
            }
            2 => {
                todo.view_items();
            }
            3 => {
                let mut index = String::new();
                print!("Enter the item number to delete: ");
                io::stdout().flush().expect("Can't flush stdout");
                io::stdin().read_line(&mut index)
                    .expect("Failed to read line");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    _ => {
                        println!("Invalid index. Please try again!");
                        continue;
                    }
                };
                todo.delete_item(index);
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again!");
            }
        }
    }
}