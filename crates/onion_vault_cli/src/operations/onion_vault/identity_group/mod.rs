pub mod account;
pub use account::credential;


use crate::{
    app::AppState,
    common::constants::*,
    operations::{
        self,
        print_error,
        warn_color,
        onion_vault::{
            get_vault_map,
            get_vault_map_mut,
            ObjectState,
        },
        ask_bool,
        EnterString,
        choose_what_to_do,
    },
    re_export::std_anyhow::*,
    re_export::uni_vault::*,
    re_export::onion_vault_core::*,
};




pub fn get_identity_group<'a>(
    identity_index: usize,
    current_onion_vault: &'a OnionVault<RageTrezorEncryptor>,
) -> ObjectState<&'a IdentityGroup<RageTrezorEncryptor>> {
    let binding = get_vault_map(current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => ObjectState::NeedsDecryption(app_state),
        ObjectState::Unencrypted(current_vault_map) => {
            let res = current_vault_map.identity_groups
                .get(identity_index)
                .unwrap();
            ObjectState::Unencrypted(res)
        },
    }
}



pub fn get_identity_group_mut<'a>(
    identity_index: usize,
    current_onion_vault: &'a mut OnionVault<RageTrezorEncryptor>,
) -> ObjectState<&'a mut IdentityGroup<RageTrezorEncryptor>> {
    let binding = get_vault_map_mut(current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => ObjectState::NeedsDecryption(app_state),
        ObjectState::Unencrypted(current_vault_map) => {
            let res = current_vault_map.identity_groups
                .get_mut(identity_index)
                .unwrap();
            ObjectState::Unencrypted(res)
        },
    }
}



pub fn get_accounts_vec_mut<'a>(
    identity_index: usize,
    current_onion_vault: &'a mut OnionVault<RageTrezorEncryptor>,
) -> (ObjectState<&'a mut Vec<Account<RageTrezorEncryptor>>>, String) {
    let binding = get_identity_group_mut(identity_index, current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => (ObjectState::NeedsDecryption(app_state), EMPTY_STR.into()),
        ObjectState::Unencrypted(selected_identity_group) => {
            let group_name = selected_identity_group.group_name.clone();
            match selected_identity_group.accounts {
                DataState::Plain(ref mut account_vec) => {
                    (ObjectState::Unencrypted(account_vec), group_name)
                },
                DataState::Encrypted(_) => {
                    let app_state = AppState::IdentityGroupToggleEncryption(identity_index);
                    (ObjectState::NeedsDecryption(app_state), group_name)
                },
            }
        },
    }
}




pub fn get_accounts_vec<'a>(
    identity_index: usize,
    current_onion_vault: &'a OnionVault<RageTrezorEncryptor>,
) -> (ObjectState<&'a Vec<Account<RageTrezorEncryptor>>>, String) {
    let binding = get_identity_group(identity_index, current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => (ObjectState::NeedsDecryption(app_state), EMPTY_STR.into()),
        ObjectState::Unencrypted(selected_identity_group) => {
            let group_name = selected_identity_group.group_name.clone();
            match selected_identity_group.accounts {
                DataState::Plain(ref account_vec) => {
                    (ObjectState::Unencrypted(account_vec), group_name)
                },
                DataState::Encrypted(_) => {
                    let app_state = AppState::IdentityGroupToggleEncryption(identity_index);
                    (ObjectState::NeedsDecryption(app_state), group_name)
                },
            }
        },
    }
}



pub fn match_app_state(
    current_state: &AppState,
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> Option<AppState> {
    match current_state {
        AppState::DeleteIdentityGroup(index) => {
            Some(operations::onion_vault::identity_group::delete(
                    *index,
                    AppState::Menu,
                    current_onion_vault,
            )
            )
        }
        AppState::CreateIdentityGroup(index) => {
            let res = operations::onion_vault::identity_group::create(
                *index,
                AppState::Menu,
                current_onion_vault,
            );
            Some(res)
        }
        AppState::IdentityGroupToggleEncryption(index) => {
            Some(operations::onion_vault::identity_group::toggle_encryption(
                    *index, current_onion_vault
            ))
        }
        AppState::ModifyIdentityGroup(index) => {
            Some(operations::onion_vault::identity_group::modify(*index, previous_state, current_onion_vault))
        }
        _ => {
            None
        }
    }
}



pub fn create(
    identity_index: usize,
    state_if_exit: AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let backup_key = current_onion_vault.backup_key.clone();
    let mut binding = get_vault_map_mut(current_onion_vault);
    let current_vault_map = match binding {
        ObjectState::Unencrypted(ref mut current_vault_map) => current_vault_map,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };


    print!("Now start creating Identity Group:\n\n");

    println!("{}", style("Are usernames in this group should be encrypted?")
        .white()
        .on_color256(57)
    );
    let is_secret_identity_group = ask_bool();


    if is_secret_identity_group {
        println!("{}", style(r#"
We recommend giving the Identity Group, in which all linked account usernames
are encrypted to protect privacy, an unrelated name to prevent the Identity Group
from revealing the purpose of its internal accounts. Consider a group name like
"My Porn Accounts" instead of "Accounts for Fighting Against Human Trafficking.""#
        )
            .white()
            .on_color256(57)
        );
    }

    let group_name = EnterString::default()
        .prompt("Enter the identity group name")
        .enter_with_confirm();

    if group_name.is_none() {
        return state_if_exit
    }
    let group_name = group_name.unwrap();


    let new_identity_group = IdentityGroupBuilder::default()
        .should_be_encrypt(is_secret_identity_group)
        .backup_key(backup_key)
        .group_name(group_name)
        .build().unwrap();

    // vault_map.add_identity_group(new_identity_group);
    current_vault_map.identity_groups.insert(identity_index, new_identity_group);

    print_json_with_line!(&current_onion_vault);

    println!("{}", warn_color(DATA_NOT_PERSISTED_WARNING));

    AppState::ModifyIdentityGroup(identity_index)
}


pub fn toggle_encryption(
    identity_index: usize,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let mut binding = get_identity_group_mut(identity_index, current_onion_vault);
    let selected_identity_group = match binding {
        ObjectState::Unencrypted(ref mut res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };

    let prompt = {
        if let DataState::Plain(_) = selected_identity_group.accounts{
            "Are you sure you want to encrypt all objects marked for encryption in this IdentityGroup?"
        } else {
            "Do you want to decrypt this IdentityGroup?\n
                All usernames will be decrypted.\n
                But all Credentials will NOT be decrypted."
        }
    };


    println!("{}{}",
        prompt,
        "\nClick the '❌' on your Trezor Safe 5 screen to cancel"
    );
    // if ask_bool(){
        let _ = selected_identity_group.toggle_encryption_by_trezor()
            .map_err(print_error);
    // }

    AppState::ModifyIdentityGroup(identity_index)
}



pub fn delete(
    identity_index: usize,
    next_state: AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let binding = get_vault_map_mut(current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => app_state,
        ObjectState::Unencrypted(current_vault_map) => {
            let _res = current_vault_map.identity_groups
                .remove(identity_index);

            next_state
        },
    }
}





pub fn modify(
    identity_index: usize,
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let (binding ,group_name) = get_accounts_vec(identity_index, current_onion_vault);
    let accounts_vec = match binding {
        ObjectState::Unencrypted(res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };

    println!("Modifying Identity Group {}", style(&group_name)
        .underlined()
        .bold()
        .white()
        .on_color256(13)
    );
    let display_name = format!("    ├── Add an Account at the beginning of IdentityGroup \"{}\"", &group_name);
    let (mut option_str, mut options) = get_options_for_modifying(identity_index, accounts_vec);
    options.insert(0, AppState::AddAccount(identity_index, 0));
    option_str.insert(0, display_name);

    option_str.push("Back to Previous".into());
    options.push(previous_state.to_owned());
    option_str.push("Menu".into());
    options.push(AppState::Menu);
    option_str.push("Back to Parent".into());
    options.push(AppState::ModifyOnionVault);

    let selection = choose_what_to_do(&option_str);
    options[selection].clone()
}




pub fn get_options_for_modifying(
    // indent_count: usize,
    identity_index: usize,
    accounts_vec: &Vec<Account<RageTrezorEncryptor>>
) -> (Vec<String>, Vec<AppState>) {
    let mut options: Vec<AppState> = Vec::new();
    let mut option_str: Vec<String> = Vec::new();
    // let indent = "    ".repeat(indent_count);




    let last_index = accounts_vec.len().checked_sub(1).unwrap_or(0);
    for (account_index, account) in accounts_vec.iter().enumerate() {
        let tree_flag = if account_index == last_index {
            "├"
        } else {
            "└"
        };
        let identifier = &account.identifier();

        let display_name = format!("    ├── Select Account \"{}\"", &identifier);
        option_str.push(display_name);
        options.push(AppState::ModifyAccount(identity_index, account_index));

        let display_name = format!("    {}── Insert a new Account after \"{}\"", tree_flag, &identifier);
        option_str.push(display_name);
        options.push(AppState::AddAccount(identity_index, account_index + 1));
    }

    option_str.push("Delete the Whole IdentityGroup".into());
    options.push(AppState::DeleteIdentityGroup(identity_index));


    (option_str, options)
}

