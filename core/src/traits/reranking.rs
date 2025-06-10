//! # Reranking服务抽象接口
//! 
//! 定义文本重排序服务的统一接口，支持多种rerank模型

use rag_deps::*;
use super::embedding::ModelInfo;

/// 重排序结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankResult {
    pub index: usize,        // 原始文档索引
    pub score: f32,          // 相关性分数
    pub document: String,    // 文档内容
}

/// Reranking服务trait
/// 
/// 职责：
/// - 对候选文档进行语义相关性重排序
/// - 支持单次和批量重排序
/// - 提供模型配置信息
#[async_trait]
pub trait RerankService: Send + Sync {
    /// 重排序
    async fn rerank(&self, query: &str, documents: &[String]) -> Result<Vec<RerankResult>>;
    
    /// 批量重排序（如果模型支持）
    async fn rerank_batch(
        &self, 
        queries: &[String], 
        documents: &[Vec<String>]
    ) -> Result<Vec<Vec<RerankResult>>>;
    
    /// 获取最大支持的文档数量
    fn max_documents(&self) -> usize;
    
    /// 获取模型信息
    fn model_info(&self) -> ModelInfo;
    
    /// 健康检查
    async fn health_check(&self) -> Result<bool>;
} 