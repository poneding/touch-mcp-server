use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Request {
    pub(crate) jsonrpc: String,
    pub(crate) method: String,
    #[serde(default)]
    pub(crate) params: Option<Value>,
    pub(crate) id: Option<Value>,
}

#[derive(Serialize, Debug)]
pub(crate) struct Response {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ErrorObject>,
    id: Option<Value>,
}

#[derive(Serialize, Debug)]
pub(crate) struct ErrorObject {
    code: i64,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl Response {
    pub(crate) fn new_result(id: Option<Value>, result: Value) -> Self {
        Response {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    pub(crate) fn new_error(id: Option<Value>, code: i64, message: String) -> Self {
        Response {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(ErrorObject {
                code,
                message,
                data: None,
            }),
            id,
        }
    }
}

pub(crate) mod error_codes {
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;
}
