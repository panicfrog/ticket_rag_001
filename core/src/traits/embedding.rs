//! # Embedding服务抽象接口
//! 
//! 定义文本向量化服务的统一接口，支持多种不同的embedding模型

use rag_deps::*;

/// 模型信息结构
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub provider: String,
    pub max_tokens: usize,
    pub cost_per_call: Option<f32>,
}

/// Embedding服务trait
/// 
/// 职责：
/// - 将文本转换为向量表示
/// - 支持单条和批量处理
/// - 提供模型信息和健康检查
#[async_trait]
pub trait EmbeddingService: Send + Sync {
    /// 单条文本向量化
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    
    /// 批量文本向量化
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
    
    /// 获取向量维度
    fn dimension(&self) -> usize;
    
    /// 获取模型信息
    fn model_info(&self) -> ModelInfo;
    
    /// 健康检查
    async fn health_check(&self) -> Result<bool>;
} 