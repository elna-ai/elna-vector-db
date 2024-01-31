use std::collections::HashMap;

use candid::CandidType;

use super::similarity::{normalize, Distance};

use super::collection::Collection;
use super::embedding::Embedding;

#[derive(Debug, thiserror::Error, PartialEq, CandidType)]
pub enum Error {
    #[error("Collection already exists")]
    UniqueViolation,

    #[error("Collection doesn't exist")]
    NotFound,

    #[error("The dimension of the vector doesn't match the dimension of the collection")]
    DimensionMismatch,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, CandidType, Default)]
pub struct Database {
    pub collections: HashMap<String, Collection>,
    content: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
            content: HashMap::new(),
        }
    }

    pub fn create_collection(
        &mut self,
        name: String,
        dimension: usize,
        distance: Distance,
    ) -> Result<Collection, Error> {
        if self.collections.contains_key(&name) {
            return Err(Error::UniqueViolation);
        }

        let collection = Collection {
            dimension,
            distance,
            embeddings: Vec::new(),
        };

        self.collections.insert(name, collection.clone());

        Ok(collection)
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }

    pub fn delete_collection(&mut self, name: &str) -> Result<(), Error> {
        if let Some(collection) = self.collections.remove(name) {
            collection.embeddings.iter().for_each(|embedding| {
                let _ = self.remove_content(&embedding.id);
            });
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn insert_into_collection(
        &mut self,
        name: &str,
        mut embedding: Embedding,
    ) -> Result<(), Error> {
        let collection = self.collections.get_mut(name).ok_or(Error::NotFound)?;

        if collection.embeddings.iter().any(|e| e.id == embedding.id) {
            return Err(Error::UniqueViolation);
        }

        if embedding.vector.len() != collection.dimension {
            return Err(Error::DimensionMismatch);
        }

        embedding.vector = normalize(&embedding.vector);
        collection.embeddings.push(embedding);
        Ok(())
    }

    pub fn add_content(&mut self, id: String, content: String) -> Result<(), Error> {
        if self.content.contains_key(&id) {
            return Err(Error::UniqueViolation);
        }

        self.content.insert(id, content);
        Ok(())
    }

    pub fn get_content(&self, id: String) {
        let content = self.content.get(&id);

        println!("{:?}", content);
    }

    pub fn remove_content(&mut self, id: &str) -> Result<(), Error> {
        match self.content.remove(id) {
            None => Err(Error::NotFound),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Database, Embedding, Error};
    use std::collections::HashMap;

    #[test]
    fn create_collection() {
        let mut db: Database = Database::new();
        let result = db.create_collection("test".to_string(), 10, super::Distance::Cosine);
        assert!(result.is_ok())
    }

    #[test]
    fn get_collection() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 10, super::Distance::Cosine);
        let result = db.get_collection("test");
        assert!(result.is_some())
    }

    #[test]
    fn create_duplicate_collection() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 10, super::Distance::Cosine);
        let result = db.create_collection("test".to_string(), 10, super::Distance::Cosine);
        let expected = Err(Error::UniqueViolation);
        assert_eq!(result, expected);
    }

    #[test]
    fn delete_existing_collection() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 10, super::Distance::Cosine);

        assert_eq!(db.delete_collection("test"), Ok(()))
    }

    #[test]
    fn delete_non_existing_collection() {
        let mut db: Database = Database::new();

        assert_eq!(db.delete_collection("test"), Err(Error::NotFound))
    }

    #[test]
    fn delete_collection_with_embeddings() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 10, super::Distance::Cosine);
        let embedding = Embedding {
            id: "test-id".to_string(),
            vector: vec![0.5, 1.3, 0.9, 5.0],
            metadata: Some(HashMap::default()),
        };
        let _ = db.insert_into_collection("test", embedding);

        assert_eq!(db.delete_collection("test"), Ok(()));
        assert!(db.content.is_empty());
    }

    #[test]
    fn insert_into_collection() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 4, super::Distance::Cosine);
        let embedding = Embedding {
            id: "test-id".to_string(),
            vector: vec![0.5, 1.3, 0.9, 5.0],
            metadata: Some(HashMap::default()),
        };
        let result = db.insert_into_collection("test", embedding);
        assert_eq!(result.unwrap(), ());
    }
    #[test]
    fn insert_into_collection_dimensions_mismatch() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 4, super::Distance::Cosine);
        let embedding = Embedding {
            id: "test-id".to_string(),
            vector: vec![0.5, 1.3, 0.9],
            metadata: Some(HashMap::default()),
        };
        let result = db.insert_into_collection("test", embedding);
        assert_eq!(result, Err(Error::DimensionMismatch));
    }

    #[test]
    fn insert_into_non_existing_collection() {
        let mut db: Database = Database::new();
        let embedding = Embedding {
            id: "test-id".to_string(),
            vector: vec![0.5, 1.3, 0.9],
            metadata: Some(HashMap::default()),
        };
        let result = db.insert_into_collection("test", embedding);
        assert_eq!(result, Err(Error::NotFound));
    }

    #[test]
    fn add_content_to_db() {
        let mut db: Database = Database::new();
        let result = db.add_content("content-id".to_string(), "Some content".to_string());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn add_duplicate_content_to_db() {
        let mut db: Database = Database::new();
        let _ = db.add_content("content-id".to_string(), "Some content".to_string());
        let result = db.add_content("content-id".to_string(), "Duplicate content".to_string());
        assert_eq!(result, Err(Error::UniqueViolation));
    }

    #[test]
    fn remove_existing_content_from_db() {
        let mut db: Database = Database::new();
        let _ = db.add_content("content-id".to_string(), "Some content".to_string());
        let result = db.remove_content("content-id");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn remove_non_existing_content_from_db() {
        let mut db: Database = Database::new();
        let result = db.remove_content("non-existing-content-id");
        assert_eq!(result, Err(Error::NotFound));
    }

    #[test]
    fn get_content() {
        let mut db = Database::new();
        let _ = db.add_content("content-id".to_string(), "Some content".to_string());
        let content = db.get_content("content-id".to_string());
        println!("{:?}", content);
    }
}
