use crate::jsonrpc::{Request, Response};

pub(crate) fn log_info(message: &str) {
    eprintln!("touch-mcp-server: INFO: {}", message);
}

pub(crate) fn log_error(message: &str) {
    eprintln!("touch-mcp-server: ERROR: {}", message);
}

pub(crate) fn log_request(req: &Request) {
    match serde_json::to_string(req) {
        Ok(s) => eprintln!("touch-mcp-server: RECV: {}", s),
        Err(e) => eprintln!("touch-mcp-server: ERROR: Failed to log request: {}", e),
    }
}

pub(crate) fn log_response(resp: &Response) {
    match serde_json::to_string(resp) {
        Ok(s) => eprintln!("touch-mcp-server: SEND: {}", s),
        Err(e) => eprintln!("touch-mcp-server: ERROR: Failed to log response: {}", e),
    }
}
