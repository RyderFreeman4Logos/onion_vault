pub mod credential;
// pub use credential;



use crate::{
    app::AppState,
    common::constants::*,
    operations::{
        self,
        warn_color,
        onion_vault::{
            identity_group::{
                get_identity_group,
                get_identity_group_mut,
                get_accounts_vec_mut,
            },
            ObjectState,
        },
        EnterString,
        choose_what_to_do,
    },
    re_export::std_anyhow::*,
    re_export::uni_vault::*,
    re_export::onion_vault_core::*,
    // re_export::std_anyhow::*,
};




pub fn get_account<'a>(
    identity_index: usize,
    account_index: usize,
    current_onion_vault: &'a OnionVault<RageTrezorEncryptor>,
) -> ObjectState<&'a Account<RageTrezorEncryptor>> {
    let binding = get_identity_group(identity_index, current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => ObjectState::NeedsDecryption(app_state),
        ObjectState::Unencrypted(selected_identity_group) => {
            match selected_identity_group.accounts {
                DataState::Plain(ref accounts) => {
                    let account = accounts.get(account_index)
                        .unwrap();
                    ObjectState::Unencrypted(account)
                },
                DataState::Encrypted(_) => {
                    let app_state = AppState::IdentityGroupToggleEncryption(identity_index);
                    ObjectState::NeedsDecryption(app_state)
                },
            }
        },
    }
}



pub fn get_account_mut<'a>(
    identity_index: usize,
    account_index: usize,
    current_onion_vault: &'a mut OnionVault<RageTrezorEncryptor>,
) -> ObjectState<&'a mut Account<RageTrezorEncryptor>> {
    let binding = get_identity_group_mut(identity_index, current_onion_vault);
    match binding {
        ObjectState::NeedsDecryption(app_state) => ObjectState::NeedsDecryption(app_state),
        ObjectState::Unencrypted(selected_identity_group) => {
            match selected_identity_group.accounts {
                DataState::Plain(ref mut accounts) => {
                    let account = accounts.get_mut(account_index)
                        .unwrap();
                    ObjectState::Unencrypted(account)
                },
                DataState::Encrypted(_) => {
                    let app_state = AppState::IdentityGroupToggleEncryption(identity_index);
                    ObjectState::NeedsDecryption(app_state)
                },
            }
        },
    }
}





pub fn match_app_state(
    current_state: &AppState,
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>
) -> Option<AppState> {
    match current_state {
        AppState::AddAccount(identity_index, account_index) => {
            Some(operations::onion_vault::identity_group::account::create(
                    *identity_index, *account_index, AppState::Menu,
                    current_onion_vault
            ))
        },
        AppState::ModifyAccount(identity_index, account_index) => {
            Some(operations::onion_vault::identity_group::account::modify(
                    *identity_index, *account_index,
                    previous_state, current_onion_vault,
            ))
        },
        AppState::DeleteAccount(identity_index, account_index) => {
            Some(operations::onion_vault::identity_group::account::delete(
                    *identity_index, *account_index,
                    AppState::Menu, current_onion_vault,
            ))
        },
        _ => {
            None
        }
    }
}



pub fn create(
    identity_index: usize,
    account_index: usize,
    state_if_exit: AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let (binding, _group_name) = get_accounts_vec_mut(identity_index, current_onion_vault);
    let accounts_vec = match binding {
        ObjectState::Unencrypted(res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };
    print!("Now start creating Account:\n\n");


    let new_account_username = EnterString::default()
        .prompt("Enter the username")
        .enter_with_confirm();

    if new_account_username.is_none() {
        return state_if_exit
    }
    let new_account_username = new_account_username.unwrap();

    print!("\n\n\n");

    let new_account_platform_id = EnterString::default()
        .prompt("Enter the platform_id (like app.element.io)")
        .enter_with_confirm();

    if new_account_platform_id.is_none() {
        return state_if_exit
    }
    let new_account_platform_id = new_account_platform_id.unwrap();


    let new_account = AccountBuilder::default()
        .username(new_account_username)
        .platform_id(new_account_platform_id)
        .build().unwrap();

    accounts_vec.insert(account_index, new_account);


    print_json_with_line!(&current_onion_vault);


    println!("{}", warn_color(DATA_NOT_PERSISTED_WARNING));

    AppState::ModifyAccount(identity_index, account_index)
}



pub fn modify(
    identity_index: usize,
    account_index: usize,
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let binding = get_account(identity_index, account_index, current_onion_vault);
    let selected_account = match binding {
        ObjectState::Unencrypted(res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };

    println!("Modifying Account {}", style(selected_account.identifier())
        .underlined()
        .bold()
        .white()
        .on_color256(13)
    );
    let (mut option_str, mut options) = get_options_for_modifying(identity_index, account_index, selected_account);

    option_str.push("Back to Previous".into());
    options.push(previous_state.to_owned());
    option_str.push("Menu".into());
    options.push(AppState::Menu);
    option_str.push("Back to Parent".into());
    options.push(AppState::ModifyIdentityGroup(identity_index));

    let selection = choose_what_to_do(&option_str);
    // debug_with_line!(&options[selection]);
    // println!("{:#?}\n{:#?}", options, option_str);
    // debug_with_line!(selection);
    options[selection].clone()
}



pub fn delete(
    identity_index: usize,
    account_index: usize,
    next_state: AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let (binding, _group_name) = get_accounts_vec_mut(identity_index, current_onion_vault);
    let account_vec = match binding {
        ObjectState::Unencrypted(res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };


    account_vec.remove(account_index);
    next_state
}



pub fn get_options_for_modifying(
    // indent_count: usize,
    identity_index: usize,
    account_index: usize,
    selected_account: &Account<RageTrezorEncryptor>
) -> (Vec<String>, Vec<AppState>) {
    // let (identity_index, account_index, credential_index) = idx;
    let mut options: Vec<AppState> = Vec::new();
    let mut option_str: Vec<String> = Vec::new();
    // let indent = "    ".repeat(indent_count);


    let display_name = format!("    ├── Add a Credential at the beginning of Account \"{}\"", &selected_account.username);
    options.push(AppState::AddCredential(identity_index, account_index, 0));
    option_str.push(display_name);

    let last_index = selected_account.credentials.len().checked_sub(1).unwrap_or(0);
    for (credential_index, credential) in selected_account.credentials.iter().enumerate() {
        let credential_display_name = format!("[{}]{}",
            credential_index,
            &credential.secret_type,
        );
        let tree_flag = if credential_index == last_index {
            "├"
        } else {
            "└"
        };
        let idx = (identity_index, account_index, credential_index);

        let display_name = format!("    ├── Select Credential \"{}\"", &credential_display_name);
        option_str.push(display_name);
        options.push(AppState::ModifyCredential(idx.0, idx.1, idx.2));

        let display_name = format!("    {}── Insert a new Credential after \"{}\"", tree_flag, &credential_display_name);
        option_str.push(display_name);
        options.push(AppState::AddCredential(idx.0, idx.1, idx.2 + 1));
    }

    option_str.push("Delete the Whole Account".into());
    options.push(AppState::DeleteAccount(identity_index, account_index));

    (option_str, options)
}
