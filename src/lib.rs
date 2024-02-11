use std::collections::{HashSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd,)]
pub struct Triple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

pub(crate) struct TripleStore {
    triples: HashSet<Triple>,
}

struct SelectQuery {
    variables: Vec<String>,
    where_clause: Vec<String>,
}

impl SelectQuery {
    fn from_query(query_vec: Vec<&str>) -> Self {
        let mut variables = Vec::new();
        let mut where_clause = Vec::new();
        let mut in_where_clause = false;

        for word in &query_vec {
            match *word {
                "SELECT" => {
                    // do nothing, just indicates start of query
                }
                "WHERE" => {
                    in_where_clause = true;
                }
                _ => {
                    if in_where_clause {
                        if word != &"{" && word != &"}" && word.starts_with("?") == false {
                            where_clause.push(word.to_string());
                        }
                    } else if word.starts_with("?") {
                        variables.push(word.to_string());
                    }
                }
            }
        }

       SelectQuery {
            variables,
            where_clause,
        }
    }

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
        let words = query.split_whitespace().collect::<Vec<_>>();

        match words.clone()[0] {
            "SELECT" => {
                let query_struct = SelectQuery::from_query(words.clone());
                self.triples.iter().for_each(|triple| {
                    let mut matches = 0;
                    for clause in &query_struct.where_clause {
                        if triple.subject == *clause || triple.predicate == *clause || triple.object == *clause {
                            matches += 1;
                        }
                    }
                    if matches == query_struct.where_clause.len() {
                        result.push(triple);
                    }
                });
            }
            _ => {
                println!("todo");
            }
        }
        result
    }

    fn select(&self, query: &str) -> Vec<&Triple> {
        let mut result = Vec::new();
        let words = query.split_whitespace().collect::<Vec<_>>();
        for word in words {
            println!("{}", word);
        }
        result
    }
}

// I will put the lexing and parsing code here
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Select,
    Where,
    OpenBrace,
    CloseBrace,
    Variable(String),
    Dot,
    Integer(usize),
    Decimal(f64),
}

// Helper functions to convert from other types to TokenKind
impl From<usize> for TokenKind {
    fn from(other: usize) -> TokenKind {
        TokenKind::Integer(other)
    }
}

impl From<f64> for TokenKind {
    fn from(other: f64) -> TokenKind {
        TokenKind::Decimal(other)
    }
}


// Tokenizing
pub fn tokenize(input: &str) -> Vec<TokenKind> {
    let mut tokens = Vec::new();
    let mut word = String::new();
    for c in input.chars() {
        match c {
            ' ' => {
                if !word.is_empty() {
                    tokens.push(match word.parse::<usize>() {
                        Ok(num) => TokenKind::Integer(num),
                        Err(_) => match word.parse::<f64>() {
                            Ok(num) => TokenKind::Decimal(num),
                            Err(_) => match word.as_str() {
                                "SELECT" => TokenKind::Select,
                                "WHERE" => TokenKind::Where,
                                "{" => TokenKind::OpenBrace,
                                "}" => TokenKind::CloseBrace,
                                _ => TokenKind::Variable(word.clone()),
                            },
                        },
                    });
                    word.clear();
                }
            }
            _ => {
                word.push(c);
            }
        }
    }
    tokens
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
            subject: "brown_dog".to_string(),
            predicate: "hat_color".to_string(),
            object: "red".to_string(),
        });

        store.add(Triple {
            subject: "brown_dog".to_string(),
            predicate: "hat_color".to_string(),
            object: "green".to_string(),
        });

        store.add(Triple {
            subject: "green_dog".to_string(),
            predicate: "hat_color".to_string(),
            object: "brown".to_string(),
        });

        // Expected

        let brown_dog_red_hat = Triple {
            subject: "brown_dog".to_string(),
            predicate: "hat_color".to_string(),
            object: "red".to_string(),
        };
        let brown_dog_green_hat = Triple {
            subject: "brown".to_string(),
            predicate: "hat_color".to_string(),
            object: "green".to_string(),
        };

        let query_l = store.query("SELECT ?predicate ?object WHERE { brown_dog }").sort();
        let expected_single = vec![&brown_dog_red_hat, &brown_dog_green_hat,].sort();
        
        assert_eq!(query_l, expected_single);

        let query_m = store.query("SELECT ?predicate ?object WHERE { brown_dog ?hat_color red }");
        let expected_multiple  = vec![&brown_dog_red_hat,];
        assert_eq!(query_m, expected_multiple);
    }

    // Tokenizing tests
    #[test]
    fn test_tokenize() {
        let input = "SELECT ?predicate ?object WHERE { brown_dog ?hat_color red }";
        let expected = vec![
            TokenKind::Select,
            TokenKind::Variable("?predicate".to_string()),
            TokenKind::Variable("?object".to_string()),
            TokenKind::Where,
            TokenKind::OpenBrace,
            TokenKind::Variable("brown_dog".to_string()),
            TokenKind::Variable("?hat_color".to_string()),
            TokenKind::Variable("red".to_string()),
        ];
        assert_eq!(tokenize(input), expected);
    }
}
