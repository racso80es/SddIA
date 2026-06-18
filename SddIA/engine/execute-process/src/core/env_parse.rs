use std::collections::HashMap;
use std::fs;
use std::path::Path;

static EXPORT_RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
static PAIR_RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();

fn export_re() -> &'static regex::Regex {
    EXPORT_RE.get_or_init(|| {
        regex::Regex::new(r"^export\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$").unwrap()
    })
}

fn pair_re() -> &'static regex::Regex {
    PAIR_RE.get_or_init(|| regex::Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$").unwrap())
}

fn strip_quotes(value: &str) -> String {
    let v = value.trim();
    if v.len() >= 2 {
        let bytes = v.as_bytes();
        if (bytes[0] == b'"' && bytes[v.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[v.len() - 1] == b'\'')
        {
            return v[1..v.len() - 1].to_string();
        }
    }
    v.to_string()
}

pub fn parse_dotenv_file(path: &Path) -> Result<HashMap<String, String>, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut result = HashMap::new();
    for (lineno, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let caps = export_re()
            .captures(line)
            .or_else(|| pair_re().captures(line));
        let Some(caps) = caps else {
            return Err(format!("{}:{}: línea dotenv inválida", path.display(), lineno + 1));
        };
        let key = caps.get(1).unwrap().as_str().to_string();
        let value = strip_quotes(caps.get(2).unwrap().as_str());
        result.insert(key, value);
    }
    Ok(result)
}
