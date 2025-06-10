//! # 监控和指标收集
//! 
//! 提供系统监控、指标收集和性能分析功能

use rag_deps::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// 指标收集器
/// 
/// 职责：
/// - 收集系统运行指标
/// - 提供性能监控数据
/// - 支持指标导出
pub struct MetricsCollector {
    request_count: AtomicU64,
    error_count: AtomicU64,
    processing_time_total: AtomicU64,
    embedding_calls: AtomicU64,
    rerank_calls: AtomicU64,
    llm_calls: AtomicU64,
    vector_searches: AtomicU64,
}

impl MetricsCollector {
    /// 创建新的指标收集器
    pub fn new() -> Self {
        Self {
            request_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            processing_time_total: AtomicU64::new(0),
            embedding_calls: AtomicU64::new(0),
            rerank_calls: AtomicU64::new(0),
            llm_calls: AtomicU64::new(0),
            vector_searches: AtomicU64::new(0),
        }
    }
    
    /// 记录请求
    pub fn record_request(&self, processing_time_ms: u64) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.processing_time_total.fetch_add(processing_time_ms, Ordering::Relaxed);
    }
    
    /// 记录错误
    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }
    
    /// 记录嵌入调用
    pub fn record_embedding_call(&self) {
        self.embedding_calls.fetch_add(1, Ordering::Relaxed);
    }
    
    /// 记录重排序调用
    pub fn record_rerank_call(&self) {
        self.rerank_calls.fetch_add(1, Ordering::Relaxed);
    }
    
    /// 记录LLM调用
    pub fn record_llm_call(&self) {
        self.llm_calls.fetch_add(1, Ordering::Relaxed);
    }
    
    /// 记录向量搜索
    pub fn record_vector_search(&self) {
        self.vector_searches.fetch_add(1, Ordering::Relaxed);
    }
    
    /// 获取当前指标
    pub fn get_metrics(&self) -> SystemMetrics {
        let request_count = self.request_count.load(Ordering::Relaxed);
        let error_count = self.error_count.load(Ordering::Relaxed);
        let processing_time_total = self.processing_time_total.load(Ordering::Relaxed);
        
        let average_processing_time = if request_count > 0 {
            processing_time_total as f64 / request_count as f64
        } else {
            0.0
        };
        
        let error_rate = if request_count > 0 {
            error_count as f64 / request_count as f64
        } else {
            0.0
        };
        
        SystemMetrics {
            request_count,
            error_count,
            error_rate,
            average_processing_time_ms: average_processing_time,
            embedding_calls: self.embedding_calls.load(Ordering::Relaxed),
            rerank_calls: self.rerank_calls.load(Ordering::Relaxed),
            llm_calls: self.llm_calls.load(Ordering::Relaxed),
            vector_searches: self.vector_searches.load(Ordering::Relaxed),
            timestamp: Utc::now(),
        }
    }
    
    /// 重置指标
    pub fn reset(&self) {
        self.request_count.store(0, Ordering::Relaxed);
        self.error_count.store(0, Ordering::Relaxed);
        self.processing_time_total.store(0, Ordering::Relaxed);
        self.embedding_calls.store(0, Ordering::Relaxed);
        self.rerank_calls.store(0, Ordering::Relaxed);
        self.llm_calls.store(0, Ordering::Relaxed);
        self.vector_searches.store(0, Ordering::Relaxed);
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// 系统指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub request_count: u64,
    pub error_count: u64,
    pub error_rate: f64,
    pub average_processing_time_ms: f64,
    pub embedding_calls: u64,
    pub rerank_calls: u64,
    pub llm_calls: u64,
    pub vector_searches: u64,
    pub timestamp: DateTime<Utc>,
}

/// 全局指标收集器实例
static GLOBAL_METRICS: once_cell::sync::Lazy<Arc<MetricsCollector>> = 
    once_cell::sync::Lazy::new(|| Arc::new(MetricsCollector::new()));

/// 获取全局指标收集器
pub fn get_metrics_collector() -> Arc<MetricsCollector> {
    GLOBAL_METRICS.clone()
} 