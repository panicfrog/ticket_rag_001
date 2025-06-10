//! # 数据传输对象 (DTO)
//! 
//! 定义API层的数据传输格式

use rag_deps::*;
use rag_core::models::{Ticket, NewTicket, TicketStatus, PagedResult, Pagination, QueryFilter, DataSource};
use rag_core::models::solution::ProcessResult;
use rag_core::traits::reranking::RerankResult;

/// 创建工单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: i32,
    pub tags: Vec<String>,
}

impl From<CreateTicketRequest> for NewTicket {
    fn from(request: CreateTicketRequest) -> Self {
        NewTicket {
            title: request.title,
            description: request.description,
            category: request.category,
            priority: request.priority,
            tags: request.tags,
        }
    }
}

/// 工单更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<TicketStatus>,
    pub tags: Option<Vec<String>>,
}

/// 工单处理响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessTicketResponse {
    pub ticket_id: Uuid,
    pub solution: String,
    pub confidence: f32,
    pub reasoning: String,
    pub similar_cases: Vec<RerankResult>,
    pub processing_time_ms: u64,
}

impl From<ProcessResult> for ProcessTicketResponse {
    fn from(result: ProcessResult) -> Self {
        Self {
            ticket_id: result.ticket_id,
            solution: result.suggested_solution,
            confidence: result.confidence,
            reasoning: result.reasoning,
            similar_cases: vec![], // 这里需要根据 similar_tickets 转换
            processing_time_ms: result.processing_time_ms,
        }
    }
}

/// 搜索参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub limit: Option<usize>,
}

/// 工单列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTicketsParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub category: Option<String>,
    pub status: Option<TicketStatus>,
    pub priority_min: Option<i32>,
    pub priority_max: Option<i32>,
    pub keywords: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl From<ListTicketsParams> for Pagination {
    fn from(params: ListTicketsParams) -> Self {
        Pagination {
            page: params.page.unwrap_or(1),
            page_size: params.page_size.unwrap_or(20),
        }
    }
}

impl From<ListTicketsParams> for QueryFilter {
    fn from(params: ListTicketsParams) -> Self {
        QueryFilter {
            category: params.category,
            status: params.status.map(|s| format!("{:?}", s)),
            priority_min: params.priority_min,
            priority_max: params.priority_max,
            date_from: None,
            date_to: None,
            keywords: params.keywords,
        }
    }
}

/// API响应包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error_code: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error_code: None,
            timestamp: Utc::now(),
        }
    }
    
    pub fn error(message: String, error_code: Option<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            error_code,
            timestamp: Utc::now(),
        }
    }
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl<T> From<PagedResult<T>> for PaginatedResponse<T> {
    fn from(result: PagedResult<T>) -> Self {
        Self {
            data: result.data,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
        }
    }
}

/// 统计信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_tickets: u64,
    pub tickets_by_status: Vec<StatusCount>,
    pub tickets_by_category: Vec<CategoryCount>,
    pub avg_resolution_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCount {
    pub status: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: u64,
}

/// 搜索工单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTicketsRequest {
    pub query: String,
    pub filters: Option<SearchFilters>,
    pub limit: Option<usize>,
}

/// 搜索过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub category: Option<String>,
    pub status: Option<TicketStatus>,
    pub priority_min: Option<i32>,
    pub priority_max: Option<i32>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
}

/// 反馈提交请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitFeedbackRequest {
    pub score: i32, // 1-5
    pub comment: Option<String>,
    pub is_helpful: bool,
    pub suggestions: Option<String>,
}

/// 相似工单查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindSimilarRequest {
    pub text: String,
    pub limit: Option<usize>,
    pub threshold: Option<f32>,
}

/// 微调任务创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFinetuneJobRequest {
    pub model_name: String,
    pub dataset_filter: Option<FinetuneDataFilter>,
    pub training_params: Option<TrainingParameters>,
}

/// 微调数据过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneDataFilter {
    pub quality_score_min: Option<f32>,
    pub data_source: Option<DataSource>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

/// 训练参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingParameters {
    pub epochs: Option<u32>,
    pub learning_rate: Option<f32>,
    pub batch_size: Option<u32>,
    pub warmup_steps: Option<u32>,
}

/// API错误响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub request_id: Option<String>,
} 