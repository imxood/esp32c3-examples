use serde::{de::DeserializeOwned, Serialize};

use crate::resource::{defines::APP_NAME, error::Result};

use super::{file_storage::FileStorage, storage::Storage};
use std::{
    env::current_exe,
    path::PathBuf,
    time::{Duration, Instant},
};

/// 在一定的时间间隔后, 如果数据变化, 就保存到文件
pub struct Persistence {
    storage: Box<dyn Storage>,
    last_auto_save: Instant,
    auto_save_interval: Duration,
}

impl Default for Persistence {
    /// 默认配置文件路径是可执行文件所在路径
    fn default() -> Self {
        let mut config_file = current_exe().unwrap();
        config_file.pop();
        config_file.push(format!("{}.json", APP_NAME));
        tracing::info!("config file: {:?}", &config_file);

        Self {
            auto_save_interval: Duration::from_secs(10),
            storage: Box::new(FileStorage::from_json_filepath(config_file).unwrap()),
            last_auto_save: Instant::now(),
        }
    }
}

impl Persistence {
    pub fn from_path(json_path: impl Into<PathBuf>) -> Result<Self> {
        Ok(Self {
            auto_save_interval: Duration::from_secs(30),
            storage: Box::new(FileStorage::from_json_filepath(json_path)?),
            last_auto_save: Instant::now(),
        })
    }

    pub fn set_auto_save_interval(&mut self, auto_save_interval: Duration) {
        self.auto_save_interval = auto_save_interval;
    }

    #[inline(always)]
    pub fn set_value<T: Serialize>(&mut self, key: &str, value: &T) {
        self.storage
            .set_string(key, serde_json::to_string(value).unwrap());
    }

    #[inline(always)]
    pub fn get_value<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.storage
            .get_string(key)
            .and_then(|value| serde_json::from_str(&value).ok())
    }

    #[inline(always)]
    pub fn save(&mut self) {
        self.storage.flush();
    }

    #[inline(always)]
    pub fn maybe_autosave(&mut self) {
        let now = Instant::now();
        if now - self.last_auto_save > self.auto_save_interval {
            self.save();
            self.last_auto_save = now;
        }
    }
}

unsafe impl Send for Persistence {}
unsafe impl Sync for Persistence {}
