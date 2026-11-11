//! Vector Search Engine & Embedding Repository
//! Sub-millisecond Cosine Similarity over 384-dimensional embeddings (MiniLM-L6-v2)
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub id: String,
    pub source_type: String,
    pub source_id: String,
    pub distance: f32,
    pub similarity: f32,
}

pub fn insert_vector_embedding(
    conn: &Connection,
    id: &str,
    source_type: &str,
    source_id: &str,
    embedding: &[f32],
) -> Result<(), rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    let embedding_json = serde_json::to_string(embedding).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        r#"
        INSERT INTO vec_memory_fallback (id, embedding_json, source_type, source_id, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT(id) DO UPDATE SET
            embedding_json = excluded.embedding_json,
            source_type = excluded.source_type,
            source_id = excluded.source_id,
            created_at = excluded.created_at
        "#,
        params![id, embedding_json, source_type, source_id, now],
    )?;

    Ok(())
}

pub fn search_similar_vectors(
    conn: &Connection,
    query_vec: &[f32],
    top_k: usize,
    source_filter: Option<String>,
) -> Result<Vec<VectorSearchResult>, rusqlite::Error> {
    let mut scored_results: Vec<VectorSearchResult> = Vec::new();

    if let Some(filter) = source_filter {
        let mut stmt = conn.prepare(
            "SELECT id, source_type, source_id, embedding_json FROM vec_memory_fallback WHERE source_type = ?1",
        )?;
        let mut rows = stmt.query(params![filter])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let source_type: String = row.get(1)?;
            let source_id: String = row.get(2)?;
            let embedding_json: String = row.get(3)?;

            if let Ok(target_vec) = serde_json::from_str::<Vec<f32>>(&embedding_json) {
                let sim = compute_cosine_similarity(query_vec, &target_vec);
                let distance = (1.0 - sim).max(0.0);

                scored_results.push(VectorSearchResult {
                    id,
                    source_type,
                    source_id,
                    distance,
                    similarity: sim,
                });
            }
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, source_type, source_id, embedding_json FROM vec_memory_fallback",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let source_type: String = row.get(1)?;
            let source_id: String = row.get(2)?;
            let embedding_json: String = row.get(3)?;

            if let Ok(target_vec) = serde_json::from_str::<Vec<f32>>(&embedding_json) {
                let sim = compute_cosine_similarity(query_vec, &target_vec);
                let distance = (1.0 - sim).max(0.0);

                scored_results.push(VectorSearchResult {
                    id,
                    source_type,
                    source_id,
                    distance,
                    similarity: sim,
                });
            }
        }
    }

    // Sort by highest similarity descending
    scored_results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
    scored_results.truncate(top_k);

    Ok(scored_results)
}

pub fn purge_vector_cache(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let rows_affected = conn.execute("DELETE FROM vec_memory_fallback", [])?;
    Ok(rows_affected)
}

/// High-performance cosine similarity computation
fn compute_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot: f32 = 0.0;
    let mut norm_a: f32 = 0.0;
    let mut norm_b: f32 = 0.0;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        (dot / denom).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        let v3 = vec![0.0, 1.0, 0.0];

        assert!((compute_cosine_similarity(&v1, &v2) - 1.0).abs() < 1e-5);
        assert!((compute_cosine_similarity(&v1, &v3) - 0.0).abs() < 1e-5);
    }
}
