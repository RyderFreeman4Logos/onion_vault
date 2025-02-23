pub mod rage_trezor;



use crate::{
    common::traits::JsonSerializable,
    re_export::std_anyhow::*,
    // re_export::rage::*,
    // encryptor::{
        // HasUserIdentifier,
        // Encryptor,
        // SecretType,
    // }
};
use chrono::{
    Local,
    format::SecondsFormat,
};



#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct PublicKey {
    /// from Identity
    #[builder(setter(into))]
    #[builder(default = "None")]
    pub recipient_str: Option<String>,

    /// Set of recipients for encryption. Each string represents a recipient's identity.
    #[builder(setter(into))]
    #[builder(default = "Local::now().to_rfc3339_opts(SecondsFormat::Secs, true)")]
    pub public_key_created_at: String,
}


impl JsonSerializable for PublicKey {}
