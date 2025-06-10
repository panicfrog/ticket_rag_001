//! # Trait定义模块
//! 
//! 定义系统中各个服务的抽象接口

pub mod embedding;
pub mod reranking;
pub mod vector_db;
pub mod llm;

// 重新导出主要trait
pub use embedding::EmbeddingService;
pub use reranking::RerankService;
pub use vector_db::VectorDatabase;
pub use llm::LLMService; 