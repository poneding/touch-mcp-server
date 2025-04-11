use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InitializeResult {
    pub(crate) protocol_version: String,
    pub(crate) server_info: ServerInfo,
    pub(crate) capabilities: Capabilities,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServerInfo {
    pub(crate) name: String,
    pub(crate) version: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Capabilities {
    pub(crate) tools: HashMap<String, Value>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListToolsResult {
    pub(crate) tools: Vec<Tool>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Tool {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) input_schema: Value,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListResourcesResult {
    pub(crate) resources: Vec<Resource>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Resource {}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListPromptsResult {
    pub(crate) prompts: Vec<Prompt>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Prompt {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CallToolParams {
    pub(crate) name: String,
    pub(crate) arguments: Option<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TouchMcpArgs {
    pub(crate) file: String,
    #[serde(default)]
    pub(crate) dest_path: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CallToolsResult {
    pub(crate) content: Vec<ToolContent>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ToolContent {
    #[serde(rename = "type")]
    pub(crate) content_type: String,
    pub(crate) text: String,
}
