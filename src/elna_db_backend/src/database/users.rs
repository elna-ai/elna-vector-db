use crate::database::memory::{get_stable_btree_memory, Memory};

use crate::database::error::Error;
use candid::{CandidType, Principal};
use elna_auth_macros::check_is_owner;
use ic_cdk::{init, query, update};
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};

use std::cell::RefCell;

#[derive(CandidType, Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub struct StorablePrincipal(pub Principal);

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
    pub static ADMINS: RefCell<StableBTreeMap<StorablePrincipal, bool, Memory>> = RefCell::new(init_stable_data());
}

#[init]
fn init(owner: Principal) {
    OWNER.with(|o| *o.borrow_mut() = owner.to_string());
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

fn init_stable_data() -> StableBTreeMap<StorablePrincipal, bool, Memory> {
    StableBTreeMap::init(get_stable_btree_memory())
}
