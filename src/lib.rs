pub mod database;
pub mod config;

use database::db::{Database,Error};
use ic_cdk::{export_candid, query, update};


use std::cell::RefCell;

thread_local! {
    static DB: RefCell<Database> = RefCell::new(Database::new())
}


#[update]
fn create_collection(name: String, dimension: usize) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(name, dimension)
    })
}


#[update]
fn insert(name: String, keys: Vec<Vec<f32>>, values: Vec<String>,file_name:String) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.insert_into_collection(&name,keys, values,file_name)
    })
}

#[update]
fn build_index(name:String) -> Result<(), String> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.build_index(&name)
    })
}

#[update]
fn delete_collection(name: String) -> Result<(), Error> {

    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.delete_collection(&name)
    })

}

#[query]
fn query(name:String,q: Vec<f32>, limit: i32) -> Vec<String> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        let result = db.query(&name,q,limit);
        match result {
            Ok(data) => {
                // Extract the Vec<(f32, String)> from the Ok variant
                let (_, strings): (Vec<_>, Vec<_>) = data.into_iter().unzip();
                // println!("Floats: {:?}", floats);
                 strings
            }
            Err(error) => {
                // Handle the error
                println!("Error: {}", error);
                vec![error]
            }
        }
        
    })

}

#[query]
fn get_collections() -> Vec<String> {

    DB.with(|db| {
        let db = db.borrow();
        db.get_all_collections()
    })

}

#[query]
fn get_docs(index_name:String) -> Result<Vec<String>,Error> {

    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.get_docs(index_name.as_str())
    })

}

export_candid!();
