/**
 * MindSet OS — File & RAG Knowledge Client
 * Strongly-typed IPC bridge to Rust fs and RAG subsystem
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export interface FileEntry {
  name: string;
  path: string;
  is_dir: bool;
  size_bytes: number;
  modified_at: string;
  extension?: string | null;
  is_indexed_rag: bool;
  subject_tag?: string | null;
  total_pages?: number | null;
  last_read_page?: number | null;
}

type bool = boolean;

export interface DirectoryListing {
  current_path: string;
  parent_path?: string | null;
  entries: FileEntry[];
  total_files: number;
  total_folders: number;
}

export interface DiskStats {
  total_bytes: number;
  free_bytes: number;
  used_bytes: number;
  used_percent: number;
  mount_point: string;
  filesystem: string;
}

export interface FileContentResponse {
  file_path: string;
  content_type: 'text' | 'base64';
  data: string;
  size_bytes: number;
}

export interface RagDocument {
  id: string;
  file_path: string;
  title: string;
  subject_tag?: string | null;
  total_pages: number;
  last_read_page: number;
  indexed_at: string;
  chunk_count: number;
}

export interface RetrievedChunk {
  chunk_id: string;
  page_number: number;
  chunk_index: number;
  content: string;
  similarity: number;
}

export interface RagIndexChunkInput {
  page_number: number;
  chunk_index: number;
  content: string;
  embedding?: number[];
}

export interface RagIndexInput {
  file_path: string;
  title: string;
  subject_tag?: string | null;
  total_pages: number;
  chunks: RagIndexChunkInput[];
}

async function invokeTauri<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[fileClient] Tauri invoke failed for '${cmd}':`, err);
    throw err;
  }
}

export const fileClient = {
  // ---------------------------------------------------------------------------
  // 1. Filesystem Operations
  // ---------------------------------------------------------------------------
  listDirectory: async (path?: string): Promise<DirectoryListing> => {
    try {
      return await invokeTauri<DirectoryListing>('fs_list_directory', { path });
    } catch {
      // Mocked offline structure if outside Tauri runtime
      return {
        current_path: path || 'Vault/Subjects',
        parent_path: path ? 'Vault' : null,
        entries: [
          {
            name: 'Physics',
            path: 'Vault/Subjects/Physics',
            is_dir: true,
            size_bytes: 0,
            modified_at: new Date().toISOString(),
            is_indexed_rag: false,
            subject_tag: 'Physics',
          },
          {
            name: 'Chemistry',
            path: 'Vault/Subjects/Chemistry',
            is_dir: true,
            size_bytes: 0,
            modified_at: new Date().toISOString(),
            is_indexed_rag: false,
            subject_tag: 'Chemistry',
          },
          {
            name: 'Mathematics',
            path: 'Vault/Subjects/Mathematics',
            is_dir: true,
            size_bytes: 0,
            modified_at: new Date().toISOString(),
            is_indexed_rag: false,
            subject_tag: 'Mathematics',
          },
          {
            name: 'Computer Science',
            path: 'Vault/Subjects/Computer Science',
            is_dir: true,
            size_bytes: 0,
            modified_at: new Date().toISOString(),
            is_indexed_rag: false,
            subject_tag: 'Computer Science',
          },
          {
            name: 'Class12_Electrostatics_Concepts.pdf',
            path: 'Vault/Textbooks/Class12_Electrostatics_Concepts.pdf',
            is_dir: false,
            size_bytes: 4_520_000,
            modified_at: new Date().toISOString(),
            extension: 'pdf',
            is_indexed_rag: true,
            subject_tag: 'Physics',
            total_pages: 48,
            last_read_page: 12,
          },
          {
            name: 'Calculus_Integration_Formulae.md',
            path: 'Vault/Notes/Calculus_Integration_Formulae.md',
            is_dir: false,
            size_bytes: 14_200,
            modified_at: new Date().toISOString(),
            extension: 'md',
            is_indexed_rag: false,
            subject_tag: 'Mathematics',
          },
        ],
        total_files: 2,
        total_folders: 4,
      };
    }
  },

  createFolder: async (parentPath: string, folderName: string): Promise<FileEntry> => {
    return invokeTauri<FileEntry>('fs_create_folder', { parentPath, folderName });
  },

  renameItem: async (currentPath: string, newName: string): Promise<FileEntry> => {
    return invokeTauri<FileEntry>('fs_rename_item', { currentPath, newName });
  },

  copyItem: async (sourcePath: string, targetFolder: string): Promise<FileEntry> => {
    return invokeTauri<FileEntry>('fs_copy_item', { sourcePath, targetFolder });
  },

  moveItem: async (sourcePath: string, targetFolder: string): Promise<FileEntry> => {
    return invokeTauri<FileEntry>('fs_move_item', { sourcePath, targetFolder });
  },

  moveToRecycleBin: async (path: string): Promise<boolean> => {
    return invokeTauri<boolean>('fs_move_to_recycle_bin', { path });
  },

  restoreFromRecycleBin: async (path: string, targetFolder?: string): Promise<boolean> => {
    return invokeTauri<boolean>('fs_restore_from_recycle_bin', { path, targetFolder });
  },

  emptyRecycleBin: async (): Promise<boolean> => {
    return invokeTauri<boolean>('fs_empty_recycle_bin');
  },

  compressToZip: async (sourcePaths: string[], outputZipPath: string): Promise<FileEntry> => {
    return invokeTauri<FileEntry>('fs_compress_to_zip', { sourcePaths, outputZipPath });
  },

  extractArchive: async (archivePath: string, destinationFolder: string): Promise<boolean> => {
    return invokeTauri<boolean>('fs_extract_archive', { archivePath, destinationFolder });
  },

  getDiskStats: async (): Promise<DiskStats> => {
    try {
      return await invokeTauri<DiskStats>('fs_get_disk_partition_stats');
    } catch {
      return {
        total_bytes: 512_000_000_000,
        free_bytes: 284_000_000_000,
        used_bytes: 228_000_000_000,
        used_percent: 44.5,
        mount_point: 'C:\\',
        filesystem: 'NTFS',
      };
    }
  },

  readFileContent: async (filePath: string, asBase64 = false): Promise<FileContentResponse> => {
    try {
      return await invokeTauri<FileContentResponse>('fs_read_file_text_or_base64', {
        filePath,
        asBase64,
      });
    } catch {
      return {
        file_path: filePath,
        content_type: asBase64 ? 'base64' : 'text',
        data: asBase64 ? '' : '# MindSet Study Notes\nContent preview available.',
        size_bytes: 1024,
      };
    }
  },

  // ---------------------------------------------------------------------------
  // 2. Zero-RAM RAG Knowledge Operations
  // ---------------------------------------------------------------------------
  rag: {
    indexDocument: async (input: RagIndexInput): Promise<RagDocument> => {
      return invokeTauri<RagDocument>('rag_index_document', { input });
    },

    updateLastReadPage: async (documentId: string, pageNumber: number): Promise<boolean> => {
      return invokeTauri<boolean>('rag_update_last_read_page', {
        documentId,
        pageNumber,
      });
    },

    getDocument: async (documentId: string): Promise<RagDocument | null> => {
      try {
        return await invokeTauri<RagDocument | null>('rag_get_document', { documentId });
      } catch {
        return null;
      }
    },

    listDocuments: async (subjectTag?: string): Promise<RagDocument[]> => {
      try {
        return await invokeTauri<RagDocument[]>('rag_list_documents', { subjectTag });
      } catch {
        return [];
      }
    },

    searchChunks: async (
      documentId: string,
      queryVector: number[],
      topK = 5
    ): Promise<RetrievedChunk[]> => {
      try {
        return await invokeTauri<RetrievedChunk[]>('rag_search_chunks', {
          documentId,
          queryVector,
          topK,
        });
      } catch {
        return [];
      }
    },

    deleteDocument: async (documentId: string): Promise<boolean> => {
      return invokeTauri<boolean>('rag_delete_document', { documentId });
    },
  },
};
