//! # 服务工厂
//! 
//! 根据配置创建具体的服务实例

use rag_deps::*;
use rag_core::{config::*, traits::*, errors::AppError};
use rag_services::{
    embedding::{QwenEmbeddingService},
    reranking::{QwenRerankService},
    vector_db::{SqliteVectorDB, QdrantVectorDB, PostgresVectorDB},
    llm::{QwenLLMService},
    database::PostgresDatabase,
};
use crate::container::ServiceContainer;
use std::sync::Arc;

/// 服务工厂
/// 
/// 职责：
/// - 根据配置创建服务实例
/// - 处理服务初始化
/// - 管理服务依赖关系
pub struct ServiceFactory;

impl ServiceFactory {
    /// 创建完整的服务容器
    pub async fn create_service_container(config: &AppConfig) -> Result<ServiceContainer> {
        info!("开始创建服务容器...");
        
        // 创建各个服务
        let embedding_service = Self::create_embedding_service(&config.embedding).await?;
        let rerank_service = Self::create_rerank_service(&config.reranking).await?;
        let vector_db = Self::create_vector_database(&config.vector_db).await?;
        let llm_service = Self::create_llm_service(&config.llm).await?;
        let database = Self::create_database(&config.database).await?;
        
        // 创建服务容器
        let container = ServiceContainer::new(
            embedding_service,
            rerank_service,
            vector_db,
            llm_service,
            database,
        );
        
        info!("服务容器创建完成");
        Ok(container)
    }
    
    /// 创建嵌入服务
    pub async fn create_embedding_service(
        config: &EmbeddingConfig,
    ) -> Result<Arc<dyn EmbeddingService + Send + Sync>> {
        info!("创建嵌入服务: {}", config.provider);
        
        match config.provider.as_str() {
            "qwen" => {
                let service = QwenEmbeddingService::new(
                    config.api_key.clone(),
                    config.model.clone(),
                    config.dimension,
                    config.endpoint.clone(),
                    100, // 默认批量大小
                );
                Ok(Arc::new(service))
            }
            _ => Err(AppError::Configuration {
                message: format!("不支持的嵌入服务提供商: {}", config.provider),
            }.into()),
        }
    }
    
    /// 创建重排序服务
    pub async fn create_rerank_service(
        config: &RerankingConfig,
    ) -> Result<Arc<dyn RerankService + Send + Sync>> {
        info!("创建重排序服务: {}", config.provider);
        
        match config.provider.as_str() {
            "qwen" => {
                let service = QwenRerankService::new(
                    config.api_key.clone(),
                    config.model.clone(),
                    config.endpoint.clone(),
                );
                Ok(Arc::new(service))
            }
            _ => Err(AppError::Configuration {
                message: format!("不支持的重排序服务提供商: {}", config.provider),
            }.into()),
        }
    }
    
    /// 创建向量数据库
    pub async fn create_vector_database(
        config: &VectorDbConfig,
    ) -> Result<Arc<dyn VectorDatabase + Send + Sync>> {
        info!("创建向量数据库: {}", config.provider);
        
        match config.provider.as_str() {
            "sqlite" => {
                let db = SqliteVectorDB::new(
                    &config.connection_string,
                    config.dimension,
                ).await?;
                Ok(Arc::new(db))
            }
            "qdrant" => {
                let db = QdrantVectorDB::new(
                    &config.connection_string,
                    "tickets".to_string(), // 默认集合名
                    config.dimension,
                ).await?;
                Ok(Arc::new(db))
            }
            "postgres" => {
                let db = PostgresVectorDB::new(
                    &config.connection_string,
                    "ticket_vectors".to_string(), // 默认表名
                    config.dimension,
                ).await?;
                Ok(Arc::new(db))
            }
            _ => Err(AppError::Configuration {
                message: format!("不支持的向量数据库提供商: {}", config.provider),
            }.into()),
        }
    }
    
    /// 创建大语言模型服务
    pub async fn create_llm_service(
        config: &LLMConfig,
    ) -> Result<Arc<dyn LLMService + Send + Sync>> {
        info!("创建LLM服务: {}", config.provider);
        
        match config.provider.as_str() {
            "qwen" => {
                let service = QwenLLMService::new(
                    config.api_key.clone(),
                    config.model.clone(),
                    config.endpoint.clone(),
                    4096, // 默认最大token
                    0.7, // 默认温度
                );
                Ok(Arc::new(service))
            }
            _ => Err(AppError::Configuration {
                message: format!("不支持的LLM服务提供商: {}", config.provider),
            }.into()),
        }
    }
    
    /// 创建数据库服务
    pub async fn create_database(
        config: &DatabaseConfig,
    ) -> Result<Arc<PostgresDatabase>> {
        info!("创建数据库连接: {}", config.url);
        
        let database = PostgresDatabase::new(&config.url).await?;
        Ok(Arc::new(database))
    }
} 