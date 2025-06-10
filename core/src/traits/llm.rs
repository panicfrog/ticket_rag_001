//! # LLM服务抽象接口
//! 
//! 定义大语言模型推理服务的统一接口

use rag_deps::*;
use super::embedding::ModelInfo;
use super::reranking::RerankResult;
use crate::models::Ticket;

/// LLM推理结果
#[derive(Debug, Clone)]
pub struct LLMResponse {
    pub content: String,
    pub confidence: f32,
    pub reasoning: String,
    pub token_usage: Option<TokenUsage>,
}

/// Token使用统计
#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// LLM服务trait
/// 
/// 职责：
/// - 基于上下文生成工单处理建议
/// - 提供推理解释
/// - 管理token使用
#[async_trait]
pub trait LLMService: Send + Sync {
    /// 生成工单处理建议
    async fn generate_solution(
        &self, 
        ticket: &Ticket, 
        similar_cases: &[RerankResult]
    ) -> Result<LLMResponse>;
    
    /// 批量生成建议
    async fn generate_solutions_batch(
        &self,
        requests: &[(Ticket, Vec<RerankResult>)]
    ) -> Result<Vec<LLMResponse>>;
    
    /// 自定义prompt推理
    async fn chat(&self, prompt: &str) -> Result<LLMResponse>;
    
    /// 获取模型信息
    fn model_info(&self) -> ModelInfo;
    
    /// 健康检查
    async fn health_check(&self) -> Result<bool>;
} 