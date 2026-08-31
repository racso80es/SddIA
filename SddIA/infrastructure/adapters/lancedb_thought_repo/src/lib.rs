use lancedb::arrow::arrow_array::builder::{FixedSizeListBuilder, Float32Builder, StringBuilder, UInt16Builder};
use lancedb::arrow::arrow_array::cast::AsArray;
use lancedb::arrow::arrow_array::types::Float32Type;
use lancedb::arrow::arrow_array::{RecordBatch, RecordBatchIterator};
use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::query::{ExecutableQuery, QueryBase};
use sddia_core_memory::error::MemoryStoreError;
use sddia_core_memory::models::thought_node::ThoughtNode;
use sddia_core_memory::ports::ThoughtGraphRepository;
use sddia_core_memory::services::inference_binding::{
    validate_embedding_dim, LocalHashingEmbedder, EMBEDDING_MODEL, EMBEDDING_NORM,
};
use sddia_core_memory::{EmbeddingGenerator, EMBEDDING_DIM};
use std::path::Path;
use std::sync::{Arc, OnceLock};

pub const TABLE_THOUGHT: &str = "thought_graph_collection";

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

fn thought_schema() -> Arc<Schema> {
    let item = Arc::new(Field::new("item", DataType::Float32, true));
    Arc::new(Schema::new(vec![
        Field::new("node_id", DataType::Utf8, false),
        Field::new("parent_id", DataType::Utf8, true),
        Field::new("content", DataType::Utf8, false),
        Field::new("metadata", DataType::Utf8, false),
        Field::new("friction_trace", DataType::Utf8, true),
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

fn thought_to_batch(thought: &ThoughtNode, embedding: &[f32]) -> Result<RecordBatch, MemoryStoreError> {
    validate_embedding_dim(embedding)?;
    let schema = thought_schema();
    let mut node_id = StringBuilder::new();
    node_id.append_value(&thought.node_id);
    let mut parent_id = StringBuilder::new();
    match &thought.parent_id {
        Some(v) => parent_id.append_value(v),
        None => parent_id.append_null(),
    }
    let mut content = StringBuilder::new();
    content.append_value(&thought.content);
    let mut metadata = StringBuilder::new();
    metadata.append_value(serde_json::to_string(&thought.metadata).map_err(|e| {
        MemoryStoreError::Io {
            reason: e.to_string(),
        }
    })?);
    let mut friction = StringBuilder::new();
    match &thought.friction_trace {
        Some(v) => friction.append_value(v),
        None => friction.append_null(),
    }
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
            Arc::new(node_id.finish()),
            Arc::new(parent_id.finish()),
            Arc::new(content.finish()),
            Arc::new(metadata.finish()),
            Arc::new(friction.finish()),
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

fn batches_to_thoughts(batches: &[RecordBatch]) -> Result<Vec<ThoughtNode>, MemoryStoreError> {
    let mut out = Vec::new();
    for batch in batches {
        let node_id = batch
            .column_by_name("node_id")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing node_id".into(),
            })?
            .as_string::<i32>();
        let parent_id = batch
            .column_by_name("parent_id")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing parent_id".into(),
            })?
            .as_string::<i32>();
        let content = batch
            .column_by_name("content")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing content".into(),
            })?
            .as_string::<i32>();
        let metadata = batch
            .column_by_name("metadata")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing metadata".into(),
            })?
            .as_string::<i32>();
        let friction = batch
            .column_by_name("friction_trace")
            .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
                reason: "missing friction_trace".into(),
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
            out.push(ThoughtNode {
                node_id: node_id.value(i).to_string(),
                parent_id: if parent_id.is_null(i) {
                    None
                } else {
                    Some(parent_id.value(i).to_string())
                },
                content: content.value(i).to_string(),
                metadata: meta,
                friction_trace: if friction.is_null(i) {
                    None
                } else {
                    Some(friction.value(i).to_string())
                },
                embedding: Some(vec),
            });
        }
    }
    Ok(out)
}

pub struct LanceDbThoughtRepo {
    db: lancedb::Connection,
}

impl LanceDbThoughtRepo {
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
        let repo = Self { db };
        rt().block_on(repo.ensure_table())?;
        Ok(repo)
    }

    async fn ensure_table(&self) -> Result<(), MemoryStoreError> {
        let names = self.db.table_names().execute().await.map_err(map_lance)?;
        if names.iter().any(|n| n == TABLE_THOUGHT) {
            let table = self
                .db
                .open_table(TABLE_THOUGHT)
                .execute()
                .await
                .map_err(map_lance)?;
            let schema = table.schema().await.map_err(map_lance)?;
            embedding_dim_of(schema.as_ref())?;
            Ok(())
        } else {
            self.db
                .create_empty_table(TABLE_THOUGHT, thought_schema())
                .execute()
                .await
                .map_err(map_lance)?;
            Ok(())
        }
    }

    async fn upsert(&self, thought: ThoughtNode) -> Result<(), MemoryStoreError> {
        let embedding = match thought.embedding.as_ref() {
            Some(v) => {
                validate_embedding_dim(v)?;
                v.clone()
            }
            None => LocalHashingEmbedder.generate_embedding(&thought.content)?,
        };
        let schema = thought_schema();
        let batch = thought_to_batch(&thought, &embedding)?;
        let reader = RecordBatchIterator::new(vec![Ok(batch)].into_iter(), schema);
        let table = self
            .db
            .open_table(TABLE_THOUGHT)
            .execute()
            .await
            .map_err(map_lance)?;
        let mut merge = table.merge_insert(&["node_id"]);
        merge
            .when_matched_update_all(None)
            .when_not_matched_insert_all();
        merge.execute(Box::new(reader)).await.map_err(map_lance)?;
        Ok(())
    }

    async fn query_filter(&self, filter: &str) -> Result<Vec<ThoughtNode>, MemoryStoreError> {
        let table = self
            .db
            .open_table(TABLE_THOUGHT)
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
        batches_to_thoughts(&batches)
    }

    async fn knn(&self, query: &[f32], limit: usize) -> Result<Vec<ThoughtNode>, MemoryStoreError> {
        validate_embedding_dim(query)?;
        if limit == 0 {
            return Ok(vec![]);
        }
        let table = self
            .db
            .open_table(TABLE_THOUGHT)
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
        batches_to_thoughts(&batches)
    }
}

impl ThoughtGraphRepository for LanceDbThoughtRepo {
    type Error = MemoryStoreError;

    fn store_thought(&self, thought: ThoughtNode) -> Result<(), Self::Error> {
        rt().block_on(self.upsert(thought))
    }

    fn get_thought_by_id(&self, node_id: &str) -> Result<Option<ThoughtNode>, Self::Error> {
        let filter = format!("node_id = '{}'", sql_quote(node_id));
        let rows = rt().block_on(self.query_filter(&filter))?;
        Ok(rows.into_iter().next())
    }

    fn get_children(&self, parent_id: &str) -> Result<Vec<ThoughtNode>, Self::Error> {
        let filter = format!("parent_id = '{}'", sql_quote(parent_id));
        rt().block_on(self.query_filter(&filter))
    }

    fn search_similar_thoughts(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<ThoughtNode>, Self::Error> {
        rt().block_on(self.knn(query_embedding, limit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    fn fixture_vec(first: f32, second: f32) -> Vec<f32> {
        let mut v = vec![0.0f32; EMBEDDING_DIM];
        v[0] = first;
        v[1] = second;
        v
    }

    fn node(content: &str, parent: Option<&str>, embedding: Vec<f32>) -> ThoughtNode {
        let mut n = ThoughtNode::new(
            parent.map(str::to_string),
            content.to_string(),
            json!({"status": "ACTIVE"}),
            None,
        );
        n.embedding = Some(embedding);
        n
    }

    #[test]
    fn thought_roundtrip_after_reopen() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lancedb");
        let original = node("roundtrip-content", None, fixture_vec(1.0, 0.0));
        let id = original.node_id.clone();
        {
            let repo = LanceDbThoughtRepo::open(&path).unwrap();
            repo.store_thought(original.clone()).unwrap();
        }
        let repo = LanceDbThoughtRepo::open(&path).unwrap();
        let got = repo.get_thought_by_id(&id).unwrap().expect("row");
        assert_eq!(got.node_id, id);
        assert_eq!(got.content, "roundtrip-content");
        assert_eq!(got.embedding.as_ref().unwrap()[0], 1.0);
    }

    #[test]
    fn thought_children_filtered_by_parent() {
        let dir = tempdir().unwrap();
        let repo = LanceDbThoughtRepo::open(dir.path().join("lancedb")).unwrap();
        let parent = node("parent", None, fixture_vec(1.0, 0.0));
        let c1 = node("c1", Some(&parent.node_id), fixture_vec(0.0, 1.0));
        let c2 = node("c2", Some(&parent.node_id), fixture_vec(0.0, 0.5));
        let other = node("other", None, fixture_vec(0.2, 0.2));
        repo.store_thought(parent.clone()).unwrap();
        repo.store_thought(c1.clone()).unwrap();
        repo.store_thought(c2.clone()).unwrap();
        repo.store_thought(other).unwrap();
        let kids = repo.get_children(&parent.node_id).unwrap();
        let mut ids: Vec<_> = kids.iter().map(|k| k.content.as_str()).collect();
        ids.sort();
        assert_eq!(ids, vec!["c1", "c2"]);
    }

    #[test]
    fn thought_knn_orders_known_vectors() {
        let dir = tempdir().unwrap();
        let repo = LanceDbThoughtRepo::open(dir.path().join("lancedb")).unwrap();
        let a = node("near", None, fixture_vec(1.0, 0.0));
        let b = node("mid", None, fixture_vec(0.8, 0.2));
        let c = node("far", None, fixture_vec(0.0, 1.0));
        repo.store_thought(a.clone()).unwrap();
        repo.store_thought(b.clone()).unwrap();
        repo.store_thought(c.clone()).unwrap();
        let hits = repo
            .search_similar_thoughts(&fixture_vec(1.0, 0.0), 2)
            .unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].content, "near");
        assert_eq!(hits[1].content, "mid");
    }

    #[test]
    fn wrong_vector_dimension_is_rejected() {
        let dir = tempdir().unwrap();
        let repo = LanceDbThoughtRepo::open(dir.path().join("lancedb")).unwrap();
        let mut n = node("bad-dim", None, vec![0.0; 8]);
        n.embedding = Some(vec![0.1; 8]);
        let err = repo.store_thought(n).unwrap_err();
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
            Field::new("node_id", DataType::Utf8, false),
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
            db.create_empty_table(TABLE_THOUGHT, wrong)
                .execute()
                .await
                .unwrap();
        });
        let err = match LanceDbThoughtRepo::open(&path) {
            Err(e) => e,
            Ok(_) => panic!("expected schema incompatible"),
        };
        assert!(matches!(err, MemoryStoreError::SchemaIncompatible { .. }));
    }
}
