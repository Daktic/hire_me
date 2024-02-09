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

        // TODO: process the input and perform actions on the triple_store
    }
}