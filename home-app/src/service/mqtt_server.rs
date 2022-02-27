use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
};

use librumqttd::{Broker, Config};
use serde::{Deserialize, Serialize};

use crate::resource::error::{AppError, Result};

#[derive(Default, Serialize, Deserialize)]
pub struct MqttServer {
    config: Config,
    #[serde(skip)]
    jh: Option<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}

impl MqttServer {
    /// 服务是否在运行
    pub fn is_running(&self) -> bool {
        if let Some(jh) = &self.jh {
            jh.is_finished()
        } else {
            false
        }
    }

    /// 停止服务
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    /// 启动服务
    pub fn start(&mut self) -> Result<()> {
        if self.config.servers.len() == 0 {
            return Err(AppError::MqttServerNoConfig);
        }
        let mut broker = Broker::new(self.config.clone());
        let builder = std::thread::Builder::new().name("mqtt-server".to_string());
        let jh = builder
            .spawn(move || {
                broker.start().unwrap();
            })
            .unwrap();
        self.jh = Some(jh);
        Ok(())
    }
}
