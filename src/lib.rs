use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Triple {
    subject: String,
    predicate: String,
    object: String,
}

struct TripleStore {
    triples: HashSet<Triple>,
}

impl TripleStore {
    fn new() -> Self {
        Self {
            triples: HashSet::new(),
        }
    }

    fn add(&mut self, triple: Triple) {
        self.triples.insert(triple);
    }

    fn get(&self, subject: &str, predicate: &str, object: &str) -> Option<&Triple> {
        self.triples.iter().find(|&triple| {
            triple.subject == subject && triple.predicate == predicate && triple.object == object
        })
    }
}