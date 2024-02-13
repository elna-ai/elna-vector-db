mod database;
mod config;
use database::db::Database;

// use std::cell::RefCell;


// thread_local! {
//     static DB:RefCell<Database> = RefCell::new(Database::new())

// }

fn main(){

    let mut db: Database = Database::new();
    let created = db.create_collection("name".to_string(), 3);
    println!("{:?}", created);
    let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,4.5],vec![10.0,11.0,10.5],vec![10.0,20.5,15.0]];
    let values: Vec<String> = vec!["red".to_string(),"green".to_string(), "blue".to_string()];
    let result=db.insert_into_collection("name", keys, values);

    println!("{:?}",result);
    let _ = db.build_index("name");

    let keys: Vec<Vec<f32>> = vec![vec![10.0,12.0,16.5],vec![10.0,30.0,40.5]];
    let values: Vec<String> = vec!["yellow".to_string(),"happy".to_string()];
    let result=db.insert_into_collection("name", keys, values);

    println!("{:?}",result);

    let _ = db.build_index("name");
    let query_vec: Vec<f32>=vec![10.0,12.5,4.5];
    let reslut = db.query("name",query_vec,1);
    println!("{:?}",reslut);


}