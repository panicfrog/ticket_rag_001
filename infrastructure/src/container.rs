//! # 服务容器
//! 
//! 管理所有服务实例的依赖注入容器

use rag_deps::*;
use rag_core::traits::*;
use rag_business::processors::TicketProcessor;
use rag_services::database::PostgresDatabase;
use std::sync::Arc;

/// 服务容器
/// 
/// 职责：
/// - 管理所有服务的生命周期
/// - 提供依赖注入
/// - 确保线程安全的服务访问
#[derive(Clone)]
pub struct ServiceContainer {
    pub embedding_service: Arc<dyn EmbeddingService + Send + Sync>,
    pub rerank_service: Arc<dyn RerankService + Send + Sync>,
    pub vector_db: Arc<dyn VectorDatabase + Send + Sync>,
    pub llm_service: Arc<dyn LLMService + Send + Sync>,
    pub database: Arc<PostgresDatabase>,
    pub ticket_processor: Arc<TicketProcessor>,
}

impl ServiceContainer {
    /// 创建新的服务容器
    pub fn new(
        embedding_service: Arc<dyn EmbeddingService + Send + Sync>,
        rerank_service: Arc<dyn RerankService + Send + Sync>,
        vector_db: Arc<dyn VectorDatabase + Send + Sync>,
        llm_service: Arc<dyn LLMService + Send + Sync>,
        database: Arc<PostgresDatabase>,
    ) -> Self {
        // 创建工单处理器，注入所需依赖
        let ticket_processor = Arc::new(TicketProcessor::new(
            embedding_service.clone(),
            rerank_service.clone(),
            vector_db.clone(),
            llm_service.clone(),
        ));
        
        Self {
            embedding_service,
            rerank_service,
            vector_db,
            llm_service,
            database,
            ticket_processor,
        }
    }
    
    /// 健康检查所有服务
    pub async fn health_check(&self) -> Result<HealthStatus> {
        let embedding_health = self.embedding_service.health_check().await.unwrap_or(false);
        let rerank_health = self.rerank_service.health_check().await.unwrap_or(false);
        let vector_db_health = self.vector_db.health_check().await.unwrap_or(false);
        let llm_health = self.llm_service.health_check().await.unwrap_or(false);
        
        let overall_healthy = embedding_health && rerank_health && vector_db_health && llm_health;
        
        Ok(HealthStatus {
            overall: overall_healthy,
            embedding_service: embedding_health,
            rerank_service: rerank_health,
            vector_database: vector_db_health,
            llm_service: llm_health,
            timestamp: Utc::now(),
        })
    }
    
    /// 获取服务统计信息
    pub async fn get_stats(&self) -> Result<ServiceStats> {
        // 这里可以收集各种统计信息
        Ok(ServiceStats {
            embedding_calls: 0,
            rerank_calls: 0,
            vector_searches: 0,
            llm_generations: 0,
            average_processing_time_ms: 0.0,
            success_rate: 1.0,
        })
    }
}

/// 健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub overall: bool,
    pub embedding_service: bool,
    pub rerank_service: bool,
    pub vector_database: bool,
    pub llm_service: bool,
    pub timestamp: DateTime<Utc>,
}

/// 服务统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStats {
    pub embedding_calls: u64,
    pub rerank_calls: u64,
    pub vector_searches: u64,
    pub llm_generations: u64,
    pub average_processing_time_ms: f64,
    pub success_rate: f64,
} 