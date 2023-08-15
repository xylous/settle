use crate::io::{dir_exists, file_exists, file_to_string, mkdir, write_to_file};
use serde::{Deserialize, Serialize};

/// The location of the database file. Unchangeable: the user doesn't need to know the location of
/// this file
pub const DATABASE_FILE: &str = ".settle.sql";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigOptions
{
    pub zettelkasten: String,
    pub template: String,
}

impl ::std::default::Default for ConfigOptions
{
    fn default() -> ConfigOptions
    {
        ConfigOptions { zettelkasten: format!("{}/zettelkasten", env!("HOME")),
                        template: String::from("") }
    }
}

impl ConfigOptions
{
    pub fn db_file(&self) -> String
    {
        format!("{}/{}", &self.zettelkasten, DATABASE_FILE)
    }
}

/// Given a path, expand environment variables and tilde at beginning if it exists
fn expand_path(path: &str) -> String
{
    if path.chars().count() < 1 {
        return path.to_string();
    }
    let result = match path.chars().next().unwrap() {
        '/' => path.to_string(),
        '~' => shellexpand::tilde(path).to_string(),
        '$' => path.to_string(),
        _ => format!("$HOME/{}", path),
    };
    shellexpand::env(&result).unwrap().to_string()
}

impl ConfigOptions
{
    pub fn load() -> ConfigOptions
    {
        let xdg_cfg_dir = option_env!("XDG_CONFIG_HOME");
        let config_path = if xdg_cfg_dir.is_none() {
            // Use $HOME/.config/settle/settle.yaml if XDG_CONFIG_HOME isn't set
            format!("{}/.config/settle", env!("HOME"))
        } else {
            // Use $XDG_CONFIG_HOME/settle/settle.yaml otherwise
            format!("{}/settle", xdg_cfg_dir.unwrap())
        };
        let config_file = format!("{}/settle.yaml", config_path);

        // If the dir doesn't exist, create it
        if !dir_exists(&config_path) {
            mkdir(&config_path);
        }

        // If the file doesn't exist, create it
        if !file_exists(&config_file) {
            let data = serde_yaml::to_string(&ConfigOptions::default()).unwrap();
            write_to_file(&config_file, &data);
        }

        // The paths inside the config file may not be absolute, and so we need to expand them
        let tmp: ConfigOptions = serde_yaml::from_str(&file_to_string(&config_file)).unwrap();

        let cfg = ConfigOptions { zettelkasten: expand_path(&tmp.zettelkasten),
                                  template: expand_path(&tmp.template) };

        // Create the Zettelkasten directory and the 'inbox'project if it doesn't exist
        if !dir_exists(&cfg.zettelkasten) {
            mkdir(&cfg.zettelkasten);
            mkdir(&format!("{}/inbox", &cfg.zettelkasten));
        }

        cfg
    }
}
