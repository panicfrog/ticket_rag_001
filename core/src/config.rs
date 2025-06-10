//! # 配置管理模块
//! 
//! 定义系统配置结构和加载逻辑

use rag_deps::*;

/// 应用程序配置
/// 
/// 职责：
/// - 统一管理所有服务的配置
/// - 支持不同环境的配置
/// - 提供配置验证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub vector_db: VectorDbConfig,
    pub embedding: EmbeddingConfig,
    pub reranking: RerankingConfig,
    pub llm: LLMConfig,
    pub logging: LoggingConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64, // seconds
}

/// 向量数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDbConfig {
    pub provider: String, // sqlite, qdrant, postgres
    pub connection_string: String,
    pub dimension: usize,
    pub collection_name: Option<String>,
    pub table_name: Option<String>,
}

/// Embedding服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    pub provider: String, // qwen, openai, local
    pub model: String,
    pub api_key: String,
    pub endpoint: String,
    pub dimension: usize,
    pub batch_size: usize,
    pub timeout: u64, // seconds
}

/// Reranking服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankingConfig {
    pub provider: String, // qwen, cohere, local
    pub model: String,
    pub api_key: String,
    pub endpoint: String,
    pub max_documents: usize,
    pub timeout: u64, // seconds
}

/// LLM服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: String, // qwen, openai, local
    pub model: String,
    pub api_key: String,
    pub endpoint: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub timeout: u64, // seconds
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
    pub json_format: bool,
}

impl AppConfig {
    /// 从配置文件加载
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    /// 从环境变量加载
    pub fn from_env() -> Result<Self> {
        // TODO: 实现从环境变量加载配置的逻辑
        todo!("从环境变量加载配置")
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // TODO: 实现配置验证逻辑
        // - 检查必要字段是否存在
        // - 验证配置值的合理性
        // - 检查服务连接性
        Ok(())
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: None,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file: None,
            json_format: false,
        }
    }
} 