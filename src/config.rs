use crate::io::{dir_exists, dirname, file_exists, file_to_string, mkdir, write_to_file};
use serde::{Deserialize, Serialize};
use std::env;

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
        ConfigOptions {
            zettelkasten: format!("{}/zettelkasten", env::var("HOME").unwrap()),
            template: String::from(""),
        }
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
        let config_file = Self::cfg_file();
        let config_dir = dirname(&config_file);

        // The configuration directory is necessary to creating the configuration file
        if !dir_exists(&config_dir) {
            mkdir(&config_dir);
        }

        // Provide a default configuration file if it doesn't exist
        if !file_exists(&config_file) {
            let data = serde_yaml::to_string(&ConfigOptions::default()).unwrap();
            write_to_file(&config_file, &data);
        }

        // The config file may have relative paths, but we only deal in absolutes
        let tmp: ConfigOptions = serde_yaml::from_str(&file_to_string(&config_file)).unwrap();
        let cfg = ConfigOptions {
            zettelkasten: expand_path(&tmp.zettelkasten),
            template: expand_path(&tmp.template),
        };

        // Create the Zettelkasten directory it doesn't exist already
        if !dir_exists(&cfg.zettelkasten) {
            mkdir(&cfg.zettelkasten);
        }

        cfg
    }

    // The configuration is determined by looking at the environment variables in this order:
    // 1. If `$SETTLE_CONFIG` is set: `$SETTLE_CONFIG`
    // 2. If `$XDG_CONFIG_HOME` is set: `$XDG_CONFIG_HOME/settle/settle.yaml`
    // 3. default: `$HOME/.config/settle/settle.yaml`
    pub fn cfg_file() -> String
    {
        let settle_cfg = env::var("SETTLE_CONFIG").unwrap_or_default();
        let xdg_cfg_home = env::var("XDG_CONFIG_HOME").unwrap_or_default();
        if !settle_cfg.is_empty() {
            settle_cfg
        } else if !xdg_cfg_home.is_empty() {
            format!("{}/settle/settle.yaml", xdg_cfg_home)
        } else {
            format!("{}/.config/settle/settle.yaml", env::var("HOME").unwrap())
        }
    }
}
