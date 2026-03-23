use std::io::{self, BufRead, Write};
use serde_json::{json, Value};

/// Start the MCP Server loop, listening on STDIN and replying on STDOUT
pub fn run_mcp_server() {
    eprintln!("VibeGuard MCP Server started. Listening on stdin...");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).unwrap_or(0);
        if bytes_read == 0 {
            break;
        }

        if let Ok(request) = serde_json::from_str::<Value>(&line) {
            
            let id = request.get("id").cloned().unwrap_or(json!(null));
            let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");

            eprintln!("Received MCP Request: {}", method);

            let response = match method {

                "initialize" => {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": "2024-11-05",
                            "serverInfo": {
                                "name": "vibeguard",
                                "version": "1.0.0"
                            },
                            "capabilities": {
                                "tools": {} 
                            }
                        }
                    })
                }


                "tools/list" => {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "tools":[
                                {
                                    "name": "scan_repo",
                                    "description": "Scans a codebase directory for security vulnerabilities (hardcoded secrets, eval, etc).",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "path": {
                                                "type": "string",
                                                "description": "The directory path to scan. Usually '.'"
                                            }
                                        },
                                        "required": ["path"]
                                    }
                                }
                            ]
                        }
                    })
                }

                "tools/call" => {
                    let params = request.get("params").and_then(|p| p.get("arguments"));
                    let target_path = params
                        .and_then(|args| args.get("path"))
                        .and_then(|p| p.as_str())
                        .unwrap_or(".");

                    eprintln!("AI requested scan of path: {}", target_path);

                    // For now, we return a mock response to prove the wiring works.
                    let mock_result = format!("Scan complete for {}. Found 0 issues (Mock)", target_path);

                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content":[
                                {
                                    "type": "text",
                                    "text": mock_result
                                }
                            ]
                        }
                    })
                }

                // We ignore notifications (like "notifications/initialized")
                _ => {
                    if id.is_null() {
                        continue; // It's a notification, no response needed
                    }
                    // Method not found error
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32601, "message": "Method not found" }
                    })
                }
            };

            // Write the JSON response back to standard output
            let response_str = serde_json::to_string(&response).unwrap();
            writeln!(stdout, "{}", response_str).unwrap();
            stdout.flush().unwrap(); // Ensure it gets sent immediately
        } else {
            eprintln!("Failed to parse JSON-RPC request");
        }
    }
}