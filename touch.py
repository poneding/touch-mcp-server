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