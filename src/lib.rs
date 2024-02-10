use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Triple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

pub(crate) struct TripleStore {
    triples: HashSet<Triple>,
}

impl TripleStore {
    pub fn new() -> Self {
        Self {
            triples: HashSet::new(),
        }
    }

    pub fn add(&mut self, triple: Triple) {
        self.triples.insert(triple);
    }

    pub fn get(&self, subject: &str, predicate: &str, object: &str) -> Option<&Triple> {
        self.triples.iter().find(|&triple| {
            triple.subject == subject && triple.predicate == predicate && triple.object == object
        })
    }

    pub fn query(&self, query: &str) -> Vec<&Triple> {
        let mut result = Vec::new();
        println!("Query: {}", query);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut store = TripleStore::new();
        store.add(Triple {
            subject: "subject".to_string(),
            predicate: "predicate".to_string(),
            object: "object".to_string(),
        });

        assert_eq!(
            store.triples,
            vec![Triple {
                subject: "subject".to_string(),
                predicate: "predicate".to_string(),
                object: "object".to_string(),
            }]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_get() {
        let mut store = TripleStore::new();
        store.add(Triple {
            subject: "subject".to_string(),
            predicate: "predicate".to_string(),
            object: "object".to_string(),
        });

        assert_eq!(
            store.get("subject", "predicate", "object"),
            Some(&Triple {
                subject: "subject".to_string(),
                predicate: "predicate".to_string(),
                object: "object".to_string(),
            })
        );
    }

    #[test]
    fn test_query() {
        let mut store = TripleStore::new();
        store.add(Triple {
            subject: "subject".to_string(),
            predicate: "predicate".to_string(),
            object: "object".to_string(),
        });

        assert_eq!(
            store.query("SELECT ?predicate ?object WHERE { subject}"),
            vec![&Triple {
                subject: "subject".to_string(),
                predicate: "predicate".to_string(),
                object: "object".to_string(),
            }]
        );
    }
}
