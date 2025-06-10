//! # 解决方案数据模型
//! 
//! 定义工单解决方案相关的数据结构

use rag_deps::*;

/// 工单解决方案
/// 
/// 职责：
/// - 表示针对工单的处理建议
/// - 记录用户反馈和采纳情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSolution {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub solution: String,
    pub confidence: f32,
    pub reasoning: String,
    pub is_accepted: bool,
    pub created_at: DateTime<Utc>,
    pub feedback_score: Option<i32>, // 1-5分评分
    pub feedback_comment: Option<String>,
}

/// 处理结果
/// 
/// 职责：
/// - 封装完整的工单处理结果
/// - 包含相似案例、建议方案等信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub ticket_id: Uuid,
    pub similar_tickets: Vec<SimilarTicket>,
    pub suggested_solution: String,
    pub confidence: f32,
    pub reasoning: String,
    pub processing_time_ms: u64,
}

/// 相似工单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarTicket {
    pub ticket_id: Uuid,
    pub title: String,
    pub description: String,
    pub similarity_score: f32,
    pub rerank_score: f32,
    pub solution: Option<String>,
}

/// 用户反馈
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub solution_id: Uuid,
    pub is_accepted: bool,
    pub score: Option<i32>,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TicketSolution {
    /// 创建新的解决方案
    pub fn new(
        ticket_id: Uuid,
        solution: String,
        confidence: f32,
        reasoning: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            ticket_id,
            solution,
            confidence,
            reasoning,
            is_accepted: false,
            created_at: Utc::now(),
            feedback_score: None,
            feedback_comment: None,
        }
    }
    
    /// 接受解决方案
    pub fn accept(&mut self, feedback: Option<Feedback>) {
        self.is_accepted = true;
        if let Some(fb) = feedback {
            self.feedback_score = fb.score;
            self.feedback_comment = fb.comment;
        }
    }
} 