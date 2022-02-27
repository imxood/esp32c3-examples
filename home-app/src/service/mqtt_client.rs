#[derive(Default, Serialize, Deserialize)]
pub struct MqttConfig {
    config: Config,
    #[serde(skip)]
    jh: Option<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}
