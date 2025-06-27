# dynmcp

> **Dynamic MCP — Seamlessly Binding Tools and Virtual Services.**

`dynmcp` is a lightweight and extensible implementation of the Model Context Protocol (MCP). It enables dynamic registration of tools and virtual services through a unified JSON-RPC interface — ideal for AI agents, LLM-based workflows, and dynamic orchestration of service backends.

---

## ✨ Features

- 🧠 Dynamically register tools and virtual services at runtime
- ⚙️ Unified JSON-RPC interface for tool invocation
- ⚡ High-performance async server powered by Axum and Tokio
- 🗂 Etcd-based storage for persistent registry management
- 🧩 Modular design: plug your own strategies, backends, or services
- 📚 Built-in support for strategy macros and service registry patterns

---

## 🧠 Use Cases

- 🤖 **AI Agent Orchestration**: Tools can be dynamically discovered, registered, and invoked by agents via virtual services.
- 🔌 **Plugin-Based Tooling**: Easily expose internal logic, workflows, or APIs as dynamically registered tools.
- 🧪 **LLM Testing and Evaluation**: Simulate and log tool calls to verify integration quality and response correctness.
- ⚙️ **Composable Microservices**: Use virtual services to abstract and decouple runtime behaviors.
