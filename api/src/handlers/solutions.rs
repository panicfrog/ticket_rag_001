//! # 解决方案API处理器

use rag_deps::*;
use rag_infrastructure::ServiceContainer;
use axum::{extract::{State, Path}, response::Json, http::StatusCode};

/// 提交反馈
pub async fn submit_feedback(
    State(_services): State<ServiceContainer>,
    Path(_id): Path<Uuid>,
) -> Result<StatusCode, String> {
    Ok(StatusCode::OK)
}

/// 接受解决方案
pub async fn accept_solution(
    State(_services): State<ServiceContainer>,
    Path(_id): Path<Uuid>,
) -> Result<StatusCode, String> {
    Ok(StatusCode::OK)
}

/// 拒绝解决方案
pub async fn reject_solution(
    State(_services): State<ServiceContainer>,
    Path(_id): Path<Uuid>,
) -> Result<StatusCode, String> {
    Ok(StatusCode::OK)
} 