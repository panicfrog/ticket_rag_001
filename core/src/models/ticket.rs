//! # 工单数据模型
//! 
//! 定义工单相关的数据结构

use rag_deps::*;

/// 工单实体
/// 
/// 职责：
/// - 表示工单的完整信息
/// - 提供工单的基本操作方法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: i32,
    pub status: TicketStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub embedding: Option<Vec<f32>>,
    pub tags: Vec<String>,
}

/// 新建工单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTicket {
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: i32,
    pub tags: Vec<String>,
}

/// 工单状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    New,        // 新建
    Processing, // 处理中
    Resolved,   // 已解决
    Closed,     // 已关闭
}

impl Ticket {
    /// 创建新工单
    pub fn new(request: NewTicket) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: request.title,
            description: request.description,
            category: request.category,
            priority: request.priority,
            status: TicketStatus::New,
            created_at: now,
            updated_at: now,
            embedding: None,
            tags: request.tags,
        }
    }
    
    /// 获取工单完整文本内容（用于向量化）
    pub fn get_full_text(&self) -> String {
        format!("{} {} {}", self.title, self.description, self.tags.join(" "))
    }
    
    /// 更新工单状态
    pub fn update_status(&mut self, status: TicketStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// 设置向量化结果
    pub fn set_embedding(&mut self, embedding: Vec<f32>) {
        self.embedding = Some(embedding);
        self.updated_at = Utc::now();
    }
} 