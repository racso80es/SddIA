mod capsules;
mod cerbero;
mod project;
mod sandbox;
mod server;
mod telemetry;
mod vault;

use server::{find_sddia_repo, ServerConfig, WorkspaceServer};
use std::env;
use std::path::PathBuf;

fn main() {
    if let Err(e) = run() {
        eprintln!("sddia-workspace-server: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut project_root: Option<PathBuf> = None;
    let mut sddia_repo: Option<PathBuf> = None;
    let mut workspace_path: Option<PathBuf> = None;
    let mut instance_root: Option<PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                project_root = Some(PathBuf::from(args.get(i).ok_or("--root value")?));
            }
            "--sddia-repo" => {
                i += 1;
                sddia_repo = Some(PathBuf::from(args.get(i).ok_or("--sddia-repo value")?));
            }
            "--workspace-path" => {
                i += 1;
                workspace_path = Some(PathBuf::from(args.get(i).ok_or("--workspace-path value")?));
            }
            "--instance-root" => {
                i += 1;
                instance_root = Some(PathBuf::from(args.get(i).ok_or("--instance-root value")?));
            }
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    let project_root = project_root.ok_or("--root is required")?;
    let sddia_repo = sddia_repo.unwrap_or_else(|| find_sddia_repo(&project_root));
    let correlation_id = env::var("SDDIA_CORRELATION_ID").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
    let server = WorkspaceServer::new(ServerConfig {
        project_root,
        sddia_repo,
        workspace_path,
        correlation_id,
        instance_root,
    })?;
    server.run_stdio()
}
