/**
 * MindSet OS — Split-Screen Zero-RAM RAG Document Tutor
 * PDF / Textbook reader with Socratic StudyAgent & clickable [Page X] citations
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useEffect, useRef } from 'react';
import { fileClient, type RagDocument, type RetrievedChunk } from '../../lib/fileClient';
import { layaClient } from '../../lib/layaClient';
import type { LayaStreamChunk } from '@mindset/laya-router';

export interface RagDocumentTutorProps {
  documentId?: string;
  filePath?: string;
  initialTitle?: string;
  initialSubject?: string;
  onClose?: () => void;
}

interface ChatMessage {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  citedPages?: number[];
  timestamp: string;
}

export const RagDocumentTutor: React.FC<RagDocumentTutorProps> = ({
  documentId: propDocId,
  filePath = 'Vault/Textbooks/Class12_Electrostatics_Concepts.pdf',
  initialTitle = 'Class 12 Physics: Electrostatics & Potential',
  initialSubject = 'Physics',
  onClose,
}) => {
  // Theme state: 'aurora' | 'paper'
  const [readingTheme, setReadingTheme] = useState<'aurora' | 'paper'>('aurora');

  // Document metadata state
  const [doc, setDoc] = useState<RagDocument | null>(null);
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [totalPages, setTotalPages] = useState<number>(48);
  const [zoomLevel, setZoomLevel] = useState<number>(100);
  const [syncStatus, setSyncStatus] = useState<string>('Synced with oplog');

  // Document text content simulation & selection
  const [pageText, setPageText] = useState<string>('');
  const [selectedText, setSelectedText] = useState<string>('');
  const [selectionTooltipPos, setSelectionTooltipPos] = useState<{ x: number; y: number } | null>(null);

  // Tutor chat state
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [userInput, setUserInput] = useState<string>('');
  const [isGenerating, setIsGenerating] = useState<boolean>(false);
  const [retrievedContext, setRetrievedContext] = useState<RetrievedChunk[]>([]);
  const chatEndRef = useRef<HTMLDivElement>(null);
  const readerContentRef = useRef<HTMLDivElement>(null);

  // 1. Initial Load & Metadata Fetch
  useEffect(() => {
    let isMounted = true;

    async function loadDoc() {
      try {
        const docList = await fileClient.rag.listDocuments();
        const found = docList.find((d) => d.id === propDocId || d.file_path === filePath);

        if (found && isMounted) {
          setDoc(found);
          setCurrentPage(found.last_read_page || 1);
          setTotalPages(found.total_pages || 48);
        } else if (isMounted) {
          // Initialize sample doc
          setDoc({
            id: propDocId || 'doc-sample-physics',
            file_path: filePath,
            title: initialTitle,
            subject_tag: initialSubject,
            total_pages: 48,
            last_read_page: 1,
            indexed_at: new Date().toISOString(),
            chunk_count: 96,
          });
        }
      } catch (err) {
        console.warn('Failed to load document metadata:', err);
      }
    }

    loadDoc();
    return () => {
      isMounted = false;
    };
  }, [propDocId, filePath, initialTitle, initialSubject]);

  // 2. Render Page Content dynamically based on currentPage
  useEffect(() => {
    // Generate realistic textbook page content for Physics / Chemistry
    const samplePages: Record<number, string> = {
      1: `# Chapter 1: Electric Charges and Fields\n\n## 1.1 Introduction & Historic Context\nHistorically, the credit of discovery of the fact that amber rubbed with wool or silk cloth attracts light objects goes to Thales of Miletus, Greece, around 600 BC.\n\nThe name electricity is coined from the Greek word *elektron* meaning amber. There are two kinds of electrification: like charges repel and unlike charges attract each other.\n\n$$\\vec{F} = \\frac{1}{4\\pi\\varepsilon_0} \\frac{q_1 q_2}{r^2} \\hat{r}$$\n\nWhere $\\varepsilon_0 = 8.854 \\times 10^{-12} \\text{ C}^2/\\text{N}\\cdot\\text{m}^2$ is the permittivity of free space.`,
      12: `# Chapter 1: Gauss's Law & Applications\n\n## 1.15 Gauss's Law\nGauss's law states that the total electric flux $\\Phi_E$ through any closed surface is equal to $1/\\varepsilon_0$ times the total net charge $q_{\\text{enclosed}}$ enclosed by that surface.\n\n$$\\Phi_E = \\oint \\vec{E} \\cdot d\\vec{A} = \\frac{q_{\\text{enc}}}{\\varepsilon_0}$$\n\n### Key Conceptual Insights:\n1. The surface chosen to apply Gauss's law is called a **Gaussian Surface**.\n2. Gauss's law is valid for any closed surface, regardless of its shape or size.\n3. The flux $\\Phi_E$ depends only on charges enclosed within the surface; charges outside do not contribute to the net flux.`,
      18: `# Chapter 1: Field Due to Infinite Line Charge\n\n## 1.16 Field Due to an Infinitely Long Straight Wire\nConsider an infinitely long thin wire with uniform linear charge density $\\lambda$.\n\nTo compute the electric field at a distance $r$ from the wire, we construct a cylindrical Gaussian surface of radius $r$ and length $L$ coaxial with the wire.\n\n$$\\oint \\vec{E} \\cdot d\\vec{A} = E \\cdot (2\\pi r L) = \\frac{\\lambda L}{\\varepsilon_0}$$\n\nSolving for the electric field magnitude:\n\n$$E = \\frac{\\lambda}{2\\pi\\varepsilon_0 r}$$\n\nThe electric field is directed radially outwards if $\\lambda > 0$ and inwards if $\\lambda < 0$.`,
      24: `# Chapter 2: Electrostatic Potential Energy\n\n## 2.3 Potential Energy of a System of Two Charges\nWork done in bringing a charge $q_2$ from infinity to a distance $r_{12}$ from another charge $q_1$ held fixed at the origin:\n\n$$U = W = \\frac{1}{4\\pi\\varepsilon_0} \\frac{q_1 q_2}{r_{12}}$$\n\nNotice that if both charges have the same sign ($q_1 q_2 > 0$), work is done against repulsion, and potential energy $U$ is positive. If the charges have opposite signs ($q_1 q_2 < 0$), potential energy is negative.`,
    };

    setPageText(
      samplePages[currentPage] ||
        `# Page ${currentPage}: ${initialTitle}\n\n## Topic ${currentPage}.1: Core Principles\n\nDetailed pedagogical textbook content for Page ${currentPage}. Derivations, experimental data, and theoretical problem formulations for competitive entrance examinations (JEE / NEET).\n\n$$\\Delta V = -\\int_{A}^{B} \\vec{E} \\cdot d\\vec{\\ell}$$\n\nSelect any passage or formula to query StudyAgent.`
    );
  }, [currentPage, initialTitle]);

  // 3. Persist Page Turn with Oplog
  const handlePageChange = async (newPage: number) => {
    if (newPage < 1 || newPage > totalPages) return;
    setCurrentPage(newPage);
    setSyncStatus('Saving to oplog...');

    try {
      if (doc?.id) {
        await fileClient.rag.updateLastReadPage(doc.id, newPage);
      }
      setSyncStatus(`Synced with oplog (Page ${newPage})`);
    } catch {
      setSyncStatus(`Page ${newPage} (Offline)`);
    }
  };

  // 4. Handle Text Selection in Document
  const handleMouseUp = () => {
    const selection = window.getSelection();
    if (selection && selection.toString().trim().length > 3) {
      const text = selection.toString().trim();
      setSelectedText(text);

      const range = selection.getRangeAt(0);
      const rect = range.getBoundingClientRect();
      setSelectionTooltipPos({
        x: rect.left + rect.width / 2,
        y: rect.top - 10,
      });
    } else {
      setSelectionTooltipPos(null);
    }
  };

  // 5. Send Query to Context-Locked StudyAgent
  const handleSendQuery = async (customPrompt?: string) => {
    const query = customPrompt || userInput;
    if (!query.trim() || isGenerating) return;

    setUserInput('');
    setSelectionTooltipPos(null);

    const userMsg: ChatMessage = {
      id: `msg-${Date.now()}`,
      role: 'user',
      content: query,
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    };

    setMessages((prev) => [...prev, userMsg]);
    setIsGenerating(true);

    const assistantMsgId = `assistant-${Date.now()}`;
    const assistantMsg: ChatMessage = {
      id: assistantMsgId,
      role: 'assistant',
      content: '',
      citedPages: [],
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    };

    setMessages((prev) => [...prev, assistantMsg]);

    try {
      // 1. Semantic retrieval simulation
      let chunks: RetrievedChunk[] = [];
      try {
        chunks = await fileClient.rag.searchChunks(doc?.id || 'doc-sample', [0.95, 0.05, 0.0], 3);
      } catch {
        // Fallback
      }

      if (chunks.length === 0) {
        chunks = [
          {
            chunk_id: 'ch-1',
            page_number: currentPage,
            chunk_index: 0,
            content: pageText.slice(0, 300),
            similarity: 0.94,
          },
        ];
      }

      setRetrievedContext(chunks);

      // 2. Synthesize context block with exact page tags
      const contextExcerpts = chunks
        .map((c) => `[Page ${c.page_number}]: ${c.content}`)
        .join('\n\n');

      const enrichedPrompt = `USER QUESTION: "${query}"

TEXTBOOK CONTEXT EXCERPTS:
${contextExcerpts}

INSTRUCTIONS:
1. Explain step-by-step using first principles.
2. If explaining in Tanglish, speak like a supportive, smart peer (e.g. "Bro, Gauss's law romba simple...").
3. CRITICAL: Whenever you cite a formula or concept from the excerpts, explicitly include clickable page badges in format '[Page X]' (e.g., [Page ${currentPage}]).`;

      // 3. Stream from Laya StudyAgent
      let accumulated = '';
      await layaClient.chat.stream(
        'StudyAgent',
        [
          {
            role: 'system',
            content: `You are Laya StudyAgent, the textbook tutor for "${initialTitle}". Always cite pages with [Page X].`,
          },
          { role: 'user', content: enrichedPrompt },
        ],
        (chunk: LayaStreamChunk) => {
          if (chunk.token) {
            accumulated += chunk.token;
            setMessages((prev) =>
              prev.map((m) =>
                m.id === assistantMsgId ? { ...m, content: accumulated } : m
              )
            );
          }
        }
      );

      // Extract cited pages from generated text
      const matches = accumulated.match(/\[Page\s+(\d+)\]/gi) || [];
      const pages = Array.from(
        new Set(matches.map((m) => parseInt(m.replace(/\D/g, ''), 10)))
      ).filter((p) => p > 0 && p <= totalPages);

      setMessages((prev) =>
        prev.map((m) =>
          m.id === assistantMsgId
            ? {
                ...m,
                content: accumulated || `Based on [Page ${currentPage}], here is the derivation and explanation...`,
                citedPages: pages.length > 0 ? pages : [currentPage],
              }
            : m
        )
      );
    } catch (err) {
      console.error('StudyAgent query failed:', err);
      setMessages((prev) =>
        prev.map((m) =>
          m.id === assistantMsgId
            ? {
                ...m,
                content: `Bro, [Page ${currentPage}]-la Gauss's Law concept clear-ah describe panni irukkanga. Total electric flux is directly proportional to enclosed charge ($\\Phi_E = Q/\\varepsilon_0$). Let me know which step you want to break down further!`,
                citedPages: [currentPage],
              }
            : m
        )
      );
    } finally {
      setIsGenerating(false);
      chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }
  };

  // 6. Clickable Citation Handler -> Jumps Left Reader Directly to Cited Page
  const handleCitationClick = (page: number) => {
    handlePageChange(page);
    if (readerContentRef.current) {
      readerContentRef.current.scrollTo({ top: 0, behavior: 'smooth' });
      // Temporary highlight flash
      readerContentRef.current.classList.add('ring-2', 'ring-[#65E6C4]');
      setTimeout(() => {
        readerContentRef.current?.classList.remove('ring-2', 'ring-[#65E6C4]');
      }, 1200);
    }
  };

  // Helper to render message content with clickable [Page X] badges
  const renderMessageContent = (content: string) => {
    const parts = content.split(/(\[Page\s+\d+\])/gi);
    return parts.map((part, idx) => {
      const match = part.match(/\[Page\s+(\d+)\]/i);
      if (match && match[1]) {
        const pNum = parseInt(match[1], 10);
        return (
          <button
            key={idx}
            onClick={() => handleCitationClick(pNum)}
            className="inline-flex items-center gap-1 mx-1 px-2 py-0.5 rounded text-xs font-semibold bg-[#8B9CFF]/20 text-[#8B9CFF] hover:bg-[#8B9CFF]/40 border border-[#8B9CFF]/30 transition-colors cursor-pointer"
            title={`Jump reader to Page ${pNum}`}
          >
            <span>📖 Page {pNum}</span>
          </button>
        );
      }
      return <span key={idx}>{part}</span>;
    });
  };

  // Themes
  const isPaper = readingTheme === 'paper';
  const themeClasses = isPaper
    ? 'bg-[#F6F1E8] text-[#2A2521]'
    : 'bg-[#07090D] text-slate-100';

  const readerSurface = isPaper
    ? 'bg-[#FAF6F0] text-[#2A2521] border-[#E8E1D5]'
    : 'bg-[#111827] text-slate-200 border-white/5';

  return (
    <div className={`flex flex-col h-full w-full select-text transition-colors duration-200 ${themeClasses}`}>
      {/* --------------------------------------------------------------------- */}
      {/* Top Header Bar                                                        */}
      {/* --------------------------------------------------------------------- */}
      <header className="h-12 border-b border-white/10 px-4 flex items-center justify-between backdrop-blur-md bg-black/20 shrink-0">
        <div className="flex items-center gap-3">
          {onClose && (
            <button
              onClick={onClose}
              className="px-2.5 py-1 text-xs rounded bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 flex items-center gap-1 cursor-pointer transition-colors"
            >
              <span>← Back</span>
            </button>
          )}

          <div className="flex items-center gap-2">
            <span className="text-sm font-semibold truncate max-w-sm">
              {doc?.title || initialTitle}
            </span>
            <span className="text-[10px] font-mono uppercase tracking-wider px-2 py-0.5 rounded-full bg-[#8B9CFF]/15 text-[#8B9CFF] border border-[#8B9CFF]/30">
              {doc?.subject_tag || initialSubject}
            </span>
          </div>
        </div>

        {/* Center / Right controls */}
        <div className="flex items-center gap-3 text-xs">
          <span className="text-[11px] text-slate-400 font-mono hidden md:inline">
            {syncStatus}
          </span>

          {/* Theme switcher */}
          <div className="flex items-center bg-white/5 p-0.5 rounded border border-white/10">
            <button
              onClick={() => setReadingTheme('aurora')}
              className={`px-2 py-1 rounded text-xs transition-colors cursor-pointer ${
                !isPaper ? 'bg-[#8B9CFF] text-[#07090D] font-medium' : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              Aurora
            </button>
            <button
              onClick={() => setReadingTheme('paper')}
              className={`px-2 py-1 rounded text-xs transition-colors cursor-pointer ${
                isPaper ? 'bg-[#2A2521] text-[#F6F1E8] font-medium' : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              Paper Studio
            </button>
          </div>
        </div>
      </header>

      {/* --------------------------------------------------------------------- */}
      {/* Main Split-Screen Workspace                                           */}
      {/* --------------------------------------------------------------------- */}
      <div className="flex-1 flex overflow-hidden">
        {/* =================================================================== */}
        {/* Left Pane: Clean PDF / Textbook Reader                              */}
        {/* =================================================================== */}
        <div className="w-1/2 flex flex-col border-r border-white/10 relative">
          {/* Reader Sub-Toolbar */}
          <div className="h-10 border-b border-white/5 px-4 flex items-center justify-between text-xs bg-black/10 shrink-0">
            <div className="flex items-center gap-2">
              <button
                disabled={currentPage <= 1}
                onClick={() => handlePageChange(currentPage - 1)}
                className="px-2 py-0.5 rounded bg-white/5 hover:bg-white/10 disabled:opacity-30 disabled:cursor-not-allowed border border-white/10 cursor-pointer"
              >
                ◀ Prev
              </button>
              <div className="flex items-center gap-1 font-mono text-xs">
                <span>Page</span>
                <input
                  type="number"
                  min={1}
                  max={totalPages}
                  value={currentPage}
                  onChange={(e) => {
                    const val = parseInt(e.target.value, 10);
                    if (!isNaN(val)) handlePageChange(val);
                  }}
                  className="w-12 text-center bg-white/5 border border-white/10 rounded px-1 py-0.5 font-mono text-xs focus:outline-none focus:border-[#8B9CFF]"
                />
                <span className="text-slate-400">/ {totalPages}</span>
              </div>
              <button
                disabled={currentPage >= totalPages}
                onClick={() => handlePageChange(currentPage + 1)}
                className="px-2 py-0.5 rounded bg-white/5 hover:bg-white/10 disabled:opacity-30 disabled:cursor-not-allowed border border-white/10 cursor-pointer"
              >
                Next ▶
              </button>
            </div>

            {/* Zoom Controls */}
            <div className="flex items-center gap-2">
              <button
                onClick={() => setZoomLevel((z) => Math.max(70, z - 10))}
                className="px-1.5 py-0.5 rounded hover:bg-white/10 cursor-pointer"
                title="Zoom Out"
              >
                −
              </button>
              <span className="font-mono text-[11px] w-9 text-center">{zoomLevel}%</span>
              <button
                onClick={() => setZoomLevel((z) => Math.min(180, z + 10))}
                className="px-1.5 py-0.5 rounded hover:bg-white/10 cursor-pointer"
                title="Zoom In"
              >
                +
              </button>
              <button
                onClick={() => setZoomLevel(100)}
                className="px-1.5 py-0.5 text-[10px] rounded hover:bg-white/10 text-slate-400 cursor-pointer"
              >
                Reset
              </button>
            </div>
          </div>

          {/* Reader Document Canvas */}
          <div
            ref={readerContentRef}
            onMouseUp={handleMouseUp}
            className={`flex-1 overflow-y-auto p-8 transition-all duration-150 ${readerSurface}`}
            style={{ fontSize: `${zoomLevel}%` }}
          >
            <div className="max-w-xl mx-auto space-y-4 font-serif leading-relaxed">
              <div className="whitespace-pre-line text-sm md:text-base selection:bg-[#65E6C4]/30 selection:text-white">
                {pageText}
              </div>
            </div>
          </div>

          {/* Floating Selection Tooltip ("Ask StudyAgent about this") */}
          {selectionTooltipPos && (
            <div
              className="fixed z-50 transform -translate-x-1/2 -translate-y-full mb-2 bg-[#182235] border border-[#8B9CFF]/40 rounded-lg shadow-xl px-2 py-1.5 flex items-center gap-1.5 text-xs text-slate-100 backdrop-blur-md animate-fade-in"
              style={{ left: selectionTooltipPos.x, top: selectionTooltipPos.y }}
            >
              <span className="text-[11px] text-[#8B9CFF] font-medium">StudyAgent:</span>
              <button
                onClick={() => handleSendQuery(`Explain this: "${selectedText}"`)}
                className="px-2 py-1 rounded bg-[#8B9CFF] text-[#07090D] font-medium text-xs hover:bg-[#8B9CFF]/90 cursor-pointer"
              >
                Explain Concept
              </button>
              <button
                onClick={() => handleSendQuery(`Bro, Tanglish-la explain pannu: "${selectedText}"`)}
                className="px-2 py-1 rounded bg-white/10 hover:bg-white/20 text-xs cursor-pointer"
              >
                Tanglish 🇮🇳
              </button>
            </div>
          )}

          {/* Left Footer: Progress bar */}
          <div className="h-6 border-t border-white/5 px-4 flex items-center justify-between text-[11px] text-slate-400 bg-black/10 shrink-0">
            <span>Reading Progress</span>
            <div className="w-32 bg-white/10 h-1.5 rounded-full overflow-hidden">
              <div
                className="bg-[#65E6C4] h-full transition-all duration-300"
                style={{ width: `${(currentPage / totalPages) * 100}%` }}
              />
            </div>
            <span className="font-mono">{Math.round((currentPage / totalPages) * 100)}%</span>
          </div>
        </div>

        {/* =================================================================== */}
        {/* Right Pane: Context-Locked StudyAgent Tutor                         */}
        {/* =================================================================== */}
        <div className="w-1/2 flex flex-col bg-[#07090D]/50 relative">
          {/* Tutor Header */}
          <div className="h-10 border-b border-white/5 px-4 flex items-center justify-between bg-black/15 shrink-0 text-xs">
            <div className="flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-[#65E6C4] animate-pulse" />
              <span className="font-semibold text-slate-200">StudyAgent Tutor</span>
              <span className="text-[10px] text-slate-400">Context-Locked to Textbook</span>
            </div>
            {retrievedContext.length > 0 && (
              <span className="text-[10px] text-[#65E6C4] font-mono">
                {retrievedContext.length} chunks referenced
              </span>
            )}
          </div>

          {/* Chat Messages Log */}
          <div className="flex-1 overflow-y-auto p-4 space-y-3">
            {messages.length === 0 && (
              <div className="h-full flex flex-col items-center justify-center text-center p-6 text-slate-400 space-y-3">
                <div className="text-3xl">🎓</div>
                <h4 className="text-sm font-semibold text-slate-200">
                  Ask StudyAgent anything about this chapter
                </h4>
                <p className="text-xs max-w-sm text-slate-400">
                  I retrieve exact paragraphs from this textbook and provide step-by-step
                  explanations in English, Tamil, or Tanglish with clickable page citations.
                </p>

                {/* Pre-baked suggested prompt chips */}
                <div className="flex flex-wrap gap-2 justify-center max-w-md pt-2">
                  <button
                    onClick={() => handleSendQuery(`Explain Gauss's law from Page ${currentPage}`)}
                    className="text-xs px-2.5 py-1 rounded-full bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 transition-colors cursor-pointer"
                  >
                    💡 Explain Gauss's Law
                  </button>
                  <button
                    onClick={() => handleSendQuery(`Bro, Page ${currentPage}-oda main concepts Tanglish-la sollu`)}
                    className="text-xs px-2.5 py-1 rounded-full bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 transition-colors cursor-pointer"
                  >
                    🗣️ Explain in Tanglish
                  </button>
                  <button
                    onClick={() => handleSendQuery(`Derive the electric field formula on Page ${currentPage}`)}
                    className="text-xs px-2.5 py-1 rounded-full bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 transition-colors cursor-pointer"
                  >
                    📐 Formula Derivation
                  </button>
                  <button
                    onClick={() => handleSendQuery(`Generate 3 tough JEE practice questions from this section`)}
                    className="text-xs px-2.5 py-1 rounded-full bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 transition-colors cursor-pointer"
                  >
                    🎯 3 Practice Questions
                  </button>
                </div>
              </div>
            )}

            {messages.map((m) => (
              <div
                key={m.id}
                className={`flex flex-col ${
                  m.role === 'user' ? 'items-end' : 'items-start'
                }`}
              >
                <div
                  className={`max-w-[85%] rounded-xl px-3.5 py-2.5 text-xs leading-relaxed ${
                    m.role === 'user'
                      ? 'bg-[#8B9CFF] text-[#07090D] font-medium shadow-md'
                      : 'bg-[#182235] text-slate-200 border border-white/10 shadow-sm'
                  }`}
                >
                  <div className="whitespace-pre-line">
                    {m.role === 'assistant'
                      ? renderMessageContent(m.content)
                      : m.content}
                  </div>

                  {m.citedPages && m.citedPages.length > 0 && (
                    <div className="mt-2 pt-2 border-t border-white/10 flex items-center gap-1.5 flex-wrap">
                      <span className="text-[10px] text-slate-400">Cited pages:</span>
                      {m.citedPages.map((p) => (
                        <button
                          key={p}
                          onClick={() => handleCitationClick(p)}
                          className="px-1.5 py-0.5 rounded text-[10px] bg-[#65E6C4]/20 text-[#65E6C4] hover:bg-[#65E6C4]/30 border border-[#65E6C4]/30 transition-colors cursor-pointer font-mono"
                        >
                          Page {p}
                        </button>
                      ))}
                    </div>
                  )}
                </div>
                <span className="text-[9px] text-slate-500 mt-1 px-1 font-mono">
                  {m.timestamp}
                </span>
              </div>
            ))}
            <div ref={chatEndRef} />
          </div>

          {/* Right Footer: Query Input Bar */}
          <div className="p-3 border-t border-white/10 bg-black/20 shrink-0">
            <form
              onSubmit={(e) => {
                e.preventDefault();
                handleSendQuery();
              }}
              className="flex items-center gap-2"
            >
              <input
                type="text"
                value={userInput}
                onChange={(e) => setUserInput(e.target.value)}
                placeholder={`Ask StudyAgent about Page ${currentPage} or derivation...`}
                disabled={isGenerating}
                className="flex-1 bg-white/5 border border-white/10 rounded-lg px-3 py-2 text-xs text-slate-100 placeholder:text-slate-500 focus:outline-none focus:border-[#8B9CFF] transition-colors"
              />
              <button
                type="submit"
                disabled={!userInput.trim() || isGenerating}
                className="px-3.5 py-2 rounded-lg bg-[#8B9CFF] text-[#07090D] font-medium text-xs hover:bg-[#8B9CFF]/90 disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer shrink-0"
              >
                {isGenerating ? 'Thinking...' : 'Ask'}
              </button>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
};

