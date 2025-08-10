<div align="center">
  <img src="https://github.com/user-attachments/assets/e2f0e415-abcb-41dc-b456-84f0067ed889" alt="DynMCP Logo" width="120" height="120">
  <h1>DynMCP - Dynamic Model Context Protocol Server</h1>
</div>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md) | [中文](README-CN.md)

DynMCP is a high-performance, dynamic Model Context Protocol (MCP) server built with Rust and Axum framework. It supports dynamic tool discovery, configuration management, and multiple data source integrations, providing flexible tool calling capabilities for AI applications.

## ✨ Features

- 🚀 **High Performance**: Built on Rust and Tokio async runtime, delivering exceptional performance
- 🔧 **Dynamic Tool Management**: Support runtime dynamic addition, deletion, and updates of MCP tools
- 🗄️ **Multi-DataSource Support**: Integrates etcd and MySQL as tool configuration storage backends, with plugin-based dynamic extension for new data sources
- 🔌 **Plugin Architecture**: Plugin system design supporting dynamic data sources, authentication mechanisms, tool extensions, and various other extension points
- 🏗️ **Modular Architecture**: Clear module separation, easy to extend and maintain
- 📊 **Real-time Monitoring**: Support real-time listening and cache updates for data source changes
- 🔐 **Security Authentication**: Built-in API key authentication mechanism with plugin-based authentication extension support
- 📝 **Structured Logging**: Complete request tracing and logging
- 🏥 **Health Checks**: Built-in health check endpoints for monitoring and deployment

## 🏗️ System Architecture

<!-- ### Overall Architecture Diagram

```
[Flow chart and architecture diagram location - Please insert your architecture diagram here]
``` -->

### Core Components

- **mcp-axum**: HTTP server layer based on Axum framework
- **mcp-core**: MCP protocol core implementation and dynamic execution engine
- **mcp-common**: Shared components including cache, configuration, data source clients, etc.
- **mcp-plugin**: Plugin extension system supporting data sources, authentication, tools, and various extension points
- **mcp-macro**: Procedural macro support for simplified development

## 🚀 Quick Start

### Requirements

- Rust 1.70+
- etcd or MySQL
- macOS/Linux/Windows

### Installation

1. Clone the project
```bash
git clone https://github.com/cj2a7t/dynmcp.git
cd dynmcp
```

2. Build the project
```bash
cargo build --release
```

3. Configure environment
```bash
cp config/example.toml config/dev.toml
# Edit configuration file to set data source connection information
```

4. Run the service
```bash
cargo run --bin mcp-axum
```

### Configuration

The project supports multi-environment configuration with main configuration items including:

- **Application Configuration**: Host, port, data source type, etc.
- **Data Source Configuration**: etcd endpoints, MySQL connection strings, etc.
- **Logging Configuration**: Log level, output format, etc.

## 📖 Usage Guide

### MCP Protocol Calls

```bash
# Send MCP HTTP Stream-able request
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

### Management APIs

#### TDS (Tool Discovery Service) Management

```bash
# Get all tool definitions
curl http://localhost:8080/admin/tds \
  -H "Authorization: Bearer your-api-key"

# Get specific tool definition
curl http://localhost:8080/admin/tds/tool123 \
  -H "Authorization: Bearer your-api-key"

# Add/Update tool definition
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

# Delete tool definition
curl -X DELETE http://localhost:8080/admin/tds/tool123 \
  -H "Authorization: Bearer your-api-key"
```

#### IDS (Instance Discovery Service) Management

```bash
# Get all instance definitions
curl http://localhost:8080/admin/ids \
  -H "Authorization: Bearer your-api-key"

# Get specific instance definition
curl http://localhost:8080/admin/ids/instance123 \
  -H "Authorization: Bearer your-api-key"

# Add/Update instance definition
curl -X PUT http://localhost:8080/admin/ids/instance123 \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "instance123",
    "name": "example_instance",
    "tool_ids": ["tool1", "tool2", "tool3"]
  }'

# Delete instance definition
curl -X DELETE http://localhost:8080/admin/ids/instance123 \
  -H "Authorization: Bearer your-api-key"
```

## 🔧 Development Guide

### Project Structure

```
dynmcp/
├── mcp-axum/          # HTTP server implementation
├── mcp-core/          # MCP protocol core
├── mcp-common/        # Shared components
├── mcp-plugin/        # Extension plugins
├── mcp-macro/         # Procedural macros
└── config/            # Configuration files
```

### Plugin System Extensions

#### Adding New Data Source Plugins

1. Create new data source implementation under `mcp-plugin/src/datasource/`
2. Implement `DataSource` trait
3. Add new factory method in `DataSourceFactory`
4. Update configuration structure

## 📊 Performance Features

- **High Concurrency**: Based on Tokio async runtime, supporting thousands of concurrent connections
- **Low Latency**: Rust zero-cost abstractions, minimizing runtime overhead
- **Memory Efficient**: Smart caching strategies, reducing memory usage
- **Horizontal Scaling**: Support for multi-instance deployment and load balancing

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## 🙏 Acknowledgments

Thanks to the following open source projects:

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [etcd](https://etcd.io/) - Distributed key-value store
- [MySQL](https://www.mysql.com/) - Relational database

## 📞 Contact Us

- Project Homepage: [https://github.com/cj2a7t/dynmcp](https://github.com/cj2a7t/dynmcp)
- Issue Reports: [Issues](https://github.com/cj2a7t/dynmcp/issues)

---

⭐ If this project helps you, please give us a Star!
