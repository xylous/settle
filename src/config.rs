use serde::{Serialize, Deserialize};
use crate::io::{file_to_string, file_exists, write_to_file};

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
            zettelkasten: format!("{}/zettelkasten", env!("HOME")),
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
            format!( "{}/.config/settle/settle.yaml", env!("HOME"))
        } else {
            // Use $XDG_CONFIG_HOME/settle/settle.yaml otherwise
            format!( "{}/settle/settle.yaml", xdg_cfg_dir.unwrap())
        };

        // If the file doesn't exist, create it
        if !file_exists(&config_path) {
            let data = serde_yaml::to_string(&ConfigOptions::default()).unwrap();
            write_to_file(&config_path, &data);
        }

        // The paths inside the config file may not be absolute, and so we need to expand them
        let tmp: ConfigOptions = serde_yaml::from_str(&file_to_string(&config_path)).unwrap();
        ConfigOptions {
            zettelkasten: expand_path(&tmp.zettelkasten),
            template: expand_path(&tmp.template),
        }
    }
}
