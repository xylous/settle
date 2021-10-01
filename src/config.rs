use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigOptions
{
    pub zettelkasten: String,
}

impl ::std::default::Default for ConfigOptions
{
    fn default() -> ConfigOptions
    {
        let zettelkasten_path = format!("{}/zettelkasten/", env!("HOME"));
        ConfigOptions {
            zettelkasten: zettelkasten_path,
        }
    }
}

impl ConfigOptions
{
    pub fn load() -> ConfigOptions
    {
        let config_path = format!(
            "{}/.config/settle/settle.yaml",
            env!("HOME"),
        );
        confy::load_path(config_path).unwrap_or_default()
    }
}
