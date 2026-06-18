//! Síntesis Mayeuta compartida (paridad `telegram_fallback_responder_core`).

use regex::Regex;
use std::sync::LazyLock;

static RESERVED_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^\s*(TODO|IDEA)\s*:").unwrap());

pub fn filter_c_should_abort(text: &str) -> bool {
    let stripped = text.trim();
    if stripped.is_empty() {
        return true;
    }
    if stripped.starts_with('/') || stripped.starts_with('!') {
        return true;
    }
    RESERVED_RE.is_match(stripped)
}

pub fn synthesize_mayeuta_response(text: &str) -> String {
    let snippet: String = text.trim().chars().take(120).collect();
    format!(
        "[Tormentosa/Aiúa] Recibo el estímulo: «{snippet}».\n\
         Lo asimilo como fricción arquitectónica — ¿es señal o ruido?"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_c_blocks_commands() {
        assert!(filter_c_should_abort("/start"));
        assert!(!filter_c_should_abort("hola lab"));
    }
}
