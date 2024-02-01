use database::Database;

use std::collections::HashMap;

use crate::collection::Collection;
use crate::embedding::Embedding;
use crate::similarity::Distance;

#[ic_cdk::query]
fn test_welcome(name: String) -> String {
    format!("Machan, {}", name)
}

fn main() {
    let mut db: Database = Database::new();
    let created = db.create_collection("name".to_string(), 5, Distance::Cosine);
    println!("{:?}", created);
    println!("{:?}", db.get_collection("name"));

    let embedding1: Embedding = Embedding::new(
        String::from("example_id1"),
        vec![1.0, 2.0, 3.0, 6.0, 5.0],
        Some({
            let mut metadata = HashMap::new();
            metadata.insert(String::from("key1"), String::from("value1"));
            metadata.insert(String::from("key2"), String::from("value2"));
            metadata
        }),
    );

    let embedding2: Embedding = Embedding::new(
        String::from("example_id2"),
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        Some({
            let mut metadata = HashMap::new();
            metadata.insert(String::from("key1"), String::from("value1"));
            metadata.insert(String::from("key2"), String::from("value2"));
            metadata
        }),
    );

    let _ = db.insert_into_collection("name", embedding1);
    let _ = db.insert_into_collection("name", embedding2);
    let _ = db.add_content("example_id1".to_string(), "hello world".to_string());
    let _ = db.add_content("example_id2".to_string(), "hello alex".to_string());

    // println!("{:?}",db.get_collection("name"));
    let query: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    let collection: Collection = db.get_collection("name").cloned().unwrap();

    println!("{:?}", collection);

    let similar = collection.get_similarity(&query, 2);

    println!("{:?}", similar);

    for i in similar {
        db.get_content(i.embedding.id);
    }
}
