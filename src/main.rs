use std::io::{self, Write};

use notes_manager::notes::Notes;

fn main() {
    let mut notes = Notes::new();

    println!("Notes Manager");
    println!("Commands: add <text> | list | delete <id> | get <id> | quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        // Split only once: "add Buy milk" keeps the rest as text
        let (cmd, rest) = match line.split_once(' ') {
            Some((c, r)) => (c, r),
            None => (line, ""),
        };

        match cmd {
            "add" => {
                if rest.trim().is_empty() {
                    eprintln!("Usage: add <text>");
                    continue;
                }
                notes.add(rest.trim());
                println!("Added.");
            }
            "list" => {
                for n in notes.list() {
                    println!("[{}] {}", n.id, n.text);
                }
            }
            "get" => {
                let id: u32 = match rest.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Usage: get <id>");
                        continue;
                    }
                };
                match notes.get(id) {
                    Some(n) => println!("[{}] {}", n.id, n.text),
                    None => println!("Not found."),
                }
            }
            "delete" => {
                let id: u32 = match rest.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Usage: delete <id>");
                        continue;
                    }
                };
                if notes.delete(id) {
                    println!("Deleted.");
                } else {
                    println!("Not found.");
                }
            }
            "quit" | "exit" => break,
            _ => eprintln!("Unknown command."),
        }
    }
}
