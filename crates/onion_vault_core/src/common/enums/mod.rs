use crate::re_export::std_anyhow::*;
use crate::re_export::uni_vault::*;


// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DataState<T, E>
where E: Encryptor + Serialize
// where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{
    Plain(T),
    Encrypted(UniVault<E>),
}

