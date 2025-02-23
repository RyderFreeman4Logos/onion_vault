use crate::identity_group::{
    IdentityGroup,
    IdentityGroupBuilder,
//     VaultMapIdentifier,
//     SecretType,
};

use crate::common::traits::*;

// use uni_vault::UniVault;

use crate::re_export::uni_vault::*;
use crate::re_export::std_anyhow::*;
// use serde::{Deserialize, Serialize};

// use anyhow::anyhow::Result;

// use std::collections::HashMap;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VaultMap<E>
where E: Encryptor + Serialize
{
    pub identity_groups: Vec<IdentityGroup<E>>,
    pub version: String,
}



impl<E> VaultMap<E>
where E: Clone + Encryptor + Serialize
{
    pub fn new() -> Self {
        VaultMap {
            identity_groups: Vec::new(),
            version: String::from("0.0.1"),
        }
    }


    pub fn add_identity_group(&mut self, new_identity_group: IdentityGroup<E>) {
        self.identity_groups.push(new_identity_group);
    }



    pub fn add_identity_group_simplely(&mut self, group_name: &str, should_be_encrypt: bool) {
        // self.identity_groups.insert(group_name.to_string(), DataState::Plain(IdentityGroup::new(group_name)));
        let new_identity_group = IdentityGroupBuilder::default()
            .group_name(group_name)
            .should_be_encrypt(should_be_encrypt)
            .build()
            .unwrap();
        self.identity_groups.push(new_identity_group);
    }


    pub fn get_identity_group_mut(&mut self, group_name: &str) -> anyhow::Result<&mut IdentityGroup<E>> {
        self.identity_groups.iter_mut().find(|identity_group| {
            identity_group.group_name == group_name
        }).ok_or_else(|| Error::msg("Identity Group not found"))
    }
}


impl<E> JsonSerializable for VaultMap<E>
where E: Encryptor + Serialize + for<'de> Deserialize<'de>
{}
