use crate::capsules::{git_invoke, invoke_skill};
use crate::cerbero::{cerbero_allow_tool, whitelist_executable};
use crate::project::load_project_config;
use crate::sandbox::{parse_project_uri, resolve_under_root, SandboxError};
use crate::telemetry::{emit_raw_execution_finished, write_capsule_envelope};
use crate::vault::child_capsule_env;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

pub struct ServerConfig {
    pub project_root: PathBuf,
    pub sddia_repo: PathBuf,
    pub workspace_path: Option<PathBuf>,
    pub correlation_id: String,
    pub instance_root: Option<PathBuf>,
}

pub struct WorkspaceServer {
    cfg: ServerConfig,
    child_env: HashMap<String, String>,
    allowed_executables: std::collections::HashSet<String>,
    docs_paths: std::collections::HashMap<String, String>,
}

impl WorkspaceServer {
    pub fn new(cfg: ServerConfig) -> Result<Self, String> {
        let proj = load_project_config(&cfg.project_root);
        let child_env = child_capsule_env(
            &cfg.project_root,
            proj.env_ref.as_deref(),
            cfg.instance_root.as_deref(),
        )?;
        Ok(Self {
            cfg,
            child_env,
            allowed_executables: proj.allowed_executables,
            docs_paths: proj.docs_paths,
        })
    }

    pub fn run_stdio(&self) -> Result<(), String> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        for line in stdin.lock().lines() {
            let line = line.map_err(|e| e.to_string())?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let msg: Value = serde_json::from_str(line).map_err(|e| format!("invalid json: {e}"))?;
            if msg.get("method").is_none() {
                continue;
            }
            let response = self.handle_message(&msg)?;
            if response.is_some() {
                let out = serde_json::to_string(&response.unwrap()).map_err(|e| e.to_string())?;
                writeln!(stdout, "{out}").map_err(|e| e.to_string())?;
                stdout.flush().map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    fn handle_message(&self, msg: &Value) -> Result<Option<Value>, String> {
        let method = msg.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let id = msg.get("id").cloned();
        if method.starts_with("notifications/") {
            return Ok(None);
        }
        let params = msg.get("params").cloned().unwrap_or(json!({}));
        let result = match method {
            "initialize" => self.method_initialize(&params),
            "resources/list" => self.method_resources_list(),
            "resources/read" => self.method_resources_read(&params),
            "tools/list" => self.method_tools_list(),
            "tools/call" => self.method_tools_call(&params),
            "ping" => Ok(json!({})),
            _ => Err(format!("unknown method: {method}")),
        };
        match result {
            Ok(r) => Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": r,
            }))),
            Err(e) => Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {"code": -32000, "message": e},
            }))),
        }
    }

    fn method_initialize(&self, _params: &Value) -> Result<Value, String> {
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "resources": {},
                "tools": {},
            },
            "serverInfo": {
                "name": "sddia-workspace-server",
                "version": "1.0.0",
            },
        }))
    }

    fn method_resources_list(&self) -> Result<Value, String> {
        Ok(json!({
            "resources": [
                {"uri": "project://tree", "name": "project-tree", "mimeType": "application/json"},
                {"uri": "project://git-status", "name": "git-status", "mimeType": "application/json"},
                {"uri": "project://file/", "name": "project-file", "mimeType": "text/plain"},
            ],
        }))
    }

    fn method_resources_read(&self, params: &Value) -> Result<Value, String> {
        let uri = params
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or("uri required")?;
        let (kind, arg) = parse_project_uri(uri).ok_or_else(|| format!("unsupported uri: {uri}"))?;
        let root = &self.cfg.project_root;
        let text = match kind.as_str() {
            "tree" => {
                let mut entries = Vec::new();
                if let Ok(rd) = fs::read_dir(root) {
                    for e in rd.flatten() {
                        entries.push(e.file_name().to_string_lossy().to_string());
                    }
                }
                entries.sort();
                serde_json::to_string(&entries).map_err(|e| e.to_string())?
            }
            "file" => {
                let path = resolve_under_root(root, &arg).map_err(sandbox_err)?;
                fs::read_to_string(&path).map_err(|e| e.to_string())?
            }
            "git-status" => {
                let (body, _, _) =
                    git_invoke(&self.cfg.sddia_repo, root, "status", &json!({}), &self.child_env)?;
                serde_json::to_string(&body).map_err(|e| e.to_string())?
            }
            "docs" => {
                let rel = self
                    .docs_paths
                    .get(&arg)
                    .ok_or_else(|| format!("unknown docs key: {arg}"))?;
                let path = resolve_under_root(root, rel).map_err(sandbox_err)?;
                fs::read_to_string(&path).map_err(|e| e.to_string())?
            }
            _ => return Err("unsupported resource".into()),
        };
        Ok(json!({
            "contents": [{
                "uri": uri,
                "mimeType": "text/plain",
                "text": text,
            }],
        }))
    }

    fn method_tools_list(&self) -> Result<Value, String> {
        let tools = [
            tool_def("fs_read", "READ_FILE on target_path"),
            tool_def("fs_write", "WRITE_FILE"),
            tool_def("fs_list", "LIST_DIR"),
            tool_def("fs_delete", "DELETE_FILE"),
            tool_def("fs_mkdir", "CREATE_DIR"),
            tool_def("fs_move", "MOVE_FILE"),
            tool_def("fs_patch", "PATCH_FILE unified diff"),
            tool_def("git_status", "git-manager status"),
            tool_def("run_check", "shell-executor whitelisted command"),
        ];
        Ok(json!({ "tools": tools }))
    }

    fn method_tools_call(&self, params: &Value) -> Result<Value, String> {
        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("name required")?;
        let args = params.get("arguments").cloned().unwrap_or(json!({}));
        if !cerbero_allow_tool(name) {
            let _ = self.finish_tool(name, &args, &json!({"denied": true}), 1, 0);
            return Ok(tool_error("CERBERO_DENY: tool not permitted"));
        }

        let root = &self.cfg.project_root;
        let (body, exit, ms) = match name {
            "fs_read" | "fs_write" | "fs_list" | "fs_delete" | "fs_mkdir" | "fs_move" | "fs_patch" => {
                let op = fs_op(name);
                let mut req = json!({
                    "operation": op,
                    "target_path": args.get("target_path").and_then(|v| v.as_str()).unwrap_or(""),
                    "workspace_root": root.to_string_lossy(),
                });
                if let Some(c) = args.get("content") {
                    req["content"] = c.clone();
                }
                if let Some(d) = args.get("destination_path") {
                    req["destination_path"] = d.clone();
                }
                invoke_skill(&self.cfg.sddia_repo, "filesystem-manager", &req, &self.child_env)?
            }
            "git_status" => git_invoke(&self.cfg.sddia_repo, root, "status", &json!({}), &self.child_env)?,
            "run_check" => {
                let executable = args
                    .get("executable")
                    .and_then(|v| v.as_str())
                    .ok_or("executable required")?;
                if !whitelist_executable(&self.allowed_executables, executable) {
                    let _ = self.finish_tool(name, &args, &json!({"whitelist": "deny"}), 1, 0);
                    return Ok(tool_error("CERBERO_DENY: executable not in whitelist"));
                }
                let arguments = args.get("arguments").cloned().unwrap_or(json!([]));
                let args_vec = arguments
                    .as_array()
                    .ok_or("arguments must be array")?
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<_>>();
                let req = json!({
                    "executable": executable,
                    "arguments": args_vec,
                    "working_directory": root.to_string_lossy(),
                    "environment_vars": self.child_env,
                });
                invoke_skill(&self.cfg.sddia_repo, "shell-executor", &req, &self.child_env)?
            }
            _ => {
                let _ = self.finish_tool(name, &args, &json!({"error": "unknown"}), 1, 0);
                return Ok(tool_error("unknown tool"));
            }
        };

        self.finish_tool(name, &args, &body, exit, ms)?;
        if exit != 0 {
            return Ok(tool_error(&body.to_string()));
        }
        Ok(json!({
            "content": [{"type": "text", "text": body.to_string()}],
            "isError": false,
        }))
    }

    fn finish_tool(
        &self,
        name: &str,
        args: &Value,
        body: &Value,
        exit: i32,
        ms: u128,
    ) -> Result<(), String> {
        if let Some(ws) = &self.cfg.workspace_path {
            let _ = write_capsule_envelope(ws, &self.cfg.correlation_id, name, args, body);
        }
        emit_raw_execution_finished(
            &self.cfg.sddia_repo,
            &self.cfg.correlation_id,
            name,
            exit,
            ms,
            self.cfg.workspace_path.as_deref(),
        )?;
        Ok(())
    }
}

fn tool_def(name: &str, desc: &str) -> Value {
    json!({
        "name": name,
        "description": desc,
        "inputSchema": {"type": "object", "properties": {}},
    })
}

fn tool_error(msg: &str) -> Value {
    json!({
        "content": [{"type": "text", "text": msg}],
        "isError": true,
    })
}

fn fs_op(tool: &str) -> &'static str {
    match tool {
        "fs_read" => "READ_FILE",
        "fs_write" => "WRITE_FILE",
        "fs_list" => "LIST_DIR",
        "fs_delete" => "DELETE_FILE",
        "fs_mkdir" => "CREATE_DIR",
        "fs_move" => "MOVE_FILE",
        "fs_patch" => "PATCH_FILE",
        _ => "READ_FILE",
    }
}

fn sandbox_err(e: SandboxError) -> String {
    format!("{}: {}", e.code(), e.message())
}

pub fn find_sddia_repo(start: &Path) -> PathBuf {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join("SddIA/core/cumulo.paths.json").is_file() {
            return cur;
        }
        if !cur.pop() {
            break;
        }
    }
    start.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cerbero::whitelist_executable;
    use std::collections::HashSet;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn resource_read_rejects_escape() {
        let td = TempDir::new().unwrap();
        let root = td.path();
        let server = WorkspaceServer {
            cfg: ServerConfig {
                project_root: root.to_path_buf(),
                sddia_repo: root.to_path_buf(),
                workspace_path: None,
                correlation_id: "t".into(),
                instance_root: None,
            },
            child_env: HashMap::new(),
            allowed_executables: HashSet::new(),
            docs_paths: std::collections::HashMap::new(),
        };
        let err = server
            .method_resources_read(&json!({"uri": "project://file/../secret"}))
            .unwrap_err();
        assert!(err.contains("PROJECT_SCOPE_ESCAPE"));
    }

    #[test]
    fn run_check_denied_off_whitelist() {
        let allow = HashSet::from([String::from("npm")]);
        assert!(!whitelist_executable(&allow, "curl"));
        assert!(whitelist_executable(&allow, "npm"));
    }

    #[test]
    #[cfg(unix)]
    fn symlink_escape_on_read() {
        let td = TempDir::new().unwrap();
        let root = td.path();
        let outside = TempDir::new().unwrap();
        fs::write(outside.path().join("secret.txt"), "x").unwrap();
        std::os::unix::fs::symlink(outside.path(), root.join("link")).unwrap();
        let err = resolve_under_root(root, "link/secret.txt").unwrap_err();
        assert_eq!(err.code(), "PROJECT_SCOPE_ESCAPE");
    }
}
