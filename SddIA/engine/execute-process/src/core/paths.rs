//! Carga SSOT Cúmulo + overlay instancia (`.SddIA/local.paths.json`).

use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn load_paths_config(repo: &Path) -> Result<Value, String> {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let text = fs::read_to_string(&cfg_path).map_err(|e| e.to_string())?;
    let mut data: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let local = repo.join(".SddIA/local.paths.json");
    if local.is_file() {
        if let Ok(overlay_text) = fs::read_to_string(&local) {
            if let Ok(overlay) = serde_json::from_str::<Value>(&overlay_text) {
                if let (Some(obj), Some(ov)) = (data.as_object_mut(), overlay.as_object()) {
                    for (k, v) in ov {
                        if let (Some(existing), Some(new_map)) =
                            (obj.get(k).and_then(|x| x.as_object()), v.as_object())
                        {
                            let mut merged = existing.clone();
                            for (nk, nv) in new_map {
                                merged.insert(nk.clone(), nv.clone());
                            }
                            obj.insert(k.clone(), Value::Object(merged));
                        } else {
                            obj.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }
    }
    Ok(data)
}
