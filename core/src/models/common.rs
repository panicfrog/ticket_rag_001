//! # 通用数据模型
//! 
//! 定义系统中的通用数据结构

use rag_deps::*;

/// 微调数据记录
/// 
/// 职责：
/// - 存储用于模型微调的训练数据
/// - 记录数据来源和质量评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneData {
    pub id: Uuid,
    pub input_text: String,
    pub target_output: String,
    pub data_source: DataSource,
    pub quality_score: f32,
    pub created_at: DateTime<Utc>,
}

/// 数据来源枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    UserFeedback,    // 用户反馈
    ExpertLabeling,  // 专家标注
    SystemGenerated, // 系统生成
}

/// 分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

/// 分页查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// 查询过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub category: Option<String>,
    pub status: Option<String>,
    pub priority_min: Option<i32>,
    pub priority_max: Option<i32>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub keywords: Option<String>,
}

impl<T> PagedResult<T> {
    pub fn new(data: Vec<T>, total: u64, page: u32, page_size: u32) -> Self {
        let total_pages = (total as f64 / page_size as f64).ceil() as u32;
        Self {
            data,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
} 