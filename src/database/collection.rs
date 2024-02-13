use crate::database::index::Vector;
use instant_distance::{HnswMap,Search};
use super::index::generate_index;

pub struct Collection {
    pub dimension: usize,
    pub inner: HnswMap<Vector, String>,
    pub keys: Vec<Vector>,
    pub values: Vec<String>,
}

impl Collection {
    pub fn new(keys: Vec<Vector>, values: Vec<String>,dimension:usize) -> Self {
        Collection { 
            dimension: dimension.clone(),
            keys: keys.clone(), 
            values: values.clone(), 
            inner: generate_index(keys, values),
        }
    }

    pub fn append(&mut self, keys: &mut Vec<Vector>, values: &mut Vec<String>) -> Result<(), String> {
        // if keys.len() != values.len() {
        //     return Err(String::from("length of keys not euqal to values'"));
        // }
        self.keys.append(keys);
        self.values.append(values);

        Ok(())
    }

    pub fn query(&self, key: &Vector, search: &mut Search, limit: i32) -> Vec<(f32, String)> {
        let mut res: Vec<(f32, String)> = vec![];
        let mut iter = self.inner.search(key, search);
        for _ in 0..limit {
            match iter.next() {
                Some(v) => {
                    res.push((v.point.cos_sim(key), (*v.value).clone()))
                },
                None => break
            }
        };

        res
    }
    pub fn build_index(&mut self) {
        self.inner = generate_index(self.keys.clone(), self.values.clone())
    }

}