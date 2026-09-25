use crate::shared::debug_logger::DebugLogger;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct FileExplorer {
    #[allow(dead_code)]
    debug_logger: Arc<Mutex<DebugLogger>>,
}

impl FileExplorer {
    pub fn new(debug_logger: Arc<Mutex<DebugLogger>>) -> Self {
        Self { debug_logger }
    }

    pub fn list_dir(self, path: &str) -> Vec<String> {
        let mut files: Vec<String> = vec![];
        let normalized_path: String = if path.is_empty() {
            ".".to_string()
        } else {
            path.to_string()
        };

        let root_dir_path = if Path::new(&normalized_path).is_dir() {
            normalized_path.clone()
        } else {
            match Path::new(&normalized_path).parent() {
                Some(parent) if parent.as_os_str().is_empty() => ".".to_string(),
                Some(parent) => parent.to_str().unwrap().to_string(),
                None => "/".to_string(),
            }
        };

        let dir = fs::read_dir(root_dir_path);
        if let Ok(dir) = dir {
            for entry in dir {
                let entry = entry.unwrap();
                if let Ok(file_type) = entry.file_type() {
                    let mut path = entry.path().display().to_string();
                    if file_type.is_dir() {
                        path.push('/');
                    }
                    files.push(path);
                }
            }
        }
        files
    }
}
