pub mod onion_vault;
pub use onion_vault::{
    identity_group,
    account,
    credential,
};



use crate::app::{
    APP_STATES_FOR_MENU,
    AppState,
};
use crate::common::constants::*;
use crate::re_export::std_anyhow::*;




use enigo::{
    Direction::{Click, Press, Release},
    Enigo,
    Key,
    Keyboard,
    Settings,
};




use std::default::Default;
use std::process::{Command, Stdio};
use std::io::Write;
use std::time::Duration;
use std::thread;






pub struct EnterString {
    prompt: String,
    default_result: String,
}

impl Default for EnterString {
    fn default() -> Self {
        EnterString {
            prompt: String::new(),
            default_result: String::from("EXIT"),
        }
    }
}

impl EnterString {
    pub fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_string();

        self
    }


    pub fn default_result(mut self, default_result: &str) -> Self {
        self.default_result = default_result.to_string();

        self
    }

    pub fn enter_with_confirm_no_exit(&self) -> String {
        let res = loop {
            let res = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                        "{}\nDefault:",
                        self.prompt.clone())
                )
                .default(self.default_result.clone())
                .interact_text()
                .unwrap_or_default();

            println!("\nYou Entered \"{}\", sure?", style(&res)
                .underlined()
                .bold()
                .white()
                .on_color256(13)
            );

            let ok = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("type 'y' or 'n'")
                .interact()
                .unwrap_or(false);

            if ok { break res }
            print!("\n\n\n\n");
        };

        res
    }

    pub fn enter_with_confirm(&self) -> Option<String> {
        let res = loop {
            let res = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                        "{}\n(Or Enter \"EXIT\" to go back to the previous step).\nDefault:",
                        self.prompt.clone())
                )
                .default(self.default_result.clone())
                .interact_text()
                .unwrap_or_default();

            println!("\nYou Entered \"{}\", sure?", style(&res)
                .underlined()
                .bold()
                .white()
                .on_color256(13)
            );

            let ok = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("type 'y' or 'n'")
                .interact()
                .unwrap_or(false);

            if ok { break res }
            print!("\n\n\n\n");
        };

        if res.eq("EXIT") {
            None
        } else {
            Some(res)
        }
    }
}



pub fn ask_bool() -> bool {
    let res = Confirm::with_theme(&ColorfulTheme::default())
        .interact()
        .unwrap_or(true);

    res
}


pub fn ask_for_retry() -> bool {
    let res = ask_bool();

    print!("\n\n\n");

    res
}




pub fn choose_what_to_do<T: ToString>(option_str: &[T]) -> usize {
    FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose what to do")
        .items(&option_str)
        .interact()
        .unwrap_or(0)
}



fn switch_window(enigo: &mut Enigo) {
    if let Err(e) = (|| -> anyhow::Result<()> {
        if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
            let _ = enigo.key(Key::Alt, Press);
            let _ = enigo.key(Key::Tab, Click);
            let _ = enigo.key(Key::Alt, Release);
        } else if cfg!(target_os = "macos") {
            let _ = enigo.key(Key::Meta, Press);
            let _ = enigo.key(Key::Tab, Click);
            let _ = enigo.key(Key::Meta, Release);
        }
        Ok(())
    })() {
        eprintln!("Error executing xdotool: {}", e);
    }
}


pub fn enigo_fill_password(mut password: &mut Zeroizing<String>) {
    remove_trailing_newlines(&mut password);

    let password_ref: &str = password.as_ref();
    if let Err(e) = (|| -> anyhow::Result<()> {
        let mut enigo = Enigo::new(&Settings::default())?;
        switch_window(&mut enigo);
        thread::sleep(Duration::from_millis(150));
        enigo.text(password_ref)?;

        Ok(())
    })() {
        eprintln!("Error executing enigo: {}", e);
    }

    password.zeroize();
}



#[allow(dead_code)]
pub fn xdotool_fill_password(mut password: &mut Zeroizing<String>) {

    remove_trailing_newlines(&mut password);
    if let Err(e) = (|| -> anyhow::Result<()> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("xdotool type --delay 50 --file -")
            .env("XDOTOOL_DEBUG", "0")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut stdin = child.stdin.take()
            .ok_or(Error::msg("stdin.take() == None"))?;

        stdin.write_all(password.as_bytes())?;
        drop(stdin);

        let _status = child.wait()?;

        Ok(())
    })() {
        eprintln!("Error executing xdotool: {}", e);
    }
    password.zeroize();
}

fn remove_trailing_newlines(s: &mut String) {
    s.truncate(s.trim_end_matches(&['\r', '\n'][..]).len());
}


pub fn creat_new_password() -> String {
    println!("{}", style("Do you want Auto-generate Strong Password?")
        .white()
        .on_color256(57)
    );
    let auto_generat_strong_password = ask_bool();

    if auto_generat_strong_password {
        let max_password_length: u8 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Please Enter Max Password Length")
            .interact_text()
            .unwrap_or(37);

        passwords::PasswordGenerator::new()
            .length(max_password_length.into())
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .spaces(false)
            .exclude_similar_characters(true)
            .strict(true)
            .generate_one()
            .unwrap_or("EXIT".to_string())
    } else {
        dialoguer::Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .with_confirmation("Repeat password", "Error: the passwords don't match.")
            .interact()
            .unwrap_or("EXIT".to_string())
    }
}


pub fn exit_process() {
    println!("TO BE EXIT :)");
    std::process::exit(0);

}



pub fn print_error(e: Error) {
    println!("{}", error_color(e));
}

pub fn error_color(e: Error) -> StyledObject<String> {
    style(e.to_string()).white().on_red()
}



pub fn print_warn(prompt: &str) {
    println!("{}", warn_color(prompt));
}

pub fn warn_color(prompt: &str) -> StyledObject<&str> {
    style(prompt).yellow().on_color256(54)
}

pub fn check_environment_safety() -> bool {
    print_warn(SECURITY_CONFIDENCE_PROMPT);
    let res = Confirm::with_theme(&ColorfulTheme::default())
        .interact()
        .unwrap_or(false);

    res
}

pub fn fuzzy_select_menu() -> AppState {
    let options: Vec<AppState> = APP_STATES_FOR_MENU.into_iter().collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose what to do")
        .items(&options)
        .interact()
        .unwrap_or(0);

    options[selection].clone()
}

