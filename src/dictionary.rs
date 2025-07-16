// src/dictionary.rs
use indexmap::IndexMap;

/// Build token-to-index and index-to-token mappings
pub struct Dictionary {
    pub token_to_idx: IndexMap<String, usize>,
    pub idx_to_token: Vec<String>,
}

impl Dictionary {
    pub fn new(tokens: &[String]) -> Self {
        let mut token_to_idx = IndexMap::new();
        for token in tokens {
            token_to_idx.entry(token.clone())
                .or_insert_with(|| token_to_idx.len());
        }
        let idx_to_token = token_to_idx.keys().cloned().collect();
        Dictionary { token_to_idx, idx_to_token }
    }
}