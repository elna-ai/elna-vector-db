use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, PartialEq)]
pub struct Embedding {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: Option<HashMap<String, String>>,
}

impl Embedding {
    pub fn new(id: String, vector: Vec<f32>, metadata: Option<HashMap<String, String>>) -> Self {
        Embedding {
            id,
            vector,
            metadata,
        }
    }
}
