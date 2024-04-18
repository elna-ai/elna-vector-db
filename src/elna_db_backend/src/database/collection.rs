use super::index::{generate_index, Vector};
use ciborium::de;
use ic_stable_structures::{storable::Bound, Storable};
use instant_distance::{HnswMap, Search};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{collections::HashSet, usize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub file_names: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub dimension: usize,
    pub metadata: Metadata,
    inner: HnswMap<Vector, String>,
    keys: Vec<Vector>,
    values: Vec<String>,
}

impl Storable for Collection {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        ciborium::ser::into_writer(self, &mut bytes).unwrap();
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let canister_wasm: Collection = de::from_reader(bytes.as_ref()).unwrap();
        canister_wasm
    }

    const BOUND: Bound = Bound::Unbounded;
}

// impl Storable for Collection {
//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(&bytes, Self).unwrap()
//     }

//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(&self).unwrap())
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

impl Collection {
    pub fn new(keys: Vec<Vector>, values: Vec<String>, dimension: usize) -> Self {
        Collection {
            keys: keys.clone(),
            values: values.clone(),
            inner: generate_index(keys, values),
            dimension,
            metadata: Metadata {
                file_names: HashSet::new(),
            },
        }
    }

    pub fn append(
        &mut self,
        keys: &mut Vec<Vector>,
        values: &mut Vec<String>,
        file_name: String,
    ) -> Result<(), String> {
        if keys.len() != values.len() {
            return Err(String::from("length of keys not euqal to values'"));
        }
        self.keys.append(keys);
        self.values.append(values);
        self.metadata.file_names.insert(file_name);

        Ok(())
    }

    pub fn query(&self, key: &Vector, search: &mut Search, limit: i32) -> Vec<(f32, String)> {
        let mut res: Vec<(f32, String)> = vec![];
        let mut iter = self.inner.search(key, search);
        for _ in 0..limit {
            match iter.next() {
                Some(v) => res.push((v.point.cos_sim(key), (*v.value).clone())),
                None => break,
            }
        }

        res
    }
    pub fn build_index(&mut self) {
        self.inner = generate_index(self.keys.clone(), self.values.clone())
    }
}
