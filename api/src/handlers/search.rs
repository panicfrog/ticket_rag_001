//! # 搜索API处理器

use rag_deps::*;
use rag_infrastructure::ServiceContainer;
use axum::{extract::State, response::Json};

/// 搜索工单
pub async fn search_tickets(
    State(_services): State<ServiceContainer>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "results": [],
        "total": 0
    }))
}

/// 查找相似工单
pub async fn find_similar(
    State(_services): State<ServiceContainer>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "similar_tickets": [],
        "scores": []
    }))
} 