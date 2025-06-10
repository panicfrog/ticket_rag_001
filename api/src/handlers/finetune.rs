//! # 微调API处理器

use rag_deps::*;
use rag_infrastructure::ServiceContainer;
use axum::{extract::{State, Path}, response::Json};

/// 导出微调数据
pub async fn export_data(
    State(_services): State<ServiceContainer>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "data": [],
        "count": 0,
        "format": "jsonl"
    }))
}

/// 启动微调任务
pub async fn start_job(
    State(_services): State<ServiceContainer>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "job_id": Uuid::new_v4(),
        "status": "queued"
    }))
}

/// 获取微调任务列表
pub async fn list_jobs(
    State(_services): State<ServiceContainer>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "jobs": [],
        "total": 0
    }))
}

/// 获取微调任务状态
pub async fn get_job_status(
    State(_services): State<ServiceContainer>,
    Path(_id): Path<Uuid>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "completed",
        "progress": 100,
        "metrics": {
            "loss": 0.1,
            "accuracy": 0.95
        }
    }))
} 