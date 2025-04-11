use jsonrpc::{INVALID_PARAMS, JSONRPCId, JSONRPCResponse, METHOD_NOT_FOUND};
use jsonrpc::{INVALID_REQUEST, PARSE_ERROR};
use mcp::ListResourcesResult;
use mcp::ListToolsResult;
use mcp::Params;
use mcp::Tool;
use mcp::{CallToolsResult, InitializeResult};
use mcp::{ListPromptsResult, ToolContent};
use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
mod jsonrpc;
mod mcp;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut writer = stdout.lock();

    log("starting\n", &[]);

    loop {
        let mut input = String::new();
        if reader.read_line(&mut input).is_err() {
            on_error(
                &mut writer,
                None,
                PARSE_ERROR,
                "failed to read input".to_string(),
            );
            break;
        }

        let request: jsonrpc::JSONRPCRequest = match serde_json::from_str(input.as_str()) {
            Ok(req) => req,
            Err(err) => {
                on_error(&mut writer, None, PARSE_ERROR, err.to_string());
                break;
            }
        };
        input.clear();

        log(
            ">> Request: \n{}\n",
            &[&serde_json::to_string_pretty(&request).unwrap()],
        );

        if request.jsonrpc != "2.0" {
            let id = request.id;
            on_error(
                &mut writer,
                id,
                INVALID_REQUEST,
                "Invalid JSON-RPC version".to_string(),
            );
            break;
        }

        let id = request.id;

        let response = match request.method.as_str() {
            "initialize" => Some(JSONRPCResponse::from_result(
                id.unwrap(),
                serde_json::to_value(InitializeResult::new()).ok(),
            )),
            "notifications/initialized" | "initialized" => None,
            "tools/list" => Some(JSONRPCResponse::from_result(
                id.unwrap(),
                serde_json::to_value(ListToolsResult::new(Some(vec![Tool {
                    name: "touch-map".to_string(),
                    description: "创建文件工具".to_string(),
                    input_schema: Some(
                        serde_json::json!({
                            "type": "object",
                            "properties": {
                                "file": {
                                    "type": "string",
                                    "description": "文件名"
                                },
                                "destPath": {
                                    "type": "string",
                                    "description": "文件路径"
                                }
                            },
                            "required": ["file_name", "file_content"]
                        })
                        .to_string()
                        .into(),
                    ),
                }])))
                .ok(),
            )), // 工具列表
            "resources/list" => Some(JSONRPCResponse::from_result(
                id.unwrap(),
                serde_json::to_value(ListResourcesResult::new(vec![])).ok(),
            )), // 可访问资源列表
            "prompts/list" => Some(JSONRPCResponse::from_result(
                id.unwrap(),
                serde_json::to_value(ListPromptsResult::new(vec![])).ok(),
            )), // 特定任务的预定义提示模版
            "tools/call" => {
                if let Some(params) = request.params {
                    let params = serde_json::from_value::<Params>(params);
                    if let Ok(params) = params {
                        if params.name.as_str() == "touch-map" {
                            if let Some(args) = params.arguments {
                                if let Some(f) = args.get("file").and_then(|v| v.as_str()) {
                                    let mut dest_path = args.get("destPath").map(|v| v.to_string());
                                    if dest_path.is_none() {
                                        dest_path = env::var("DEFAULT_TOUCH_PATH").ok();
                                        if dest_path.is_none() {
                                            dest_path = dirs::home_dir()
                                                .map(|v| v.to_string_lossy().to_string());
                                        }
                                    }
                                    let dest_path = dest_path.unwrap_or("/tmp".to_string());
                                    let path = Path::new(dest_path.as_str()).join(f);
                                    let resp = match OpenOptions::new()
                                        .write(true)
                                        .create(true)
                                        .open(&path)
                                    {
                                        Ok(_) => Some(JSONRPCResponse::from_result(
                                            id.unwrap(),
                                            serde_json::to_value(CallToolsResult::new(Some(vec![
                                                ToolContent {
                                                    tool_type: "file".to_string(),
                                                    content: format!(
                                                        "File created: {}",
                                                        path.display()
                                                    ),
                                                },
                                            ])))
                                            .ok(),
                                        )),
                                        Err(e) => Some(JSONRPCResponse::from_error_message(
                                            id,
                                            INVALID_PARAMS,
                                            format!("failed to open file: {}", e),
                                        )),
                                    };
                                    resp
                                } else {
                                    Some(JSONRPCResponse::from_error_message(
                                        id,
                                        INVALID_PARAMS,
                                        "file is required".to_string(),
                                    ))
                                }
                            } else {
                                Some(JSONRPCResponse::from_error_message(
                                    id,
                                    INVALID_PARAMS,
                                    "arguments is required".to_string(),
                                ))
                            }
                        } else {
                            Some(JSONRPCResponse::from_error_message(
                                id,
                                INVALID_PARAMS,
                                format!("tool {} not found", params.name),
                            ))
                        }
                    } else {
                        Some(JSONRPCResponse::from_error_message(
                            id,
                            INVALID_PARAMS,
                            "failed to parse params".to_string(),
                        ))
                    }
                } else {
                    Some(JSONRPCResponse::from_error_message(
                        id,
                        INVALID_PARAMS,
                        "params is required".to_string(),
                    ))
                }
            }
            "cancelled" => None,
            _ => {
                on_error(
                    &mut writer,
                    id,
                    METHOD_NOT_FOUND,
                    format!("method {} not found", request.method),
                );
                continue;
            }
        };

        if let Some(resp) = response {
            on_response(&mut writer, resp);
        }
    }

    log("exiting\n", &[]);
}

fn on_error(writer: &mut impl Write, id: Option<JSONRPCId>, code: i32, message: String) {
    let response = JSONRPCResponse::from_error_message(id, code, message);
    on_response(writer, response);
}

fn on_response(writer: &mut impl Write, response: JSONRPCResponse) {
    let json_resp = match serde_json::to_string_pretty(&response) {
        Ok(resp) => resp,
        Err(err) => {
            println!("failed to marshal response: {}", err);
            return;
        }
    };

    log(">> Response: {}\n", &[&json_resp]);

    if let Err(err) = serde_json::to_writer(&mut *writer, &response) {
        println!("failed to encode response: {}", err);
    }

    // 确保响应被发送
    if let Err(err) = writer.flush() {
        println!("failed to flush output: {}", err);
    }
}

fn log(msg: &str, args: &[&str]) {
    // 确保日志目录存在
    let log_path = "/Users/dp/touch-map-server.log";

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(log_path);

    let mut file = match file {
        Ok(file) => file,
        Err(err) => {
            println!("failed to open log file: {}", err);
            return;
        }
    };

    let formatted_msg = if args.is_empty() {
        msg.to_string()
    } else {
        let mut result = msg.to_string();
        for arg in args {
            // 非常简单的替换，生产环境应使用更健壮的实现
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos + 2, arg);
            }
        }
        result
    };

    if let Err(err) = file.write_all(formatted_msg.as_bytes()) {
        println!("failed to write to log file: {}", err);
    }
}
