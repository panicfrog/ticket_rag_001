#!/bin/bash
# RAG工单处理系统 - 快速设置脚本

set -e

echo "🚀 RAG工单处理系统 - 开发环境设置"
echo "=================================="

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 检查操作系统
check_os() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo -e "${GREEN}✅ 检测到 macOS 系统${NC}"
        OS="macos"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo -e "${GREEN}✅ 检测到 Linux 系统${NC}"
        OS="linux"
    else
        echo -e "${RED}❌ 不支持的操作系统: $OSTYPE${NC}"
        exit 1
    fi
}

# 检查依赖
check_dependencies() {
    echo -e "\n${BLUE}🔍 检查依赖...${NC}"
    
    # 检查 Docker
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker 未安装${NC}"
        echo "请先安装 Docker: https://docs.docker.com/get-docker/"
        exit 1
    else
        echo -e "${GREEN}✅ Docker 已安装${NC}"
    fi
    
    # 检查 Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        echo -e "${RED}❌ Docker Compose 未安装${NC}"
        echo "请先安装 Docker Compose"
        exit 1
    else
        echo -e "${GREEN}✅ Docker Compose 已安装${NC}"
    fi
    
    # 检查 Ollama (macOS)
    if [[ "$OS" == "macos" ]]; then
        if ! command -v ollama &> /dev/null; then
            echo -e "${YELLOW}⚠️  Ollama 未安装，将自动安装...${NC}"
            install_ollama_mac
        else
            echo -e "${GREEN}✅ Ollama 已安装${NC}"
        fi
    fi
}

# 安装 Ollama (macOS)
install_ollama_mac() {
    echo -e "\n${BLUE}📦 安装 Ollama...${NC}"
    
    if command -v brew &> /dev/null; then
        brew install ollama
    else
        curl -fsSL https://ollama.ai/install.sh | sh
    fi
    
    # 启动 Ollama 服务
    echo -e "${BLUE}🚀 启动 Ollama 服务...${NC}"
    ollama serve &
    sleep 5
}

# 部署远程服务
deploy_remote_services() {
    echo -e "\n${BLUE}🐳 部署远程服务 (PostgreSQL + Qdrant + Redis)...${NC}"
    
    # 创建必要的目录
    mkdir -p init-scripts
    mkdir -p qdrant-config
    mkdir -p redis-config
    mkdir -p nginx-config/conf.d
    mkdir -p prometheus-config
    mkdir -p grafana-config/provisioning
    
    # 创建 Redis 配置
    cat > redis-config/redis.conf << EOF
# Redis 配置文件
bind 0.0.0.0
port 6379
timeout 300
tcp-keepalive 300
daemonize no
supervised no
pidfile /var/run/redis_6379.pid
loglevel notice
logfile ""
databases 16
save 900 1
save 300 10
save 60 10000
stop-writes-on-bgsave-error yes
rdbcompression yes
rdbchecksum yes
dbfilename dump.rdb
dir ./
maxmemory 2gb
maxmemory-policy allkeys-lru
EOF
    
    # 创建 Qdrant 配置
    cat > qdrant-config/production.yaml << EOF
service:
  host: 0.0.0.0
  http_port: 6333
  grpc_port: 6334

storage:
  storage_path: /qdrant/storage

log_level: INFO

# 性能优化
hnsw_config:
  m: 16
  ef_construct: 100

quantization:
  scalar:
    type: int8
    quantile: 0.99
    always_ram: true
EOF
    
    # 启动服务
    echo -e "${BLUE}🚀 启动 Docker Compose 服务...${NC}"
    docker-compose up -d postgres qdrant redis
    
    # 等待服务启动
    echo -e "${BLUE}⏳ 等待服务启动...${NC}"
    sleep 15
    
    # 检查服务状态
    check_remote_services
}

# 检查远程服务状态
check_remote_services() {
    echo -e "\n${BLUE}🔍 检查远程服务状态...${NC}"
    
    # 检查 PostgreSQL
    if docker-compose exec -T postgres pg_isready -U rag_user -d rag_tickets; then
        echo -e "${GREEN}✅ PostgreSQL 服务正常${NC}"
    else
        echo -e "${RED}❌ PostgreSQL 服务异常${NC}"
    fi
    
    # 检查 Qdrant
    if curl -f http://localhost:6333/health &> /dev/null; then
        echo -e "${GREEN}✅ Qdrant 服务正常${NC}"
    else
        echo -e "${RED}❌ Qdrant 服务异常${NC}"
    fi
    
    # 检查 Redis
    if docker-compose exec -T redis redis-cli ping | grep -q PONG; then
        echo -e "${GREEN}✅ Redis 服务正常${NC}"
    else
        echo -e "${RED}❌ Redis 服务异常${NC}"
    fi
}

# 部署本地模型
deploy_local_models() {
    echo -e "\n${BLUE}🤖 部署本地模型...${NC}"
    
    # 部署 Embedding 模型
    echo -e "${BLUE}📥 拉取 qwen3-embedding 模型...${NC}"
    ollama pull qwen3-embedding
    
    # 部署主要 LLM 模型 (7B)
    echo -e "${BLUE}📥 拉取 qwen3:7b 模型...${NC}"
    ollama pull qwen3:7b
    
    # 部署微调模型 (1.7B)
    echo -e "${BLUE}📥 拉取 qwen3:1.7b 模型...${NC}"
    ollama pull qwen3:1.7b
    
    # 测试模型
    test_local_models
}

# 测试本地模型
test_local_models() {
    echo -e "\n${BLUE}🧪 测试本地模型...${NC}"
    
    # 测试 Embedding 模型
    echo -e "${BLUE}测试 Embedding 模型...${NC}"
    curl -X POST http://localhost:11434/api/embeddings \
        -H "Content-Type: application/json" \
        -d '{"model": "qwen3-embedding", "prompt": "测试文本"}' \
        &> /dev/null && echo -e "${GREEN}✅ Embedding 模型正常${NC}" || echo -e "${RED}❌ Embedding 模型异常${NC}"
    
    # 测试主要 LLM 模型
    echo -e "${BLUE}测试 qwen3:7b 模型...${NC}"
    curl -X POST http://localhost:11434/api/generate \
        -H "Content-Type: application/json" \
        -d '{"model": "qwen3:7b", "prompt": "你好", "stream": false}' \
        &> /dev/null && echo -e "${GREEN}✅ qwen3:7b 模型正常${NC}" || echo -e "${RED}❌ qwen3:7b 模型异常${NC}"
    
    # 测试微调模型
    echo -e "${BLUE}测试 qwen3:1.7b 模型...${NC}"
    curl -X POST http://localhost:11434/api/generate \
        -H "Content-Type: application/json" \
        -d '{"model": "qwen3:1.7b", "prompt": "分类任务测试", "stream": false}' \
        &> /dev/null && echo -e "${GREEN}✅ qwen3:1.7b 模型正常${NC}" || echo -e "${RED}❌ qwen3:1.7b 模型异常${NC}"
}

# 创建测试数据
setup_test_data() {
    echo -e "\n${BLUE}🗄️  创建测试数据...${NC}"
    
    # 创建数据库表
    docker-compose exec -T postgres psql -U rag_user -d rag_tickets << EOF
-- 创建工单表
CREATE TABLE IF NOT EXISTS tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    priority VARCHAR(50),
    status VARCHAR(50) DEFAULT 'open',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建向量表
CREATE TABLE IF NOT EXISTS ticket_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID REFERENCES tickets(id),
    vector_data FLOAT8[],
    embedding_model VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 插入测试数据
INSERT INTO tickets (title, description, category, priority) VALUES
('网络连接问题', '用户反馈无法连接到公司VPN', 'network', 'high'),
('软件安装失败', 'Office 365安装过程中出现错误代码0x80070005', 'software', 'medium'),
('打印机故障', '三楼的HP打印机无法正常打印，显示缺纸错误', 'hardware', 'low'),
('邮箱问题', '收不到外部邮件，内部邮件正常', 'email', 'high'),
('系统性能慢', '电脑开机和运行程序都很慢，怀疑是硬盘问题', 'performance', 'medium');

COMMIT;
EOF
    
    # 在 Qdrant 中创建集合
    curl -X PUT "http://localhost:6333/collections/tickets" \
        -H "Content-Type: application/json" \
        -d '{
            "vectors": {
                "size": 1024,
                "distance": "Cosine"
            }
        }' &> /dev/null && echo -e "${GREEN}✅ Qdrant 集合创建成功${NC}" || echo -e "${RED}❌ Qdrant 集合创建失败${NC}"
}

# 显示配置信息
show_configuration() {
    echo -e "\n${GREEN}🎉 环境设置完成！${NC}"
    echo -e "\n${BLUE}📋 服务信息:${NC}"
    echo -e "  PostgreSQL: localhost:5432"
    echo -e "  Qdrant:     localhost:6333"
    echo -e "  Redis:      localhost:6379"
    echo -e "  Ollama:     localhost:11434"
    echo -e "\n${BLUE}🤖 已部署模型:${NC}"
    echo -e "  • qwen3-embedding (向量化)"
    echo -e "  • qwen3:7b (主要LLM)"
    echo -e "  • qwen3:1.7b (微调模型)"
    echo -e "\n${YELLOW}⚠️  注意事项:${NC}"
    echo -e "  1. 请更新 config/development.toml 中的远程服务器IP地址"
    echo -e "  2. 设置环境变量中的密码"
    echo -e "  3. 查看 TODO.md 了解下一步开发任务"
    echo -e "\n${BLUE}🔍 有用的命令:${NC}"
    echo -e "  检查Docker服务: docker-compose ps"
    echo -e "  查看日志:       docker-compose logs"
    echo -e "  停止服务:       docker-compose down"
    echo -e "  重启服务:       docker-compose restart"
    echo -e "  查看Ollama模型: ollama list"
}

# 主函数
main() {
    check_os
    check_dependencies
    deploy_remote_services
    deploy_local_models
    setup_test_data
    show_configuration
}

# 执行主函数
main "$@" 