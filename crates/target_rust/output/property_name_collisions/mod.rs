// Code generated by jetted for Rust v0.1.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    #[serde(rename = "Foo")]
    pub foo: String,

    #[serde(rename = "foo")]
    pub foo0: String,
}
