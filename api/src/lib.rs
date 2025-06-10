//! # API层
//! 
//! 提供HTTP REST API接口，处理客户端请求

pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod dto;
pub mod server;

// 重新导出核心组件
pub use server::ApiServer;
pub use routes::create_app_router; 