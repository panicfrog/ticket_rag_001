//! # 核心抽象层
//! 
//! 定义系统的核心抽象接口和数据模型
//! 主要职责：
//! - 定义各种服务的trait接口
//! - 定义核心数据模型
//! - 定义统一的错误类型
//! - 定义配置结构

pub mod traits;
pub mod models;
pub mod errors;
pub mod config; 