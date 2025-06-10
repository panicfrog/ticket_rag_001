//! # 基础设施层
//! 
//! 提供跨领域的基础设施服务和工具

pub mod container;
pub mod factory;
pub mod monitoring;
pub mod logging;
pub mod configuration;

// 重新导出核心组件
pub use container::ServiceContainer;
pub use factory::ServiceFactory; 