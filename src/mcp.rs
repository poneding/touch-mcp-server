use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Params {
    pub(crate) name: String,
    pub(crate) arguments: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub(crate) struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub(crate) protocol_version: String,
    #[serde(rename = "serverInfo")]
    pub(crate) server_info: ServerInfo,
    pub(crate) capabilities: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub(crate) struct ServerInfo {
    pub(crate) name: String,
    pub(crate) version: String,
}

#[derive(Debug, Serialize)]
struct Capabilities {
    tools: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ListToolsResult {
    pub(crate) tools: Option<Vec<Tool>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Tool {
    pub(crate) name: String,
    pub(crate) description: String,
    #[serde(rename = "inputSchema")]
    pub(crate) input_schema: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub(crate) struct CallToolsResult {
    pub(crate) content: Option<Vec<ToolContent>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ToolContent {
    #[serde(rename = "type")]
    pub(crate) tool_type: String,
    pub(crate) content: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct ListResourcesResult {
    pub(crate) resources: Vec<Resource>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Resource {
    pub(crate) uri: String,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    #[serde(rename = "mimeType")]
    pub(crate) mime_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ListPromptsResult {
    pub(crate) prompts: Vec<Prompt>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Prompt {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) arguments: Option<Vec<PromptArgument>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct PromptArgument {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) required: Option<bool>,
}

impl InitializeResult {
    pub(crate) fn new() -> Self {
        InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            server_info: ServerInfo {
                name: "touch-map-server".to_string(),
                version: "1.0.0".to_string(),
            },
            capabilities: serde_json::json!({}),
        }
    }
}

impl ListToolsResult {
    pub(crate) fn new(tools: Option<Vec<Tool>>) -> Self {
        ListToolsResult { tools }
    }
}

impl ListResourcesResult {
    pub(crate) fn new(resources: Vec<Resource>) -> Self {
        ListResourcesResult { resources }
    }
}

impl ListPromptsResult {
    pub(crate) fn new(prompts: Vec<Prompt>) -> Self {
        ListPromptsResult { prompts }
    }
}

impl CallToolsResult {
    pub(crate) fn new(content: Option<Vec<ToolContent>>) -> Self {
        CallToolsResult { content }
    }
}
