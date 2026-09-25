use crate::shared::any_error::AnyError;
use crate::shared::assistant_config::AssistantConfig;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const CONFIGS_DIR: &str = "harness_configs";

/// Saves the config to `harness_configs/<name>.yml` in the current directory.
/// Refuses to overwrite an existing file.
pub fn save(config: &AssistantConfig) -> Result<PathBuf, AnyError> {
    let file_name: String = config
        .name
        .trim()
        .chars()
        .map(|c| if c == '/' || c == '\\' || c.is_whitespace() { '-' } else { c })
        .collect();
    let path = PathBuf::from(CONFIGS_DIR).join(format!("{file_name}.yml"));

    fs::create_dir_all(CONFIGS_DIR)?;
    let yaml = serde_norway::to_string(config)?;
    // create_new fails if the file already exists
    let mut file = OpenOptions::new().write(true).create_new(true).open(&path)?;
    file.write_all(yaml.as_bytes())?;

    Ok(path)
}
