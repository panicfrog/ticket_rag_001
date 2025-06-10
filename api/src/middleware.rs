//! # 中间件
//! 
//! 提供HTTP请求的中间件功能

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use rag_deps::*;

/// 请求日志中间件
pub async fn request_logging(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    log::info!("Started {} {}", method, uri);
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    log::info!(
        "Completed {} {} {} in {:?}",
        method,
        uri,
        status,
        duration
    );
    
    response
}

/// 错误处理中间件
pub async fn error_handling(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    
    // 如果是错误状态码，记录更多信息
    if response.status().is_server_error() {
        log::error!("Server error: {}", response.status());
    } else if response.status().is_client_error() {
        log::warn!("Client error: {}", response.status());
    }
    
    response
} 