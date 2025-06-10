//! # API服务器
//! 
//! 负责启动和管理HTTP服务器

use rag_deps::*;
use rag_core::{config::AppConfig, errors::AppError};
use rag_business::processors::TicketProcessor;
use rag_infrastructure::ServiceContainer;
use crate::routes::create_app_router;

/// API服务器
/// 
/// 职责：
/// - 管理HTTP服务器生命周期
/// - 处理优雅关闭
/// - 集成中间件
pub struct ApiServer {
    config: AppConfig,
    service_container: ServiceContainer,
}

impl ApiServer {
    /// 创建新的API服务器实例
    pub fn new(config: AppConfig, service_container: ServiceContainer) -> Self {
        Self {
            config,
            service_container,
        }
    }
    
    /// 启动API服务器
    pub async fn start(self) -> Result<()> {
        info!("正在启动API服务器...");
        
        // 创建应用路由
        let app = create_app_router(self.service_container).await?;
        
        // 创建监听器
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        let listener = tokio::net::TcpListener::bind(&addr).await
            .map_err(|e| AppError::Internal { 
                message: format!("无法绑定到地址 {}: {}", addr, e) 
            })?;
        
        info!("API服务器启动在: {}", addr);
        info!("API文档地址: http://{}/docs", addr);
        
        // 启动服务器
        axum::serve(listener, app).await
            .map_err(|e| AppError::Internal { 
                message: format!("服务器运行错误: {}", e) 
            })?;
        
        Ok(())
    }
} 