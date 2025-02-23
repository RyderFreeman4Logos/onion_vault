use crate::{
    app::AppState,
    common::constants::*,
    operations::{
        self,
        print_error,
        warn_color,
        onion_vault::{
            identity_group::{
                get_accounts_vec,
                account::{
                    get_account,
                    get_account_mut,
                },
            },
            ObjectState
        },
        // ask_bool,
        choose_what_to_do,
    },
    re_export::std_anyhow::*,
    re_export::uni_vault::SecretType,
    re_export::onion_vault_core::*,
    // re_export::std_anyhow::*,
};


use uni_vault::HasUserIdentifier;




pub fn get_credential_mut<'a>(
    idx: (usize, usize, usize),
    current_onion_vault: &'a mut OnionVault<RageTrezorEncryptor>,
) -> ObjectState<&'a mut AccountCredential<RageTrezorEncryptor>> {
    let (_identity_index, _account_index, credential_index) = idx;
    let binding = get_account_mut(idx.0, idx.1, current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => ObjectState::NeedsDecryption(app_state),
        ObjectState::Unencrypted(selected_account) => {
            let selected_credential = selected_account.credentials
                .get_mut(credential_index)
                .unwrap();

            ObjectState::Unencrypted(selected_credential)
            // match selected_credential.secret {
            //     DataState::Plain(_) => {
            //         ObjectState::Unencrypted(selected_credential)
            //     },
            //     DataState::Encrypted(_) => {
            //         let app_state = AppState::CredentialToggleEncryption(idx.0, idx.1, idx.2);
            //         ObjectState::NeedsDecryption(app_state)
            //     },
            // }
        },
    }
}



#[allow(dead_code)]
pub fn get_credential<'a>(
    idx: (usize, usize, usize),
    current_onion_vault: &'a OnionVault<RageTrezorEncryptor>,
) -> ObjectState<&'a AccountCredential<RageTrezorEncryptor>> {
    let (_identity_index, _account_index, credential_index) = idx;
    let binding = get_account(idx.0, idx.1,current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => ObjectState::NeedsDecryption(app_state),
        ObjectState::Unencrypted(selected_account) => {
            let selected_credential = selected_account.credentials
                .get(credential_index)
                .unwrap();
            match selected_credential.secret {
                DataState::Plain(_) => {
                    ObjectState::Unencrypted(selected_credential)
                },
                DataState::Encrypted(_) => {
                    let app_state = AppState::CredentialToggleEncryption(idx.0, idx.1, idx.2);
                    ObjectState::NeedsDecryption(app_state)
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
        AppState::ListAllCredentials(identity_index, account_index) => {
            Some(operations::onion_vault::identity_group::account::credential::fuzzy_select_credentials(
                    *identity_index,
                    *account_index,
                    AppState::Menu,
                    current_onion_vault,
            ))
        },
        AppState::AddCredential(identity_index, account_index, credential_index) => {
            Some(operations::onion_vault::identity_group::account::credential::create(
                    (*identity_index, *account_index, *credential_index),
                    AppState::Menu,
                    current_onion_vault,
                    // AppState::ModifyAccount(*identity_index, *account_index),
            ))
        },
        AppState::ModifyCredential(identity_index, account_index, credential_index) => {
            Some(operations::onion_vault::identity_group::account::credential::modify(
                    (*identity_index, *account_index, *credential_index),
                    previous_state,
                    current_onion_vault,
            ))
        },
        AppState::DeleteCredential(identity_index, account_index, credential_index) => {
            Some(operations::onion_vault::identity_group::account::credential::delete(
                    (*identity_index, *account_index, *credential_index),
                    AppState::Menu,
                    current_onion_vault,
            ))
        },
        AppState::CredentialToggleEncryption(identity_index, account_index, credential_index) => {
            Some(operations::onion_vault::identity_group::account::credential::toggle_encryption(
                    (*identity_index, *account_index, *credential_index),
                    current_onion_vault,
            ))
        },
        _ => {
            None
        }
    }
}



pub fn create(
    idx: (usize, usize, usize),
    state_if_exit: AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let (identity_index, account_index, credential_index) = idx;
    let backup_key = current_onion_vault.backup_key.clone();

    let binding = get_account_mut(idx.0, idx.1, current_onion_vault);
    let selected_account = match binding {
        ObjectState::Unencrypted(selected_account) => selected_account,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };
    print!("Now start creating Account:\n\n");



    let password = operations::creat_new_password();
    if password.eq("EXIT") {
        return state_if_exit
    }


    let new_credential = AccountCredentialBuilder::default()
        .should_be_encrypt(true)
        .backup_key(backup_key)
        .username(&selected_account.username)
        .platform_id(&selected_account.platform_id)
        .secret(DataState::Plain(password))
        .secret_type(SecretType::Password)
        .build().unwrap();

    selected_account.credentials.insert(credential_index, new_credential);

    print_json_with_line!(&current_onion_vault);

    println!("{}", warn_color(DATA_NOT_PERSISTED_WARNING));

    AppState::ModifyAccount(identity_index, account_index)
}




pub fn modify(
    idx: (usize, usize, usize),
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let binding = get_account(idx.0, idx.1, current_onion_vault);
    let selected_account = match binding {
        ObjectState::Unencrypted(selected_account) => selected_account,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };
    let (mut option_str, mut options) = get_options_for_modifying(idx, selected_account);

    option_str.push("Back to Previous".into());
    options.push(previous_state.to_owned());
    option_str.push("Back to Parent".into());
    options.push(AppState::ModifyAccount(idx.0, idx.1));

    let selection = choose_what_to_do(&option_str);
    options[selection].clone()
}




pub fn fuzzy_select_credentials(
    identity_index: usize,
    account_index: usize,
    previous_state: AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let (binding, _group_name) = get_accounts_vec(identity_index, current_onion_vault);
    let accounts_vec = match binding {
        ObjectState::Unencrypted(ref accounts_vec) => accounts_vec,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };


    let account_modifying = &accounts_vec.get(account_index).unwrap();

    let mut options: Vec<AppState> = Vec::new();
    let mut option_str: Vec<String> = Vec::new();

    if accounts_vec.len() > 0 {
        println!("List Credentials for {}", style(&account_modifying.identifier())
            .underlined()
            .bold()
            .white()
            .on_color256(13)
        );
    }
    let display_str = format!("Add an Credential for \"{}\"", &account_modifying.identifier());
    options.push(AppState::AddCredential(identity_index, account_index, 0));
    option_str.push(display_str);


    let max_index = account_modifying.credentials.len().checked_sub(1).unwrap_or(0);
    // List all Credentials of this Account
    for (credential_index, credential) in account_modifying.credentials
        .iter().enumerate()
    {
        let (action, action_str, lock_flag) = match credential.secret {
            DataState::Encrypted(_) => {
                let action = AppState::CredentialToggleEncryption(identity_index, account_index, credential_index);
                let action_str = "Decrypt";
                let lock_flag = "🔒";
                (action, action_str, lock_flag)
            },
            DataState::Plain(_) => {
                let action = AppState::ModifyCredential(identity_index, account_index, credential_index);
                let action_str = "Modify";
                let lock_flag = "✅";
                (action, action_str, lock_flag)
            },
        };
        let display_str = format!(
            "├── {} {} For \"{}\" {}",
            action_str,
            &credential.secret_type,
            &account_modifying.identifier(),
            lock_flag
        );
        options.push(action);
        option_str.push(display_str);


        let tree_flag = if credential_index == max_index {
            "├"
        } else {
            "└"
        };


        let display_str = format!("{}── Insert an Credential after {}", tree_flag, &credential.secret_type.to_string_name());
        options.push(AppState::AddCredential(identity_index, account_index, credential_index + 1));
        option_str.push(display_str);
    }
    option_str.push("Back to Previous".into());
    options.push(previous_state);
    option_str.push("Menu".into());
    options.push(AppState::Menu);
    option_str.push("Back to Parent".into());
    options.push(AppState::ModifyAccount(identity_index, account_index));



    let selection = choose_what_to_do(&option_str);
    options[selection].clone()
}




pub fn toggle_encryption(
    (identity_index, account_index, credential_index): (usize, usize, usize),

    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let binding = get_credential_mut((identity_index, account_index, credential_index), current_onion_vault);
    let selected_identity_group = match binding {
        ObjectState::Unencrypted(res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };
    let prompt = {
        if let DataState::Plain(_) = selected_identity_group.secret {
            "Are you sure you want to encrypt this Credential?"
        } else {
            "Do you want to decrypt this Credential?"
        }
    };


    println!("{}\n{}",
        prompt,
        "\nClick the '❌' on your Trezor Safe 5 screen to cancel"
    );
    // if ask_bool(){
        let _ = selected_identity_group.toggle_encryption_by_trezor()
            .map_err(print_error);
    // }

    let idx = (identity_index, account_index, credential_index);
    AppState::ModifyCredential(idx.0, idx.1, idx.2)
}



pub fn delete(
    (identity_index, account_index, credential_index): (usize, usize, usize),

    next_state: AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let binding = get_account_mut(identity_index, account_index, current_onion_vault);
    let selected_account = match binding {
        ObjectState::Unencrypted(res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };


    selected_account.credentials.remove(credential_index);
    next_state
}



pub fn get_options_for_modifying(
    // indent_count: usize,
    idx: (usize, usize, usize),
    selected_account: &Account<RageTrezorEncryptor>
) -> (Vec<String>, Vec<AppState>) {
    let (_identity_index, _account_index, credential_index) = idx;
    let mut options: Vec<AppState> = Vec::new();
    let mut option_str: Vec<String> = Vec::new();
    // let indent = "    ".repeat(indent_count);

    let selected_credential = &selected_account.credentials
        .get(credential_index)
        .unwrap();
    let display_str = format!("{} for {}",
        selected_credential.secret_type,
        selected_account.identifier(),
    );
    println!("Modifying Credential {}", style(display_str)
        .underlined()
        .bold()
        .white()
        .on_color256(13)
    );

    let selected_credential_toggle_encryption = AppState::CredentialToggleEncryption(idx.0, idx.1, idx.2);
    match &selected_credential.secret {
        DataState::Plain(secret) => {
            option_str.push("Show Credential".into());
            options.push(AppState::Print(Zeroizing::new(secret.clone())));

            option_str.push("Delete Credential".into());
            options.push(AppState::DeleteCredential(idx.0, idx.1, idx.2));

            if let SecretType::Password = &selected_credential.secret_type {
                option_str.push("Auto Fill Password".into());
                options.push(AppState::FillPassword(Zeroizing::new(secret.clone())));
            }
        },
        DataState::Encrypted(_) => {
            option_str.push("Decrypt Credential".into());
            options.push(selected_credential_toggle_encryption);
        }
    };

    (option_str, options)
}

