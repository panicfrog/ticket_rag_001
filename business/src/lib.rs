//! # 业务逻辑层
//! 
//! 实现核心业务逻辑和工作流编排
//! 主要职责：
//! - 编排RAG工单处理流程
//! - 实现业务规则和验证
//! - 管理数据流转
//! - 提供业务级别的API

pub mod processors;
pub mod workflows;
pub mod validators; 