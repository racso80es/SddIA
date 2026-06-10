use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn get_repo_root() -> Result<PathBuf, String> {
    let current_exe = env::current_exe().map_err(|e| format!("Failed to get current exe: {}", e))?;
    let mut current_dir = current_exe.parent().unwrap_or(Path::new(""));
    loop {
        if current_dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(current_dir.to_path_buf());
        }
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            return Err("No se encontró raíz del workspace".to_string());
        }
    }
}

fn split_cells(line: &str) -> Vec<String> {
    let line = line.trim();
    let line = if line.starts_with('|') { &line[1..] } else { line };
    let line = if line.ends_with('|') { &line[..line.len()-1] } else { line };
    line.split('|').map(|s| s.trim().to_string()).collect()
}

fn is_separator_row(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') { return false; }
    let content = &trimmed[1..].trim();
    if content.is_empty() { return false; }
    content.chars().all(|c| c == '-' || c == ':' || c == '|' || c.is_whitespace())
}

fn is_table_row(line: &str) -> bool {
    line.trim().starts_with('|') && !is_separator_row(line)
}

struct TableInfo {
    header_idx: usize,
    separator_idx: Option<usize>,
    data_row_indices: Vec<usize>,
    headers: Vec<String>,
}

fn locate_tables(lines: &[String]) -> Vec<TableInfo> {
    let mut tables = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if !is_table_row(&lines[i]) {
            i += 1;
            continue;
        }
        let header_idx = i;
        let headers = split_cells(&lines[i]);
        i += 1;
        let sep_idx = if i < lines.len() && is_separator_row(&lines[i]) {
            let idx = i;
            i += 1;
            Some(idx)
        } else {
            None
        };
        let mut data_rows = Vec::new();
        while i < lines.len() && is_table_row(&lines[i]) {
            data_rows.push(i);
            i += 1;
        }
        tables.push(TableInfo {
            header_idx,
            separator_idx: sep_idx,
            data_row_indices: data_rows,
            headers,
        });
    }
    tables
}

fn column_index(headers: &[String], key_column: Option<&Value>) -> Option<usize> {
    match key_column {
        Some(Value::String(s)) => {
            let s_lower = s.trim().to_lowercase();
            headers.iter().position(|h| h.to_lowercase() == s_lower)
        }
        Some(Value::Number(n)) => {
            if let Some(idx) = n.as_u64() {
                let idx = idx as usize;
                if idx < headers.len() { Some(idx) } else { None }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn row_matches(
    line: &str,
    headers: &[String],
    key_column: Option<&Value>,
    row_data: &Value,
    match_token: Option<&str>,
) -> bool {
    if !is_table_row(line) { return false; }
    let cells = split_cells(line);
    if let Some(col) = column_index(headers, key_column) {
        if col < cells.len() {
            let row_obj = row_data.as_object();
            let mut key_val = None;
            if let Some(obj) = row_obj {
                key_val = obj.get(&headers[col]);
                if key_val.is_none() {
                    key_val = obj.values().next();
                }
            }
            if let Some(v) = key_val {
                let v_str = match v {
                    Value::String(s) => s.to_string(),
                    _ => v.to_string(),
                };
                if v_str == cells[col] || cells[col].contains(&v_str) {
                    return true;
                }
            }
        }
    }
    let token = match_token
        .or_else(|| row_data.get("token").and_then(|v| v.as_str()))
        .or_else(|| row_data.get("match_token").and_then(|v| v.as_str()));

    if let Some(t) = token {
        return line.contains(t);
    }

    if let Some(obj) = row_data.as_object() {
        for v in obj.values() {
            let v_str = match v {
                Value::String(s) => s.to_string(),
                _ => v.to_string(),
            };
            if line.contains(&v_str) {
                return true;
            }
        }
    }
    false
}

fn format_row(cells: &[String]) -> String {
    format!("| {} |\n", cells.join(" | "))
}

struct TableSession {
    path: PathBuf,
    lines: Vec<String>,
    tables: Vec<TableInfo>,
    modified: bool,
}

impl TableSession {
    fn new(path: PathBuf) -> Result<Self, String> {
        let text = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;
        // Keep endings is tricky with splitlines.
        // A simple way: split by \n, then add \n back.
        let mut lines: Vec<String> = text.split('\n').map(|s| format!("{}\n", s)).collect();
        // If text doesn't end with \n, the last line will be "something\n" but that's close enough, or we can trim it.
        if !text.ends_with('\n') && !lines.is_empty() {
            let last = lines.len() - 1;
            lines[last] = lines[last].trim_end_matches('\n').to_string();
            if lines[last].is_empty() { lines.pop(); }
        }

        let tables = locate_tables(&lines);
        Ok(TableSession { path, lines, tables, modified: false })
    }

    fn table(&self, index: usize) -> Result<&TableInfo, String> {
        if index < self.tables.len() {
            Ok(&self.tables[index])
        } else {
            Err(format!("table_index {} fuera de rango ({} tablas)", index, self.tables.len()))
        }
    }

    fn parse(&self, index: usize) -> Result<Value, String> {
        let tbl = self.table(index)?;
        let rows: Vec<Vec<String>> = tbl.data_row_indices.iter().map(|&i| split_cells(&self.lines[i])).collect();
        Ok(json!({
            "headers": tbl.headers,
            "rows": rows,
            "row_count": rows.len(),
            "target_path": self.path.to_string_lossy(),
        }))
    }

    fn row_exists(&self, index: usize, key_column: Option<&Value>, row_data: &Value, match_token: Option<&str>) -> Result<bool, String> {
        let tbl = self.table(index)?;
        for &idx in &tbl.data_row_indices {
            if row_matches(&self.lines[idx], &tbl.headers, key_column, row_data, match_token) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn delete_row(&mut self, index: usize, key_column: Option<&Value>, row_data: &Value, match_token: Option<&str>) -> Result<usize, String> {
        let tbl = self.table(index)?;
        let mut skip = std::collections::HashSet::new();
        let mut removed = 0;
        for &idx in &tbl.data_row_indices {
            if row_matches(&self.lines[idx], &tbl.headers, key_column, row_data, match_token) {
                skip.insert(idx);
                removed += 1;
            }
        }
        if removed == 0 { return Ok(0); }

        let mut new_lines = Vec::new();
        for (i, line) in self.lines.iter().enumerate() {
            if !skip.contains(&i) {
                new_lines.push(line.clone());
            }
        }
        self.lines = new_lines;
        self.tables = locate_tables(&self.lines);
        self.modified = true;
        Ok(removed)
    }

    fn upsert_row(&mut self, index: usize, key_column: Option<&Value>, row_data: &Value, match_token: Option<&str>) -> Result<String, String> {
        let tbl = self.table(index)?;
        let headers = &tbl.headers;
        if headers.is_empty() { return Err("tabla sin cabeceras".to_string()); }

        let mut cells = Vec::new();
        if let Some(obj) = row_data.as_object() {
            for h in headers {
                if let Some(v) = obj.get(h) {
                    cells.push(match v { Value::String(s) => s.to_string(), _ => v.to_string() });
                } else {
                    cells.push(String::new());
                }
            }
            if cells.iter().all(|c| c.is_empty()) {
                cells.clear();
                for v in obj.values() {
                    cells.push(match v { Value::String(s) => s.to_string(), _ => v.to_string() });
                }
                while cells.len() < headers.len() { cells.push(String::new()); }
            }
        }

        let mut new_line = format_row(&cells[..std::cmp::min(cells.len(), headers.len())]);

        for &idx in &tbl.data_row_indices {
            if row_matches(&self.lines[idx], headers, key_column, row_data, match_token) {
                // Ensure the line has a newline
                if !new_line.ends_with('\n') && self.lines[idx].ends_with('\n') {
                    new_line.push('\n');
                } else if new_line.ends_with('\n') && !self.lines[idx].ends_with('\n') {
                    new_line.pop();
                }

                if self.lines[idx] != new_line {
                    self.lines[idx] = new_line;
                    self.modified = true;
                }
                return Ok("updated".to_string());
            }
        }

        let insert_at = if !tbl.data_row_indices.is_empty() {
            tbl.data_row_indices.last().unwrap() + 1
        } else {
            tbl.separator_idx.unwrap_or(tbl.header_idx) + 1
        };

        self.lines.insert(insert_at, new_line);
        self.tables = locate_tables(&self.lines);
        self.modified = true;
        Ok("inserted".to_string())
    }

    fn save(&self, dry_run: bool) -> Result<(), String> {
        if !self.modified || dry_run { return Ok(()); }
        let content = self.lines.join("");
        fs::write(&self.path, content).map_err(|e| format!("Failed to write: {}", e))
    }
}

fn main() {
    let payload = read_stdin_json();

    let file_path = match payload.get("file_path").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => { emit_error("file_path es obligatorio", 1); return; }
    };
    let operation = match payload.get("operation").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim().to_lowercase(),
        _ => { emit_error("operation es obligatorio", 1); return; }
    };

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => { emit_error(&e, 1); return; }
    };

    let target = repo.join(file_path);
    let canonical_repo = repo.canonicalize().unwrap_or(repo.clone());
    let canonical_target = target.canonicalize().unwrap_or(target.clone());

    if !canonical_target.starts_with(&canonical_repo) {
        emit_error("file_path escapes workspace", 1);
        return;
    }
    if !canonical_target.is_file() {
        emit_error(&format!("Archivo no encontrado: {}", file_path), 1);
        return;
    }

    let table_index = payload.get("table_index").and_then(|v| {
        if let Some(n) = v.as_u64() { Some(n as usize) }
        else if let Some(s) = v.as_str() { s.parse().ok() }
        else { None }
    }).unwrap_or(0);

    let key_column = payload.get("key_column");
    let default_row_data = json!({});
    let row_data = payload.get("row_data").unwrap_or(&default_row_data);

    let match_token = payload.get("match_token").and_then(|v| v.as_str());
    let dry_run = payload.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);

    let mut session = match TableSession::new(canonical_target.clone()) {
        Ok(s) => s,
        Err(e) => { emit_error(&e, 1); return; }
    };

    match operation.as_str() {
        "parse" => {
            match session.parse(table_index) {
                Ok(res) => emit_success(Some(json!({
                    "message": format!("Tabla {} parseada.", table_index),
                    "result": res
                }))),
                Err(e) => emit_error(&e, 1),
            }
        }
        "row_exists" => {
            match session.row_exists(table_index, key_column, row_data, match_token) {
                Ok(exists) => emit_success(Some(json!({
                    "message": if exists { "Fila encontrada." } else { "Fila no encontrada." },
                    "result": { "exists": exists, "target_path": canonical_target.to_string_lossy() }
                }))),
                Err(e) => emit_error(&e, 1),
            }
        }
        "delete_row" => {
            match session.delete_row(table_index, key_column, row_data, match_token) {
                Ok(removed) => {
                    if let Err(e) = session.save(dry_run) {
                        emit_error(&e, 1);
                        return;
                    }
                    emit_success(Some(json!({
                        "message": if removed > 0 { format!("{} fila(s) eliminada(s).", removed) } else { "Sin filas a eliminar (idempotente).".to_string() },
                        "result": {
                            "modified": removed > 0,
                            "rows_removed": removed,
                            "target_path": canonical_target.strip_prefix(&canonical_repo).unwrap_or(&canonical_target).to_string_lossy().replace("\\", "/")
                        }
                    })))
                }
                Err(e) => emit_error(&e, 1),
            }
        }
        "upsert_row" => {
            match session.upsert_row(table_index, key_column, row_data, match_token) {
                Ok(action) => {
                    if let Err(e) = session.save(dry_run) {
                        emit_error(&e, 1);
                        return;
                    }
                    emit_success(Some(json!({
                        "message": format!("Fila {}.", action),
                        "result": { "modified": session.modified, "action": action, "target_path": canonical_target.to_string_lossy() }
                    })))
                }
                Err(e) => emit_error(&e, 1),
            }
        }
        "save" => {
            if let Err(e) = session.save(dry_run) {
                emit_error(&e, 1);
                return;
            }
            emit_success(Some(json!({
                "message": if session.modified { "Persistencia completada." } else { "Sin cambios pendientes." },
                "result": { "modified": session.modified, "target_path": canonical_target.to_string_lossy() }
            })))
        }
        _ => emit_error(&format!("operation no soportada: {}", operation), 1)
    }
}
