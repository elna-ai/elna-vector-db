use schemars::JsonSchema;
use std::collections::{HashMap,BinaryHeap};
use rayon::prelude::*;

use crate::similarity::{get_cache_attr, get_distance_fn, normalize, Distance, ScoreIndex};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct SimilarityResult {
	score: f32,
	embedding: Embedding,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Collection already exists")]
	UniqueViolation,

	#[error("Collection doesn't exist")]
	NotFound,

	#[error("The dimension of the vector doesn't match the dimension of the collection")]
	DimensionMismatch,
}




#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Embedding {
	pub id: String,
	pub vector: Vec<f32>,
	pub metadata: Option<HashMap<String, String>>,
}

impl Embedding {
    pub fn new(id: String, vector: Vec<f32>, metadata: Option<HashMap<String, String>>) -> Self {
        Embedding { id, vector, metadata }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
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
	pub fn get_similarity(&self, query: &[f32], k: usize) {
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


		println!("{:?}",scores);

		// let mut heap = BinaryHeap::new();
		// for score_index in scores {
		// 	if heap.len() < k || score_index < *heap.peek().unwrap() {
		// 		heap.push(score_index);

		// 		if heap.len() > k {
		// 			heap.pop();
		// 		}
		// 	}
		// }

	// 	heap.into_sorted_vec()
	// 		.into_iter()
	// 		.map(|ScoreIndex { score, index }| SimilarityResult {
	// 			score,
	// 			embedding: self.embeddings[index].clone(),
	// 		})
	// 		.collect()
	}
}


// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct Db {
// 	pub collections: HashMap<String, Collection>,
// }


// impl Db {
// 	pub fn new() -> Self {
// 		Self {
// 			collections: HashMap::new(),
// 		}
// 	}

// 	pub fn create_collection(
// 		&mut self,
// 		name: String,
// 		dimension: usize,
// 		distance: Distance,
// 	) -> Result<Collection, Error> {
// 		if self.collections.contains_key(&name) {
// 			return Err(Error::UniqueViolation);
// 		}

// 		let collection = Collection {
// 			dimension,
// 			distance,
// 			embeddings: Vec::new(),
// 		};

// 		self.collections.insert(name, collection.clone());

// 		Ok(collection)
// 	}
// }




