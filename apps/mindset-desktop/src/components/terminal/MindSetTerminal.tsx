/**
 * MindSet OS — Integrated MindSet Terminal & Developer Sandbox
 * Interactive CLI supporting Bash, Zsh, PowerShell & built-in msos commands
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState, useRef, useEffect } from 'react';
import { Button } from '@mindset/ui-core';
import { monitorClient } from '../../lib/monitorClient';

interface TerminalHistoryItem {
  id: string;
  command: string;
  output: string;
  exitCode: number;
  timeMs: number;
  timestamp: string;
}

const QUICK_COMMANDS = [
  { label: 'msos status', cmd: 'msos status' },
  { label: 'msos ram purge', cmd: 'msos ram purge' },
  { label: 'msos ai bench', cmd: 'msos ai bench' },
  { label: 'msos focus start', cmd: 'msos focus start 25' },
  { label: 'msos dev on', cmd: 'msos dev on' },
  { label: 'msos exam lock', cmd: 'msos exam lock' },
  { label: 'help', cmd: 'help' },
];

export const MindSetTerminal: React.FC = () => {
  const [inputVal, setInputVal] = useState<string>('');
  const [isExecuting, setIsExecuting] = useState<boolean>(false);
  const [developerMode, setDeveloperMode] = useState<boolean>(true);
  const [examLock, setExamLock] = useState<boolean>(false);

  const [history, setHistory] = useState<TerminalHistoryItem[]>([
    {
      id: 'init-0',
      command: 'msos status',
      output:
        'MindSet OS Aurora Core [LKS PVT LTD]\n' +
        'Kernel Boot Stage: SYSTEM_READY (Phase 5 of 5)\n' +
        'OS Shell RSS: 36.8 MB (Ceiling: 120.0 MB) [PASS]\n' +
        'Local AI Memory: 1840.0 MB (Ollama Flash Q4_K_M)\n' +
        'Free RAM for Local LLM: 8.3 GB / 12.0 GB Total\n' +
        'cgroups v2 & Hardware Governor: Active (EcoStudy / Balanced)',
      exitCode: 0,
      timeMs: 8,
      timestamp: new Date().toLocaleTimeString(),
    },
  ]);

  const terminalEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    terminalEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [history, isExecuting]);

  const handleRunCommand = async (cmdToRun?: string) => {
    const raw = (cmdToRun !== undefined ? cmdToRun : inputVal).trim();
    if (!raw) return;

    if (raw.toLowerCase() === 'clear') {
      setHistory([]);
      setInputVal('');
      return;
    }

    setIsExecuting(true);
    setInputVal('');

    try {
      const res = await monitorClient.executeTerminalCommand(raw);

      // Check for state updates
      if (raw.toLowerCase().includes('dev on')) setDeveloperMode(true);
      if (raw.toLowerCase().includes('dev off')) setDeveloperMode(false);
      if (raw.toLowerCase().includes('exam lock')) setExamLock(true);
      if (raw.toLowerCase().includes('exam unlock')) setExamLock(false);

      setHistory((prev) => [
        ...prev,
        {
          id: `cmd-${Date.now()}`,
          command: res.command,
          output: res.output,
          exitCode: res.exit_code,
          timeMs: res.execution_time_ms,
          timestamp: new Date().toLocaleTimeString(),
        },
      ]);
    } catch (err) {
      setHistory((prev) => [
        ...prev,
        {
          id: `cmd-${Date.now()}`,
          command: raw,
          output: `Error executing command: ${String(err)}`,
          exitCode: 1,
          timeMs: 0,
          timestamp: new Date().toLocaleTimeString(),
        },
      ]);
    } finally {
      setIsExecuting(false);
      inputRef.current?.focus();
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#05070A] text-white font-mono text-xs rounded-xl overflow-hidden border border-white/10 shadow-2xl">
      {/* ------------------------------------------------------------------- */}
      {/* Terminal Title Bar & Toggles                                        */}
      {/* ------------------------------------------------------------------- */}
      <div className="flex-none px-4 py-2.5 bg-[#0C1017] border-b border-white/5 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <div className="flex space-x-1.5">
            <span className="w-2.5 h-2.5 rounded-full bg-rose-500/80 inline-block" />
            <span className="w-2.5 h-2.5 rounded-full bg-amber-500/80 inline-block" />
            <span className="w-2.5 h-2.5 rounded-full bg-emerald-500/80 inline-block" />
          </div>
          <span className="text-[11px] font-semibold text-white/80 ml-2">
            msos@aurora-shell: ~ (PTY Terminal)
          </span>
        </div>

        <div className="flex items-center space-x-3">
          {/* Developer Mode Badge */}
          <button
            onClick={() => handleRunCommand(`msos dev ${developerMode ? 'off' : 'on'}`)}
            className={`text-[10px] px-2 py-0.5 rounded border transition-all ${
              developerMode
                ? 'bg-cyan-500/20 text-cyan-300 border-cyan-500/30'
                : 'bg-white/5 text-white/40 border-white/10'
            }`}
          >
            {developerMode ? '🛠️ Dev Mode: ON' : '🛠️ Dev Mode: OFF'}
          </button>

          {/* Exam Lock Badge */}
          <button
            onClick={() => handleRunCommand(`msos exam ${examLock ? 'unlock' : 'lock'}`)}
            className={`text-[10px] px-2 py-0.5 rounded border transition-all ${
              examLock
                ? 'bg-rose-500/20 text-rose-300 border-rose-500/30 font-semibold'
                : 'bg-white/5 text-white/40 border-white/10'
            }`}
          >
            {examLock ? '🔒 Exam Locked' : '🔓 Exam Unlocked'}
          </button>

          <button
            onClick={() => setHistory([])}
            className="text-[11px] text-white/40 hover:text-white transition-colors"
            title="Clear output"
          >
            Clear
          </button>
        </div>
      </div>

      {/* ------------------------------------------------------------------- */}
      {/* Quick Action Pills                                                  */}
      {/* ------------------------------------------------------------------- */}
      <div className="flex-none px-4 py-2 bg-[#080B10] border-b border-white/5 flex items-center space-x-1.5 overflow-x-auto scrollbar-none">
        <span className="text-[10px] text-white/40 mr-1 uppercase tracking-wider">Shortcuts:</span>
        {QUICK_COMMANDS.map((qc) => (
          <button
            key={qc.label}
            onClick={() => handleRunCommand(qc.cmd)}
            disabled={isExecuting}
            className="px-2 py-0.5 rounded bg-white/[0.04] hover:bg-white/[0.08] text-[11px] text-cyan-300 border border-white/5 hover:border-cyan-500/30 transition-all whitespace-nowrap"
          >
            {qc.label}
          </button>
        ))}
      </div>

      {/* ------------------------------------------------------------------- */}
      {/* Output Console                                                      */}
      {/* ------------------------------------------------------------------- */}
      <div className="flex-1 p-4 overflow-y-auto space-y-3 font-mono leading-relaxed select-text">
        {history.map((item) => (
          <div key={item.id} className="space-y-1">
            <div className="flex items-center space-x-2 text-white/60">
              <span className="text-cyan-400">msos@aurora:~$</span>
              <span className="text-white font-semibold">{item.command}</span>
              <span className="text-[10px] text-white/30 ml-auto tabular-nums">
                {item.timeMs}ms • {item.timestamp}
              </span>
            </div>

            <pre className="text-white/80 whitespace-pre-wrap pl-4 border-l border-white/10 text-[11px]">
              {item.output}
            </pre>
          </div>
        ))}

        {isExecuting && (
          <div className="flex items-center space-x-2 text-cyan-400 animate-pulse pl-4">
            <span>Executing...</span>
          </div>
        )}

        <div ref={terminalEndRef} />
      </div>

      {/* ------------------------------------------------------------------- */}
      {/* Input Prompt Bar                                                    */}
      {/* ------------------------------------------------------------------- */}
      <form
        onSubmit={(e) => {
          e.preventDefault();
          handleRunCommand();
        }}
        className="flex-none p-3 bg-[#0C1017] border-t border-white/5 flex items-center space-x-2"
      >
        <span className="text-cyan-400 font-bold">msos@aurora:~$</span>
        <input
          ref={inputRef}
          type="text"
          value={inputVal}
          onChange={(e) => setInputVal(e.target.value)}
          placeholder="Type msos status, msos ram purge, msos ai bench, or any shell command..."
          disabled={isExecuting}
          className="flex-1 bg-transparent text-white placeholder-white/30 focus:outline-none font-mono text-xs"
          autoFocus
        />
        <Button
          type="submit"
          variant="secondary"
          size="sm"
          disabled={isExecuting || !inputVal.trim()}
          className="text-xs px-3 py-1 bg-cyan-500/10 text-cyan-300 border-cyan-500/20 hover:bg-cyan-500/20"
        >
          Run ↵
        </Button>
      </form>
    </div>
  );
};

