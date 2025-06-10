//! # RAG工单处理系统
//! 
//! 这是一个基于RAG技术的智能工单处理系统
//! 
//! 主要功能：
//! - 工单向量化和语义检索
//! - 智能重排序和相似度匹配
//! - LLM生成处理建议
//! - 用户反馈收集和模型微调
//! 
//! 请使用 `cargo run --bin rag-server` 启动服务

fn main() {
    println!("请使用以下命令启动RAG服务:");
    println!("cargo run --bin rag-server");
    println!();
    println!("或者直接运行编译后的二进制文件:");
    println!("./target/debug/rag-server");
}
