use std::collections::HashMap;

use strsim::{levenshtein, normalized_levenshtein};

pub struct SimSearch<Tag>
where
    Tag: PartialEq + Clone,
{
    id_next: u32,
    tags: Vec<(Tag, u32)>,
    forward_map: HashMap<u32, Vec<String>>,
    reverse_map: HashMap<String, Vec<u32>>,
}

impl<Tag> SimSearch<Tag>
where
    Tag: PartialEq + Clone,
{
    pub fn new() -> Self {
        SimSearch {
            id_next: 0,
            tags: Vec::new(),
            forward_map: HashMap::new(),
            reverse_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tag: Tag, terms: &[&str]) {
        self.delete(&tag);

        let id = self.id_next;
        self.id_next += 1;

        self.tags.push((tag, id));

        let terms: Vec<String> = terms
            .iter()
            .map(|term| term.trim().to_lowercase())
            .filter(|term| !term.is_empty())
            .collect();

        for term in terms.clone() {
            self.reverse_map
                .entry(term)
                .or_insert_with(|| Vec::with_capacity(1))
                .push(id);
        }

        self.forward_map.insert(id, terms);
    }

    pub fn delete(&mut self, tag: &Tag) {
        let id = self.tags.iter().find(|(t, _)| t == tag).map(|(_, id)| id);
        if let Some(id) = id {
            for term in &self.forward_map[id] {
                self.reverse_map.get_mut(term).unwrap().retain(|i| i != id);
            }
            self.forward_map.remove(id);
            self.tags.retain(|(t, _)| t != tag);
        }
    }

    pub fn search(&self, tokens: &[&str]) -> Vec<Tag> {
        let tokens: Vec<String> = tokens
            .iter()
            .map(|token| token.trim().to_lowercase())
            .filter(|token| !token.is_empty())
            .collect();

        let mut term_scores: HashMap<String, f64> = HashMap::new();

        for token in tokens {
            for term in self.reverse_map.keys() {
                let distance = levenshtein(&token, term);
                let len_diff = term.len().saturating_sub(token.len());
                let score = 1. - ((distance.saturating_sub(len_diff)) as f64 / token.len() as f64);

                if score > 0.7 {
                    let prefix_len = term.len() / 2;
                    let prefix_term =
                        String::from_utf8_lossy(term.as_bytes().split_at(prefix_len).0);
                    let score = (score
                        + normalized_levenshtein(&prefix_term, &token) as f64 / prefix_len as f64)
                        / 2.;
                    let score_current = term_scores.get(term).map(|score| *score).unwrap_or(1.);
                    term_scores.insert(term.to_owned(), score_current.min(score));
                }
            }
        }

        let mut result_scores: HashMap<u32, f64> = HashMap::new();

        for (term, score) in term_scores.drain() {
            for id in &self.reverse_map[&term] {
                *result_scores.entry(*id).or_insert(0.) += score;
            }
        }

        let mut result_scores: Vec<(u32, f64)> = result_scores.drain().collect();
        result_scores.sort_by(|lhs, rhs| rhs.1.partial_cmp(&lhs.1).unwrap());

        let result_tags: Vec<Tag> = result_scores
            .iter()
            .map(|(id, _)| {
                self.tags
                    .iter()
                    .find(|(_, i)| i == id)
                    .map(|(tag, _)| tag.clone())
                    .unwrap()
            }).collect();

        result_tags
    }
}
