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

        match input.split_whitespace().collect::<Vec<_>>().as_slice() {
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
            _ => {
                println!("Invalid command");
            }
        }
    }
}