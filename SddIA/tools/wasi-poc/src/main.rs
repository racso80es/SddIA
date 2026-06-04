use std::io::{self, Read};
use std::process;
use serde_json::Value;

fn main() {
    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_err() {
        process::exit(1);
    }

    let parsed: Result<Value, _> = serde_json::from_str(&buffer);
    let mut json_obj = match parsed {
        Ok(Value::Object(map)) => map,
        _ => {
            process::exit(1);
        }
    };

    json_obj.insert(
        "wasi_status".to_string(),
        Value::String("S+ Grade_Sealed".to_string()),
    );

    let output_json = Value::Object(json_obj);
    let serialized = serde_json::to_string(&output_json).unwrap_or_else(|_| {
        process::exit(1);
    });

    println!("{}", serialized);
}
