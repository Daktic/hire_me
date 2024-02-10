mod lib;

use lib::TripleStore;
use std::io::{self, Write};

fn main() {
    let mut triple_store = TripleStore::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // print the prompt

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim(); // remove trailing newline

        if input == "exit" {
            break;
        }

        // split the input into words for further processing
        match input.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["help"] => {
                println!("Commands:");
                println!("  add <subject> <predicate> <object>");
                println!("  get <subject> <predicate> <object>");
                println!("  query <raw query string>");
                println!("  exit -> exit the program");
            }
            ["add", subject, predicate, object] => {
                triple_store.add(lib::Triple {
                    subject: subject.to_string(),
                    predicate: predicate.to_string(),
                    object: object.to_string(),
                });
            }
            ["get", subject, predicate, object] => {
                if let Some(triple) = triple_store.get(subject, predicate, object) {
                    println!("{:?}", triple);
                } else {
                    println!("Not found");
                }
            }
            command @ ["get", ..] if command.len() < 4 || command.len() > 4 => {
                println!("Invalid number or arguments. 'get' Expected 3, got {}", command.len() - 1);
            }
            command @ ["query", ..] => {
                let query = command[1..].join(" ");
                let result = triple_store.query(&query);
                println!("{:?}", result);
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}