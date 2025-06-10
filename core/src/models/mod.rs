//! # 数据模型模块
//! 
//! 定义系统中的核心数据结构

pub mod ticket;
pub mod solution;
pub mod common;

// 重新导出主要模型
pub use ticket::*;
pub use solution::*;
pub use common::*; 