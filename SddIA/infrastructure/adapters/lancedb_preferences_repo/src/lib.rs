use futures::TryStreamExt;
use lancedb::arrow::arrow_array::builder::{
    FixedSizeListBuilder, Float32Builder, StringBuilder, UInt16Builder,
};
use lancedb::arrow::arrow_array::cast::AsArray;
use lancedb::arrow::arrow_array::types::Float32Type;
use lancedb::arrow::arrow_array::{Array, RecordBatch, RecordBatchIterator};
use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::query::{ExecutableQuery, QueryBase};
use sddia_core_memory::error::MemoryStoreError;
use sddia_core_memory::services::inference_binding::{
    validate_embedding_dim, LocalHashingEmbedder, EMBEDDING_MODEL, EMBEDDING_NORM,
};
use sddia_core_memory::{EmbeddingGenerator, EMBEDDING_DIM};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use user_preference_core::{
    authority_snake, context_block_from_prefs, list_head_revisions, sort_preference_hits,
    status_snake, PreferenceAuthority, PreferenceStatus, QuerySpec, ScopeType, UserPreference,
    UserPreferenceStore,
};

pub const TABLE_PREFERENCES: &str = "user_preferences";
const VECTOR_STORE_DEFAULT: &str = ".SddIA/vector_store";
const LANCEDB_SUBDIR: &str = "lancedb";

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

fn io_err(reason: impl Into<String>) -> MemoryStoreError {
    MemoryStoreError::Io {
        reason: reason.into(),
    }
}

pub fn default_lancedb_uri(repo: &Path) -> PathBuf {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let fallback = repo.join(VECTOR_STORE_DEFAULT).join(LANCEDB_SUBDIR);
    let Ok(text) = std::fs::read_to_string(&cfg_path) else {
        return fallback;
    };
    let Ok(cfg) = serde_json::from_str::<Value>(&text) else {
        return fallback;
    };
    cfg.get("paths")
        .and_then(|p| p.get("vectorStore"))
        .and_then(|v| v.as_str())
        .map(|s| {
            let rel = s.trim().trim_start_matches("./");
            repo.join(rel).join(LANCEDB_SUBDIR)
        })
        .unwrap_or(fallback)
}

fn preferences_schema() -> Arc<Schema> {
    let item = Arc::new(Field::new("item", DataType::Float32, true));
    Arc::new(Schema::new(vec![
        Field::new("preference_id", DataType::Utf8, false),
        Field::new("revision_id", DataType::Utf8, false),
        Field::new("subject_kind", DataType::Utf8, false),
        Field::new("subject_key", DataType::Utf8, false),
        Field::new("predicate", DataType::Utf8, false),
        Field::new("value", DataType::Utf8, false),
        Field::new("scope_type", DataType::Utf8, false),
        Field::new("scope_id", DataType::Utf8, true),
        Field::new("status", DataType::Utf8, false),
        Field::new("authority", DataType::Utf8, false),
        Field::new("sensitivity", DataType::Utf8, false),
        Field::new("valid_from", DataType::Utf8, true),
        Field::new("valid_until", DataType::Utf8, true),
        Field::new("supersedes", DataType::Utf8, true),
        Field::new("provenance", DataType::Utf8, false),
        Field::new("recorded_at", DataType::Utf8, false),
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

fn append_opt(builder: &mut StringBuilder, value: Option<&str>) {
    match value {
        Some(v) => builder.append_value(v),
        None => builder.append_null(),
    }
}

fn pref_to_batch(pref: &UserPreference, embedding: &[f32]) -> Result<RecordBatch, MemoryStoreError> {
    validate_embedding_dim(embedding)?;
    let schema = preferences_schema();
    let mut preference_id = StringBuilder::new();
    preference_id.append_value(&pref.preference_id);
    let mut revision_id = StringBuilder::new();
    revision_id.append_value(&pref.revision_id);
    let mut subject_kind = StringBuilder::new();
    subject_kind.append_value(&pref.subject_kind);
    let mut subject_key = StringBuilder::new();
    subject_key.append_value(&pref.subject_key);
    let mut predicate = StringBuilder::new();
    predicate.append_value(&pref.predicate);
    let mut value = StringBuilder::new();
    value.append_value(serde_json::to_string(&pref.value).map_err(|e| io_err(e.to_string()))?);
    let mut scope_type = StringBuilder::new();
    scope_type.append_value(user_preference_core::scope_type_snake(&pref.scope_type));
    let mut scope_id = StringBuilder::new();
    append_opt(&mut scope_id, pref.scope_id.as_deref());
    let mut status = StringBuilder::new();
    status.append_value(status_snake(&pref.status));
    let mut authority = StringBuilder::new();
    authority.append_value(authority_snake(&pref.authority));
    let mut sensitivity = StringBuilder::new();
    sensitivity.append_value(&pref.sensitivity);
    let mut valid_from = StringBuilder::new();
    append_opt(&mut valid_from, pref.valid_from.as_deref());
    let mut valid_until = StringBuilder::new();
    append_opt(&mut valid_until, pref.valid_until.as_deref());
    let mut supersedes = StringBuilder::new();
    append_opt(&mut supersedes, pref.supersedes.as_deref());
    let mut provenance = StringBuilder::new();
    provenance.append_value(serde_json::to_string(&pref.provenance).map_err(|e| io_err(e.to_string()))?);
    let mut recorded_at = StringBuilder::new();
    recorded_at.append_value(&pref.recorded_at);
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
            Arc::new(preference_id.finish()),
            Arc::new(revision_id.finish()),
            Arc::new(subject_kind.finish()),
            Arc::new(subject_key.finish()),
            Arc::new(predicate.finish()),
            Arc::new(value.finish()),
            Arc::new(scope_type.finish()),
            Arc::new(scope_id.finish()),
            Arc::new(status.finish()),
            Arc::new(authority.finish()),
            Arc::new(sensitivity.finish()),
            Arc::new(valid_from.finish()),
            Arc::new(valid_until.finish()),
            Arc::new(supersedes.finish()),
            Arc::new(provenance.finish()),
            Arc::new(recorded_at.finish()),
            Arc::new(list.finish()),
            Arc::new(model.finish()),
            Arc::new(dim.finish()),
            Arc::new(norm.finish()),
        ],
    )
    .map_err(|e| io_err(e.to_string()))
}

fn utf8_col<'a>(
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a lancedb::arrow::arrow_array::StringArray, MemoryStoreError> {
    Ok(batch
        .column_by_name(name)
        .ok_or_else(|| MemoryStoreError::SchemaIncompatible {
            reason: format!("missing {name}"),
        })?
        .as_string::<i32>())
}

fn opt_utf8(col: &lancedb::arrow::arrow_array::StringArray, i: usize) -> Option<String> {
    if col.is_null(i) {
        None
    } else {
        Some(col.value(i).to_string())
    }
}

fn parse_scope(raw: &str) -> Result<ScopeType, MemoryStoreError> {
    match raw {
        "global" => Ok(ScopeType::Global),
        "domain" => Ok(ScopeType::Domain),
        "project" => Ok(ScopeType::Project),
        "channel" => Ok(ScopeType::Channel),
        other => Err(MemoryStoreError::StoreCorrupt {
            reason: format!("scope_type '{other}'"),
        }),
    }
}

fn parse_status(raw: &str) -> Result<PreferenceStatus, MemoryStoreError> {
    match raw {
        "proposed" => Ok(PreferenceStatus::Proposed),
        "active" => Ok(PreferenceStatus::Active),
        "revoked" => Ok(PreferenceStatus::Revoked),
        "superseded" => Ok(PreferenceStatus::Superseded),
        other => Err(MemoryStoreError::StoreCorrupt {
            reason: format!("status '{other}'"),
        }),
    }
}

fn parse_authority(raw: &str) -> Result<PreferenceAuthority, MemoryStoreError> {
    match raw {
        "explicit_user" => Ok(PreferenceAuthority::ExplicitUser),
        "inferred" => Ok(PreferenceAuthority::Inferred),
        other => Err(MemoryStoreError::StoreCorrupt {
            reason: format!("authority '{other}'"),
        }),
    }
}

fn batches_to_prefs(batches: &[RecordBatch]) -> Result<Vec<UserPreference>, MemoryStoreError> {
    let mut out = Vec::new();
    for batch in batches {
        let preference_id = utf8_col(batch, "preference_id")?;
        let revision_id = utf8_col(batch, "revision_id")?;
        let subject_kind = utf8_col(batch, "subject_kind")?;
        let subject_key = utf8_col(batch, "subject_key")?;
        let predicate = utf8_col(batch, "predicate")?;
        let value = utf8_col(batch, "value")?;
        let scope_type = utf8_col(batch, "scope_type")?;
        let scope_id = utf8_col(batch, "scope_id")?;
        let status = utf8_col(batch, "status")?;
        let authority = utf8_col(batch, "authority")?;
        let sensitivity = utf8_col(batch, "sensitivity")?;
        let valid_from = utf8_col(batch, "valid_from")?;
        let valid_until = utf8_col(batch, "valid_until")?;
        let supersedes = utf8_col(batch, "supersedes")?;
        let provenance = utf8_col(batch, "provenance")?;
        let recorded_at = utf8_col(batch, "recorded_at")?;
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
            let value_json: Value =
                serde_json::from_str(value.value(i)).unwrap_or_else(|_| Value::Null);
            let provenance_json: Value =
                serde_json::from_str(provenance.value(i)).unwrap_or_else(|_| serde_json::json!({}));
            out.push(UserPreference {
                preference_id: preference_id.value(i).to_string(),
                revision_id: revision_id.value(i).to_string(),
                subject_kind: subject_kind.value(i).to_string(),
                subject_key: subject_key.value(i).to_string(),
                predicate: predicate.value(i).to_string(),
                value: value_json,
                scope_type: parse_scope(scope_type.value(i))?,
                scope_id: opt_utf8(scope_id, i),
                status: parse_status(status.value(i))?,
                authority: parse_authority(authority.value(i))?,
                sensitivity: sensitivity.value(i).to_string(),
                valid_from: opt_utf8(valid_from, i),
                valid_until: opt_utf8(valid_until, i),
                supersedes: opt_utf8(supersedes, i),
                provenance: provenance_json,
                recorded_at: recorded_at.value(i).to_string(),
                embedding: Some(vec),
            });
        }
    }
    Ok(out)
}

fn resolve_embedding(pref: &UserPreference) -> Result<Vec<f32>, MemoryStoreError> {
    match pref.embedding.as_ref() {
        Some(v) if !v.is_empty() => {
            validate_embedding_dim(v)?;
            Ok(v.clone())
        }
        _ => {
            let text = canonical_embedding_text(pref).map_err(io_err)?;
            LocalHashingEmbedder.generate_embedding(&text)
        }
    }
}

fn status_prefilter(include_proposed: bool) -> String {
    if include_proposed {
        "status != 'revoked' AND status != 'superseded'".into()
    } else {
        "status != 'revoked' AND status != 'superseded' AND status != 'proposed'".into()
    }
}

fn matches_spec(pref: &UserPreference, spec: &QuerySpec) -> bool {
    let include_proposed = spec.include_proposed.unwrap_or(false);
    match pref.status {
        PreferenceStatus::Revoked | PreferenceStatus::Superseded => return false,
        PreferenceStatus::Proposed if !include_proposed => return false,
        _ => {}
    }
    if let Some(sk) = spec.subject_key.as_deref() {
        if pref.subject_key != sk {
            return false;
        }
    }
    if let Some(pred) = spec.predicate.as_deref() {
        if pref.predicate != pred {
            return false;
        }
    }
    if let Some(st) = spec.scope_type.as_ref() {
        if &pref.scope_type != st {
            return false;
        }
    }
    if let Some(sid) = spec.scope_id.as_deref() {
        if pref.scope_id.as_deref() != Some(sid) {
            return false;
        }
    }
    true
}

fn spec_filter(spec: &QuerySpec) -> String {
    let mut parts = vec![status_prefilter(spec.include_proposed.unwrap_or(false))];
    if let Some(sk) = spec.subject_key.as_deref() {
        parts.push(format!("subject_key = '{}'", sql_quote(sk)));
    }
    if let Some(pred) = spec.predicate.as_deref() {
        parts.push(format!("predicate = '{}'", sql_quote(pred)));
    }
    if let Some(st) = spec.scope_type.as_ref() {
        parts.push(format!(
            "scope_type = '{}'",
            sql_quote(user_preference_core::scope_type_snake(st))
        ));
    }
    if let Some(sid) = spec.scope_id.as_deref() {
        parts.push(format!("scope_id = '{}'", sql_quote(sid)));
    }
    parts.join(" AND ")
}

pub struct LanceDbPreferenceAdapter {
    db: lancedb::Connection,
}

impl LanceDbPreferenceAdapter {
    pub fn table_exists(path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        if !path.exists() {
            return false;
        }
        let Some(uri) = path.to_str() else {
            return false;
        };
        rt()
            .block_on(async {
                let db = lancedb::connect(uri).execute().await.ok()?;
                let names = db.table_names().execute().await.ok()?;
                Some(names.iter().any(|n| n == TABLE_PREFERENCES))
            })
            .unwrap_or(false)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, MemoryStoreError> {
        let path = path.as_ref();
        std::fs::create_dir_all(path).map_err(|e| io_err(e.to_string()))?;
        let uri = path.to_str().ok_or_else(|| io_err("path is not utf-8"))?;
        let db = rt()
            .block_on(async { lancedb::connect(uri).execute().await })
            .map_err(map_lance)?;
        let repo = Self { db };
        rt().block_on(repo.ensure_table())?;
        Ok(repo)
    }

    async fn ensure_table(&self) -> Result<(), MemoryStoreError> {
        let names = self.db.table_names().execute().await.map_err(map_lance)?;
        if names.iter().any(|n| n == TABLE_PREFERENCES) {
            let table = self
                .db
                .open_table(TABLE_PREFERENCES)
                .execute()
                .await
                .map_err(map_lance)?;
            let schema = table.schema().await.map_err(map_lance)?;
            embedding_dim_of(schema.as_ref())?;
            Ok(())
        } else {
            self.db
                .create_empty_table(TABLE_PREFERENCES, preferences_schema())
                .execute()
                .await
                .map_err(map_lance)?;
            Ok(())
        }
    }

    async fn upsert(&self, pref: UserPreference) -> Result<UserPreference, MemoryStoreError> {
        let mut pref = user_preference_core::finalize_preference_ids(pref);
        let embedding = resolve_embedding(&pref)?;
        pref.embedding = Some(embedding.clone());
        let schema = preferences_schema();
        let batch = pref_to_batch(&pref, &embedding)?;
        let reader = RecordBatchIterator::new(vec![Ok(batch)].into_iter(), schema);
        let table = self
            .db
            .open_table(TABLE_PREFERENCES)
            .execute()
            .await
            .map_err(map_lance)?;
        let mut merge = table.merge_insert(&["revision_id"]);
        merge
            .when_matched_update_all(None)
            .when_not_matched_insert_all();
        merge.execute(Box::new(reader)).await.map_err(map_lance)?;
        Ok(pref)
    }

    async fn query_filter(&self, filter: &str) -> Result<Vec<UserPreference>, MemoryStoreError> {
        let table = self
            .db
            .open_table(TABLE_PREFERENCES)
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
        batches_to_prefs(&batches)
    }

    async fn knn(
        &self,
        query: &[f32],
        spec: &QuerySpec,
        limit: usize,
    ) -> Result<Vec<UserPreference>, MemoryStoreError> {
        validate_embedding_dim(query)?;
        if limit == 0 {
            return Ok(vec![]);
        }
        let table = self
            .db
            .open_table(TABLE_PREFERENCES)
            .execute()
            .await
            .map_err(map_lance)?;
        let n = table.count_rows(None).await.map_err(map_lance)?;
        if n == 0 {
            return Ok(vec![]);
        }
        let stream = table
            .query()
            .nearest_to(query.to_vec())
            .map_err(map_lance)?
            .limit(n)
            .execute()
            .await
            .map_err(map_lance)?;
        let batches: Vec<RecordBatch> = stream.try_collect().await.map_err(map_lance)?;
        let mut rows = batches_to_prefs(&batches)?;
        rows.retain(|p| matches_spec(p, spec));
        rows.truncate(limit);
        Ok(rows)
    }

    pub fn count_rows(&self) -> Result<usize, MemoryStoreError> {
        rt().block_on(async {
            let table = self
                .db
                .open_table(TABLE_PREFERENCES)
                .execute()
                .await
                .map_err(map_lance)?;
            table.count_rows(None).await.map_err(map_lance)
        })
    }

    pub fn get_by_revision_id(
        &self,
        revision_id: &str,
    ) -> Result<Option<UserPreference>, MemoryStoreError> {
        let filter = format!("revision_id = '{}'", sql_quote(revision_id));
        let rows = rt().block_on(self.query_filter(&filter))?;
        Ok(rows.into_iter().next())
    }

    async fn delete_preference(&self, preference_id: &str) -> Result<(), MemoryStoreError> {
        let table = self
            .db
            .open_table(TABLE_PREFERENCES)
            .execute()
            .await
            .map_err(map_lance)?;
        let pred = format!("preference_id = '{}'", sql_quote(preference_id));
        table.delete(&pred).await.map_err(map_lance)?;
        Ok(())
    }
}

impl UserPreferenceStore for LanceDbPreferenceAdapter {
    type Error = MemoryStoreError;

    fn put_revision(&self, pref: UserPreference) -> Result<UserPreference, Self::Error> {
        rt().block_on(self.upsert(pref))
    }

    fn get_active(&self, preference_id: &str) -> Result<Option<UserPreference>, Self::Error> {
        let filter = format!(
            "preference_id = '{}' AND {}",
            sql_quote(preference_id),
            status_prefilter(true)
        );
        let rows = rt().block_on(self.query_filter(&filter))?;
        Ok(rows.into_iter().next())
    }

    fn query(&self, spec: &QuerySpec) -> Result<Vec<UserPreference>, Self::Error> {
        let max = spec.max_results.unwrap_or(8).min(32);
        let filter = spec_filter(spec);
        if spec.query_embedding.is_some() || spec.query_text.is_some() {
            let embedding = if let Some(v) = spec.query_embedding.as_ref() {
                validate_embedding_dim(v)?;
                v.clone()
            } else {
                let text = spec.query_text.as_deref().unwrap_or("");
                LocalHashingEmbedder.generate_embedding(text)?
            };
            rt().block_on(self.knn(&embedding, spec, max))
        } else {
            let mut hits = rt().block_on(self.query_filter(&filter))?;
            sort_preference_hits(&mut hits, max);
            Ok(hits)
        }
    }

    fn query_context_block(&self, spec: &QuerySpec) -> Result<Value, Self::Error> {
        let prefs = self.query(spec)?;
        Ok(context_block_from_prefs(&prefs))
    }

    fn purge_preference(&self, preference_id: &str) -> Result<(), Self::Error> {
        rt().block_on(self.delete_preference(preference_id))
    }
}

pub fn canonical_embedding_text(pref: &UserPreference) -> Result<String, String> {
    let value = serde_json::to_string(&pref.value).map_err(|e| e.to_string())?;
    Ok(format!(
        "{}:{}:{}:{value}",
        pref.subject_kind,
        pref.predicate,
        user_preference_core::scope_type_snake(&pref.scope_type)
    ))
}

pub fn migrate_json_to_lancedb(repo: &Path) -> Result<usize, MemoryStoreError> {
    migrate_json_to_lancedb_at(repo, &default_lancedb_uri(repo))
}

pub fn migrate_json_to_lancedb_at(
    repo: &Path,
    lancedb_path: &Path,
) -> Result<usize, MemoryStoreError> {
    let adapter = LanceDbPreferenceAdapter::open(lancedb_path)?;
    let heads = list_head_revisions(repo).map_err(io_err)?;
    let n = heads.len();
    for pref in heads {
        UserPreferenceStore::put_revision(&adapter, pref)?;
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;
    use user_preference_core::{put_revision, PreferenceAuthority, PreferenceStatus, ScopeType};

    fn fixture_vec(first: f32, second: f32) -> Vec<f32> {
        let mut v = vec![0.0f32; EMBEDDING_DIM];
        v[0] = first;
        v[1] = second;
        v
    }

    fn setup_json_repo(tmp: &Path) {
        std::fs::create_dir_all(tmp.join("SddIA/core")).unwrap();
        std::fs::write(
            tmp.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"userPreferencesStore":".SddIA/vector_store/user_preferences","vectorStore":".SddIA/vector_store/"}}"#,
        )
        .unwrap();
    }

    fn base_pref(key: &str, status: PreferenceStatus, embedding: Option<Vec<f32>>) -> UserPreference {
        UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "topic".into(),
            subject_key: key.into(),
            predicate: "mute".into(),
            value: json!({"muted": true}),
            scope_type: ScopeType::Global,
            scope_id: None,
            status,
            authority: PreferenceAuthority::ExplicitUser,
            sensitivity: "internal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({}),
            recorded_at: "2026-09-08T12:00:00Z".into(),
            embedding,
        }
    }

    #[test]
    fn table_exists_false_when_path_missing() {
        let dir = tempdir().unwrap();
        assert!(!LanceDbPreferenceAdapter::table_exists(
            dir.path().join("lancedb")
        ));
    }

    #[test]
    fn preferences_roundtrip_after_reopen() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lancedb");
        let stored = {
            let adapter = LanceDbPreferenceAdapter::open(&path).unwrap();
            UserPreferenceStore::put_revision(
                &adapter,
                base_pref("k1", PreferenceStatus::Active, None),
            )
            .unwrap()
        };
        let adapter = LanceDbPreferenceAdapter::open(&path).unwrap();
        let got = adapter
            .get_by_revision_id(&stored.revision_id)
            .unwrap()
            .expect("row");
        assert_eq!(got.subject_key, "k1");
        assert_eq!(got.value["muted"], true);
        assert_eq!(got.embedding.as_ref().unwrap().len(), EMBEDDING_DIM);
    }

    #[test]
    fn duplicate_revision_ids_are_idempotent() {
        let dir = tempdir().unwrap();
        let adapter = LanceDbPreferenceAdapter::open(dir.path().join("lancedb")).unwrap();
        let pref = UserPreferenceStore::put_revision(
            &adapter,
            base_pref("idem", PreferenceStatus::Active, None),
        )
        .unwrap();
        let n1 = adapter.count_rows().unwrap();
        UserPreferenceStore::put_revision(&adapter, pref).unwrap();
        let n2 = adapter.count_rows().unwrap();
        assert_eq!(n1, n2);
        assert_eq!(n1, 1);
    }

    #[test]
    fn preferences_knn_orders_known_vectors() {
        let dir = tempdir().unwrap();
        let adapter = LanceDbPreferenceAdapter::open(dir.path().join("lancedb")).unwrap();
        let mut near = base_pref("near", PreferenceStatus::Active, Some(fixture_vec(1.0, 0.0)));
        near.recorded_at = "2026-09-08T12:00:01Z".into();
        let mut mid = base_pref("mid", PreferenceStatus::Active, Some(fixture_vec(0.8, 0.2)));
        mid.recorded_at = "2026-09-08T12:00:02Z".into();
        let mut far = base_pref("far", PreferenceStatus::Active, Some(fixture_vec(0.0, 1.0)));
        far.recorded_at = "2026-09-08T12:00:03Z".into();
        UserPreferenceStore::put_revision(&adapter, near).unwrap();
        UserPreferenceStore::put_revision(&adapter, mid).unwrap();
        UserPreferenceStore::put_revision(&adapter, far).unwrap();
        let hits = UserPreferenceStore::query(
            &adapter,
            &QuerySpec {
                query_embedding: Some(fixture_vec(1.0, 0.0)),
                max_results: Some(2),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].subject_key, "near");
        assert_eq!(hits[1].subject_key, "mid");
    }

    #[test]
    fn knn_omits_revoked_even_if_vector_near() {
        let dir = tempdir().unwrap();
        let adapter = LanceDbPreferenceAdapter::open(dir.path().join("lancedb")).unwrap();
        let active = base_pref("alive", PreferenceStatus::Active, Some(fixture_vec(0.0, 1.0)));
        let mut revoked =
            base_pref("dead", PreferenceStatus::Revoked, Some(fixture_vec(1.0, 0.0)));
        revoked.recorded_at = "2026-09-08T12:00:09Z".into();
        UserPreferenceStore::put_revision(&adapter, active).unwrap();
        let stored_revoked = UserPreferenceStore::put_revision(&adapter, revoked).unwrap();
        assert!(adapter
            .get_by_revision_id(&stored_revoked.revision_id)
            .unwrap()
            .is_some());
        let hits = UserPreferenceStore::query(
            &adapter,
            &QuerySpec {
                query_embedding: Some(fixture_vec(1.0, 0.0)),
                max_results: Some(8),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(hits.iter().all(|p| p.subject_key != "dead"));
        assert!(hits.iter().any(|p| p.subject_key == "alive"));
    }

    #[test]
    fn wrong_vector_dimension_is_rejected() {
        let dir = tempdir().unwrap();
        let adapter = LanceDbPreferenceAdapter::open(dir.path().join("lancedb")).unwrap();
        let err = UserPreferenceStore::put_revision(
            &adapter,
            base_pref("bad", PreferenceStatus::Active, Some(vec![0.1; 8])),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            MemoryStoreError::DimensionMismatch {
                expected: 384,
                actual: 8
            }
        ));
    }

    #[test]
    fn migrate_preserves_json_and_filter_parity() {
        let tmp = tempdir().unwrap();
        setup_json_repo(tmp.path());
        let mut global = base_pref("racso", PreferenceStatus::Active, None);
        global.predicate = "priority".into();
        global.value = json!({"level": "low"});
        global.scope_type = ScopeType::Global;
        put_revision(tmp.path(), global).unwrap();
        let mut channel = base_pref("racso", PreferenceStatus::Active, None);
        channel.predicate = "priority".into();
        channel.value = json!({"level": "max"});
        channel.scope_type = ScopeType::Channel;
        channel.scope_id = Some("telegram".into());
        channel.recorded_at = "2026-09-08T12:00:01Z".into();
        put_revision(tmp.path(), channel).unwrap();

        let lance_path = tmp.path().join(".SddIA/vector_store/lancedb");
        let n = migrate_json_to_lancedb_at(tmp.path(), &lance_path).unwrap();
        assert_eq!(n, 2);
        let json_file = tmp
            .path()
            .join(".SddIA/vector_store/user_preferences/head_index.json");
        assert!(json_file.is_file());

        let adapter = LanceDbPreferenceAdapter::open(&lance_path).unwrap();
        let spec = QuerySpec {
            subject_key: Some("racso".into()),
            predicate: Some("priority".into()),
            max_results: Some(2),
            ..Default::default()
        };
        let json_hits = user_preference_core::query(tmp.path(), &spec).unwrap();
        let lance_hits = UserPreferenceStore::query(&adapter, &spec).unwrap();
        assert_eq!(json_hits.len(), 2);
        assert_eq!(lance_hits.len(), 2);
        assert_eq!(json_hits[0].scope_type, ScopeType::Channel);
        assert_eq!(lance_hits[0].scope_type, ScopeType::Channel);
        assert_eq!(json_hits[0].value["level"], lance_hits[0].value["level"]);
        assert_eq!(json_hits[1].preference_id, lance_hits[1].preference_id);
    }
}
