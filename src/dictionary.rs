// src/dictionary.rs

use indexmap::IndexMap;

pub struct Dictionary {
    pub token_to_idx: IndexMap<String, usize>,
    pub idx_to_token: Vec<String>,
}

impl Dictionary {
    /// Build a dictionary mapping each unique token to a unique index (in insertion order),
    /// and also provides an index→token Vec for reverse lookup.
    pub fn new(tokens: &[String]) -> Self {
        let mut map = IndexMap::new();

        // Insert each token only once, assigning it the next available index.
        for token in tokens {
            if !map.contains_key(token) {
                let idx = map.len();
                map.insert(token.clone(), idx);
            }
        }

        // Build the reverse map: index → token
        let idx_to_token = map.keys().cloned().collect::<Vec<String>>();

        Dictionary {
            token_to_idx: map,
            idx_to_token,
        }
    }
}
