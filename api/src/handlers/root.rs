//! # 根路径处理器
//! 
//! 处理根路径和健康检查相关的HTTP请求

use axum::response::Json;
use rag_deps::*;
use std::env;

/// 根路径响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootResponse {
    pub service: String,
    pub version: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub build_info: BuildInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildInfo {
    pub build_time: String,
    pub git_sha: String,
    pub rust_version: String,
}

/// 根路径处理器
pub async fn root() -> Json<RootResponse> {
    Json(RootResponse {
        service: "RAG工单处理系统".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: "运行中".to_string(),
        timestamp: Utc::now(),
        build_info: BuildInfo {
            build_time: env::var("VERGEN_BUILD_TIMESTAMP").unwrap_or_else(|_| "未知".to_string()),
            git_sha: "未知".to_string(),
            rust_version: env::var("VERGEN_RUSTC_SEMVER").unwrap_or_else(|_| "未知".to_string()),
        }
    })
}

/// 健康检查
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "uptime": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

/// 欢迎页面
pub async fn welcome() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "欢迎使用RAG工单处理系统",
        "version": env!("CARGO_PKG_VERSION"),
        "status": "running",
        "api_docs": "/docs"
    }))
}

/// API文档
pub async fn api_docs() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "RAG工单处理系统API",
        "version": "v1",
        "endpoints": {
            "tickets": "/api/v1/tickets",
            "search": "/api/v1/search",
            "admin": "/api/v1/admin",
            "stats": "/api/v1/stats"
        },
        "authentication": "Bearer token or ApiKey",
        "rate_limits": "1000 requests per hour per IP"
    }))
}

/// 版本信息
pub async fn version() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "build_time": env::var("VERGEN_BUILD_TIMESTAMP").unwrap_or_else(|_| "未知".to_string()),
        "git_sha": "未知",
        "rust_version": env::var("VERGEN_RUSTC_SEMVER").unwrap_or_else(|_| "未知".to_string())
    }))
} 