//! # RAG工单处理系统主程序
//! 
//! 系统启动入口，负责：
//! - 加载配置
//! - 初始化各个服务
//! - 启动HTTP服务器

use rag_deps::*;
use rag_core::config::AppConfig;
use rag_infrastructure::{ServiceFactory, logging};
use rag_api::ApiServer;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    logging::init_logging()?;
    info!("启动RAG工单处理系统");
    
    // 加载配置
    let config = load_config().await?;
    info!("配置加载完成");
    
    // 创建服务容器
    let service_container = ServiceFactory::create_service_container(&config).await?;
    info!("服务容器创建完成");
    
    // 创建并启动API服务器
    let api_server = ApiServer::new(config, service_container);
    api_server.start().await?;
    
    Ok(())
}

/// 加载应用配置
async fn load_config() -> Result<AppConfig> {
    use rag_infrastructure::configuration::ConfigurationLoader;
    
    // 优先从环境变量获取配置文件路径
    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "config/development.toml".to_string());
    
    info!("从配置文件加载配置: {}", config_path);
    
    let config = ConfigurationLoader::load_from_file(&config_path)?;
    ConfigurationLoader::validate_config(&config)?;
    
    Ok(config)
} 