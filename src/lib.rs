// mod collection;
pub mod database;

use database::{db::Database,collection::Collection};
// use ic_cdk::{init, query};
use std::cell::RefCell;

use ic_cdk_macros::{query,export_candid};

thread_local! {
    static DB:RefCell<Database> = RefCell::new(Database::new())

}

// #[init]
// fn init() {
//     ic_cdk::println!("Db initialized...");
// }

#[query]
fn get_collection(name: String) -> Option<Collection> {
    DB.with(|db| {
        let db = db.borrow();
        db.get_collection(&name).cloned()
    })
}

// #[candid_method(update)]
// #[update]
// fn create_collection(name: String, dimension: usize) -> Result<Collection, db::Error> {
//     DB.with(|db| {
//         let db = db.borrow();
//         db.create_collection(name, dimension, Distance::Euclidean)
//     })
// }

// #[query(name = "__get_candid_interface_tmp_hack")]
// fn export_candid() -> String {
//     export_service!();
//     __export_service()
// }


export_candid!();
