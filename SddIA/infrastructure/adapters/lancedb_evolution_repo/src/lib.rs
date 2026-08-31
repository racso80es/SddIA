use lancedb::arrow::arrow_array::builder::{FixedSizeListBuilder, Float32Builder, StringBuilder, UInt16Builder};
use lancedb::arrow::arrow_array::cast::AsArray;
use lancedb::arrow::arrow_array::types::Float32Type;
use lancedb::arrow::arrow_array::{RecordBatch, RecordBatchIterator};
use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::query::{ExecutableQuery, QueryBase};
use sddia_core_memory::error::MemoryStoreError;
use sddia_core_memory::models::evolution_node::{EvolutionEvent, SpatialPolarity};
use sddia_core_memory::ports::EvolutionStore;
use sddia_core_memory::services::inference_binding::{
    validate_embedding_dim, LocalHashingEmbedder, SemanticInference, EMBEDDING_MODEL,
    EMBEDDING_NORM,
};
use sddia_core_memory::EMBEDDING_DIM;
use std::fs;
use std::path::Path;
use std::sync::{Arc, OnceLock};

pub const TABLE_EVOLUTION: &str = "evolution";

static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn rt() -> &'static tokio::runtime::Runtime {
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("lancedb tokio runtime")
    })
}

fn map_lance(err: lancedb::Error) -> MemoryStoreError {
    MemoryStoreError::StoreCorrupt {
        reason: err.to_string(),
    }
}

fn sql_quote(value: &str) -> String {
    value.replace('\'', "''")
}

fn polarity_str(p: &SpatialPolarity) -> &'static str {
    match p {
        SpatialPolarity::EfficientSymmetry => "EfficientSymmetry",
        SpatialPolarity::StructuralFracture => "StructuralFracture",
    }
}

fn polarity_from_str(s: &str) -> Result<SpatialPolarity, MemoryStoreError> {
    match s {
        "EfficientSymmetry" => Ok(SpatialPolarity::EfficientSymmetry),
        "StructuralFracture" => Ok(SpatialPolarity::StructuralFracture),
        other => Err(MemoryStoreError::StoreCorrupt {
            reason: format!("unknown polarity {other}"),
        }),
    }
}

fn evolution_schema() -> Arc<Schema> {
    let item = Arc::new(Field::new("item", DataType::Float32, true));
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("polarity", DataType::Utf8, false),
        Field::new("payload", DataType::Utf8, false),
        Field::new("operational_metadata", DataType::Utf8, false),
        Field::new(
            "embedding",
            DataType::FixedSizeList(item, EMBEDDING_DIM as i32),
            false,
        ),
        Field::new("embedding_model", DataType::Utf8, false),
        Field::new("embedding_dim", DataType::UInt16, false),
        Field::new("embedding_norm", DataType::Utf8, false),
    ]))
}

fn embedding_dim_of(schema: &Schema) -> Result<usize, MemoryStoreError> {
    let field = schema
        .field_with_name("embedding")
        .map_err(|e| MemoryStoreError::SchemaIncompatible {
            reason: e.to_string(),
        })?;
    match field.data_type() {
        DataType::FixedSizeList(_, n) if *n as usize == EMBEDDING_DIM => Ok(*n as usize),
        DataType::FixedSizeList(_, n) => Err(MemoryStoreError::SchemaIncompatible {
            reason: format!("embedding dim {n}, expected {EMBEDDING_DIM}"),
        }),
        other => Err(MemoryStoreError::SchemaIncompatible {
            reason: format!("embedding type {other:?}"),
        }),
    }
}

fn event_to_batch(event: &EvolutionEvent, embedding: &[f32]) -> Result<RecordBatch, MemoryStoreError> {
    validate_embedding_dim(embedding)?;
    let schema = evolution_schema();
    let mut id = StringBuilder::new();
    id.append_value(&event.id);
    let mut polarity = StringBuilder::new();
    polarity.append_value(polarity_str(&event.polarity));
    let mut payload = StringBuilder::new();
    payload.append_value(&event.payload);
    let mut metadata = StringBuilder::new();
    metadata.append_value(serde_json::to_string(&event.operational_metadata).map_err(|e| {
        MemoryStoreError::Io {
            reason: e.to_string(),
        }
    })?);
    let mut floats = Float32Builder::with_capacity(EMBEDDING_DIM);
    for x in embedding {
        floats.append_value(*x);
    }
    let mut list = FixedSizeListBuilder::new(floats, EMBEDDING_DIM as i32);
    list.append(true);
    let mut model = StringBuilder::new();
    model.append_value(EMBEDDING_MODEL);
    let mut dim = UInt16Builder::new();
    dim.append_value(EMBEDDING_DIM as u16);
    let mut norm = StringBuilder::new();
    norm.append_value(EMBEDDING_NORM);
    RecordBatch::try_new(
        schema,
        vec![
            Arc::new(id.finish()),
            Arc::new(polarity.finish()),
            Arc::new(payload.finish()),
            Arc::new(metadata.finish()),
            Arc::new(list.finish()),
            Arc::new(model.finish()),
            Arc::new(dim.finish()),
            Arc::new(norm.finish()),
        ],
    )
    .map_err(|e| MemoryStoreError::Io {
        reason: e.to_string(),
    })
}

fn batches_to_events(batches: &[RecordBatch]) -> Result<Vec<EvolutionEvent>, MemoryStoreError> {
    let mut out = Vec::new();
    for batch in batches {
        let id = batch
            .column_by_name("id")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing id".into(),
            })?
            .as_string::<i32>();
        let polarity = batch
            .column_by_name("polarity")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing polarity".into(),
            })?
            .as_string::<i32>();
        let payload = batch
            .column_by_name("payload")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing payload".into(),
            })?
            .as_string::<i32>();
        let metadata = batch
            .column_by_name("operational_metadata")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing operational_metadata".into(),
            })?
            .as_string::<i32>();
        let embedding = batch
            .column_by_name("embedding")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing embedding".into(),
            })?
            .as_fixed_size_list();
        for i in 0..batch.num_rows() {
            let values = embedding.value(i);
            let floats = values.as_primitive::<Float32Type>();
            let vec: Vec<f32> = (0..floats.len()).map(|j| floats.value(j)).collect();
            let meta_raw = metadata.value(i);
            let meta = serde_json::from_str(meta_raw).unwrap_or(serde_json::json!({}));
            out.push(EvolutionEvent {
                id: id.value(i).to_string(),
                polarity: polarity_from_str(polarity.value(i))?,
                payload: payload.value(i).to_string(),
                operational_metadata: meta,
                embedding: Some(vec),
            });
        }
    }
    Ok(out)
}

fn is_zero_or_missing(embedding: &Option<Vec<f32>>) -> bool {
    match embedding {
        None => true,
        Some(v) if v.is_empty() => true,
        Some(v) => v.iter().all(|x| *x == 0.0),
    }
}

pub struct LanceDbEvolutionAdapter {
    db: lancedb::Connection,
}

impl LanceDbEvolutionAdapter {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, MemoryStoreError> {
        let path = path.as_ref();
        std::fs::create_dir_all(path).map_err(|e| MemoryStoreError::Io {
            reason: e.to_string(),
        })?;
        let uri = path.to_str().ok_or_else(|| MemoryStoreError::Io {
            reason: "path is not utf-8".into(),
        })?;
        let db = rt()
            .block_on(async { lancedb::connect(uri).execute().await })
            .map_err(map_lance)?;
        let adapter = Self { db };
        rt().block_on(adapter.ensure_table())?;
        Ok(adapter)
    }

    pub fn row_count(&self) -> Result<usize, MemoryStoreError> {
        rt().block_on(async {
            let table = self
                .db
                .open_table(TABLE_EVOLUTION)
                .execute()
                .await
                .map_err(map_lance)?;
            table.count_rows(None).await.map_err(map_lance)
        })
    }

    async fn ensure_table(&self) -> Result<(), MemoryStoreError> {
        let names = self.db.table_names().execute().await.map_err(map_lance)?;
        if names.iter().any(|n| n == TABLE_EVOLUTION) {
            let table = self
                .db
                .open_table(TABLE_EVOLUTION)
                .execute()
                .await
                .map_err(map_lance)?;
            let schema = table.schema().await.map_err(map_lance)?;
            embedding_dim_of(schema.as_ref())?;
            Ok(())
        } else {
            self.db
                .create_empty_table(TABLE_EVOLUTION, evolution_schema())
                .execute()
                .await
                .map_err(map_lance)?;
            Ok(())
        }
    }

    async fn upsert(&self, mut event: EvolutionEvent) -> Result<(), MemoryStoreError> {
        if is_zero_or_missing(&event.embedding) {
            LocalHashingEmbedder.embed_event(&mut event)?;
        } else if let Some(v) = event.embedding.as_ref() {
            validate_embedding_dim(v)?;
        }
        let embedding = event.embedding.clone().ok_or_else(|| {
            MemoryStoreError::EmbeddingFailed {
                reason: "embedding missing after generate".into(),
            }
        })?;
        let schema = evolution_schema();
        let batch = event_to_batch(&event, &embedding)?;
        let reader = RecordBatchIterator::new(vec![Ok(batch)].into_iter(), schema);
        let table = self
            .db
            .open_table(TABLE_EVOLUTION)
            .execute()
            .await
            .map_err(map_lance)?;
        let mut merge = table.merge_insert(&["id"]);
        merge
            .when_matched_update_all(None)
            .when_not_matched_insert_all();
        merge.execute(Box::new(reader)).await.map_err(map_lance)?;
        Ok(())
    }

    async fn query_filter(&self, filter: &str) -> Result<Vec<EvolutionEvent>, MemoryStoreError> {
        let table = self
            .db
            .open_table(TABLE_EVOLUTION)
            .execute()
            .await
            .map_err(map_lance)?;
        let stream = table
            .query()
            .only_if(filter)
            .execute()
            .await
            .map_err(map_lance)?;
        let batches: Vec<RecordBatch> = stream.try_collect().await.map_err(map_lance)?;
        batches_to_events(&batches)
    }

    async fn knn(
        &self,
        query: &[f32],
        limit: usize,
    ) -> Result<Vec<EvolutionEvent>, MemoryStoreError> {
        validate_embedding_dim(query)?;
        if limit == 0 {
            return Ok(vec![]);
        }
        let table = self
            .db
            .open_table(TABLE_EVOLUTION)
            .execute()
            .await
            .map_err(map_lance)?;
        let n = table.count_rows(None).await.map_err(map_lance)?;
        if n == 0 {
            return Ok(vec![]);
        }
        let stream = table
            .query()
            .nearest_to(query)
            .map_err(map_lance)?
            .limit(limit)
            .execute()
            .await
            .map_err(map_lance)?;
        let batches: Vec<RecordBatch> = stream.try_collect().await.map_err(map_lance)?;
        batches_to_events(&batches)
    }
}

impl EvolutionStore for LanceDbEvolutionAdapter {
    type Error = MemoryStoreError;

    fn store_event(&self, event: EvolutionEvent) -> Result<(), Self::Error> {
        rt().block_on(self.upsert(event))
    }

    fn get_event_by_id(&self, id: &str) -> Result<Option<EvolutionEvent>, Self::Error> {
        let filter = format!("id = '{}'", sql_quote(id));
        let rows = rt().block_on(self.query_filter(&filter))?;
        Ok(rows.into_iter().next())
    }

    fn search_similar_events(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<EvolutionEvent>, Self::Error> {
        rt().block_on(self.knn(query_embedding, limit))
    }
}

pub fn import_legacy_evolution_json(
    src_dir: &Path,
    adapter: &LanceDbEvolutionAdapter,
) -> Result<usize, MemoryStoreError> {
    let rd = match fs::read_dir(src_dir) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => {
            return Err(MemoryStoreError::Io {
                reason: e.to_string(),
            })
        }
    };
    let mut n = 0usize;
    for ent in rd {
        let ent = ent.map_err(|e| MemoryStoreError::Io {
            reason: e.to_string(),
        })?;
        let path = ent.path();
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if !name.ends_with(".json") || name.ends_with(".tmp") {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| MemoryStoreError::Io {
            reason: e.to_string(),
        })?;
        let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| MemoryStoreError::Io {
            reason: format!("{}: {e}", path.display()),
        })?;
        let id = v
            .get("id")
            .and_then(|x| x.as_str())
            .ok_or_else(|| MemoryStoreError::Io {
                reason: format!("{} missing id", path.display()),
            })?
            .to_string();
        let polarity = polarity_from_str(
            v.get("polarity")
                .and_then(|x| x.as_str())
                .unwrap_or("StructuralFracture"),
        )?;
        let payload = match v.get("payload") {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => String::new(),
        };
        let operational_metadata = v
            .get("operational_metadata")
            .cloned()
            .unwrap_or(serde_json::json!({}));
        let embedding = v.get("embedding").and_then(|e| {
            e.as_array().map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_f64().map(|f| f as f32))
                    .collect::<Vec<f32>>()
            })
        });
        let event = EvolutionEvent {
            id,
            polarity,
            payload,
            operational_metadata,
            embedding,
        };
        adapter.store_event(event)?;
        n += 1;
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    fn fixture_vec(first: f32) -> Vec<f32> {
        let mut v = vec![0.0f32; EMBEDDING_DIM];
        v[0] = first;
        v
    }

    #[test]
    fn evolution_roundtrip_after_reopen() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lancedb");
        let event = EvolutionEvent {
            id: "evt-roundtrip".into(),
            polarity: SpatialPolarity::EfficientSymmetry,
            payload: "payload-stable".into(),
            operational_metadata: json!({"success": true, "entity_id": "feature"}),
            embedding: Some(fixture_vec(1.0)),
        };
        {
            let adapter = LanceDbEvolutionAdapter::open(&path).unwrap();
            adapter.store_event(event.clone()).unwrap();
        }
        let adapter = LanceDbEvolutionAdapter::open(&path).unwrap();
        let got = adapter.get_event_by_id("evt-roundtrip").unwrap().unwrap();
        assert_eq!(got.payload, "payload-stable");
        assert_eq!(got.polarity, SpatialPolarity::EfficientSymmetry);
        assert_eq!(got.operational_metadata["entity_id"], json!("feature"));
        assert_eq!(got.embedding.as_ref().unwrap()[0], 1.0);
    }

    #[test]
    fn duplicate_ids_are_idempotent() {
        let dir = tempdir().unwrap();
        let adapter = LanceDbEvolutionAdapter::open(dir.path().join("lancedb")).unwrap();
        let event = EvolutionEvent {
            id: "evt-dup".into(),
            polarity: SpatialPolarity::StructuralFracture,
            payload: "once".into(),
            operational_metadata: json!({}),
            embedding: Some(fixture_vec(0.5)),
        };
        adapter.store_event(event.clone()).unwrap();
        adapter.store_event(event).unwrap();
        assert_eq!(adapter.row_count().unwrap(), 1);
        assert!(adapter.get_event_by_id("evt-dup").unwrap().is_some());
    }

    #[test]
    fn wrong_vector_dimension_is_rejected() {
        let dir = tempdir().unwrap();
        let adapter = LanceDbEvolutionAdapter::open(dir.path().join("lancedb")).unwrap();
        let event = EvolutionEvent {
            id: "evt-dim".into(),
            polarity: SpatialPolarity::EfficientSymmetry,
            payload: "x".into(),
            operational_metadata: json!({}),
            embedding: Some(vec![0.1; 8]),
        };
        let err = adapter.store_event(event).unwrap_err();
        assert!(matches!(
            err,
            MemoryStoreError::DimensionMismatch {
                expected: 384,
                actual: 8
            }
        ));
    }

    #[test]
    fn schema_mismatch_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lancedb");
        std::fs::create_dir_all(&path).unwrap();
        let item = Arc::new(Field::new("item", DataType::Float32, true));
        let wrong = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new(
                "embedding",
                DataType::FixedSizeList(item, 8),
                false,
            ),
        ]));
        rt().block_on(async {
            let db = lancedb::connect(path.to_str().unwrap())
                .execute()
                .await
                .unwrap();
            db.create_empty_table(TABLE_EVOLUTION, wrong)
                .execute()
                .await
                .unwrap();
        });
        let err = match LanceDbEvolutionAdapter::open(&path) {
            Err(e) => e,
            Ok(_) => panic!("expected schema incompatible"),
        };
        assert!(matches!(err, MemoryStoreError::SchemaIncompatible { .. }));
    }

    #[test]
    fn import_legacy_json_is_idempotent() {
        let dir = tempdir().unwrap();
        let json_dir = dir.path().join("evolution");
        std::fs::create_dir_all(&json_dir).unwrap();
        let rec = json!({
            "id": "legacy-1",
            "polarity": "EfficientSymmetry",
            "payload": "legacy-payload",
            "operational_metadata": {"k": 1},
            "embedding": null
        });
        std::fs::write(json_dir.join("legacy-1.json"), serde_json::to_string(&rec).unwrap()).unwrap();
        let adapter = LanceDbEvolutionAdapter::open(dir.path().join("lancedb")).unwrap();
        assert_eq!(import_legacy_evolution_json(&json_dir, &adapter).unwrap(), 1);
        assert_eq!(import_legacy_evolution_json(&json_dir, &adapter).unwrap(), 1);
        assert_eq!(adapter.row_count().unwrap(), 1);
        let got = adapter.get_event_by_id("legacy-1").unwrap().unwrap();
        assert_eq!(got.payload, "legacy-payload");
    }
}
