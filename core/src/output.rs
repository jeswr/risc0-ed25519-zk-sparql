use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Output {
    pub result_string: String,
    pub pub_keys: Vec<String>,
    pub query_string: String,
}
