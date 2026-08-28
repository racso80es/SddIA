//! Utilidades compartidas para paths de `git status --porcelain`.

/// Desescapa path `git status --porcelain` entrecomillado (C-style / octal UTF-8).
pub fn unescape_git_cquoted_path(raw: &str) -> String {
    let s = raw.trim().trim_end_matches('/');
    if !(s.starts_with('"') && s.ends_with('"') && s.len() >= 2) {
        return s.to_string();
    }
    let inner = &s[1..s.len() - 1];
    let bytes = inner.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'n' => {
                    out.push(b'\n');
                    i += 2;
                }
                b't' => {
                    out.push(b'\t');
                    i += 2;
                }
                b'r' => {
                    out.push(b'\r');
                    i += 2;
                }
                b'\\' => {
                    out.push(b'\\');
                    i += 2;
                }
                b'"' => {
                    out.push(b'"');
                    i += 2;
                }
                b'a' => {
                    out.push(0x07);
                    i += 2;
                }
                b'b' => {
                    out.push(0x08);
                    i += 2;
                }
                b'f' => {
                    out.push(0x0c);
                    i += 2;
                }
                b'v' => {
                    out.push(0x0b);
                    i += 2;
                }
                c if (b'0'..=b'7').contains(&c) => {
                    let mut val: u8 = 0;
                    let mut count = 0;
                    while count < 3 && i + 1 + count < bytes.len() {
                        let d = bytes[i + 1 + count];
                        if !(b'0'..=b'7').contains(&d) {
                            break;
                        }
                        val = val * 8 + (d - b'0');
                        count += 1;
                    }
                    out.push(val);
                    i += 1 + count;
                }
                _ => {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).unwrap_or_else(|_| inner.to_string())
}

/// Extrae el path relativo de una línea porcelain v1 (modified, untracked, rename).
pub fn porcelain_path_from_line(line: &str) -> Option<String> {
    let line = line.trim_end();
    if line.is_empty() {
        return None;
    }
    let raw = if let Some(rest) = line.strip_prefix("?? ") {
        rest.trim()
    } else if line.len() >= 4 {
        let rest = line[3..].trim();
        if let Some(idx) = rest.find(" -> ") {
            rest[idx + 4..].trim()
        } else {
            rest
        }
    } else {
        return None;
    };
    let path = unescape_git_cquoted_path(raw);
    if path.is_empty() {
        None
    } else {
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unescape_octal_utf8_path() {
        let raw = r#""docs/todos/pending/[REGRESI\303\223N] route-domain-event \342\200\224 fractura sist\303\251mica (6a49e0ad310e)-R1.md""#;
        let path = unescape_git_cquoted_path(raw);
        assert_eq!(
            path,
            "docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md"
        );
    }

    #[test]
    fn porcelain_line_modified_octal() {
        let line = r#" M "docs/todos/pending/[REGRESI\303\223N] route-domain-event \342\200\224 fractura sist\303\251mica (6a49e0ad310e)-R1.md""#;
        let path = porcelain_path_from_line(line).expect("path");
        assert!(path.contains("REGRESIÓN"));
        assert!(path.contains("sistémica"));
    }
}
