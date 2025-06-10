//! # 业务验证模块
//! 
//! 实现各种业务规则和数据验证

use rag_deps::*;
use rag_core::{models::*, errors::AppError};

/// 工单验证器
/// 
/// 职责：
/// - 验证工单数据的完整性和合法性
/// - 实施业务规则检查
pub struct TicketValidator;

impl TicketValidator {
    /// 验证新工单请求
    pub fn validate_new_ticket(request: &NewTicket) -> Result<(), AppError> {
        // 验证标题
        if request.title.trim().is_empty() {
            return Err(AppError::validation("title", "工单标题不能为空"));
        }
        
        if request.title.len() > 200 {
            return Err(AppError::validation("title", "工单标题不能超过200字符"));
        }
        
        // 验证描述
        if request.description.trim().is_empty() {
            return Err(AppError::validation("description", "工单描述不能为空"));
        }
        
        if request.description.len() > 5000 {
            return Err(AppError::validation("description", "工单描述不能超过5000字符"));
        }
        
        // 验证分类
        if request.category.trim().is_empty() {
            return Err(AppError::validation("category", "工单分类不能为空"));
        }
        
        // 验证优先级
        if !(1..=5).contains(&request.priority) {
            return Err(AppError::validation("priority", "优先级必须在1-5之间"));
        }
        
        // 验证标签
        if request.tags.len() > 10 {
            return Err(AppError::validation("tags", "标签数量不能超过10个"));
        }
        
        for tag in &request.tags {
            if tag.len() > 50 {
                return Err(AppError::validation("tags", "单个标签不能超过50字符"));
            }
        }
        
        Ok(())
    }
    
    /// 验证工单更新
    pub fn validate_ticket_update(ticket: &Ticket, updates: &TicketUpdate) -> Result<(), AppError> {
        // TODO: 实现更新验证逻辑
        todo!("实现工单更新验证")
    }
}

/// 解决方案验证器
/// 
/// 职责：
/// - 验证解决方案的质量和合理性
/// - 检查置信度评分
pub struct SolutionValidator;

impl SolutionValidator {
    /// 验证解决方案
    pub fn validate_solution(solution: &TicketSolution) -> Result<(), AppError> {
        // 验证解决方案内容
        if solution.solution.trim().is_empty() {
            return Err(AppError::validation("solution", "解决方案内容不能为空"));
        }
        
        if solution.solution.len() > 10000 {
            return Err(AppError::validation("solution", "解决方案内容不能超过10000字符"));
        }
        
        // 验证置信度
        if !(0.0..=1.0).contains(&solution.confidence) {
            return Err(AppError::validation("confidence", "置信度必须在0-1之间"));
        }
        
        // 验证反馈评分
        if let Some(score) = solution.feedback_score {
            if !(1..=5).contains(&score) {
                return Err(AppError::validation("feedback_score", "反馈评分必须在1-5之间"));
            }
        }
        
        Ok(())
    }
}

/// 配置验证器
/// 
/// 职责：
/// - 验证系统配置的正确性
/// - 检查API密钥和连接参数
pub struct ConfigValidator;

impl ConfigValidator {
    /// 验证API配置
    pub fn validate_api_config(config: &EmbeddingConfig) -> Result<(), AppError> {
        if config.api_key.trim().is_empty() {
            return Err(AppError::validation("api_key", "API密钥不能为空"));
        }
        
        if config.endpoint.trim().is_empty() {
            return Err(AppError::validation("endpoint", "API端点不能为空"));
        }
        
        // 验证URL格式
        if !config.endpoint.starts_with("http://") && !config.endpoint.starts_with("https://") {
            return Err(AppError::validation("endpoint", "API端点必须是有效的HTTP(S) URL"));
        }
        
        // 验证维度
        if config.dimension == 0 || config.dimension > 4096 {
            return Err(AppError::validation("dimension", "向量维度必须在1-4096之间"));
        }
        
        Ok(())
    }
}

/// 工单更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
    pub priority: Option<i32>,
    pub tags: Option<Vec<String>>,
}

// 重新导出配置类型 - 临时解决方案
use rag_core::config::EmbeddingConfig; 