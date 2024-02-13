use super::index::Vector;
use instant_distance::Search;
use rayon::result;
use std::collections::HashMap;
use crate::database::collection::Collection;
use candid::CandidType;

#[derive(Debug, thiserror::Error, PartialEq, CandidType)]
pub enum Error {
    #[error("Collection already exists")]
    UniqueViolation,

    #[error("Collection doesn't exist")]
    NotFound,

    #[error("The dimension of the vector doesn't match the dimension of the collection")]
    DimensionMismatch,
}
impl From<Error> for String {
    fn from(error: Error) -> Self {
        // Convert the Error to a String representation
        error.to_string()
    }
}

pub struct Database {
    pub collections: HashMap<String, Collection>,
}


impl Database {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
        }
    }

    pub fn create_collection(
        &mut self,
        name: String,
        dimension: usize,
    ) -> Result<(), Error> {
        if self.collections.contains_key(&name) {
            return Err(Error::UniqueViolation);
        }
        let collection = Collection::new(vec![], vec![], dimension);
        self.collections.insert(name, collection);
        Ok(())
    }

    pub fn insert_into_collection(
        &mut self,
        name: &str,
        keys: Vec<Vec<f32>>, 
        values: Vec<String>    ) -> Result<(), Error> {
        let collection = self.collections.get_mut(name).ok_or(Error::NotFound)?;

        if keys.len() != values.len() {
            return Err(Error::DimensionMismatch);
        }

        let all_same_length = keys.iter().all(|inner| inner.len()==collection.dimension);

        println!("{}",all_same_length);

        if !all_same_length{

            return Err(Error::DimensionMismatch);

        }

        let mut points: Vec<Vector> = vec![];
        let mut _values: Vec<String> = vec![];

        for i in 0..keys.len() {
            let key = &keys[i];
            if key.len() !=  collection.dimension {
                continue;
            }
            let point = Vector::from((*key).clone());
            points.push(point);
            _values.push(values[i].clone());
        }

        let _ = collection.append(&mut points, &mut _values);
        // collection.build_index();
        Ok(())
    }

    pub fn build_index(&mut self,name:&str) -> Result<(), String> {
        let collection = self.collections.get_mut(name).ok_or(Error::NotFound)?;
        collection.build_index();

        Ok(())
    }

    pub fn test(
        &mut self,
        name: &str,
        keys: Vec<Vec<f32>>, 
        values: Vec<String>    ) -> Result<(), Error> {
        let collection = self.collections.get_mut(name).ok_or(Error::NotFound)?;

        // if collection.keys.iter().any(|e| e.id == embedding.id) {
        //     return Err(Error::UniqueViolation);
        // }

        // if collection.keys.len() != collection.dimension {
        //     return Err(Error::DimensionMismatch);
        // }

        // if collection.keys.len() != collection.values.len() {
        //     return Err(Error::DimensionMismatch);
        // }
        let mut points: Vec<Vector> = vec![];
        let mut _values: Vec<String> = vec![];

        for i in 0..keys.len() {
            let key = &keys[i];
            if key.len() !=  collection.dimension {
                continue;
            }
            let point = Vector::from((*key).clone());
            points.push(point);
            _values.push(values[i].clone());
        }

        collection.append(&mut points, &mut _values);
        collection.build_index();
        Ok(())
    }

    pub fn query(&mut self,name: &str,q: Vec<f32>, limit: i32) -> Result<Vec<(f32, String)>, String> {
        // let collection = self.collections.get_mut(name).ok_or_else(|| Error::NotFound)?;

        let collection = match self.collections.get_mut(name) {
            Some(value) => value,
            None => return Err(Error::NotFound.to_string()),
        };
        

        if q.len() != collection.dimension {
            return Err(String::from("query malformed"))
        }
    
        let mut search = Search::default();
        let v = Vector::from(q);
        let result=collection.query(&v, &mut search, limit);    
    
        Ok(result)
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }

    pub fn delete_collection(&mut self, name: &str) -> Result<(), Error> {
        if let Some(_) = self.collections.remove(name) {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }
    
}

