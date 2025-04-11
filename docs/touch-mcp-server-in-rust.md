# touch-mcp-server-in-rust

`touch-mcp-server` 是一个使用 `Rust` 实现的创建文件的 `MCP Server`，用于入门 `MCP Server` 开发。

## 安装

```bash
git clone https://github.com/poneding/touch-mcp-server.git
cd touch-mcp-server
cargo install --path .
```

## 配置

在 `VSCode` 中使用 `Cline` 插件添加以下 MCP Server 配置：

```json
{
  "mcpServers": {
   "touch-mcp-server": {
      "command": "~/.cargo/bin/touch-mcp-server",
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
