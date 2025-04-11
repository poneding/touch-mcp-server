# touch-mcp-server-in-python

`touch-mcp-server` 是一个使用 `Python` 实现的创建文件的 `MCP Server`，用于入门 `MCP Server` 开发。

## 实现

```bash
uv init
uv venv
source .venv/bin/activate
uv add mcp
touch touch.py
```

touch.py:

```python
from mcp.server.fastmcp import FastMCP
import os

mcp = FastMCP("touch-mcp-server")

@mcp.tool()
def touch_file(file: str, destPath : str):
    """
    Create a new file
    """
    # join path
    file = os.path.join(destPath, file)

    # create file
    with open(file, "w") as f:
        pass

    return f"File {file} created"

if __name__ == "__main__":
    mcp.run(transport='stdio')
```

## 配置

在 `VSCode` 中使用 `Cline` 插件添加以下 MCP Server 配置：

```json
{
  "mcpServers": {
    "touch-mcp-server": {
      "command": "uv",
      "args": [
        "--directory",
        "/Users/dp/src/touch-mcp-server", // 替换路径
        "run",
        "touch.py"
      ],
    }
  }
}
```

## 逻辑

1. 获取用户输入内容;
2. AI 从中提取出文件名;
3. 创建文件。
