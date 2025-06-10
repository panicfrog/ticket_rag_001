//! # 管理员API处理器

use rag_deps::*;
use rag_infrastructure::ServiceContainer;
use axum::{extract::State, response::Json};

/// 系统健康检查
pub async fn health_check(
    State(services): State<ServiceContainer>,
) -> Json<serde_json::Value> {
    let embedding_health = services.embedding_service.health_check().await.unwrap_or(false);
    let rerank_health = services.rerank_service.health_check().await.unwrap_or(false);
    let vector_db_health = services.vector_db.health_check().await.unwrap_or(false);
    let llm_health = services.llm_service.health_check().await.unwrap_or(false);
    
    let overall_health = embedding_health && rerank_health && vector_db_health && llm_health;
    
    Json(serde_json::json!({
        "status": if overall_health { "healthy" } else { "unhealthy" },
        "timestamp": Utc::now(),
        "services": {
            "embedding": embedding_health,
            "reranking": rerank_health,
            "vector_db": vector_db_health,
            "llm": llm_health
        }
    }))
}

/// 获取系统指标
pub async fn get_metrics() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "requests_total": 0,
        "requests_per_second": 0.0,
        "average_response_time_ms": 0.0,
        "error_rate": 0.0,
        "memory_usage_mb": 0,
        "cpu_usage_percent": 0.0
    }))
}

/// 获取配置信息
pub async fn get_config() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "environment": "development",
        "log_level": "info",
        "features": {
            "caching": true,
            "metrics": true,
            "rate_limiting": true
        }
    }))
}

/// 获取服务状态
pub async fn services_status(
    State(services): State<ServiceContainer>,
) -> Json<serde_json::Value> {
    let embedding_info = services.embedding_service.model_info();
    let rerank_info = services.rerank_service.model_info();
    let vector_db_info = services.vector_db.database_info();
    let llm_info = services.llm_service.model_info();
    
    Json(serde_json::json!({
        "embedding_service": {
            "name": embedding_info.name,
            "provider": embedding_info.provider,
            "version": embedding_info.version,
            "max_tokens": embedding_info.max_tokens
        },
        "rerank_service": {
            "name": rerank_info.name,
            "provider": rerank_info.provider,
            "version": rerank_info.version
        },
        "vector_database": {
            "name": vector_db_info.name,
            "version": vector_db_info.version,
            "supports_hybrid_search": vector_db_info.supports_hybrid_search
        },
        "llm_service": {
            "name": llm_info.name,
            "provider": llm_info.provider,
            "version": llm_info.version,
            "max_tokens": llm_info.max_tokens
        }
    }))
} 