// Code generated by jetted for Rust v0.1.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Root0 {
    #[serde(rename = "Bar")]
    Bar,

    #[serde(rename = "Baz")]
    Baz,

    #[serde(rename = "Foo")]
    Foo,
}

pub type Root = Option<Box<Root0>>;
