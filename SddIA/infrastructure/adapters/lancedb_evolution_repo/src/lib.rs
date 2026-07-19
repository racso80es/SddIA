use sddia_core_memory::models::evolution_node::EvolutionEvent;
use sddia_core_memory::ports::EvolutionStore;
use std::fs;
use std::path::{Path, PathBuf};

pub struct LanceDbEvolutionAdapter {
    pub connection_string: String,
}

impl LanceDbEvolutionAdapter {
    pub fn new() -> Self {
        Self {
            connection_string: ".SddIA/vector_store/evolution/".to_string(),
        }
    }

    fn store_root(&self) -> PathBuf {
        PathBuf::from(&self.connection_string)
    }

    fn record_path(&self, id: &str) -> PathBuf {
        self.store_root().join(format!("{id}.json"))
    }
}

impl Default for LanceDbEvolutionAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl EvolutionStore for LanceDbEvolutionAdapter {
    type Error = String;

    fn store_event(&self, event: EvolutionEvent) -> Result<(), Self::Error> {
        let root = self.store_root();
        fs::create_dir_all(&root).map_err(|e| e.to_string())?;
        let path = self.record_path(&event.id);
        if path.is_file() {
            return Ok(());
        }
        let record = serde_json::json!({
            "id": event.id,
            "polarity": format!("{:?}", event.polarity),
            "payload": event.payload,
            "operational_metadata": event.operational_metadata,
            "embedding": event.embedding,
        });
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, serde_json::to_string_pretty(&record).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
        let _ = Path::new(&path);
        Ok(())
    }
}
