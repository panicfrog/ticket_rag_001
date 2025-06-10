//! # 配置加载器
//! 
//! 提供配置文件加载和环境变量覆盖功能

use rag_deps::*;
use rag_core::{config::AppConfig, errors::AppError};

/// 配置加载器
/// 
/// 职责：
/// - 从多个源加载配置
/// - 支持环境变量覆盖
/// - 配置验证
pub struct ConfigurationLoader;

impl ConfigurationLoader {
    /// 从文件加载配置
    pub fn load_from_file(config_path: &str) -> Result<AppConfig> {
        info!("从文件加载配置: {}", config_path);
        
        let config_content = std::fs::read_to_string(config_path)
            .map_err(|e| AppError::Configuration {
                message: format!("无法读取配置文件 {}: {}", config_path, e),
            })?;
        
        let config: AppConfig = toml::from_str(&config_content)
            .map_err(|e| AppError::Configuration {
                message: format!("配置文件解析失败: {}", e),
            })?;
        
        Self::apply_env_overrides(config)
    }
    
    /// 应用环境变量覆盖
    fn apply_env_overrides(mut config: AppConfig) -> Result<AppConfig> {
        // 服务器配置覆盖
        if let Ok(host) = std::env::var("RAG_SERVER_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = std::env::var("RAG_SERVER_PORT") {
            config.server.port = port.parse().unwrap_or(config.server.port);
        }
        
        // 数据库配置覆盖
        if let Ok(db_url) = std::env::var("RAG_DATABASE_URL") {
            config.database.url = db_url;
        }
        
        // 嵌入服务配置覆盖
        if let Ok(api_key) = std::env::var("RAG_EMBEDDING_API_KEY") {
            config.embedding.api_key = api_key;
        }
        if let Ok(endpoint) = std::env::var("RAG_EMBEDDING_ENDPOINT") {
            config.embedding.endpoint = endpoint;
        }
        
        // 重排序服务配置覆盖
        if let Ok(api_key) = std::env::var("RAG_RERANK_API_KEY") {
            config.reranking.api_key = api_key;
        }
        
        // LLM服务配置覆盖
        if let Ok(api_key) = std::env::var("RAG_LLM_API_KEY") {
            config.llm.api_key = api_key;
        }
        
        info!("环境变量覆盖应用完成");
        Ok(config)
    }
    
    /// 验证配置
    pub fn validate_config(config: &AppConfig) -> Result<()> {
        // 验证必要的配置项
        if config.embedding.api_key.trim().is_empty() {
            return Err(AppError::Configuration {
                message: "嵌入服务API密钥不能为空".to_string(),
            }.into());
        }
        
        if config.reranking.api_key.trim().is_empty() {
            return Err(AppError::Configuration {
                message: "重排序服务API密钥不能为空".to_string(),
            }.into());
        }
        
        if config.llm.api_key.trim().is_empty() {
            return Err(AppError::Configuration {
                message: "LLM服务API密钥不能为空".to_string(),
            }.into());
        }
        
        if config.database.url.trim().is_empty() {
            return Err(AppError::Configuration {
                message: "数据库连接字符串不能为空".to_string(),
            }.into());
        }
        
        info!("配置验证通过");
        Ok(())
    }
} 