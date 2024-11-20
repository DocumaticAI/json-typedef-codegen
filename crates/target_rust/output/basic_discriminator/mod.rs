// Code generated by jtd-codegen for Rust v0.2.1

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "foo")]
pub enum Root {
    #[serde(rename = "BAR_BAZ")]
    BarBaz(RootBarBaz),

    #[serde(rename = "QUUX")]
    Quux(RootQuux),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootBarBaz {
    #[serde(rename = "baz")]
    pub baz: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootQuux {
    #[serde(rename = "quuz")]
    pub quuz: String,
}
