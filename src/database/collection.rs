use super::embedding::Embedding;
use candid::CandidType;
use rayon::prelude::*;

use super::similarity::{get_cache_attr, get_distance_fn, Distance, ScoreIndex};
use schemars::JsonSchema;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct SimilarityResult {
    score: f32,
    pub embedding: Embedding,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, PartialEq, CandidType)]
pub struct Collection {
    /// Dimension of the vectors in the collection
    pub dimension: usize,
    /// Distance metric used for querying
    pub distance: Distance,
    /// Embeddings in the collection
    #[serde(default)]
    pub embeddings: Vec<Embedding>,
}

impl Collection {
    pub fn get_similarity(&self, query: &[f32], k: usize) -> Vec<SimilarityResult> {
        let memo_attr = get_cache_attr(self.distance, query);
        let distance_fn = get_distance_fn(self.distance);

        let scores = self
            .embeddings
            .par_iter()
            .enumerate()
            .map(|(index, embedding)| {
                let score = distance_fn(&embedding.vector, query, memo_attr);
                ScoreIndex { score, index }
            })
            .collect::<Vec<_>>();

        let mut heap = BinaryHeap::new();
        for score_index in scores {
            if heap.len() < k || score_index < *heap.peek().unwrap() {
                heap.push(score_index);

                if heap.len() > k {
                    heap.pop();
                }
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|ScoreIndex { score, index }| SimilarityResult {
                score,
                embedding: self.embeddings[index].clone(),
            })
            .collect()
    }
}
