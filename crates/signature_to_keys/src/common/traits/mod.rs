pub mod json_compact_data;
// pub use crate::common::enums::*;
use crate::re_export::std_anyhow::*;

use json_compact_data::JsonCompactData;

pub trait JsonSerializable: Serialize + for<'de> Deserialize<'de> {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap().compact_data_field()
    }

    fn from_json(json_str: &str) -> anyhow::Result<Self> where Self: Sized {
        Ok(serde_json::from_str(json_str)?)
    }
}
