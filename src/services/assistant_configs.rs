use crate::shared::any_error::AnyError;
use crate::shared::assistant_config::{AssistantConfig, StoredAssistant};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

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

pub fn load_all() -> Result<(Vec<StoredAssistant>, Vec<String>), AnyError> {
    let dir = match fs::read_dir(CONFIGS_DIR) {
        Ok(dir) => dir,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok((vec![], vec![])),
        Err(err) => return Err(err.into()),
    };

    let mut assistants = vec![];
    let mut errors = vec![];
    for entry in dir {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("yml") {
            continue;
        }
        let parsed = fs::read_to_string(&path)
            .map_err(AnyError::from)
            .and_then(|yaml| serde_norway::from_str::<AssistantConfig>(&yaml).map_err(AnyError::from));
        match parsed {
            Ok(config) => assistants.push(StoredAssistant { path, config }),
            Err(err) => errors.push(format!("{}: {err}", path.display())),
        }
    }
    assistants.sort_by(|a, b| a.config.name.cmp(&b.config.name));

    Ok((assistants, errors))
}

pub fn remove(path: &Path) -> Result<(), AnyError> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn load_system_prompt(config: &AssistantConfig) -> Result<String, AnyError> {
    let raw_path = config.system_prompt_path.trim();
    let path = raw_path.strip_prefix('@').unwrap_or(raw_path);
    let path = match path.strip_prefix('~') {
        Some(rest) => format!("{}{rest}", std::env::var("HOME")?),
        None => path.to_string(),
    };
    fs::read_to_string(&path).map_err(|err| format!("{path}: {err}").into())
}
