//! # 错误处理模块
//! 
//! 定义系统中的统一错误类型

use rag_deps::*;

/// 应用程序错误类型
/// 
/// 职责：
/// - 统一不同组件的错误表示
/// - 提供详细的错误信息
/// - 支持错误链追踪
#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库操作失败: {message}")]
    Database { message: String },
    
    #[error("向量数据库操作失败: {message}")]
    VectorDatabase { message: String },
    
    #[error("Embedding服务错误: {message}")]
    EmbeddingService { message: String },
    
    #[error("Rerank服务错误: {message}")]
    RerankService { message: String },
    
    #[error("LLM服务错误: {message}")]
    LLMService { message: String },
    
    #[error("配置错误: {message}")]
    Configuration { message: String },
    
    #[error("验证错误: {field}: {message}")]
    Validation { field: String, message: String },
    
    #[error("资源未找到: {resource}: {id}")]
    NotFound { resource: String, id: String },
    
    #[error("权限不足: {action}")]
    Permission { action: String },
    
    #[error("网络请求失败: {message}")]
    Network { message: String },
    
    #[error("内部错误: {message}")]
    Internal { message: String },
}

/// 结果类型别名
pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// 创建数据库错误
    pub fn database<T: ToString>(message: T) -> Self {
        Self::Database {
            message: message.to_string(),
        }
    }
    
    /// 创建验证错误
    pub fn validation<T: ToString, U: ToString>(field: T, message: U) -> Self {
        Self::Validation {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
    
    /// 创建未找到错误
    pub fn not_found<T: ToString, U: ToString>(resource: T, id: U) -> Self {
        Self::NotFound {
            resource: resource.to_string(),
            id: id.to_string(),
        }
    }
}

// 错误转换实现
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Internal {
            message: format!("JSON处理错误: {}", err),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal {
            message: err.to_string(),
        }
    }
} 