/*
 * MCP(Model Context Protocol) types
 *
 */
package mcp

import "encoding/json"

type Params struct {
	// RequestId string         `json:"requestId"`
	Name      string         `json:"name"`
	Arguments map[string]any `json:"arguments,omitempty"`
	// Reason    string         `json:"reason,omitempty"`
}

type InitializeResult struct {
	ProtocolVersion string       `json:"protocolVersion"`
	ServerInfo      ServerInfo   `json:"serverInfo"`
	Capabilities    Capabilities `json:"capabilities"`
}

type ServerInfo struct {
	Name    string `json:"name"`
	Version string `json:"version"`
}

type Capabilities struct {
	Tools map[string]any `json:"tools"`
}

// ListToolsResult as the result for tools/list method
type ListToolsResult struct {
	Tools []Tool `json:"tools"`
}

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// CallToolResult as the result for tools/call method
type CallToolsResult struct {
	Content []ToolContent `json:"content"`
}

type ToolContent struct {
	Type string `json:"type"`
	Text string `json:"text"`
}

// ListResourcesResult as the result for resources/list method
type ListResourcesResult struct {
	Resources []Resource `json:"resources"`
}

type Resource struct {
	URI         string `json:"uri"`
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
	MimeType    string `json:"mimeType,omitempty"`
}

// ListPromptsResult as the result for prompts/list method
type ListPromptsResult struct {
	Prompts []Prompt `json:"prompts"`
}

type Prompt struct {
	Name        string           `json:"name"`
	Description string           `json:"description,omitempty"`
	Arguments   []PromptArgument `json:"arguments,omitempty"`
}

type PromptArgument struct {
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
	Required    bool   `json:"required,omitempty"`
}
