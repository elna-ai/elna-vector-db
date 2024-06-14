extern crate elna_auth_macros;

mod database;
use candid::{CandidType, Principal};
use database::db::DB;
use database::error::Error;
use database::memory::get_upgrades_memory;
use elna_auth_macros::{check_authorization, check_is_owner};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_cdk_macros::export_candid;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    writer::Writer,
    DefaultMemoryImpl, Memory as _, StableBTreeMap, Storable,
};
use std::cell::RefCell;

#[derive(CandidType, Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]
struct StorablePrincipal(Principal);

type Memory = VirtualMemory<DefaultMemoryImpl>;

impl Storable for StorablePrincipal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Borrowed(self.0.as_slice())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(Principal::from_slice(&bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 50,
        is_fixed_size: false,
    };
}

thread_local! {
    pub static OWNER: RefCell<String> = RefCell::new(String::new());
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static ADMINS: RefCell<StableBTreeMap<StorablePrincipal, bool, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );
}

#[init]
fn init(owner: Principal) {
    OWNER.with(|o| *o.borrow_mut() = owner.to_string());
}

#[update]
#[check_authorization]
fn create_collection(name: String, dimension: usize) -> Result<(), Error> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        db.create_collection(name, dimension)
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

#[query]
#[check_is_owner]
fn get_admins() -> Result<Vec<Principal>, Error> {
    ADMINS.with(|admins| {
        let admins = admins.borrow().iter().map(|(k, _)| k.0).collect::<Vec<_>>();
        Ok(admins)
    })
}

#[update]
#[check_is_owner]
fn add_admin(principal_id: Principal) -> Result<(), Error> {
    let target_principal = StorablePrincipal(principal_id);
    ADMINS.with(|admins| {
        let mut admins = admins.borrow_mut();
        let exists = admins.contains_key(&target_principal);
        if exists {
            return Err(Error::UniqueViolation);
        }
        admins.insert(target_principal, true);
        return Ok(());
    })
}

#[update]
#[check_is_owner]
fn remove_admin(principal_id: Principal) -> Result<(), Error> {
    let target_principal = StorablePrincipal(principal_id);
    ADMINS.with(|admins| {
        let mut admins = admins.borrow_mut();
        let exists = admins.contains_key(&target_principal);
        if !exists {
            return Err(Error::NotFound);
        }

        admins.remove(&target_principal);
        return Ok(());
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
    init(owner);
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
