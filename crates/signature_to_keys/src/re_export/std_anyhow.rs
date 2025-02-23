pub use derive_builder::Builder;


pub use serde::{
    Deserialize,
    Serialize
};

pub use anyhow;
pub use anyhow::Error;
// pub use anyhow::{
    // anyhow,
    // Result,
    // Error,
// };

pub use std::{
    env,
    collections::HashMap,
    fmt::Write,
};



pub fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default_value.to_string())
}


pub use std::mem::drop;
