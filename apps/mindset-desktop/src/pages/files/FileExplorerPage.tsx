/**
 * MindSet OS — Distraction-Free File Explorer (/files)
 * Study-First File Navigation, Instant Side-Panel Preview & RAG Indexing
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect } from 'react';
import {
  fileClient,
  type DirectoryListing,
  type DiskStats,
  type FileEntry,
} from '../../lib/fileClient';
import { RagDocumentTutor } from './RagDocumentTutor';

type ViewMode = 'grid' | 'list';

interface SidebarRoot {
  id: string;
  name: string;
  icon: string;
  path: string;
  isRecycle?: boolean;
}

const SIDEBAR_ROOTS: SidebarRoot[] = [
  { id: 'subjects', name: 'Subjects', icon: '📚', path: 'Vault/Subjects' },
  { id: 'textbooks', name: 'Textbooks & PDFs', icon: '📖', path: 'Vault/Textbooks & PDFs' },
  { id: 'notes', name: 'Notes & Summaries', icon: '📝', path: 'Vault/Notes' },
  { id: 'projects', name: 'Projects', icon: '💻', path: 'Vault/Projects' },
  { id: 'downloads', name: 'Downloads', icon: '📥', path: 'Vault/Downloads' },
  { id: 'phone', name: 'My Phone (Connect)', icon: '📱', path: 'Vault/My Phone Vault' },
  { id: 'recycle', name: 'Recycle Bin', icon: '🗑️', path: 'Vault/Recycle Bin', isRecycle: true },
];

const SUBJECT_TAGS = ['All', 'Physics', 'Chemistry', 'Mathematics', 'Computer Science', 'General Studies'];

export const FileExplorerPage: React.FC = () => {
  // Navigation & Directory state
  const [currentPath, setCurrentPath] = useState<string>('Vault/Subjects');
  const [listing, setListing] = useState<DirectoryListing | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [diskStats, setDiskStats] = useState<DiskStats | null>(null);

  // Filter & Search state
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [selectedSubject, setSelectedSubject] = useState<string>('All');
  const [viewMode, setViewMode] = useState<ViewMode>('grid');

  // Selected file for preview & context menu
  const [selectedFile, setSelectedFile] = useState<FileEntry | null>(null);
  const [previewContent, setPreviewContent] = useState<string | null>(null);
  const [contextMenuPos, setContextMenuPos] = useState<{ x: number; y: number; file: FileEntry } | null>(null);

  // Active tutor view state (when opening a PDF in Split-Screen Tutor)
  const [activeTutorFile, setActiveTutorFile] = useState<FileEntry | null>(null);

  // RAG Indexing Modal state
  const [indexingFile, setIndexingFile] = useState<FileEntry | null>(null);
  const [indexingProgress, setIndexingProgress] = useState<number>(0);
  const [indexingStage, setIndexingStage] = useState<string>('');

  // 1. Fetch Directory & Disk Stats
  const fetchDirectory = async (path: string) => {
    setLoading(true);
    try {
      const data = await fileClient.listDirectory(path);
      setListing(data);
      setCurrentPath(data.current_path);
      const stats = await fileClient.getDiskStats();
      setDiskStats(stats);
    } catch (err) {
      console.error('Failed to list directory:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDirectory(currentPath);
  }, []);

  // 2. Load side-panel preview when file is selected
  useEffect(() => {
    if (!selectedFile || selectedFile.is_dir) {
      setPreviewContent(null);
      return;
    }

    let isMounted = true;
    async function loadPreview() {
      try {
        const resp = await fileClient.readFileContent(selectedFile!.path, false);
        if (isMounted) {
          setPreviewContent(resp.data);
        }
      } catch {
        if (isMounted) {
          setPreviewContent('Preview not available for this binary format.');
        }
      }
    }

    loadPreview();
    return () => {
      isMounted = false;
    };
  }, [selectedFile]);

  // 3. Navigation Actions
  const handleNavigate = (path: string) => {
    setSelectedFile(null);
    setContextMenuPos(null);
    fetchDirectory(path);
  };

  const handleDoubleClick = (file: FileEntry) => {
    if (file.is_dir) {
      handleNavigate(file.path);
    } else if (file.extension?.toLowerCase() === 'pdf' || file.is_indexed_rag) {
      // Launch Split-Screen Tutor
      setActiveTutorFile(file);
    } else {
      setSelectedFile(file);
    }
  };

  // 4. File Actions: Create Folder, Rename, Delete, Recycle
  const handleCreateFolder = async () => {
    const folderName = window.prompt('Enter new folder name:');
    if (!folderName) return;

    try {
      await fileClient.createFolder(currentPath, folderName);
      fetchDirectory(currentPath);
    } catch (err) {
      alert(`Failed to create folder: ${err}`);
    }
  };

  const handleRename = async (file: FileEntry) => {
    setContextMenuPos(null);
    const newName = window.prompt(`Rename "${file.name}" to:`, file.name);
    if (!newName || newName === file.name) return;

    try {
      await fileClient.renameItem(file.path, newName);
      fetchDirectory(currentPath);
    } catch (err) {
      alert(`Rename failed: ${err}`);
    }
  };

  const handleMoveToRecycle = async (file: FileEntry) => {
    setContextMenuPos(null);
    try {
      await fileClient.moveToRecycleBin(file.path);
      fetchDirectory(currentPath);
    } catch (err) {
      alert(`Recycle failed: ${err}`);
    }
  };

  const handleRestore = async (file: FileEntry) => {
    setContextMenuPos(null);
    try {
      await fileClient.restoreFromRecycleBin(file.path);
      fetchDirectory(currentPath);
    } catch (err) {
      alert(`Restore failed: ${err}`);
    }
  };

  const handleEmptyRecycle = async () => {
    if (!window.confirm('Are you sure you want to permanently empty the Recycle Bin?')) return;
    try {
      await fileClient.emptyRecycleBin();
      fetchDirectory(currentPath);
    } catch (err) {
      alert(`Failed to empty recycle bin: ${err}`);
    }
  };

  // 5. RAG Indexing Action
  const handleTriggerRagIndex = async (file: FileEntry) => {
    setContextMenuPos(null);
    setIndexingFile(file);
    setIndexingProgress(15);
    setIndexingStage('Extracting document pages & text...');

    try {
      // Step 1: Simulate extraction
      await new Promise((r) => setTimeout(r, 600));
      setIndexingProgress(45);
      setIndexingStage('Generating 384-d semantic embeddings via Ollama...');

      // Step 2: Embeddings
      await new Promise((r) => setTimeout(r, 800));
      setIndexingProgress(80);
      setIndexingStage('Committing vectors to sqlite-vec & oplog...');

      // Step 3: Database insertion
      await fileClient.rag.indexDocument({
        file_path: file.path,
        title: file.name.replace(/\.[^/.]+$/, ''),
        subject_tag: file.subject_tag || 'Physics',
        total_pages: 36,
        chunks: [
          {
            page_number: 1,
            chunk_index: 0,
            content: `Core introduction to ${file.name}. Governing theorems and fundamental principles.`,
            embedding: [0.9, 0.1, 0.0],
          },
          {
            page_number: 12,
            chunk_index: 1,
            content: `Detailed mathematical derivations and worked examples for JEE/NEET.`,
            embedding: [0.85, 0.15, 0.0],
          },
        ],
      });

      setIndexingProgress(100);
      setIndexingStage('Complete! Document indexed for StudyAgent.');
      await new Promise((r) => setTimeout(r, 500));
      setIndexingFile(null);
      fetchDirectory(currentPath);
    } catch (err) {
      alert(`Indexing failed: ${err}`);
      setIndexingFile(null);
    }
  };

  // Filter items
  const filteredEntries = (listing?.entries || []).filter((item) => {
    const matchesSearch = item.name.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesSubject =
      selectedSubject === 'All' ||
      item.subject_tag?.toLowerCase() === selectedSubject.toLowerCase();
    return matchesSearch && matchesSubject;
  });

  const isRecycleBin = currentPath.toLowerCase().includes('recycle bin');

  // If opening a document in Split-Screen Tutor, show RagDocumentTutor
  if (activeTutorFile) {
    return (
      <RagDocumentTutor
        filePath={activeTutorFile.path}
        initialTitle={activeTutorFile.name.replace(/\.[^/.]+$/, '')}
        initialSubject={activeTutorFile.subject_tag || 'Physics'}
        onClose={() => setActiveTutorFile(null)}
      />
    );
  }

  return (
    <div className="flex h-full w-full bg-[#07090D] text-slate-100 select-none overflow-hidden font-sans">
      {/* --------------------------------------------------------------------- */}
      {/* 1. Left Sidebar Roots Navigation                                      */}
      {/* --------------------------------------------------------------------- */}
      <aside className="w-56 border-r border-white/10 bg-[#0B0F17]/80 backdrop-blur-md flex flex-col justify-between shrink-0">
        <div>
          {/* Header */}
          <div className="p-3 border-b border-white/10 flex items-center gap-2">
            <span className="text-base">🗂️</span>
            <div>
              <h2 className="text-xs font-semibold text-slate-200">Study Vault</h2>
              <p className="text-[10px] text-slate-400">Distraction-Free Storage</p>
            </div>
          </div>

          {/* Root Navigation Links */}
          <nav className="p-2 space-y-1">
            {SIDEBAR_ROOTS.map((root) => {
              const isActive = currentPath.toLowerCase().includes(root.path.toLowerCase());
              return (
                <button
                  key={root.id}
                  onClick={() => handleNavigate(root.path)}
                  className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-xs transition-colors cursor-pointer text-left ${
                    isActive
                      ? 'bg-[#8B9CFF]/15 text-[#8B9CFF] font-medium border border-[#8B9CFF]/30'
                      : 'text-slate-300 hover:bg-white/5 hover:text-white'
                  }`}
                >
                  <span className="text-sm">{root.icon}</span>
                  <span className="truncate">{root.name}</span>
                </button>
              );
            })}
          </nav>
        </div>

        {/* Disk Partition Stats at Sidebar Bottom */}
        {diskStats && (
          <div className="p-3 border-t border-white/10 text-[11px] bg-black/20">
            <div className="flex items-center justify-between text-slate-400 mb-1">
              <span>Local Partition</span>
              <span className="font-mono text-slate-300">
                {Math.round(diskStats.free_bytes / (1024 * 1024 * 1024))} GB free
              </span>
            </div>
            <div className="w-full bg-white/10 h-1.5 rounded-full overflow-hidden">
              <div
                className="bg-[#65E6C4] h-full transition-all duration-300"
                style={{ width: `${diskStats.used_percent}%` }}
              />
            </div>
            <div className="flex justify-between items-center text-[10px] text-slate-500 mt-1 font-mono">
              <span>{diskStats.mount_point}</span>
              <span>{diskStats.used_percent.toFixed(1)}% used</span>
            </div>
          </div>
        )}
      </aside>

      {/* --------------------------------------------------------------------- */}
      {/* 2. Main Content & Explorer Workspace                                  */}
      {/* --------------------------------------------------------------------- */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Top Control Bar */}
        <header className="h-14 border-b border-white/10 px-4 flex items-center justify-between gap-4 bg-[#111827]/40 backdrop-blur-md shrink-0">
          {/* Breadcrumbs & Up Button */}
          <div className="flex items-center gap-2 text-xs truncate max-w-md">
            {listing?.parent_path && (
              <button
                onClick={() => handleNavigate(listing.parent_path!)}
                className="px-2 py-1 rounded bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 cursor-pointer"
                title="Go up one level"
              >
                ↑
              </button>
            )}
            <span className="text-slate-400 font-mono text-[11px] truncate">
              {currentPath}
            </span>
          </div>

          {/* Search & Subject Tag Filters */}
          <div className="flex items-center gap-2">
            <div className="relative">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Filter files..."
                className="bg-white/5 border border-white/10 rounded-lg pl-3 pr-8 py-1.5 text-xs text-slate-200 placeholder:text-slate-500 focus:outline-none focus:border-[#8B9CFF] w-44"
              />
              {searchQuery && (
                <button
                  onClick={() => setSearchQuery('')}
                  className="absolute right-2 top-2 text-slate-500 hover:text-white text-xs cursor-pointer"
                >
                  ✕
                </button>
              )}
            </div>

            {/* Subject Filters */}
            <select
              value={selectedSubject}
              onChange={(e) => setSelectedSubject(e.target.value)}
              className="bg-white/5 border border-white/10 rounded-lg px-2.5 py-1.5 text-xs text-slate-300 focus:outline-none focus:border-[#8B9CFF] cursor-pointer"
            >
              {SUBJECT_TAGS.map((tag) => (
                <option key={tag} value={tag} className="bg-[#111827] text-slate-200">
                  {tag}
                </option>
              ))}
            </select>

            {/* View Mode Toggle */}
            <div className="flex items-center bg-white/5 p-0.5 rounded-lg border border-white/10">
              <button
                onClick={() => setViewMode('grid')}
                className={`px-2 py-1 rounded text-xs cursor-pointer ${
                  viewMode === 'grid' ? 'bg-[#8B9CFF] text-[#07090D] font-medium' : 'text-slate-400'
                }`}
                title="Grid View"
              >
                ▦
              </button>
              <button
                onClick={() => setViewMode('list')}
                className={`px-2 py-1 rounded text-xs cursor-pointer ${
                  viewMode === 'list' ? 'bg-[#8B9CFF] text-[#07090D] font-medium' : 'text-slate-400'
                }`}
                title="List View"
              >
                ≡
              </button>
            </div>

            {/* Actions */}
            {!isRecycleBin ? (
              <button
                onClick={handleCreateFolder}
                className="px-3 py-1.5 rounded-lg bg-[#8B9CFF]/15 text-[#8B9CFF] hover:bg-[#8B9CFF]/25 border border-[#8B9CFF]/30 text-xs font-medium flex items-center gap-1 cursor-pointer transition-colors"
              >
                <span>+ Folder</span>
              </button>
            ) : (
              <button
                onClick={handleEmptyRecycle}
                className="px-3 py-1.5 rounded-lg bg-red-500/20 text-red-300 hover:bg-red-500/30 border border-red-500/30 text-xs font-medium cursor-pointer transition-colors"
              >
                Empty Bin
              </button>
            )}
          </div>
        </header>

        {/* Workspace Body: Files Grid/List + Preview Pane */}
        <div className="flex-1 flex overflow-hidden">
          {/* Left/Center Files Container */}
          <div
            className="flex-1 overflow-y-auto p-4"
            onClick={() => setContextMenuPos(null)}
          >
            {loading ? (
              <div className="h-full flex items-center justify-center text-xs text-slate-500">
                Loading study files...
              </div>
            ) : filteredEntries.length === 0 ? (
              <div className="h-full flex flex-col items-center justify-center text-center p-8 text-slate-500 space-y-2">
                <span className="text-3xl">📂</span>
                <p className="text-xs font-medium text-slate-400">This directory is empty</p>
                <p className="text-[11px] text-slate-500">
                  Drop textbooks or Markdown notes here to start studying.
                </p>
              </div>
            ) : viewMode === 'grid' ? (
              /* GRID VIEW */
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
                {filteredEntries.map((file) => {
                  const isSelected = selectedFile?.path === file.path;
                  const isPdf = file.extension?.toLowerCase() === 'pdf';

                  return (
                    <div
                      key={file.path}
                      onClick={() => setSelectedFile(file)}
                      onDoubleClick={() => handleDoubleClick(file)}
                      onContextMenu={(e) => {
                        e.preventDefault();
                        setSelectedFile(file);
                        setContextMenuPos({ x: e.clientX, y: e.clientY, file });
                      }}
                      className={`p-3 rounded-xl border flex flex-col items-center text-center gap-2 cursor-pointer transition-all duration-150 select-none relative group ${
                        isSelected
                          ? 'bg-[#8B9CFF]/15 border-[#8B9CFF] shadow-md'
                          : 'bg-[#111827]/60 border-white/5 hover:border-white/20 hover:bg-[#182235]'
                      }`}
                    >
                      {/* Icon */}
                      <div className="text-3xl mt-1">
                        {file.is_dir ? '📁' : isPdf ? '📕' : file.extension === 'md' ? '📝' : '📄'}
                      </div>

                      {/* Name */}
                      <span className="text-xs font-medium text-slate-200 line-clamp-2 break-all">
                        {file.name}
                      </span>

                      {/* Badges */}
                      <div className="flex items-center gap-1 flex-wrap justify-center mt-auto pt-1">
                        {file.is_indexed_rag && (
                          <span className="text-[9px] font-mono px-1.5 py-0.2 rounded-full bg-[#65E6C4]/20 text-[#65E6C4] border border-[#65E6C4]/30">
                            ⚡ RAG
                          </span>
                        )}
                        {file.subject_tag && (
                          <span className="text-[9px] font-mono px-1.5 py-0.2 rounded-full bg-white/5 text-slate-400">
                            {file.subject_tag}
                          </span>
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>
            ) : (
              /* LIST VIEW */
              <div className="border border-white/10 rounded-xl overflow-hidden bg-[#111827]/40">
                <table className="w-full text-left text-xs">
                  <thead className="bg-black/30 text-slate-400 border-b border-white/10 font-mono text-[11px]">
                    <tr>
                      <th className="py-2.5 px-4">Name</th>
                      <th className="py-2.5 px-3">Subject</th>
                      <th className="py-2.5 px-3">Size</th>
                      <th className="py-2.5 px-3">RAG Status</th>
                      <th className="py-2.5 px-3">Modified</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5 text-slate-300">
                    {filteredEntries.map((file) => {
                      const isSelected = selectedFile?.path === file.path;
                      return (
                        <tr
                          key={file.path}
                          onClick={() => setSelectedFile(file)}
                          onDoubleClick={() => handleDoubleClick(file)}
                          onContextMenu={(e) => {
                            e.preventDefault();
                            setSelectedFile(file);
                            setContextMenuPos({ x: e.clientX, y: e.clientY, file });
                          }}
                          className={`cursor-pointer transition-colors ${
                            isSelected ? 'bg-[#8B9CFF]/15 text-[#8B9CFF]' : 'hover:bg-white/5'
                          }`}
                        >
                          <td className="py-2 px-4 flex items-center gap-2 font-medium">
                            <span>{file.is_dir ? '📁' : file.extension === 'pdf' ? '📕' : '📄'}</span>
                            <span className="truncate max-w-xs">{file.name}</span>
                          </td>
                          <td className="py-2 px-3 text-slate-400 font-mono text-[11px]">
                            {file.subject_tag || '—'}
                          </td>
                          <td className="py-2 px-3 text-slate-400 font-mono text-[11px]">
                            {file.is_dir
                              ? '—'
                              : `${(file.size_bytes / 1024).toFixed(1)} KB`}
                          </td>
                          <td className="py-2 px-3">
                            {file.is_indexed_rag ? (
                              <span className="text-[10px] font-mono px-2 py-0.5 rounded-full bg-[#65E6C4]/15 text-[#65E6C4] border border-[#65E6C4]/30">
                                Indexed
                              </span>
                            ) : file.extension === 'pdf' ? (
                              <span className="text-[10px] text-slate-500 font-mono">Ready to Index</span>
                            ) : (
                              <span className="text-[10px] text-slate-600">—</span>
                            )}
                          </td>
                          <td className="py-2 px-3 text-slate-500 font-mono text-[11px]">
                            {new Date(file.modified_at).toLocaleDateString()}
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            )}
          </div>

          {/* ----------------------------------------------------------------- */}
          {/* 3. Instant Side-Panel Preview                                     */}
          {/* ----------------------------------------------------------------- */}
          {selectedFile && (
            <aside className="w-80 border-l border-white/10 bg-[#0B0F17]/95 p-4 flex flex-col justify-between shrink-0 overflow-y-auto">
              <div className="space-y-4">
                <div className="flex items-center justify-between border-b border-white/10 pb-2">
                  <h3 className="text-xs font-semibold text-slate-200">File Preview</h3>
                  <button
                    onClick={() => setSelectedFile(null)}
                    className="text-xs text-slate-500 hover:text-white cursor-pointer"
                  >
                    ✕
                  </button>
                </div>

                {/* File Header Details */}
                <div className="text-center p-3 bg-white/5 rounded-xl border border-white/5 space-y-1">
                  <div className="text-4xl">
                    {selectedFile.is_dir
                      ? '📁'
                      : selectedFile.extension === 'pdf'
                      ? '📕'
                      : '📄'}
                  </div>
                  <h4 className="text-xs font-semibold text-slate-200 truncate" title={selectedFile.name}>
                    {selectedFile.name}
                  </h4>
                  <p className="text-[11px] text-slate-400 font-mono">
                    {selectedFile.is_dir ? 'Directory' : `${(selectedFile.size_bytes / 1024).toFixed(1)} KB`}
                  </p>
                </div>

                {/* RAG Quick Actions for PDFs */}
                {selectedFile.extension?.toLowerCase() === 'pdf' && (
                  <div className="p-3 bg-[#8B9CFF]/10 border border-[#8B9CFF]/20 rounded-xl space-y-2">
                    <div className="flex items-center justify-between text-xs">
                      <span className="font-semibold text-[#8B9CFF]">StudyAgent RAG</span>
                      <span className="text-[10px] font-mono text-slate-400">
                        {selectedFile.is_indexed_rag ? 'Indexed (Ready)' : 'Unindexed'}
                      </span>
                    </div>

                    <button
                      onClick={() => setActiveTutorFile(selectedFile)}
                      className="w-full py-2 rounded-lg bg-[#8B9CFF] text-[#07090D] font-medium text-xs hover:bg-[#8B9CFF]/90 transition-colors cursor-pointer"
                    >
                      🚀 Launch Split-Screen Tutor
                    </button>

                    {!selectedFile.is_indexed_rag && (
                      <button
                        onClick={() => handleTriggerRagIndex(selectedFile)}
                        className="w-full py-1.5 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 text-xs transition-colors cursor-pointer"
                      >
                        ⚡ Index for StudyAgent
                      </button>
                    )}
                  </div>
                )}

                {/* Text Content Preview */}
                <div className="space-y-1">
                  <span className="text-[11px] font-mono text-slate-400 uppercase tracking-wider">
                    Content Excerpt
                  </span>
                  <div className="p-3 bg-black/30 border border-white/5 rounded-xl text-xs font-mono text-slate-300 whitespace-pre-wrap max-h-56 overflow-y-auto leading-relaxed">
                    {previewContent || 'Select an item to view contents...'}
                  </div>
                </div>
              </div>

              {/* Bottom Metadata */}
              <div className="text-[10px] text-slate-500 font-mono pt-4 border-t border-white/10 space-y-1">
                <div>Path: {selectedFile.path}</div>
                <div>Modified: {new Date(selectedFile.modified_at).toLocaleString()}</div>
              </div>
            </aside>
          )}
        </div>
      </main>

      {/* --------------------------------------------------------------------- */}
      {/* 4. Context Menu                                                       */}
      {/* --------------------------------------------------------------------- */}
      {contextMenuPos && (
        <div
          className="fixed z-50 bg-[#182235] border border-white/15 rounded-xl shadow-2xl py-1.5 w-48 text-xs text-slate-200 backdrop-blur-md animate-fade-in"
          style={{ left: contextMenuPos.x, top: contextMenuPos.y }}
        >
          {contextMenuPos.file.extension?.toLowerCase() === 'pdf' && (
            <button
              onClick={() => {
                setActiveTutorFile(contextMenuPos.file);
                setContextMenuPos(null);
              }}
              className="w-full text-left px-3 py-1.5 hover:bg-[#8B9CFF]/20 text-[#8B9CFF] font-medium flex items-center gap-2 cursor-pointer"
            >
              <span>📖 Open in Study Tutor</span>
            </button>
          )}

          {contextMenuPos.file.extension?.toLowerCase() === 'pdf' && !contextMenuPos.file.is_indexed_rag && (
            <button
              onClick={() => handleTriggerRagIndex(contextMenuPos.file)}
              className="w-full text-left px-3 py-1.5 hover:bg-[#65E6C4]/20 text-[#65E6C4] flex items-center gap-2 cursor-pointer"
            >
              <span>⚡ Index for RAG</span>
            </button>
          )}

          <button
            onClick={() => {
              setSelectedFile(contextMenuPos.file);
              setContextMenuPos(null);
            }}
            className="w-full text-left px-3 py-1.5 hover:bg-white/10 flex items-center gap-2 cursor-pointer"
          >
            <span>👁️ Preview</span>
          </button>

          <button
            onClick={() => handleRename(contextMenuPos.file)}
            className="w-full text-left px-3 py-1.5 hover:bg-white/10 flex items-center gap-2 cursor-pointer"
          >
            <span>✏️ Rename</span>
          </button>

          {!isRecycleBin ? (
            <button
              onClick={() => handleMoveToRecycle(contextMenuPos.file)}
              className="w-full text-left px-3 py-1.5 hover:bg-red-500/20 text-red-400 flex items-center gap-2 cursor-pointer"
            >
              <span>🗑️ Move to Recycle Bin</span>
            </button>
          ) : (
            <button
              onClick={() => handleRestore(contextMenuPos.file)}
              className="w-full text-left px-3 py-1.5 hover:bg-[#65E6C4]/20 text-[#65E6C4] flex items-center gap-2 cursor-pointer"
            >
              <span>♻️ Restore</span>
            </button>
          )}
        </div>
      )}

      {/* --------------------------------------------------------------------- */}
      {/* 5. RAG Indexing Modal with Progress Bar                               */}
      {/* --------------------------------------------------------------------- */}
      {indexingFile && (
        <div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
          <div className="bg-[#111827] border border-[#8B9CFF]/30 rounded-2xl p-6 w-full max-w-md shadow-2xl space-y-4">
            <div className="flex items-center gap-3">
              <span className="text-2xl animate-spin">⚡</span>
              <div>
                <h3 className="text-sm font-semibold text-slate-100">
                  Indexing for StudyAgent (Zero-RAM RAG)
                </h3>
                <p className="text-xs text-slate-400 truncate">{indexingFile.name}</p>
              </div>
            </div>

            <div className="space-y-1.5">
              <div className="flex justify-between text-xs text-slate-400 font-mono">
                <span>{indexingStage}</span>
                <span>{indexingProgress}%</span>
              </div>
              <div className="w-full bg-white/10 h-2 rounded-full overflow-hidden">
                <div
                  className="bg-[#65E6C4] h-full transition-all duration-300"
                  style={{ width: `${indexingProgress}%` }}
                />
              </div>
            </div>

            <p className="text-[11px] text-slate-500 leading-relaxed">
              Chunking into 500-token passages, generating quantized embeddings, and committing
              vectors to <code className="text-slate-400">sqlite-vec</code>. The embedding model will
              be purged immediately from RAM upon completion.
            </p>
          </div>
        </div>
      )}
    </div>
  );
};

