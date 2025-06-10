# RAG工单处理系统 - 项目架构文档

## 📋 项目概述

**RAG工单处理系统** 是一个基于Rust的智能工单处理平台，采用检索增强生成(RAG)技术，为用户提供智能化的工单处理建议和解决方案。

### 🎯 核心特性

- **智能检索**: 基于向量相似度的语义检索历史工单
- **重排序优化**: 使用Qwen3-reranking提升检索精度
- **AI生成方案**: 基于历史案例生成个性化解决方案  
- **反馈学习**: 收集用户反馈，持续优化模型性能
- **模块化架构**: 支持多种AI服务提供商的无缝切换

## 🏗️ 系统架构

### 分层架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                      🌐 API Layer                          │
│                   (HTTP RESTful API)                       │
├─────────────────────────────────────────────────────────────┤
│                   🏢 Business Layer                        │
│              (工作流编排 & 业务逻辑)                          │
├─────────────────────────────────────────────────────────────┤
│                   🔧 Services Layer                        │
│        (AI服务 & 数据库 & 向量数据库 实现)                    │
├─────────────────────────────────────────────────────────────┤
│                   🧩 Core Layer                            │
│              (Traits抽象 & 数据模型)                         │
├─────────────────────────────────────────────────────────────┤
│                 🏗️ Infrastructure Layer                    │
│           (服务容器 & 配置管理 & 监控)                        │
└─────────────────────────────────────────────────────────────┘
```

## 📁 项目结构

### 目录组织

```
rag_001/
├── 📄 Cargo.toml                 # 工作空间配置
├── 📄 ARCHITECTURE.md             # 架构文档
├── 📄 bin/
│   └── server.rs                  # 主程序入口
├── 🔧 deps/                       # 依赖管理层
│   ├── Cargo.toml
│   └── src/lib.rs                 # 统一依赖重导出
├── 🧩 core/                       # 核心抽象层
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── traits/                # 服务抽象接口
│       │   ├── mod.rs
│       │   ├── embedding.rs       # 嵌入服务trait
│       │   ├── reranking.rs       # 重排序服务trait
│       │   ├── vector_db.rs       # 向量数据库trait
│       │   └── llm.rs             # LLM服务trait
│       ├── models/                # 数据模型
│       │   ├── mod.rs
│       │   ├── ticket.rs          # 工单模型
│       │   ├── solution.rs        # 解决方案模型
│       │   └── common.rs          # 通用模型
│       ├── config.rs              # 配置结构
│       └── errors.rs              # 错误处理
├── 🔧 services/                   # 服务实现层
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── embedding.rs           # 嵌入服务实现
│       ├── reranking.rs           # 重排序服务实现
│       ├── vector_db.rs           # 向量数据库实现
│       ├── llm.rs                 # LLM服务实现
│       └── database.rs            # 关系数据库实现
├── 🏢 business/                   # 业务逻辑层
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── processors.rs          # 工单处理器
│       ├── workflows.rs           # 工作流编排
│       └── validators.rs          # 业务验证
├── 🌐 api/                        # API接口层
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── server.rs              # HTTP服务器
│       ├── routes.rs              # 路由配置
│       ├── middleware.rs          # 中间件
│       ├── dto.rs                 # 数据传输对象
│       └── handlers/              # 请求处理器
│           ├── mod.rs
│           ├── tickets.rs         # 工单相关API
│           ├── solutions.rs       # 解决方案API
│           ├── search.rs          # 搜索API
│           ├── stats.rs           # 统计API
│           ├── admin.rs           # 管理API
│           ├── finetune.rs        # 微调API
│           └── root.rs            # 根路径API
└── 🏗️ infrastructure/             # 基础设施层
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── container.rs           # 服务容器
        ├── factory.rs             # 服务工厂
        ├── monitoring.rs          # 监控系统
        ├── logging.rs             # 日志配置
        └── configuration.rs       # 配置加载器
```

## 🔧 技术栈详细说明

### 核心技术选型

| 技术组件 | 选择方案 | 备选方案 | 说明 |
|---------|---------|---------|------|
| **编程语言** | Rust | Python, Go | 高性能、类型安全、内存安全 |
| **Web框架** | Axum | Actix-web, Warp | 现代异步、中间件丰富 |
| **嵌入模型** | Qwen3-embedding | OpenAI, 本地模型 | 中文优化、性能优秀 |
| **重排序模型** | Qwen3-reranking | Cohere, 本地模型 | 语义理解强、速度快 |
| **大语言模型** | Qwen3-turbo | GPT-4, 本地模型 | 推理能力强、成本优化 |
| **向量数据库** | Qdrant/PostgreSQL+pgvector | Pinecone, Weaviate | 开源、可扩展 |
| **关系数据库** | PostgreSQL | MySQL, SQLite | 功能完整、扩展性好 |
| **序列化** | serde + JSON | protobuf | 生态成熟、易调试 |

### AI服务架构

#### 多提供商支持策略
```rust
// Trait抽象层支持多个AI服务提供商
trait EmbeddingService {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
}

// 具体实现
impl EmbeddingService for QwenEmbeddingService { ... }
impl EmbeddingService for OpenAIEmbeddingService { ... }
impl EmbeddingService for LocalEmbeddingService { ... }
```

#### RAG工作流程
```mermaid
graph TB
    A[用户工单] --> B[文本预处理]
    B --> C[向量化/Embedding]
    C --> D[向量相似度检索]
    D --> E[候选工单Top-100]
    E --> F[Rerank重排序]
    F --> G[精选工单Top-10]
    G --> H[LLM推理生成]
    H --> I[解决方案+置信度]
    I --> J[用户反馈收集]
    J --> K[训练数据生成]
    K --> L[模型微调优化]
```

## 🗂️ 核心组件详解

### 1. 📦 依赖管理层 (`deps/`)

**职责**: 统一管理和重导出所有依赖包

**核心特性**:
- 工作空间级别的依赖版本统一
- 避免版本冲突和重复依赖
- 简化其他模块的import路径

### 2. 🧩 核心抽象层 (`core/`)

**职责**: 定义系统的核心抽象和数据模型

**主要组件**:
- **Traits**: 定义各种服务的统一接口
- **Models**: 工单、解决方案等核心数据结构
- **Config**: 应用配置管理
- **Errors**: 统一错误处理

**设计原则**:
- 面向接口编程，实现解耦
- 类型安全，编译时错误检查
- 扩展性强，支持新服务类型

### 3. 🔧 服务实现层 (`services/`)

**职责**: 实现具体的AI服务和数据存储服务

**服务清单**:
| 服务类型 | 实现方案 | 说明 |
|---------|---------|------|
| **嵌入服务** | QwenEmbedding, OpenAI | 文本向量化 |
| **重排序服务** | QwenRerank, Cohere | 语义相关性排序 |
| **向量数据库** | SQLite+vec0, Qdrant, PostgreSQL+pgvector | 向量存储检索 |
| **LLM服务** | Qwen, OpenAI, 本地模型 | 生成式AI推理 |
| **关系数据库** | PostgreSQL | 结构化数据存储 |

### 4. 🏢 业务逻辑层 (`business/`)

**职责**: 编排业务流程，实现核心业务逻辑

**核心组件**:
- **TicketProcessor**: RAG工单处理编排器
- **WorkflowEngine**: 工作流状态管理
- **ValidationEngine**: 业务规则验证

**处理流程**:
1. **输入验证** → 数据格式和业务规则检查
2. **向量化处理** → 调用嵌入服务
3. **相似度检索** → 向量数据库查询
4. **重排序优化** → 语义相关性重新排序
5. **AI推理生成** → LLM生成解决方案
6. **结果封装** → 统一格式返回

### 5. 🌐 API接口层 (`api/`)

**职责**: 提供HTTP RESTful API服务

**API分组**:
```
/api/v1/
├── 📝 tickets/          # 工单管理
├── 🔍 search/           # 搜索服务  
├── 💡 solutions/        # 解决方案
├── 📊 stats/            # 统计分析
├── ⚙️ admin/            # 系统管理
└── 🎯 finetune/         # 模型微调
```

**核心特性**:
- RESTful设计规范
- 统一错误处理和响应格式
- 请求限流和安全认证
- API文档自动生成

### 6. 🏗️ 基础设施层 (`infrastructure/`)

**职责**: 提供横切关注点和基础设施服务

**核心组件**:
- **ServiceContainer**: 依赖注入容器
- **ServiceFactory**: 服务实例工厂
- **MetricsCollector**: 系统指标收集
- **ConfigurationLoader**: 配置加载管理

## 🚀 部署架构

### 环境配置

#### 开发环境
```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgresql://user:pass@localhost/rag_dev"

[vector_db]
provider = "sqlite"
connection_string = "./dev_vectors.db"

[embedding]
provider = "qwen"
model = "text-embedding-v1"
```

#### 生产环境
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 8

[database]
url = "postgresql://user:pass@db-cluster/rag_prod"
max_connections = 20

[vector_db]
provider = "qdrant"
connection_string = "http://qdrant-cluster:6333"

[embedding]
provider = "qwen"
model = "text-embedding-v2"
```

### 容器化部署

```dockerfile
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/rag-server /usr/local/bin/
CMD ["rag-server"]
```

## 📊 性能优化策略

### 1. 缓存策略
- **嵌入向量缓存**: Redis缓存常用文本的向量结果
- **检索结果缓存**: 缓存热点查询的候选工单
- **配置缓存**: 应用级配置热加载

### 2. 并发优化
- **异步I/O**: 全链路async/await架构
- **连接池**: 数据库和HTTP客户端连接复用
- **批处理**: 向量化和推理批量处理

### 3. 扩展性设计
- **水平扩展**: 无状态服务支持多实例部署
- **负载均衡**: API网关分发请求
- **数据分片**: 大规模向量数据分片存储

## 🔒 安全设计

### 认证授权
- **API密钥**: 服务级别的访问控制
- **JWT令牌**: 用户会话管理
- **权限分级**: 不同角色的操作权限

### 数据安全
- **敏感数据加密**: 工单内容和用户信息加密存储
- **传输加密**: HTTPS/TLS全链路加密
- **审计日志**: 操作轨迹完整记录

## 📈 监控体系

### 系统监控
- **性能指标**: QPS、延迟、错误率
- **资源监控**: CPU、内存、网络、存储
- **业务指标**: 工单处理成功率、用户满意度

### 告警机制
- **阈值告警**: 指标异常自动报警
- **链路追踪**: 分布式请求跟踪
- **日志聚合**: 集中化日志分析

## 🔄 持续集成/持续部署

### CI/CD流水线
```yaml
stages:
  - 🧪 test        # 单元测试 + 集成测试
  - 🔍 lint        # 代码质量检查
  - 🏗️ build       # 编译构建
  - 📦 package     # 容器镜像构建
  - 🚀 deploy      # 自动化部署
  - ✅ verify      # 部署验证
```

### 质量保证
- **单元测试覆盖率 > 80%**
- **集成测试覆盖关键业务流程**
- **性能基准测试**
- **安全漏洞扫描**

## 🎯 未来规划

### 短期目标 (1-3个月)
- [ ] 完成API层实现和测试
- [ ] 添加完整的配置文件示例
- [ ] 实现基础的Web管理界面
- [ ] 完善监控和日志系统

### 中期目标 (3-6个月)
- [ ] 支持更多AI服务提供商
- [ ] 实现模型微调功能
- [ ] 添加A/B测试框架
- [ ] 支持多租户架构

### 长期目标 (6-12个月)
- [ ] 智能工单路由和分派
- [ ] 自动化解决方案执行
- [ ] 知识图谱增强检索
- [ ] 多模态内容支持(图片、文档)

---

## 📚 开发指南

### 快速开始
```bash
# 克隆项目
git clone <repository-url>
cd rag_001

# 环境检查
cargo check

# 运行开发服务器
cargo run --bin rag-server

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 贡献指南
1. Fork项目仓库
2. 创建功能分支: `git checkout -b feature/your-feature`
3. 提交代码: `git commit -am 'Add some feature'`
4. 推送分支: `git push origin feature/your-feature`
5. 创建Pull Request

### 文档更新
- 架构变更需同步更新此文档
- API变更需更新接口文档
- 新增依赖需更新说明文档

---

*这个架构文档会随着项目的发展持续更新，确保与实际实现保持同步。* 