use crate::{ColumnType, contract_schemas};

fn column_type_to_cozo(ct: &ColumnType) -> &'static str {
    match ct {
        ColumnType::Str => "String",
        ColumnType::Int => "Int",
        ColumnType::Float => "Float",
        ColumnType::Bool => "Bool",
    }
}

/// Generates CozoScript `:create` blocks for all contract relations.
///
/// `eval_result` uses `request_id` as its key (separated by `=>`);
/// the other relations treat all columns as key columns.
pub fn create_relations_cozoscript() -> String {
    let schemas = contract_schemas();
    let mut out = String::new();

    for schema in &schemas {
        out.push_str(&format!(":create {} {{\n", schema.name));

        if schema.name == "eval_result" {
            // First column is the key, remaining are values.
            if let Some((key, vals)) = schema.columns.split_first() {
                out.push_str(&format!(
                    "  {}: {} =>\n",
                    key.name,
                    column_type_to_cozo(&key.col_type)
                ));
                for (i, col) in vals.iter().enumerate() {
                    let comma = if i + 1 < vals.len() { "," } else { "" };
                    out.push_str(&format!(
                        "  {}: {}{}\n",
                        col.name,
                        column_type_to_cozo(&col.col_type),
                        comma
                    ));
                }
            }
        } else {
            for (i, col) in schema.columns.iter().enumerate() {
                let comma = if i + 1 < schema.columns.len() {
                    ","
                } else {
                    ""
                };
                out.push_str(&format!(
                    "  {}: {}{}\n",
                    col.name,
                    column_type_to_cozo(&col.col_type),
                    comma
                ));
            }
        }

        out.push_str("}\n\n");
    }

    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cozoscript_contains_all_relations() {
        let script = create_relations_cozoscript();
        assert!(script.contains(":create transpiler_version"));
        assert!(script.contains(":create eval_request"));
        assert!(script.contains(":create eval_result"));
        assert!(script.contains("=>"));
    }
}
