use image::ImageError;
use thiserror::Error;
use winit::window::BadIcon;

// #[non_exhaustive]
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum AppError {
    #[error("{0}")]
    Error(String),

    #[error("Mqtt服务未配置")]
    MqttServerNoConfig,

    #[error("未知错误, 请联系开发人员.")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, AppError>;

impl From<BadIcon> for AppError {
    fn from(e: BadIcon) -> Self {
        AppError::Error(e.to_string())
    }
}

impl From<ImageError> for AppError {
    fn from(e: ImageError) -> Self {
        AppError::Error(e.to_string())
    }
}

impl From<String> for AppError {
    fn from(e: String) -> Self {
        AppError::Error(e)
    }
}
