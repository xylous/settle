use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigOptions
{
    pub zettelkasten: String,
    pub template: String,
    db_file: String,
}

impl ::std::default::Default for ConfigOptions
{
    fn default() -> ConfigOptions
    {
        let zettelkasten_path = format!("{}/zettelkasten", env!("HOME"));
        ConfigOptions {
            zettelkasten: zettelkasten_path,
            db_file: String::from("metadata.sql"),
            template: String::from(""),
        }
    }
}

impl ConfigOptions
{
    pub fn db_file(&self) -> String
    {
        format!("{}/{}", &self.zettelkasten, &self.db_file)
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
        let config_path = format!(
            "{}/.config/settle/settle.yaml",
            env!("HOME"),
        );

        let tmp: ConfigOptions = confy::load_path(config_path).unwrap_or_default();
        ConfigOptions {
            zettelkasten: expand_path(&tmp.zettelkasten),
            template: expand_path(&tmp.template),
            db_file: tmp.db_file,
        }
    }
}
