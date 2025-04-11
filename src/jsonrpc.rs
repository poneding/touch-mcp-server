use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Id {
    Number(u64),
    String(String),
}

pub(crate) type JSONRPCId = Option<i32>;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JSONRPCRequest {
    pub(crate) jsonrpc: String,
    pub(crate) method: String,
    pub(crate) params: Option<serde_json::Value>,
    // pub(crate) id: Option<serde_json::Value>,
    pub(crate) id: Option<JSONRPCId>,
}

#[derive(Debug, Serialize)]
pub(crate) struct JSONRPCResponse {
    pub(crate) jsonrpc: String,
    pub(crate) result: Option<serde_json::Value>,
    pub(crate) error: Option<JSONRPCError>,
    pub(crate) id: Option<JSONRPCId>,
}

#[derive(Debug, Serialize)]
pub(crate) struct JSONRPCError {
    pub(crate) code: i32,
    pub(crate) message: String,
    pub(crate) data: Option<serde_json::Value>,
}

pub(crate) const PARSE_ERROR: i32 = -32700;
pub(crate) const INVALID_REQUEST: i32 = -32600;
pub(crate) const METHOD_NOT_FOUND: i32 = -32601;
pub(crate) const INVALID_PARAMS: i32 = -32602;
pub(crate) const INTERNAL_ERROR: i32 = -32603;

impl JSONRPCResponse {
    pub(crate) fn from_result(id: JSONRPCId, result: Option<serde_json::Value>) -> Self {
        JSONRPCResponse {
            jsonrpc: "2.0".to_string(),
            result,
            error: None,
            id: Some(id),
        }
    }
    pub(crate) fn from_error_message(id: Option<JSONRPCId>, code: i32, err_msg: String) -> Self {
        JSONRPCResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JSONRPCError {
                code,
                message: err_msg,
                data: None,
            }),
            id,
        }
    }
}
