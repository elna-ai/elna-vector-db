extern crate elna_auth_macros;

mod database;
use std::cell::RefCell;

use candid::CandidType;
use candid::Principal;
use database::db::DB;
use database::error::Error;
use database::memory::get_upgrades_memory;
use database::users::{ADMINS, OWNER};
use elna_auth_macros::check_authorization;
use ic_cdk::api::canister_balance;
use ic_cdk::{post_upgrade, pre_upgrade, query, update};
use ic_cdk_macros::export_candid;
use ic_stable_structures::writer::Writer;
use ic_stable_structures::Memory as _;
use serde::Deserialize;

thread_local! {
    static LOGS: RefCell<Vec<LogEntry>> = RefCell::default();

}

#[derive(Deserialize, CandidType, Debug, Clone)]
pub struct LogEntry {
    timestamp: u64,
    caller: Principal,
    function_name: String,
    total_cycles: u64,
}

//  implement Default for LogEntry
impl Default for LogEntry {
    fn default() -> Self {
        LogEntry {
            timestamp: 0,
            caller: Principal::anonymous(), // Use a default Principal (e.g., anonymous)
            function_name: String::new(),
            total_cycles: 0,
        }
    }
}

#[update]
#[check_authorization]
fn create_collection(name: String, dimension: usize) -> Result<(), Error> {
    let initial_cycles = canister_balance();
    ic_cdk::println!("fn:create_collection: initial_cycles:{}", initial_cycles);

    let result = DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(&name, dimension)
    });

    // Get the final cycle count

    let final_cycles = canister_balance();
    ic_cdk::println!("fn:create_collection: final_cycles:{}", final_cycles);
    // Calculate the cycles used
    let cycles_used = initial_cycles - final_cycles;
    ic_cdk::println!("fn:create_collection: cycles_used:{}", cycles_used);

    // Create a new log entry
    let log_entry = LogEntry {
        timestamp: ic_cdk::api::time(), // Use the current timestamp
        caller: ic_cdk::caller(),
        function_name: "create_collection".to_string(),
        total_cycles: cycles_used,
    };

    // Append the log entry to the LOGS
    LOGS.with(|logs| {
        logs.borrow_mut().push(log_entry);
    });

    result
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
    let initial_cycles = canister_balance();
    ic_cdk::println!("fn:create_index: initial_cycles:{}", initial_cycles);

    let result = DB.with(|db| {
        let mut db = db.borrow_mut();
        let _ = db.create_collection(&name, dimension);
        let _ = db.insert_into_collection(&name, embeddings, docs, file_name);
        db.build_index(&name)
    });

    // Get the final cycle count

    let final_cycles = canister_balance();
    ic_cdk::println!("fn:create_index: final_cycles:{}", final_cycles);
    // Calculate the cycles used
    let cycles_used = initial_cycles - final_cycles;
    ic_cdk::println!("fn:create_index: cycles_used:{}", cycles_used);

    // Create a new log entry
    let log_entry = LogEntry {
        timestamp: ic_cdk::api::time(), // Use the current timestamp
        caller: ic_cdk::caller(),
        function_name: "create_index".to_string(),
        total_cycles: cycles_used,
    };

    // Append the log entry to the LOGS
    LOGS.with(|logs| {
        logs.borrow_mut().push(log_entry);
    });

    result
}

#[update]
#[check_authorization]
fn insert(
    name: String,
    keys: Vec<Vec<f32>>,
    values: Vec<String>,
    file_name: String,
) -> Result<(), Error> {
    let initial_cycles = canister_balance();
    ic_cdk::println!("fn:insert: initial_cycles:{}", initial_cycles);

    let _ = DB.with(|db| {
        let mut db = db.borrow_mut();
        db.insert_into_collection(&name, keys, values, file_name)
    });

    // Get the final cycle count

    let final_cycles = canister_balance();
    ic_cdk::println!("fn:insert: final_cycles:{}", final_cycles);
    // Calculate the cycles used
    let cycles_used = initial_cycles - final_cycles;
    ic_cdk::println!("fn:insert: cycles_used:{}", cycles_used);

    // Create a new log entry
    let log_entry = LogEntry {
        timestamp: ic_cdk::api::time(), // Use the current timestamp
        caller: ic_cdk::caller(),
        function_name: "insert".to_string(),
        total_cycles: cycles_used,
    };

    // Append the log entry to the LOGS
    LOGS.with(|logs| {
        logs.borrow_mut().push(log_entry);
    });
    Ok(())
}

#[update]
#[check_authorization]

fn build_index(name: String) -> Result<(), Error> {
    let initial_cycles = canister_balance();
    ic_cdk::println!("fn:build_index: initial_cycles:{}", initial_cycles);
    let _ = DB.with(|db| {
        let mut db = db.borrow_mut();
        db.build_index(&name)
    });

    // Get the final cycle count

    let final_cycles = canister_balance();
    ic_cdk::println!("fn:build_index: final_cycles:{}", final_cycles);
    // Calculate the cycles used
    let cycles_used = initial_cycles - final_cycles;
    ic_cdk::println!("fn:build_index: cycles_used:{}", cycles_used);

    // Create a new log entry
    let log_entry = LogEntry {
        timestamp: ic_cdk::api::time(), // Use the current timestamp
        caller: ic_cdk::caller(),
        function_name: "build_index".to_string(),
        total_cycles: cycles_used,
    };

    // Append the log entry to the LOGS
    LOGS.with(|logs| {
        logs.borrow_mut().push(log_entry);
    });
    Ok(())
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
    let initial_cycles = canister_balance();
    ic_cdk::println!("fn:query: initial_cycles:{}", initial_cycles);

    DB.with(|db| {
        let mut db = db.borrow_mut();
        let result = db.query(&name, q, limit);
        match result {
            Ok(data) => {
                // Extract the Vec<(f32, String)> from the Ok variant
                let (_, strings): (Vec<_>, Vec<_>) = data.into_iter().unzip();
                // Get the final cycle count

                let final_cycles = canister_balance();
                ic_cdk::println!("fn:query: final_cycles:{}", final_cycles);
                // Calculate the cycles used
                let cycles_used = initial_cycles - final_cycles;
                ic_cdk::println!("fn:query: cycles_used:{}", cycles_used);

                // Create a new log entry
                let log_entry = LogEntry {
                    timestamp: ic_cdk::api::time(), // Use the current timestamp
                    caller: ic_cdk::caller(),
                    function_name: "query".to_string(),
                    total_cycles: cycles_used,
                };

                // Append the log entry to the LOGS
                LOGS.with(|logs| {
                    logs.borrow_mut().push(log_entry);
                });

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
