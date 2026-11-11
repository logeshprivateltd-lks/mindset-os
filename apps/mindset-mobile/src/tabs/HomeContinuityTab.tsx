import React, { useState } from 'react';
import {
  SparklesIcon,
  ClockIcon,
  ShieldLockIcon,
} from '../components/MobileIcons';

export const HomeContinuityTab: React.FC = () => {
  const [isFocusActive] = useState(true);
  const [remainingSeconds] = useState(1480);
  const [showBypassModal, setShowBypassModal] = useState(false);
  const [bypassSuccess, setBypassSuccess] = useState(false);

  // Daily Tasks State
  const [tasks, setTasks] = useState([
    { id: '1', title: "Young's Double Slit Derivation (Physics)", done: true, marks: 5 },
    { id: '2', title: 'Aldehydes & Ketones IUPAC Drill (Chemistry)', done: false, marks: 3 },
    { id: '3', title: 'Calculus Definite Integrals Practice 5.2', done: false, marks: 4 },
  ]);

  const toggleTask = (id: string) => {
    setTasks((prev) =>
      prev.map((t) => (t.id === id ? { ...t, done: !t.done } : t))
    );
  };

  const handleRequestBypass = () => {
    setBypassSuccess(true);
    setTimeout(() => {
      setBypassSuccess(false);
      setShowBypassModal(false);
    }, 2000);
  };

  return (
    <div className="p-4 space-y-4 pb-24 overflow-y-auto max-w-lg mx-auto">
      {/* Top Mobile Status Header */}
      <div className="flex items-center justify-between pt-2">
        <div>
          <span className="text-[11px] font-semibold text-cyan-400 uppercase tracking-wider">
            MindSet Mobile Mesh
          </span>
          <h1 className="text-xl font-bold text-white tracking-tight">Today's Focus & Handoff</h1>
        </div>
        <div className="flex items-center space-x-1.5 px-2.5 py-1 rounded-full bg-emerald-500/20 text-emerald-300 text-xs font-semibold border border-emerald-500/30">
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
          <span>Synced with Laptop</span>
        </div>
      </div>

      {/* Cross-Device Focus Guard Lockdown Banner */}
      {isFocusActive && (
        <div className="bg-gradient-to-br from-purple-950/70 to-slate-900 border border-purple-500/40 rounded-2xl p-4 backdrop-blur-xl relative overflow-hidden shadow-lg shadow-purple-950/40">
          <div className="flex items-center justify-between">
            <span className="flex items-center space-x-1.5 text-xs font-semibold text-purple-300">
              <ShieldLockIcon className="w-4 h-4 text-purple-400" />
              <span>Cross-Device Study Lockdown</span>
            </span>
            <span className="text-[10px] px-2 py-0.5 rounded-full bg-purple-500/20 text-purple-200 border border-purple-500/30 font-medium">
              Social Media Blocked
            </span>
          </div>

          <div className="mt-3">
            <h3 className="text-base font-bold text-white">
              Target: Master Wave Optics Derivation
            </h3>
            <p className="text-xs text-slate-300 mt-0.5">
              Laptop focus session enforced on mobile. Non-educational notifications are silenced.
            </p>
          </div>

          <div className="mt-4 flex items-center justify-between pt-3 border-t border-purple-500/20">
            <div className="flex items-center space-x-2">
              <ClockIcon className="w-4 h-4 text-purple-400" />
              <span className="text-lg font-bold font-mono text-purple-200 tabular-nums">
                {Math.floor(remainingSeconds / 60)}:{String(remainingSeconds % 60).padStart(2, '0')}
              </span>
            </div>

            <button
              onClick={() => setShowBypassModal(true)}
              className="px-3 py-1.5 rounded-xl bg-purple-500/20 hover:bg-purple-500/30 border border-purple-500/30 text-purple-200 text-xs font-semibold transition"
            >
              Emergency Bypass
            </button>
          </div>
        </div>
      )}

      {/* Continuity: "Continue Where You Left Off" */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl">
        <div className="flex items-center justify-between">
          <span className="flex items-center space-x-1.5 text-xs font-semibold text-cyan-400 uppercase tracking-wider">
            <SparklesIcon className="w-4 h-4" />
            <span>Continue Where You Left Off</span>
          </span>
          <span className="text-[10px] text-slate-400">Beamed from Laptop</span>
        </div>

        <div className="mt-2.5 p-3 rounded-xl bg-slate-950/80 border border-slate-800/80">
          <h4 className="text-sm font-semibold text-slate-100">
            Physics Ch 4: Ray Optics and Optical Instruments
          </h4>
          <p className="text-xs text-slate-400 mt-1 flex items-center justify-between">
            <span>Exact Bookmark: Page <strong className="text-white tabular-nums">84</strong> / 210</span>
            <span className="text-cyan-400 font-semibold tabular-nums">40% Read</span>
          </p>

          <div className="mt-2.5 w-full bg-slate-800 h-1.5 rounded-full overflow-hidden">
            <div className="bg-cyan-500 h-full rounded-full" style={{ width: '40%' }} />
          </div>

          <div className="mt-3 flex gap-2">
            <button
              onClick={() => alert('Opening Ray Optics at page 84 in mobile viewer.')}
              className="flex-1 py-2 rounded-xl bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-semibold text-center transition"
            >
              Resume Page 84 on Phone
            </button>
          </div>
        </div>
      </div>

      {/* Daily Syllabus Micro-Tasks Check-in */}
      <div className="bg-slate-900/80 border border-slate-800 rounded-2xl p-4 backdrop-blur-xl">
        <div className="flex items-center justify-between pb-3 border-b border-slate-800/60">
          <div>
            <h3 className="text-sm font-bold text-white">Daily Board Exam Targets</h3>
            <p className="text-[11px] text-slate-400">Class 12 State Board 590/600 Sprint</p>
          </div>
          <span className="text-xs text-cyan-400 font-mono font-semibold tabular-nums">
            {tasks.filter((t) => t.done).length}/{tasks.length} Done
          </span>
        </div>

        <div className="mt-3 space-y-2">
          {tasks.map((task) => (
            <div
              key={task.id}
              onClick={() => toggleTask(task.id)}
              className={`p-3 rounded-xl border flex items-center justify-between transition cursor-pointer ${
                task.done
                  ? 'bg-emerald-950/20 border-emerald-500/30'
                  : 'bg-slate-950/60 border-slate-800 hover:border-slate-700'
              }`}
            >
              <div className="flex items-center space-x-3">
                <span
                  className={`w-5 h-5 rounded-lg border flex items-center justify-center transition ${
                    task.done
                      ? 'bg-emerald-500 border-emerald-400 text-slate-950 font-bold'
                      : 'border-slate-600'
                  }`}
                >
                  {task.done && '✓'}
                </span>
                <span
                  className={`text-xs font-medium ${
                    task.done ? 'line-through text-slate-400' : 'text-slate-200'
                  }`}
                >
                  {task.title}
                </span>
              </div>
              <span className="text-[10px] px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-mono tabular-nums">
                +{task.marks}m
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Emergency Bypass Modal */}
      {showBypassModal && (
        <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex items-center justify-center p-4">
          <div className="bg-slate-900 border border-slate-800 rounded-3xl p-5 max-w-sm w-full shadow-2xl">
            <h3 className="text-base font-bold text-white">Emergency Study Bypass</h3>
            <p className="text-xs text-slate-300 mt-1">
              Need urgent phone access for an emergency or parent communication?
            </p>
            {bypassSuccess ? (
              <div className="mt-4 p-3 rounded-xl bg-emerald-500/20 text-emerald-300 text-xs font-semibold text-center">
                ✓ Approved: 10 minutes emergency window granted.
              </div>
            ) : (
              <div className="mt-4 flex space-x-2">
                <button
                  onClick={() => setShowBypassModal(false)}
                  className="flex-1 py-2 rounded-xl bg-slate-800 text-slate-300 text-xs font-semibold"
                >
                  Cancel
                </button>
                <button
                  onClick={handleRequestBypass}
                  className="flex-1 py-2 rounded-xl bg-purple-600 hover:bg-purple-500 text-white text-xs font-semibold"
                >
                  Confirm Unlock
                </button>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};
