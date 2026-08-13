use sddia_evolution_register::execute;
use serde_json::Value;
use std::io::{self, Read};
use std::process;

fn main() {
    let mut buf = String::new();
    if io::stdin().read_to_string(&mut buf).is_err() {
        process::exit(1);
    }
    let payload: Value = match serde_json::from_str(buf.trim()) {
        Ok(v) => v,
        Err(_) => {
            println!(
                "{{\"meta\":{{\"schemaVersion\":\"2.0\",\"entityKind\":\"skill\",\"entityId\":\"sddia-evolution-register\"}},\"success\":false,\"exitCode\":1,\"message\":\"invalid JSON stdin\"}}"
            );
            process::exit(1);
        }
    };
    let env = execute(&payload);
    let out = env.to_json();
    println!("{}", serde_json::to_string(&out).unwrap_or_else(|_| "{}".into()));
    process::exit(env.exit_code);
}
