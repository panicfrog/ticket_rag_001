//! # 日志配置
//! 
//! 提供统一的日志配置和管理

use rag_deps::*;

/// 初始化日志系统
/// 
/// 配置日志格式、级别和输出目标
pub fn init_logging() -> Result<()> {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .init();
    
    info!("日志系统初始化完成");
    Ok(())
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
    pub file_path: Option<String>,
} 