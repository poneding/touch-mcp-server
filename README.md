# touch-mcp-server

`touch-mcp-server` 是一个使用 `Golang` 实现的创建文件的 `MCP Server`，用于入门 `MCP Server` 开发。

## JSON-RPC 2.0

MCP Server 使用 `JSON-RPC 2.0` 协议进行通信。

[JSON-RPC 2.0](./docs/json-rpc-2.0.md)

## 安装

```bash
git clone https://github.com/poneding/touch-mcp-server.git
cd touch-mcp-server
go install
```

## 配置

在 `VSCode` 中使用 `Cline` 插件添加以下 MCP Server 配置：

```json
{
  "mcpServers": {
   "touch-mcp-server": {
      "command": "~/go/bin/touch-mcp-server",
      "env": {
        "DEFAULT_TOUCH_PATH": "/tmp"
      }
    }
  }
}
```

## 逻辑

1. 获取用户输入内容;
2. AI 从中提取出文件名;
3. 创建文件。

## 其他

- [Rust 实现](./docs/touch-mcp-server-in-rust.md)
- [Python 实现](./docs/touch-mcp-server-in-python.md)
