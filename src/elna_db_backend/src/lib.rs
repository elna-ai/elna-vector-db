extern crate elna_auth_macros;

mod database;
use candid::Principal;
use database::db::DB;
use database::error::Error;
use database::memory::get_upgrades_memory;
use database::users::{ADMINS, OWNER};
use elna_auth_macros::check_authorization;
use ic_cdk::{post_upgrade, pre_upgrade, query, update};
use ic_cdk_macros::export_candid;
use ic_stable_structures::writer::Writer;
use ic_stable_structures::Memory as _;

#[update]
#[check_authorization]
fn create_collection(name: String, dimension: usize) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(&name, dimension)
    })
}

#[update]
#[check_authorization]
fn create_index(
    name: String,
    dimension: usize,
    docs: Vec<String>,
    embeddings: Vec<Vec<f32>>,
    file_name: String,
) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(&name, dimension);
        db.insert_into_collection(&name, embeddings, docs, file_name);
        db.build_index(&name)
    })
}

#[update]
#[check_authorization]
fn insert(
    name: String,
    keys: Vec<Vec<f32>>,
    values: Vec<String>,
    file_name: String,
) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.insert_into_collection(&name, keys, values, file_name)
    })
}

#[update]
#[check_authorization]
fn build_index(name: String) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.build_index(&name)
    })
}

#[update]
#[check_authorization]
fn delete_collection(name: String) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.delete_collection(&name)
    })
}

#[query]
#[check_authorization]
fn query(name: String, q: Vec<f32>, limit: i32) -> Result<Vec<String>, Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        let result = db.query(&name, q, limit);
        match result {
            Ok(data) => {
                // Extract the Vec<(f32, String)> from the Ok variant
                let (_, strings): (Vec<_>, Vec<_>) = data.into_iter().unzip();
                // println!("Floats: {:?}", floats);
                Ok(strings)
            }
            Err(error) => {
                println!("Error: {}", error);
                Err(Error::NotFound)
            }
        }
    })
}

#[query]
#[check_authorization]
fn get_collections() -> Result<Vec<String>, Error> {
    DB.with(|db| {
        let db = db.borrow();
        Ok(db.get_all_collections())
    })
}

#[query]
#[check_authorization]
fn get_docs(index_name: String) -> Result<Vec<String>, Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.get_docs(&index_name)
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    DB.with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

// A post-upgrade hook for deserializing the data back into the heap.
#[post_upgrade]
fn post_upgrade(owner: Principal) {
    OWNER.with(|o| *o.borrow_mut() = owner.to_string());

    let memory = get_upgrades_memory();
    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    DB.with(|s| *s.borrow_mut() = state);
}

export_candid!();
