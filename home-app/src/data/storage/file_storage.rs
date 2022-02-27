use crate::resource::error::Result;

use super::storage::Storage;
use std::{collections::HashMap, path::PathBuf};
use std::{fs::File, io::BufReader};

pub struct FileStorage {
    json_filepath: PathBuf,
    kv: HashMap<String, String>,
    dirty: bool,
}

/// 本代码取自 epi crate, 修改满足我们的需求

impl FileStorage {
    /// 使用 绝对路径 后缀是.json的配置文件
    pub fn from_json_filepath(json_filepath: impl Into<PathBuf>) -> Result<Self> {
        let read_json = |json_filepath| match File::open(json_filepath) {
            Ok(file) => serde_json::from_reader(BufReader::new(file)).ok(),
            Err(_err) => None,
        };
        let json_filepath = json_filepath.into();
        let kv: HashMap<String, String> = read_json(&json_filepath).unwrap_or_default();
        Ok(Self {
            kv,
            json_filepath,
            dirty: false,
        })
    }
}

impl Storage for FileStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.kv.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        if self.kv.get(key) != Some(&value) {
            self.kv.insert(key.to_owned(), value);
            self.dirty = true;
        }
    }

    fn flush(&mut self) {
        if self.dirty {
            log::debug!("Persisted to {}", self.json_filepath.display());
            let file = std::fs::File::create(&self.json_filepath).unwrap();
            if serde_json::to_writer_pretty(file, &self.kv).is_ok() {
                self.dirty = false;
            }
        }
    }
}
