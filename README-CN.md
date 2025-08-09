# DynMCP - 动态模型上下文协议服务器

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md) | [中文](README-CN.md)

DynMCP 是一个基于 Rust 和 Axum 框架构建的高性能、动态 Model Context Protocol (MCP) 服务器。它支持动态工具发现、配置管理和多种数据源集成，为 AI 应用提供灵活的工具调用能力。

## ✨ 特性

- 🚀 **高性能**: 基于 Rust 和 Tokio 异步运行时，提供卓越的性能表现
- 🔧 **动态工具管理**: 支持运行时动态添加、删除和更新 MCP 工具
- 🗄️ **多数据源支持**: 集成 etcd 和 MySQL 作为工具配置存储后端，并基于插件可动态扩展新数据源
- 🔌 **插件化架构**: 基于插件系统设计，支持动态数据源、认证机制、工具扩展等多种扩展点
- 🏗️ **模块化架构**: 清晰的模块分离，易于扩展和维护
- 📊 **实时监控**: 支持数据源变更的实时监听和缓存更新
- 🔐 **安全认证**: 内置 API 密钥认证机制，支持插件化认证扩展
- 📝 **结构化日志**: 完整的请求追踪和日志记录
- 🏥 **健康检查**: 内置健康检查端点，便于监控和部署

## 🏗️ 系统架构

### 整体架构图

```
[流程图和架构图位置 - 请在此处插入你的架构图]
```

### 核心组件

- **mcp-axum**: HTTP 服务器层，基于 Axum 框架
- **mcp-core**: MCP 协议核心实现和动态执行引擎
- **mcp-common**: 共享组件，包括缓存、配置、数据源客户端等
- **mcp-plugin**: 插件扩展系统，支持数据源、认证、工具等多种扩展点
- **mcp-macro**: 过程宏支持，简化开发

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- etcd 或 MySQL
- macOS/Linux/Windows

### 安装

1. 克隆项目
```bash
git clone https://github.com/cj2a7t/dynmcp.git
cd dynmcp
```

2. 构建项目
```bash
cargo build --release
```

3. 配置环境
```bash
cp config/example.toml config/dev.toml
# 编辑配置文件，设置数据源连接信息
```

4. 运行服务
```bash
cargo run --bin mcp-axum
```

### 配置说明

项目支持多环境配置，主要配置项包括：

- **应用配置**: 主机、端口、数据源类型等
- **数据源配置**: etcd 端点、MySQL 连接字符串等
- **日志配置**: 日志级别、输出格式等

## 📖 使用指南

### MCP 协议调用

```bash
# 发送 MCP HTTP Stream-able 请求
curl -X POST http://localhost:8080/mcp/instance123 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-api-key" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/list",
    "params": {}
  }'
```

### 管理 API

#### TDS (Tool Discovery Service) 管理

```bash
# 获取所有工具定义
curl http://localhost:8080/admin/tds \
  -H "Authorization: Bearer your-api-key"

# 获取特定工具定义
curl http://localhost:8080/admin/tds/tool123 \
  -H "Authorization: Bearer your-api-key"

# 添加/更新工具定义
curl -X PUT http://localhost:8080/admin/tds/tool123 \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "tool123",
    "name": "example_tool",
    "description": "An example tool",
    "input_schema": {},
    "tds_ext_info": {
      "domain": "api.example.com",
      "method": "GET",
      "path": "/v1/example",
      "required_params": {},
      "ext_info": {}
    }
  }'

# 删除工具定义
curl -X DELETE http://localhost:8080/admin/tds/tool123 \
  -H "Authorization: Bearer your-api-key"
```

#### IDS (Instance Discovery Service) 管理

```bash
# 获取所有实例定义
curl http://localhost:8080/admin/ids \
  -H "Authorization: Bearer your-api-key"

# 获取特定实例定义
curl http://localhost:8080/admin/ids/instance123 \
  -H "Authorization: Bearer your-api-key"

# 添加/更新实例定义
curl -X PUT http://localhost:8080/admin/ids/instance123 \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "instance123",
    "name": "example_instance",
    "tool_ids": ["tool1", "tool2", "tool3"]
  }'

# 删除实例定义
curl -X DELETE http://localhost:8080/admin/ids/instance123 \
  -H "Authorization: Bearer your-api-key"
```

## 🔧 开发指南

### 项目结构

```
dynmcp/
├── mcp-axum/          # HTTP 服务器实现
├── mcp-core/          # MCP 协议核心
├── mcp-common/        # 共享组件
├── mcp-plugin/        # 扩展插件
├── mcp-macro/         # 过程宏
└── config/            # 配置文件
```

### 插件系统扩展

#### 添加新的数据源插件

1. 在 `mcp-plugin/src/datasource/` 下创建新的数据源实现
2. 实现 `DataSource` trait
3. 在 `DataSourceFactory` 中添加新的工厂方法
4. 更新配置结构体

## 📊 性能特性

- **高并发**: 基于 Tokio 异步运行时，支持数千并发连接
- **低延迟**: Rust 零成本抽象，最小化运行时开销
- **内存高效**: 智能缓存策略，减少内存占用
- **水平扩展**: 支持多实例部署和负载均衡

## 📄 开源协议

本项目采用 [MIT 协议](LICENSE) 开源。

## 🙏 致谢

感谢以下开源项目的支持：

- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Axum](https://github.com/tokio-rs/axum) - Web 框架
- [Tokio](https://tokio.rs/) - 异步运行时
- [etcd](https://etcd.io/) - 分布式键值存储
- [MySQL](https://www.mysql.com/) - 关系型数据库

## 📞 联系我们

- 项目主页: [https://github.com/cj2a7t/dynmcp](https://github.com/cj2a7t/dynmcp)
- 问题反馈: [Issues](https://github.com/cj2a7t/dynmcp/issues)

---

⭐ 如果这个项目对你有帮助，请给我们一个 Star！ 