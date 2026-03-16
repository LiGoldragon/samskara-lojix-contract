mod cozoscript;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetLang {
    TypeScript,
    Rust,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspilerVersion {
    pub id: String,
    pub hash: String,
    pub lang: TargetLang,
    pub live: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalRequest {
    pub request_id: String,
    pub version_id: String,
    pub input_hash: String,
    pub lojix_source_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputKind {
    Value,
    TypeError,
    RuntimeError,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult {
    pub request_id: String,
    pub version_id: String,
    pub input_hash: String,
    pub output_kind: OutputKind,
    pub value: String,
    pub live: bool,
}

// ---------------------------------------------------------------------------
// Schema metadata
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnType {
    Str,
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationSchema {
    pub name: String,
    pub columns: Vec<Column>,
}

// ---------------------------------------------------------------------------
// Contract helpers
// ---------------------------------------------------------------------------

fn col(name: &str, col_type: ColumnType) -> Column {
    Column {
        name: name.to_string(),
        col_type,
    }
}

/// Returns the canonical relation schemas for the contract relations.
pub fn contract_schemas() -> Vec<RelationSchema> {
    vec![
        RelationSchema {
            name: "transpiler_version".to_string(),
            columns: vec![
                col("id", ColumnType::Str),
                col("hash", ColumnType::Str),
                col("lang", ColumnType::Str),
                col("live", ColumnType::Bool),
            ],
        },
        RelationSchema {
            name: "eval_request".to_string(),
            columns: vec![
                col("request_id", ColumnType::Str),
                col("version_id", ColumnType::Str),
                col("input_hash", ColumnType::Str),
                col("lojix_source_hash", ColumnType::Str),
            ],
        },
        RelationSchema {
            name: "eval_result".to_string(),
            columns: vec![
                col("request_id", ColumnType::Str),
                col("version_id", ColumnType::Str),
                col("input_hash", ColumnType::Str),
                col("output_kind", ColumnType::Str),
                col("value", ColumnType::Str),
                col("live", ColumnType::Bool),
            ],
        },
    ]
}

/// Returns CozoScript `:create` statements for all contract relations.
pub fn create_relations_cozoscript() -> String {
    cozoscript::create_relations_cozoscript()
}

/// Initialize contract relations in the given CozoDB instance.
/// Loads each `:create` statement from the embedded AI-init.cozo.
pub fn init(db: &criome_cozo::CriomeDb) -> Result<(), Box<dyn std::error::Error>> {
    let script = include_str!("../AI-init.cozo");
    for stmt in criome_cozo::split_cozo_statements(script) {
        if !stmt.trim().is_empty() {
            db.run_script(stmt)?;
        }
    }
    Ok(())
}
