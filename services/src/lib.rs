//! # 服务实现层
//! 
//! 实现core层定义的各种服务trait
//! 主要职责：
//! - 提供Embedding服务的具体实现
//! - 提供Reranking服务的具体实现
//! - 提供向量数据库的具体实现
//! - 提供LLM服务的具体实现
//! - 提供关系数据库访问服务

pub mod embedding;
pub mod reranking;
pub mod vector_db;
pub mod llm;
pub mod database; 