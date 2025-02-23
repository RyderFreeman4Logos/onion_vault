pub mod identity_group;
pub use identity_group::{
    account,
    credential,
};



use crate::{
    app::AppState,
    common::{
        // test_helpers::debug_with_line,
        constants::*,
    },
    operations::{
        self,
        ask_bool,
        warn_color,
        print_error,
        ask_for_retry,
        EnterString,
        choose_what_to_do,
    },
    re_export::std_anyhow::*,
    re_export::onion_vault_core::*,
};



use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;



#[derive(Debug)]
pub enum ObjectState<T> {
    Unencrypted(T),
    NeedsDecryption(AppState),
}


pub fn match_app_state(
    current_state: &AppState,
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>
) -> Option<AppState> {
    match current_state {
        AppState::OnionVaultToggleEncryption => {
            Some(operations::onion_vault::toggle_encryption(&AppState::Menu, current_onion_vault))
        },
        AppState::ModifyOnionVault => {
            Some(operations::onion_vault::modify(previous_state, current_onion_vault))
        },
        _ => {
            None
        },
    }
}



pub fn get_vault_map<'a>(
    current_onion_vault: &'a OnionVault<RageTrezorEncryptor>
) -> ObjectState<&'a VaultMap<RageTrezorEncryptor>> {
    match current_onion_vault.vault_map{
        DataState::Plain(ref current_vault_map) => ObjectState::Unencrypted(current_vault_map),
        DataState::Encrypted(_) => ObjectState::NeedsDecryption(AppState::OnionVaultToggleEncryption)
    }

}



pub fn get_vault_map_mut<'a>(
    current_onion_vault: &'a mut OnionVault<RageTrezorEncryptor>
) -> ObjectState<&'a mut VaultMap<RageTrezorEncryptor>> {
    match current_onion_vault.vault_map{
        DataState::Plain(ref mut current_vault_map) => ObjectState::Unencrypted(current_vault_map),
        DataState::Encrypted(_) => ObjectState::NeedsDecryption(AppState::OnionVaultToggleEncryption)
    }

}



pub fn create() -> OnionVault<RageTrezorEncryptor> {
    println!("Enter the {}", style("Owner Name for PasswordManager")
        .underlined()
        .bold()
        .white()
        .on_color256(13)
    );
    let ownername = EnterString::default()
        .enter_with_confirm();

    if ownername.is_none() {
        operations::exit_process();
    }
    let ownername = ownername.unwrap();



    // Creat an onion_vault
    let mut onion_vault4owner: OnionVault<RageTrezorEncryptor> = OnionVaultBuilder::default()
        .should_be_encrypt(true)
        .ownername(&ownername)
        .build().unwrap();

    let backup_key = loop {
        print!("{}\n", style("Generating Backup Key, Please Touch Your Trezor")
            .black()
            .on_color256(40)
        );

        if let Ok(backup_key) = onion_vault4owner.generate_backup_key_by_trezor()
            .map_err(print_error) {
                break backup_key
            } else {
                println!("{}", style("Don't worry, Do you want to retry?").white().on_blue());

                if !ask_for_retry() {
                    operations::exit_process();
                }
            }
    };

    onion_vault4owner.backup_key = backup_key;


    println!("### Now Owner Name is set: \"{}\" ###", &ownername);
    println!("{}", warn_color(DATA_NOT_PERSISTED_WARNING));

    onion_vault4owner
}



pub fn toggle_encryption(
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let prompt = {
        if let DataState::Plain(_) = current_onion_vault.vault_map {
            "Are you sure you want to encrypt all objects marked for encryption in the OnionVault?"
        } else {
            "Do you want to decrypt the OnionVault?\nAll Credentials and all usernames within encrypted identity groups will not be decrypted."
        }
    };


    println!("{}{}",
        prompt,
        // "\nPress 'y' to confirm, press 'n' to return to the previous operation."
        "\nClick the '❌' on your Trezor Safe 5 screen to cancel"
    );
    // if ask_bool(){
        let _ = current_onion_vault.toggle_encryption_by_trezor()
            .map_err(print_error);

    // }

    previous_state.to_owned()
}




pub fn modify(
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> AppState {
    let binding = get_vault_map(current_onion_vault);
    let current_vault_map = match binding {
        ObjectState::Unencrypted(res) => res,
        ObjectState::NeedsDecryption(app_state) => return app_state,
    };

    println!("Modifying OnionVault for {}", style(&current_onion_vault.ownername)
        .underlined()
        .bold()
        .white()
        .on_color256(13)
    );
    let (mut option_str, mut options) = get_options_for_modifying(&current_vault_map.identity_groups);
    let display_name = format!("    ├── Add an IdentityGroup at the beginning of OnionVault"); // \"{}\"", &group_name);
    options.insert(0, AppState::CreateIdentityGroup(0));
    option_str.insert(0, display_name);

    option_str.push("Back to Previous".into());
    options.push(previous_state.to_owned());
    option_str.push("Menu".into());
    options.push(AppState::Menu);

    let selection = choose_what_to_do(&option_str);
    options[selection].clone()
}



pub fn get_options_for_modifying(
    // indent_count: usize,
    identity_group_vec: &Vec<IdentityGroup<RageTrezorEncryptor>>
) -> (Vec<String>, Vec<AppState>) {
    let mut options: Vec<AppState> = Vec::new();
    let mut option_str: Vec<String> = Vec::new();
    // let indent = "    ".repeat(indent_count);

    let last_index = identity_group_vec.len().checked_sub(1).unwrap_or(0);
    for (identity_index, identity_group) in identity_group_vec.iter().enumerate() {
        let tree_flag = if identity_index == last_index {
            "├"
        } else {
            "└"
        };

        let display_name = format!("    ├── Select IdentityGroup \"{}\"", &identity_group.group_name);
        option_str.push(display_name);
        options.push(AppState::ModifyIdentityGroup(identity_index));

        let display_name = format!("    {}── Insert a new IdentityGroup after \"{}\"", tree_flag, &identity_group.group_name);
        option_str.push(display_name);
        options.push(AppState::CreateIdentityGroup(identity_index + 1));
    }


    (option_str, options)
}




pub fn save_to_disk(
    path_str: &str,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> anyhow::Result<()> {
    let file_name = EnterString::default()
        .prompt("Enter the PasswordManager file name you want:")
        .default_result(DEFAULT_ONION_VAULT_JSON_FILE_NAME)
        .enter_with_confirm_no_exit();

    let mut file_path = PathBuf::from(path_str.trim());
    std::fs::create_dir_all(&file_path)?;
    file_path = file_path.join(&file_name);

    let res = lock_onion_vault(current_onion_vault);
    if res.is_err() {
        return res;
    }

    let binding = current_onion_vault.to_json_pretty();
    let data=  binding.as_bytes();

    write_json_to_disk(&file_path, data, &file_name)?;

    Ok(())
}


fn lock_onion_vault(
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>,
) -> anyhow::Result<()> {
    loop {
        let res = current_onion_vault.traverse_and_encrypt_if_necessary();

        match res {
            Ok(_) => return Ok(()),
            Err(e) => {
                let error_msg = format!(
                    "Error: {}\nDo you want to retry?",
                    e.to_string()
                );
                println!("{}", error_msg);

                if ask_bool() {
                    continue
                } else {
                    return Err(e.into());
                }
            },
        }
    }
}


fn write_json_to_disk(
    file_path: &PathBuf,
    data: &[u8],
    file_name: &str,
) -> anyhow::Result<()> {
    let result = write_file(file_path, data);

    match result {
        Ok(_) => println!("File written successfully to '{}'", file_path.display()),
        Err(e) => {
            let pwd = env::current_dir()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("")
                .to_string();

            println!("Failed to write to '{}'", file_path.display());
            loop {
                let new_path_str = EnterString::default()
                    .prompt("Please provide an alternative path:")
                    .default_result(&pwd)
                    .enter_with_confirm();

                if new_path_str.is_none() {
                    return Err(e.into());
                }
                let new_path_str = new_path_str.unwrap();

                let new_path = PathBuf::from(new_path_str.trim())
                    .join(&file_name);

                match write_file(&new_path, data) {
                    Ok(_) => {
                        println!("File written successfully to '{}'", new_path.display());
                        break;
                    },
                    Err(e) => println!("Error: {}\n\n", e.to_string()),
                }
            }
        }
    }

    Ok(())
}



fn write_file(file_path: &PathBuf, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
}



#[allow(dead_code)]
fn touch_file(file_path: &str) -> io::Result<()> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)?;
    Ok(())
}

