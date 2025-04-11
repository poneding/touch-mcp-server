use jsonrpc::*;
use log::{log_error, log_info, log_request, log_response};
use mcp::*;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write}; // 使用 BufReader 进行高效的行读取
use std::path::PathBuf;
mod jsonrpc;
mod log;
mod mcp;

fn main() {
    log_info("starting");

    let stdin = io::stdin();
    // 使用 BufReader 逐行读取，每行一个 JSON 对象(JSON-RPC 请求)
    let reader = BufReader::new(stdin.lock());
    let mut stdout = io::stdout().lock(); // 锁定 stdout，提高性能

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                log_error(&format!("Failed to read line from stdin: {}", e));
                on_error(
                    None,
                    error_codes::INTERNAL_ERROR,
                    &format!("Failed to read line from stdin: {}", e),
                    &mut stdout,
                );
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let request_result: Result<Request, _> = serde_json::from_str(&line);

        let req = match request_result {
            Ok(req) => req,
            Err(e) => {
                on_error(
                    None,
                    error_codes::PARSE_ERROR,
                    &format!("Failed to decode request: {}", e),
                    &mut stdout,
                );
                continue;
            }
        };

        log_request(&req);

        let id = req.id.clone();

        if req.jsonrpc != "2.0" {
            on_error(
                id,
                error_codes::INVALID_REQUEST,
                "Invalid jsonrpc version",
                &mut stdout,
            );
            continue;
        }

        #[allow(unused_assignments)]
        let mut resp: Option<Response> = None;

        match req.method.as_str() {
            "initialize" => {
                resp = Some(Response::new_result(
                    id,
                    json!(InitializeResult {
                        protocol_version: "2024-11-05".to_string(),
                        server_info: ServerInfo {
                            name: "touch-mcp-server".to_string(),
                            version: "1.0.0".to_string(),
                        },
                        capabilities: Capabilities {
                            tools: HashMap::new(),
                        },
                    }),
                ));
            }
            "notifications/initialized" | "initialized" => {
                continue;
            }
            "tools/list" => {
                let schema = json!({
                    "type": "object",
                    "properties": {
                        "file": {
                            "type":        "string",
                            "description": "文件名 (Filename)"
                        },
                        "destPath": {
                            "type":        "string",
                            "description": "目标路径 (Destination path)"
                        }
                    },
                    "required": ["file"]
                });
                resp = Some(Response::new_result(
                    id,
                    json!(ListToolsResult {
                        tools: vec![Tool {
                            name: "touch-mcp".to_string(),
                            description: "创建文件工具 (Create file tool)".to_string(),
                            input_schema: schema,
                        }],
                    }),
                ));
            }
            "resources/list" => {
                resp = Some(Response::new_result(
                    id,
                    json!(ListResourcesResult { resources: vec![] }),
                ));
            }
            "prompts/list" => {
                resp = Some(Response::new_result(
                    id,
                    json!(ListPromptsResult { prompts: vec![] }),
                ));
            }
            "tools/call" => match req.params {
                Some(params_value) => {
                    match serde_json::from_value::<CallToolParams>(params_value) {
                        Ok(params) => match params.name.as_str() {
                            "touch-mcp" => match params.arguments {
                                Some(args_value) => {
                                    match serde_json::from_value::<TouchMcpArgs>(args_value) {
                                        Ok(args) => {
                                            if args.file.is_empty() {
                                                on_error(
                                                    id,
                                                    error_codes::INVALID_PARAMS,
                                                    "Missing or empty file name",
                                                    &mut stdout,
                                                );
                                                continue;
                                            }

                                            let dest_path_str = args
                                                .dest_path
                                                .or_else(|| env::var("DEFAULT_TOUCH_PATH").ok())
                                                .unwrap_or_else(|| {
                                                    dirs::home_dir()
                                                        .map(|p| p.to_string_lossy().into_owned())
                                                        .unwrap_or_else(|| ".".to_string())
                                                });

                                            let mut full_path =
                                                PathBuf::from(dest_path_str.clone());
                                            full_path.push(&args.file);

                                            match File::create(&full_path) {
                                                Ok(_) => {
                                                    resp = Some(Response::new_result(
                                                        id,
                                                        json!(CallToolsResult {
                                                            content: vec![ToolContent {
                                                                content_type: "text".to_string(),
                                                                text: format!(
                                                                    "File {} created successfully at {}",
                                                                    args.file, dest_path_str
                                                                ),
                                                            }],
                                                        }),
                                                    ));
                                                }
                                                Err(e) => {
                                                    on_error(
                                                        id,
                                                        error_codes::INTERNAL_ERROR,
                                                        &format!(
                                                            "Failed to create file '{}': {}",
                                                            full_path.display(),
                                                            e
                                                        ),
                                                        &mut stdout,
                                                    );
                                                    continue;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            on_error(
                                                id,
                                                error_codes::INVALID_PARAMS,
                                                &format!(
                                                    "Failed to parse 'touch-mcp' arguments: {}",
                                                    e
                                                ),
                                                &mut stdout,
                                            );
                                            continue;
                                        }
                                    }
                                }
                                None => {
                                    on_error(
                                        id,
                                        error_codes::INVALID_PARAMS,
                                        "Missing 'arguments' for 'touch-mcp'",
                                        &mut stdout,
                                    );
                                    continue;
                                }
                            },
                            _ => {
                                resp = Some(Response::new_error(
                                    id,
                                    error_codes::METHOD_NOT_FOUND,
                                    format!("Tool '{}' not found", params.name),
                                ));
                            }
                        },
                        Err(e) => {
                            on_error(
                                id,
                                error_codes::INVALID_PARAMS,
                                &format!("Failed to parse 'tools/call' params: {}", e),
                                &mut stdout,
                            );
                            continue;
                        }
                    }
                }
                None => {
                    on_error(
                        id,
                        error_codes::INVALID_PARAMS,
                        "Missing 'params' for 'tools/call'",
                        &mut stdout,
                    );
                    continue;
                }
            },
            "notifications/cancelled" | "cancelled" => {
                continue;
            }
            _ => {
                resp = Some(Response::new_error(
                    id,
                    error_codes::METHOD_NOT_FOUND,
                    format!("Method '{}' not found", req.method),
                ));
            }
        }

        if let Some(resp) = resp {
            on_response(&resp, &mut stdout);
        }
    }
}

fn on_error(id: Option<Value>, code: i64, message: &str, writer: &mut impl Write) {
    let error_resp = Response::new_error(id, code, message.to_string());
    on_response(&error_resp, writer);
}

fn on_response(resp: &Response, writer: &mut impl Write) {
    if let Err(e) = serde_json::to_writer(&mut *writer, resp) {
        log_error(&format!("Failed to encode response: {}", e));
        return;
    }
    if let Err(e) = writer.write_all(b"\n") {
        log_error(&format!("Failed to write newline: {}", e));
        return;
    }
    if let Err(e) = writer.flush() {
        log_error(&format!("Failed to flush stdout: {}", e));
        return;
    }
    log_response(resp);
}
