//! # API路由配置
//! 
//! 定义所有的HTTP路由和端点

use rag_deps::*;
use rag_infrastructure::ServiceContainer;
use crate::{handlers, middleware};
use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware as axum_middleware,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
};
use std::time::Duration;

/// 创建应用路由器
/// 
/// 集成所有路由、中间件和服务依赖
pub async fn create_app_router(service_container: ServiceContainer) -> Result<Router> {
    info!("正在配置API路由...");
    
    // 中间件栈
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(axum_middleware::from_fn(middleware::request_logging))
        .layer(axum_middleware::from_fn(middleware::error_handling));
    
    // API路由
    let api_routes = Router::new()
        // 工单相关路由
        .route("/tickets", post(handlers::tickets::create_ticket))
        .route("/tickets", get(handlers::tickets::list_tickets))
        .route("/tickets/:id", get(handlers::tickets::get_ticket))
        .route("/tickets/:id", put(handlers::tickets::update_ticket))
        .route("/tickets/:id", delete(handlers::tickets::delete_ticket))
        .route("/tickets/:id/process", post(handlers::tickets::process_ticket))
        .route("/tickets/:id/solutions", get(handlers::tickets::get_solutions))
        
        // 解决方案相关路由
        .route("/solutions/:id/feedback", post(handlers::solutions::submit_feedback))
        .route("/solutions/:id/accept", post(handlers::solutions::accept_solution))
        .route("/solutions/:id/reject", post(handlers::solutions::reject_solution))
        
        // 搜索相关路由
        .route("/search/tickets", post(handlers::search::search_tickets))
        .route("/search/similar", post(handlers::search::find_similar))
        
        // 统计和分析路由
        .route("/stats/overview", get(handlers::stats::get_overview))
        .route("/stats/performance", get(handlers::stats::get_performance))
        .route("/stats/quality", get(handlers::stats::get_quality))
        
        // 系统管理路由
        .route("/admin/health", get(handlers::admin::health_check))
        .route("/admin/metrics", get(handlers::admin::get_metrics))
        .route("/admin/config", get(handlers::admin::get_config))
        .route("/admin/services/status", get(handlers::admin::services_status))
        
        // 微调相关路由
        .route("/finetune/data/export", get(handlers::finetune::export_data))
        .route("/finetune/jobs", post(handlers::finetune::start_job))
        .route("/finetune/jobs", get(handlers::finetune::list_jobs))
        .route("/finetune/jobs/:id", get(handlers::finetune::get_job_status));
    
    // 根路由
    let app = Router::new()
        .route("/", get(handlers::root::welcome))
        .route("/docs", get(handlers::root::api_docs))
        .route("/version", get(handlers::root::version))
        .nest("/api/v1", api_routes)
        .layer(middleware_stack)
        .with_state(service_container);
    
    info!("API路由配置完成");
    Ok(app)
} 