//! MindSet OS — Filesystem & RAG Knowledge IPC Subsystem
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod explorer;

#[allow(unused_imports)]
pub use explorer::{
    DirectoryListing, DiskStats, FileContentResponse, FileEntry, RagChunkDto, RagDocumentDto,
    RagIndexChunkInput, RagIndexInput, RetrievedChunkDto,
};

use crate::kernel::MindSetKernel;

// ---------------------------------------------------------------------------
// 1. Filesystem Methods
// ---------------------------------------------------------------------------

pub async fn fs_list_directory(
    path: Option<String>,
    kernel: &MindSetKernel,
) -> Result<DirectoryListing, String> {
    let vault_base = kernel.paths.base_dir.join("vault");
    let core_conn = kernel.db.get_core_connection();
    let lock = core_conn.lock().map_err(|e| e.to_string())?;

    explorer::list_directory(&vault_base, &lock, path.as_deref())
}

pub async fn fs_create_folder(
    parent_path: String,
    folder_name: String,
) -> Result<FileEntry, String> {
    explorer::create_folder(&parent_path, &folder_name)
}

pub async fn fs_rename_item(current_path: String, new_name: String) -> Result<FileEntry, String> {
    explorer::rename_item(&current_path, &new_name)
}

pub async fn fs_copy_item(source_path: String, target_folder: String) -> Result<FileEntry, String> {
    explorer::copy_item(&source_path, &target_folder)
}

pub async fn fs_move_item(source_path: String, target_folder: String) -> Result<FileEntry, String> {
    explorer::move_item(&source_path, &target_folder)
}

pub async fn fs_move_to_recycle_bin(path: String, kernel: &MindSetKernel) -> Result<bool, String> {
    let vault_base = kernel.paths.base_dir.join("vault");
    explorer::move_to_recycle_bin(&vault_base, &path)
}

pub async fn fs_restore_from_recycle_bin(
    path: String,
    target_folder: Option<String>,
    kernel: &MindSetKernel,
) -> Result<bool, String> {
    let vault_base = kernel.paths.base_dir.join("vault");
    explorer::restore_from_recycle_bin(&vault_base, &path, target_folder.as_deref())
}

pub async fn fs_empty_recycle_bin(kernel: &MindSetKernel) -> Result<bool, String> {
    let vault_base = kernel.paths.base_dir.join("vault");
    explorer::empty_recycle_bin(&vault_base)
}

pub async fn fs_compress_to_zip(
    source_paths: Vec<String>,
    output_zip_path: String,
) -> Result<FileEntry, String> {
    explorer::compress_to_zip(&source_paths, &output_zip_path)
}

pub async fn fs_extract_archive(
    archive_path: String,
    destination_folder: String,
) -> Result<bool, String> {
    explorer::extract_archive(&archive_path, &destination_folder)
}

pub async fn fs_get_disk_partition_stats(kernel: &MindSetKernel) -> Result<DiskStats, String> {
    explorer::get_disk_partition_stats(&kernel.paths.base_dir)
}

pub async fn fs_read_file_text_or_base64(
    file_path: String,
    as_base64: bool,
) -> Result<FileContentResponse, String> {
    explorer::read_file_text_or_base64(&file_path, as_base64)
}

// ---------------------------------------------------------------------------
// 2. Zero-RAM RAG Knowledge Methods
// ---------------------------------------------------------------------------

pub async fn rag_index_document(
    input: RagIndexInput,
    kernel: &MindSetKernel,
) -> Result<RagDocumentDto, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    explorer::index_rag_document(&core_lock, &oplog_lock, &hlc, Some(&kernel.bus), input)
}

pub async fn rag_update_last_read_page(
    document_id: String,
    page_number: u32,
    kernel: &MindSetKernel,
) -> Result<bool, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    explorer::update_last_read_page(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &document_id,
        page_number,
    )
}

pub async fn rag_get_document(
    document_id: String,
    kernel: &MindSetKernel,
) -> Result<Option<RagDocumentDto>, String> {
    let core_conn = kernel.db.get_core_connection();
    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;

    explorer::get_rag_document(&core_lock, &document_id)
}

pub async fn rag_list_documents(
    subject_tag: Option<String>,
    kernel: &MindSetKernel,
) -> Result<Vec<RagDocumentDto>, String> {
    let core_conn = kernel.db.get_core_connection();
    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;

    explorer::list_rag_documents(&core_lock, subject_tag.as_deref())
}

pub async fn rag_search_chunks(
    document_id: String,
    query_vector: Vec<f32>,
    top_k: Option<usize>,
    kernel: &MindSetKernel,
) -> Result<Vec<RetrievedChunkDto>, String> {
    let core_conn = kernel.db.get_core_connection();
    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;

    explorer::search_rag_chunks(&core_lock, &document_id, &query_vector, top_k.unwrap_or(5))
}

pub async fn rag_delete_document(
    document_id: String,
    kernel: &MindSetKernel,
) -> Result<bool, String> {
    let core_conn = kernel.db.get_core_connection();
    let oplog_conn = kernel.db.get_oplog_connection();
    let hlc = kernel.db.get_hlc();

    let core_lock = core_conn.lock().map_err(|e| e.to_string())?;
    let oplog_lock = oplog_conn.lock().map_err(|e| e.to_string())?;

    explorer::delete_rag_document(
        &core_lock,
        &oplog_lock,
        &hlc,
        Some(&kernel.bus),
        &document_id,
    )
}
