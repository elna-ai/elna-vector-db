pub mod database;

use database::{collection::Collection, db::Database, db::Error, similarity::Distance};
use ic_cdk::storage::{stable_restore, stable_save};
use std::{cell::RefCell, mem};

use ic_cdk_macros::{export_candid, post_upgrade, pre_upgrade, query, update};

thread_local! {
    static DB:RefCell<Database> = RefCell::new(Database::new())

}

#[query]
fn get_collection(name: String) -> Option<Collection> {
    DB.with(|db| {
        let db = db.borrow();
        db.get_collection(&name).cloned()
    })
}

#[update]
fn create_collection(name: String, dimension: usize) -> Result<Collection, Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(name, dimension, Distance::Euclidean)
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    let db: Database = DB.with(|db| mem::take(&mut *db.borrow_mut()));
    stable_save((db,)).expect("Unable to store db in pre_upgrade")
}

#[post_upgrade]
fn post_upgrade() {
    let (db,) = stable_restore().unwrap();
    DB.with(|db_new| *db_new.borrow_mut() = db)
}

export_candid!();
