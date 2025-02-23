use crate::{
    // common::macros::impl_display_for_enum,
    operations::{
        self,
        ask_bool,
        onion_vault,
        identity_group,
        account,
        credential,
        check_environment_safety,
        print_warn,
        warn_color,
        fuzzy_select_menu,
    },
    common::constants::*,

    re_export::{
        std_anyhow::*,
        onion_vault_core::*,
    },
};



use lazy_static::lazy_static;
// use enum_map::{
//     enum_map,
//     EnumMap,
//     EnumArray,
// };

// use directories::ProjectDirs;
use std::path::Path;
use std::fmt;
use std::env;
use std::fmt::Debug;

// use std::path::Path;
// use std::path::PathBuf;
// use std::env;



lazy_static! {
    pub static ref TERM: Term = Term::stdout();
}



#[derive(Debug, Builder)]
pub struct App {
    // #[builder(default = "ProjectDirs::from(\"onion\", ORGANIZATION, APPLICATION_NAME)")]
    #[builder(setter(into))]
    pub bip32_path: String,

    #[builder(setter(into))]
    pub data_dir: String,

    /// If the value is true, you need to use the mouse wheel
    /// after each Enter key press
    /// for the history of interactions.
    #[builder(setter(into))]
    pub auto_clear: bool,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum AppState {
    Exit,
    Print(Zeroizing<String>),
    FillPassword(Zeroizing<String>),
    Menu,
    CheckEnvironmentSafety,
    CreateOnionVault,
    OnionVaultToggleEncryption,
    IdentityGroupToggleEncryption(usize),
    CreateIdentityGroup(usize),
    DeleteIdentityGroup(usize),
    ModifyIdentityGroup(usize),
    AddAccount(usize, usize),
    ModifyAccount(usize, usize),
    DeleteAccount(usize, usize),
    AddCredential(usize, usize, usize),
    ModifyCredential(usize, usize, usize),
    DeleteCredential(usize, usize, usize),
    CredentialToggleEncryption(usize, usize, usize),
    ListAllCredentials(usize, usize),
    ModifyOnionVault,
    SaveOnionVault,
    // ListAllIdentityGroups,
    // ListAllItems,
    // CreateLoginInfo,
    // AskSaveToDisk,
    // InitOnionVault,
}

impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppState::Exit => write!(f, "Exit"),
            AppState::Menu => write!(f, "Menu"),
            AppState::Print(_) => write!(f, ""),
            AppState::FillPassword(_) => write!(f, "Fill Password"),
            AppState::CheckEnvironmentSafety => write!(f, "Check Environment Safety"),
            AppState::CreateOnionVault => write!(f, "Create Onion Vault"),
            AppState::OnionVaultToggleEncryption => write!(f, "_"),
            AppState::IdentityGroupToggleEncryption(_) => write!(f, "_"),
            AppState::CredentialToggleEncryption(_, _, _) => write!(f, "_"),
            AppState::CreateIdentityGroup(_) => write!(f, "Create Identity Group"),
            AppState::DeleteIdentityGroup(_) => write!(f, "Delete Identity Group"),
            AppState::ModifyIdentityGroup(_) => write!(f, "Modify Identity Group"),
            // AppState::ListAllIdentityGroups => write!(f, "List All Identity Groups"),
            AppState::ModifyOnionVault => write!(f, "Modify Onion Vault"),
            AppState::SaveOnionVault => write!(f, "Save Onion Vault"),
            AppState::AddAccount(_, _) => write!(f, "Add an Account to an IdentityGroup"),
            AppState::ModifyAccount(_, _) => write!(f, "Modify Account"),
            AppState::DeleteAccount(_, _) => write!(f, "Delete Account"),
            // AppState::ListAllItems => write!(f, "List All Identity Group, Accounts and Credentials"),
            AppState::ListAllCredentials(_, _) => write!(f, "List All Credentials"),
            AppState::AddCredential(_, _, _) => write!(f, "Add a Credential to an Account"),
            AppState::DeleteCredential(_, _, _) => write!(f, "Delete Credential"),
            AppState::ModifyCredential(_, _, _) => write!(f, "Modify Credential"),
        }
    }
}

const MENU_OPTIONS: usize = 2;
pub const APP_STATES_FOR_MENU: [AppState; MENU_OPTIONS] = [
    AppState::ModifyOnionVault,
    // AppState::ListAllItems,
    AppState::Exit,
    // AppState::CreateIdentityGroup,
    // AppState::Menu,
    // AppState::CheckEnvironmentSafety,
    // AppState::CreateOnionVault,
];

fn read_onion_vault_json_from_data_dir(data_dir: &str) -> Option<String> {
    let mut file_name_vec: Vec<String> = Vec::new();
    let entrys = std::fs::read_dir(data_dir);
    let entrys = entrys.ok()?;
    for entry in entrys {
        let entry = entry.ok();
        let path = entry?.path();

        // Check if the entry is a file and ends with .json
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            file_name_vec.push(path.display().to_string())
        }
    }
    file_name_vec.push("Create a new file".into());

    // debug_with_line!(&file_name_vec);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which file do you want to open?")
        .items(&file_name_vec)
        .interact()
        .unwrap_or(0);

    let file_to_be_open = &file_name_vec[selection];

    let file_path = Path::new(file_to_be_open);
    std::fs::read_to_string(file_path).ok()
}


impl App {
    pub fn main_loop(&mut self){
        env::set_var("BIP32_PATH", &self.bip32_path);


        println!("\n{}", warn_color(ONION_VAULT_CREATION_WARNING));
        if check_environment_safety() {
            let mut current_onion_vault = match read_onion_vault_json_from_data_dir(&self.data_dir) {
                Some(json_str) => OnionVault::<RageTrezorEncryptor>::from_json(&json_str).unwrap(),
                _ => operations::onion_vault::create()
            };



            let mut previous_state = AppState::Exit;
            let mut current_state = AppState::Menu;
            // let mut current_state = AppState::FillPassword(Zeroizing::new("my password".into()));

            loop {
                if let AppState::SaveOnionVault = current_state {
                    let _ = onion_vault::save_to_disk(
                        &self.data_dir,
                        &mut current_onion_vault,
                    );
                    current_state = AppState::Menu;
                    continue
                }
                if let AppState::Print(ref mut secret_str) = current_state {
                    let str_ref: &str = &secret_str;
                    println!("{}", str_ref);
                    secret_str.zeroize();
                    current_state = previous_state.clone();
                    continue
                }
                if let AppState::FillPassword(ref mut secret_str) = current_state {
                    print_warn("Are you sure you've made the mouse cursor blink in the password input field?");
                    loop {
                        if ask_bool() { break }
                    }
                    operations::enigo_fill_password(secret_str);
                    secret_str.zeroize();
                    current_state = previous_state.clone();
                    continue
                }
                let temporary_state = current_state.clone();

                current_state = match_app_state(&current_state, &previous_state, &mut current_onion_vault);

                if let AppState::Exit = current_state {
                    println!("TO BE EXIT :)");
                    let res = onion_vault::save_to_disk(
                        &self.data_dir,
                        &mut current_onion_vault,
                    );

                    if res.is_err() {
                        current_state = AppState::Menu;
                        continue
                    } else {
                        break
                    }
                }

                println!("\n\n");

                if self.auto_clear {
                    let _ = TERM.clear_screen();
                }

                previous_state = temporary_state;
            }
        }

        println!("\n\nAleardy EXIT :)");
    }
}



fn match_app_state(
    current_state: &AppState,
    previous_state: &AppState,
    current_onion_vault: &mut OnionVault<RageTrezorEncryptor>
) -> AppState {
    match current_state {
        AppState::Menu => {
            fuzzy_select_menu()
        },

        _ => {
            [
                onion_vault::match_app_state,
                identity_group::match_app_state,
                account::match_app_state,
                credential::match_app_state,
            ].into_iter()
                .find_map(|f| f(current_state, previous_state, current_onion_vault))
                .unwrap_or(AppState::Exit)
                // AppState::Exit
        }
    }
}


