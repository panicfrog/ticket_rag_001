//! # 依赖管理层
//! 
//! 统一管理项目中所有通用依赖，提供一致的接口供其他层使用
//! 主要职责：
//! - 重新导出常用的第三方库
//! - 提供版本统一管理
//! - 简化其他crate的依赖声明

// 重新导出核心依赖
pub use tokio;
pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use uuid::{Uuid};
pub use chrono::{DateTime, Utc};
pub use anyhow::{Result, Error as AnyhowError};
pub use thiserror::Error;
pub use async_trait::async_trait;
pub use log::{debug, info, warn, error};
pub use toml;

// 提供常用类型别名
pub type DatabaseResult<T> = Result<T>;
pub type ServiceResult<T> = Result<T>; 