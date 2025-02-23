use crate::{
    common::constants::{
    APPLICATION_NAME,
    ORGANIZATION,
    },
    re_export::std_anyhow::*,
};


use lazy_static::lazy_static;
use directories::ProjectDirs;


use std::path::PathBuf;
use std::env;



lazy_static! {
    // pub static ref CLI_ARGS: RwLock<Cli> = RwLock::new(Cli::parse());
    // pub static ref ProjectDir: Option<ProjectDirs> = ProjectDirs::from("onion", ORGANIZATION, APPLICATION_NAME);
    pub static ref PROJECTDIR: ProjectDir = ProjectDirBuilder::default()
        .build()
        .unwrap();
}



// #[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[derive(Debug, Builder)]
pub struct ProjectDir {
    #[builder(default = "ProjectDirs::from(\"onion\", ORGANIZATION, APPLICATION_NAME)")]
    project_dirs: Option<ProjectDirs>
}

impl ProjectDir {
    pub fn pwd(&self) -> PathBuf {
        env::current_dir().unwrap()
    }

    pub fn data_dir(&self) -> String {
        let pwd = self.pwd();
        let data_dir = match &self.project_dirs {
            Some(project_dirs) => {
                project_dirs.data_dir()
            },
            None => &pwd.as_path(),
        };

        data_dir.to_owned().to_string_lossy().into_owned()
    }

    pub fn config_file(&self) -> String {
        let pwd = self.pwd();
        let config_dir = match &self.project_dirs {
            Some(project_dirs) => {
                project_dirs.config_dir()
            },
            None => &pwd.as_path(),
        };

        config_dir.join("config.json").to_string_lossy().into_owned()
    }
}
