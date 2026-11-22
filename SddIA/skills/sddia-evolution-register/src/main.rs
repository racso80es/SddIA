use sddia_evolution_register::execute;
use serde_json::Value;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let payload: Value = match sddia_io::read_capsule_json_from_args(&args, false) {
        Ok(v) => v,
        Err(msg) => {
            let out = serde_json::json!({
                "meta": {
                    "schemaVersion": "2.0",
                    "entityKind": "skill",
                    "entityId": "sddia-evolution-register"
                },
                "success": false,
                "exitCode": 1,
                "message": msg
            });
            println!("{}", serde_json::to_string(&out).unwrap_or_else(|_| "{}".into()));
            process::exit(1);
        }
    };
    let env = execute(&payload);
    let out = env.to_json();
    println!("{}", serde_json::to_string(&out).unwrap_or_else(|_| "{}".into()));
    process::exit(env.exit_code);
}
