//! # 向量数据库抽象接口
//! 
//! 定义向量数据库的统一接口，支持多种向量数据库实现

use rag_deps::*;

/// 向量记录
#[derive(Debug, Clone)]
pub struct VectorRecord {
    pub id: Uuid,
    pub vector: Vec<f32>,
    pub metadata: VectorMetadata,
}

/// 向量元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMetadata {
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: Uuid,
    pub score: f32,
    pub metadata: VectorMetadata,
    pub vector: Option<Vec<f32>>,
}

/// 向量过滤器
#[derive(Debug, Clone)]
pub struct VectorFilter {
    pub category: Option<String>,
    pub priority_range: Option<(i32, i32)>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub tags: Option<Vec<String>>,
}

/// 数据库统计信息
#[derive(Debug)]
pub struct DatabaseStats {
    pub total_vectors: u64,
    pub dimension: usize,
    pub storage_size: u64,
    pub index_type: String,
}

/// 数据库信息
#[derive(Debug, Clone)]
pub struct DatabaseInfo {
    pub name: String,
    pub version: String,
    pub supports_hybrid_search: bool,
    pub supports_filtering: bool,
    pub max_dimension: usize,
    pub recommended_batch_size: usize,
}

/// 向量数据库trait
/// 
/// 职责：
/// - 存储和检索向量数据
/// - 支持向量相似度搜索
/// - 支持元数据过滤
/// - 提供批量操作能力
#[async_trait]
pub trait VectorDatabase: Send + Sync {
    /// 插入单个向量
    async fn insert(&self, id: Uuid, vector: &[f32], metadata: VectorMetadata) -> Result<()>;
    
    /// 批量插入向量
    async fn insert_batch(&self, records: &[VectorRecord]) -> Result<()>;
    
    /// 向量相似度搜索
    async fn search(
        &self, 
        query_vector: &[f32], 
        limit: usize, 
        filter: Option<VectorFilter>
    ) -> Result<Vec<SearchResult>>;
    
    /// 混合搜索（向量 + 关键词）
    async fn hybrid_search(
        &self, 
        query_vector: &[f32], 
        keywords: &[String], 
        limit: usize, 
        filter: Option<VectorFilter>
    ) -> Result<Vec<SearchResult>>;
    
    /// 删除向量
    async fn delete(&self, id: Uuid) -> Result<()>;
    
    /// 更新向量
    async fn update(&self, id: Uuid, vector: &[f32], metadata: Option<VectorMetadata>) -> Result<()>;
    
    /// 获取向量
    async fn get(&self, id: Uuid) -> Result<Option<VectorRecord>>;
    
    /// 统计信息
    async fn stats(&self) -> Result<DatabaseStats>;
    
    /// 健康检查
    async fn health_check(&self) -> Result<bool>;
    
    /// 获取数据库信息
    fn database_info(&self) -> DatabaseInfo;
} 