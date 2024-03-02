use super::index::Vector;
use instant_distance::Search;
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
        values: Vec<String>,
        file_name:String ) -> Result<(), Error> {
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

        let _ = collection.append(&mut points, &mut _values,file_name);
        // collection.build_index();
        Ok(())
    }

    pub fn build_index(&mut self,name:&str) -> Result<(), String> {
        let collection = self.collections.get_mut(name).ok_or(Error::NotFound)?;
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

    // pub fn get_collection(&self, name: &str) -> Option<&Collection> {
    //     self.collections.get(name)
    // }

    pub fn delete_collection(&mut self, name: &str) -> Result<(), Error> {
        if let Some(_) = self.collections.remove(name) {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }
    
    pub fn get_all_collections(&self)->Vec<String> {

        self.collections.keys().cloned().collect()

    }

    pub fn get_docs(&mut self,index_name:&str)->Result<Vec<String>,Error>{


        let collection = match self.collections.get_mut(index_name) {
            Some(value) => value,
            None => return Err(Error::NotFound),
        };
        Ok(Vec::from_iter(collection.metadata.file_names.clone()))

    }



}

#[cfg(test)]
mod tests {
    use super::{Database, Error};

    #[test]
    fn create_collection() {
        let mut db: Database = Database::new();
        let result = db.create_collection("test".to_string(), 3);
        assert!(result.is_ok())
    }

    #[test]
    fn create_duplicate_collection() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 3);
        let result = db.create_collection("test".to_string(), 3);
        let expected = Err(Error::UniqueViolation);
        assert_eq!(result, expected);
    }

    #[test]
    fn delete_existing_collection() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 3);

        assert_eq!(db.delete_collection("test"), Ok(()))
    }

    #[test]
    fn delete_non_existing_collection() {
        let mut db: Database = Database::new();

        assert_eq!(db.delete_collection("test"), Err(Error::NotFound))
    }

    #[test]
    fn build_index() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 3);
        let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
        let values: Vec<String> = vec!["red".to_string(),"green".to_string(), "blue".to_string()];
        let _=db.insert_into_collection("test", keys, values,"test_file_name".to_string());
        let result = db.build_index("test");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn append_and_build_index() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 3);

        let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
        let values: Vec<String> = vec!["red".to_string(),"green".to_string(), "blue".to_string()];
        let _=db.insert_into_collection("test", keys, values,"test_file_name".to_string());
        let _ = db.build_index("test");

        let keys: Vec<Vec<f32>> = vec![vec![20.0,20.5,15.0]];
        let values: Vec<String> = vec!["black".to_string()];
        let _=db.insert_into_collection("test", keys, values,"test_file_name".to_string());
        let result = db.build_index("test");
        assert_eq!(result, Ok(()));
    }



    #[test]
    fn delete_collection_with_embeddings() {
        let mut db: Database = Database::new();
        let _ = db.create_collection("test".to_string(), 3);
        let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
        let values: Vec<String> = vec!["red".to_string(),"green".to_string(), "blue".to_string()];
        let _=db.insert_into_collection("test", keys, values,"test_file_name".to_string());
        let _ = db.build_index("test");
        assert_eq!(db.delete_collection("test"), Ok(()));
    }

 
    #[test]
    fn insert_into_collection_dimensions_mismatch_keys_values() {
        let mut db: Database = Database::new();

        let _ = db.create_collection("test".to_string(), 3);

        let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
        let values: Vec<String> = vec!["red".to_string(),"green".to_string()];
        let result=db.insert_into_collection("test", keys, values,"test_file_name".to_string());

        assert_eq!(result, Err(Error::DimensionMismatch));
    }

    #[test]
    fn insert_into_collection_dimensions_mismatch_keys() {
        let mut db: Database = Database::new();

        let _ = db.create_collection("test".to_string(), 3);

        let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0,10.2]];
        let values: Vec<String> = vec!["red".to_string(),"green".to_string(),"blue".to_string()];
        let result=db.insert_into_collection("test", keys, values,"test_file_name".to_string());

        assert_eq!(result, Err(Error::DimensionMismatch));
    }


    #[test]
    fn insert_into_collection_dimensions_mismatch() {
        let mut db: Database = Database::new();

        let _ = db.create_collection("test".to_string(), 4);

        let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
        let values: Vec<String> = vec!["red".to_string(),"green".to_string(),"blue".to_string()];
        let result=db.insert_into_collection("test", keys, values,"test_file_name".to_string());

        assert_eq!(result, Err(Error::DimensionMismatch));
    }


    #[test]
    fn insert_into_non_existing_collection() {
        let mut db: Database = Database::new();

        let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
        let values: Vec<String> = vec!["red".to_string(),"green".to_string(),"blue".to_string()];

        let result=db.insert_into_collection("test", keys, values,"test_file_name".to_string());

        assert_eq!(result, Err(Error::NotFound));
    }

    #[test]
    fn query() {
        let mut db = Database::new();
    let _ = db.create_collection("test".to_string(), 3);
    let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
    let values: Vec<String> = vec!["red".to_string(),"green".to_string(), "blue".to_string()];
    let _=db.insert_into_collection("test", keys, values,"test_file_name".to_string());

    let _ = db.build_index("test");

    let query_vec: Vec<f32>=vec![10.0,12.5,4.5];
    let result = db.query("test",query_vec,1);
    assert_eq!(result,Ok(vec![(0.9997943, "red".to_string())]));

    }

    #[test]
    fn query_with_append() {
        let mut db = Database::new();
    let _ = db.create_collection("test".to_string(), 3);
    let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
    let values: Vec<String> = vec!["red".to_string(),"green".to_string(), "blue".to_string()];
    let _=db.insert_into_collection("test", keys, values,"test_file_name".to_string());

    let _ = db.build_index("test");

    let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,16.5],vec![10.0,30.0,40.5]];
    let values: Vec<String> = vec!["yellow".to_string(),"happy".to_string()];
    let _=db.insert_into_collection("test", keys, values,"test_file_name".to_string());

    let _ = db.build_index("test");

    let query_vec: Vec<f32>=vec![10.0,30.5,35.5];
    let result = db.query("test",query_vec,1);
    assert_eq!(result,Ok(vec![(0.9973914, "happy".to_string())]));

    }
}