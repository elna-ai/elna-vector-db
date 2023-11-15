pub mod db;
pub mod similarity;

use db::Db;
use schemars::JsonSchema;

use std::collections::HashMap;

use crate::db::{Collection, Embedding};
use crate::similarity::Distance;

// fn get_embdding()->Embedding{

//     // Creating a sample Embedding object
//     let embedding = Embedding {
//         id: String::from("sample_id"),
//         vector: vec![1.0, 2.0, 3.0],
//         metadata: Some({
//             let mut metadata = HashMap::new();
//             metadata.insert(String::from("key1"), String::from("value1"));
//             metadata.insert(String::from("key2"), String::from("value2"));
//             metadata
//         }),
//     };

//     embedding

// }

fn get_collection(embedding: Embedding) -> Collection {
    // Creating a sample Collection object
    let collection = Collection {
        dimension: 3,
        distance: Distance::Cosine,
        embeddings: vec![embedding],
    };

    collection
}

fn main() {
    let mut db: Db = Db::new();
    let created = db.create_collection("name".to_string(), 5, Distance::Cosine);
    println!("{:?}", created);
    println!("{:?}", db.get_collection("name"));
    let created = db.create_collection("name".to_string(), 5, Distance::Cosine);
    println!("{:?}", created);

    // let embedding: Embedding = Embedding::new(
    //     String::from("example_id"),
    //     vec![1.0, 2.0, 3.0],
    //     Some({
    //         let mut metadata = HashMap::new();
    //         metadata.insert(String::from("key1"), String::from("value1"));
    //         metadata.insert(String::from("key2"), String::from("value2"));
    //         metadata
    //     }),
    // );
    // let query:[f32; 3]  = [5.0, 2.0, 3.0];

    // println!("{:?}", embedding);

    // let collection=get_collection(embedding);

    // println!("{:?}", collection);

    // collection.get_similarity(&query,1)
}
