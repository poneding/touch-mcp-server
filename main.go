package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path"

	"github.com/poneding/touch-mcp-server/pkg/jsonrpc"
	"github.com/poneding/touch-mcp-server/pkg/mcp"
)

var (
	decoder = json.NewDecoder(os.Stdin)
	encoder = json.NewEncoder(os.Stdout)
)

func main() {
	jsonrpc.Log("starting")

	for {
		var req jsonrpc.Request
		if err := decoder.Decode(&req); err != nil {
			onError(nil, jsonrpc.ParseError, fmt.Sprintf("failed to decode request: %v", err))
			break
		}

		jsonrpc.LogRequest(req)

		if req.JSONRPC != "2.0" {
			onError(req.Id, jsonrpc.InvalidRequest, "invalid jsonrpc version")
			break
		}

		var resp *jsonrpc.Response

		switch req.Method {
		case "initialize":
			resp = jsonrpc.NewResultResponse(req.Id, mcp.InitializeResult{
				ProtocolVersion: "2024-11-05",
				ServerInfo: mcp.ServerInfo{
					Name:    "touch-mcp-server",
					Version: "1.0.0",
				},
				Capabilities: mcp.Capabilities{
					Tools: map[string]any{},
				},
			})
		case "notifications/initialized", "initialized":
			continue
		case "tools/list": // 工具列表
			schema := json.RawMessage(`{
				"type": "object",
				"properties": {
					"file": {
						"type":        "string",
						"description": "文件名"
					},
					"destPath": {
						"type":        "string",
						"description": "目标路径"
					}
				},
				"required": ["file"]
			}`)
			resp = jsonrpc.NewResultResponse(req.Id, mcp.ListToolsResult{
				Tools: []mcp.Tool{
					{
						Name:        "touch-mcp",
						Description: "创建文件工具",
						InputSchema: schema,
					},
				},
			})
		case "resources/list": // 可访问资源列表
			resp = jsonrpc.NewResultResponse(req.Id, mcp.ListResourcesResult{
				Resources: []mcp.Resource{},
			})
		case "prompts/list": // 特定任务的预定义提示模版
			resp = jsonrpc.NewResultResponse(req.Id, mcp.ListPromptsResult{
				Prompts: []mcp.Prompt{},
			})
		case "tools/call":
			params, ok := req.Params.(map[string]any)
			if !ok {
				onError(req.Id, jsonrpc.InvalidParams, "failed to parse params")
				continue
			}
			toolName, ok := params["name"].(string)
			if !ok {
				onError(req.Id, jsonrpc.InvalidParams, "failed to parse tool name")
				continue
			}
			switch toolName {
			case "touch-mcp":
				args, ok := params["arguments"].(map[string]any)
				if !ok {
					onError(req.Id, jsonrpc.InvalidParams, "failed to parse arguments")
					continue
				}

				file, ok := args["file"].(string)
				if !ok || file == "" {
					onError(req.Id, jsonrpc.InvalidParams, "failed to parse file name")
					continue
				}
				destPath, ok := args["destPath"].(string)
				if !ok {
					if v := os.Getenv("DEFAULT_TOUCH_PATH"); v != "" {
						destPath = v
					} else {
						destPath, _ = os.UserHomeDir()
					}
				}
				if _, err := os.Create(path.Join(destPath, file)); err != nil {
					onError(req.Id, jsonrpc.InternalError, fmt.Sprintf("failed to create file: %v", err))
					continue
				}
				resp = jsonrpc.NewResultResponse(req.Id, mcp.CallToolsResult{
					Content: []mcp.ToolContent{
						{
							Type: "text",
							Text: fmt.Sprintf("File %s created successfully at %s", file, destPath),
						},
					},
				})
			default:
				resp = jsonrpc.NewErrorResponse(req.Id, jsonrpc.MethodNotFound, fmt.Sprintf("tool %s not found", toolName))
			}
		case "cancelled":
			continue
		default:
			onError(req.Id, jsonrpc.MethodNotFound, fmt.Sprintf("method %s not found", req.Method))
			continue
		}

		if resp != nil {
			onResponse(resp)
		}
	}

	fmt.Println("touch-mcp-server: exiting")
	jsonrpc.Log("exiting\n")
}

func onError(id any, code int, message string) {
	errorResp := jsonrpc.NewErrorResponse(id, code, message)

	onResponse(errorResp)
}

func onResponse(resp *jsonrpc.Response) {
	if err := encoder.Encode(resp); err != nil {
		jsonrpc.Log("failed to encode response: %v", err)
		return
	}
	jsonrpc.LogResponse(resp)
}
