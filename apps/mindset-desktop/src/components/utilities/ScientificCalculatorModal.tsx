/**
 * MindSet OS — Scientific Calculator & Unit Converter
 * Draggable floating scientific calculator with dedicated Physics & Chemistry unit converter
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import React, { useState } from 'react';
import { GlassSurface } from '@mindset/ui-core';

export interface ScientificCalculatorModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export type CalcMode = 'calculator' | 'converter';
export type UnitCategory = 'length' | 'mass' | 'pressure' | 'energy' | 'temperature';

export const ScientificCalculatorModal: React.FC<ScientificCalculatorModalProps> = ({
  isOpen,
  onClose,
}) => {
  const [mode, setMode] = useState<CalcMode>('calculator');
  const [display, setDisplay] = useState<string>('0');
  const [expression, setExpression] = useState<string>('');

  // Unit Converter State
  const [unitCat, setUnitCat] = useState<UnitCategory>('temperature');
  const [fromValue, setFromValue] = useState<number>(25);
  const [fromUnit, setFromUnit] = useState<string>('C');
  const [toUnit, setToUnit] = useState<string>('K');

  if (!isOpen) return null;

  const handleInput = (val: string) => {
    if (display === '0' || display === 'Error') {
      setDisplay(val);
    } else {
      setDisplay(display + val);
    }
  };

  const handleClear = () => {
    setDisplay('0');
    setExpression('');
  };

  const handleBackspace = () => {
    if (display.length <= 1) {
      setDisplay('0');
    } else {
      setDisplay(display.slice(0, -1));
    }
  };

  const handleCalculate = () => {
    try {
      // Safe math evaluation
      const sanitized = display
        .replace(/π/g, 'Math.PI')
        .replace(/e/g, 'Math.E')
        .replace(/sin\(/g, 'Math.sin(')
        .replace(/cos\(/g, 'Math.cos(')
        .replace(/tan\(/g, 'Math.tan(')
        .replace(/log\(/g, 'Math.log10(')
        .replace(/ln\(/g, 'Math.log(')
        .replace(/sqrt\(/g, 'Math.sqrt(')
        .replace(/\^/g, '**');

      // Use Function constructor for isolated evaluation
      const result = new Function(`return (${sanitized})`)();
      setExpression(display + ' =');
      setDisplay(Number.isFinite(result) ? String(result) : 'Error');
    } catch {
      setDisplay('Error');
    }
  };

  // Convert Units
  const convertUnits = (): number => {
    const val = Number(fromValue);
    if (Number.isNaN(val)) return 0;

    if (unitCat === 'temperature') {
      let celsius = val;
      if (fromUnit === 'F') celsius = ((val - 32) * 5) / 9;
      if (fromUnit === 'K') celsius = val - 273.15;

      if (toUnit === 'C') return Math.round(celsius * 100) / 100;
      if (toUnit === 'F') return Math.round(((celsius * 9) / 5 + 32) * 100) / 100;
      if (toUnit === 'K') return Math.round((celsius + 273.15) * 100) / 100;
    }

    if (unitCat === 'pressure') {
      // Base: Pascal
      const toPa: Record<string, number> = { Pa: 1, atm: 101325, bar: 100000, torr: 133.322 };
      const fromFactor = toPa[fromUnit] ?? 1;
      const toFactor = toPa[toUnit] ?? 1;
      const inPa = val * fromFactor;
      return Math.round((inPa / toFactor) * 1000) / 1000;
    }

    if (unitCat === 'energy') {
      // Base: Joules
      const toJ: Record<string, number> = { J: 1, kJ: 1000, cal: 4.184, eV: 1.60218e-19 };
      const fromFactor = toJ[fromUnit] ?? 1;
      const toFactor = toJ[toUnit] ?? 1;
      const inJ = val * fromFactor;
      return Math.round((inJ / toFactor) * 10000) / 10000;
    }

    if (unitCat === 'length') {
      // Base: Meters
      const toM: Record<string, number> = { m: 1, km: 1000, cm: 0.01, mm: 0.001, ft: 0.3048, in: 0.0254 };
      const fromFactor = toM[fromUnit] ?? 1;
      const toFactor = toM[toUnit] ?? 1;
      const inM = val * fromFactor;
      return Math.round((inM / toFactor) * 10000) / 10000;
    }

    if (unitCat === 'mass') {
      // Base: Kilograms
      const toKg: Record<string, number> = { kg: 1, g: 0.001, mg: 0.000001, lb: 0.453592 };
      const fromFactor = toKg[fromUnit] ?? 1;
      const toFactor = toKg[toUnit] ?? 1;
      const inKg = val * fromFactor;
      return Math.round((inKg / toFactor) * 10000) / 10000;
    }

    return val;
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md animate-fadeIn">
      <GlassSurface variant="modal" className="max-w-md w-full p-5 space-y-4 animate-scaleUp select-none">
        {/* Header */}
        <div className="flex items-center justify-between pb-2 border-b border-[var(--border-subtle)]">
          <div className="flex items-center space-x-2">
            <span className="text-xl">🧮</span>
            <span className="text-sm font-bold text-[var(--text-dominant)]">Scientific Calculator</span>
          </div>

          <div className="flex items-center space-x-2">
            <div className="flex items-center space-x-1 p-0.5 rounded-lg bg-[var(--surface-secondary)] text-xs">
              <button
                type="button"
                onClick={() => setMode('calculator')}
                className={`px-2 py-0.5 rounded-md font-semibold transition-all ${
                  mode === 'calculator' ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]' : 'text-[var(--text-muted)]'
                }`}
              >
                Math
              </button>
              <button
                type="button"
                onClick={() => setMode('converter')}
                className={`px-2 py-0.5 rounded-md font-semibold transition-all ${
                  mode === 'converter' ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]' : 'text-[var(--text-muted)]'
                }`}
              >
                Physics Units
              </button>
            </div>

            <button
              type="button"
              onClick={onClose}
              className="text-xs text-[var(--text-muted)] hover:text-[var(--text-dominant)] p-1 rounded-md"
            >
              ✕
            </button>
          </div>
        </div>

        {/* ============================================================= */}
        {/* MODE 1: SCIENTIFIC CALCULATOR                                 */}
        {/* ============================================================= */}
        {mode === 'calculator' && (
          <div className="space-y-3">
            {/* Display screen */}
            <div className="p-3.5 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-right font-mono">
              <div className="text-[10px] text-[var(--text-muted)] h-4 overflow-hidden">{expression}</div>
              <div className="text-2xl font-bold tabular-nums text-[var(--text-dominant)] truncate">{display}</div>
            </div>

            {/* Scientific Function Keys */}
            <div className="grid grid-cols-5 gap-1.5 text-xs font-mono">
              {['sin(', 'cos(', 'tan(', 'log(', 'ln('].map((fn) => (
                <button
                  key={fn}
                  type="button"
                  onClick={() => handleInput(fn)}
                  className="p-2 rounded-lg bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)]/20 text-[#65E6C4] font-semibold text-center transition-all"
                >
                  {fn.replace('(', '')}
                </button>
              ))}

              {['sqrt(', '^', 'π', 'e', '('].map((fn) => (
                <button
                  key={fn}
                  type="button"
                  onClick={() => handleInput(fn)}
                  className="p-2 rounded-lg bg-[var(--surface-secondary)] hover:bg-[var(--accent-primary)]/20 text-[#8B9CFF] font-semibold text-center transition-all"
                >
                  {fn.replace('(', '')}
                </button>
              ))}
            </div>

            {/* Standard Keypad */}
            <div className="grid grid-cols-4 gap-1.5 text-sm font-mono">
              <button
                type="button"
                onClick={handleClear}
                className="p-2.5 rounded-lg bg-[#FF8A8A]/20 text-[#FF8A8A] font-bold"
              >
                C
              </button>
              <button
                type="button"
                onClick={handleBackspace}
                className="p-2.5 rounded-lg bg-[var(--surface-secondary)] text-[var(--text-muted)]"
              >
                ⌫
              </button>
              <button
                type="button"
                onClick={() => handleInput(')')}
                className="p-2.5 rounded-lg bg-[var(--surface-secondary)] text-[var(--text-muted)]"
              >
                )
              </button>
              <button
                type="button"
                onClick={() => handleInput('/')}
                className="p-2.5 rounded-lg bg-[var(--accent-primary)]/20 text-[var(--accent-primary)] font-bold"
              >
                ÷
              </button>

              {['7', '8', '9'].map((n) => (
                <button
                  key={n}
                  type="button"
                  onClick={() => handleInput(n)}
                  className="p-2.5 rounded-lg bg-[var(--surface-secondary)] text-[var(--text-dominant)] font-semibold hover:bg-[var(--surface-secondary)]/80"
                >
                  {n}
                </button>
              ))}
              <button
                type="button"
                onClick={() => handleInput('*')}
                className="p-2.5 rounded-lg bg-[var(--accent-primary)]/20 text-[var(--accent-primary)] font-bold"
              >
                ×
              </button>

              {['4', '5', '6'].map((n) => (
                <button
                  key={n}
                  type="button"
                  onClick={() => handleInput(n)}
                  className="p-2.5 rounded-lg bg-[var(--surface-secondary)] text-[var(--text-dominant)] font-semibold hover:bg-[var(--surface-secondary)]/80"
                >
                  {n}
                </button>
              ))}
              <button
                type="button"
                onClick={() => handleInput('-')}
                className="p-2.5 rounded-lg bg-[var(--accent-primary)]/20 text-[var(--accent-primary)] font-bold"
              >
                -
              </button>

              {['1', '2', '3'].map((n) => (
                <button
                  key={n}
                  type="button"
                  onClick={() => handleInput(n)}
                  className="p-2.5 rounded-lg bg-[var(--surface-secondary)] text-[var(--text-dominant)] font-semibold hover:bg-[var(--surface-secondary)]/80"
                >
                  {n}
                </button>
              ))}
              <button
                type="button"
                onClick={() => handleInput('+')}
                className="p-2.5 rounded-lg bg-[var(--accent-primary)]/20 text-[var(--accent-primary)] font-bold"
              >
                +
              </button>

              <button
                type="button"
                onClick={() => handleInput('0')}
                className="p-2.5 rounded-lg bg-[var(--surface-secondary)] text-[var(--text-dominant)] font-semibold col-span-2"
              >
                0
              </button>
              <button
                type="button"
                onClick={() => handleInput('.')}
                className="p-2.5 rounded-lg bg-[var(--surface-secondary)] text-[var(--text-dominant)] font-semibold"
              >
                .
              </button>
              <button
                type="button"
                onClick={handleCalculate}
                className="p-2.5 rounded-lg bg-[var(--accent-primary)] text-[var(--surface-primary)] font-bold shadow-sm"
              >
                =
              </button>
            </div>
          </div>
        )}

        {/* ============================================================= */}
        {/* MODE 2: PHYSICS & CHEMISTRY UNIT CONVERTER                    */}
        {/* ============================================================= */}
        {mode === 'converter' && (
          <div className="space-y-4">
            {/* Category tabs */}
            <div className="flex items-center space-x-1 overflow-x-auto pb-1 text-xs">
              {[
                { id: 'temperature', label: '🌡️ Temp' },
                { id: 'pressure', label: '💨 Pressure' },
                { id: 'energy', label: '⚡ Energy' },
                { id: 'length', label: '📏 Length' },
                { id: 'mass', label: '⚖️ Mass' },
              ].map((c) => (
                <button
                  key={c.id}
                  type="button"
                  onClick={() => {
                    setUnitCat(c.id as UnitCategory);
                    if (c.id === 'temperature') { setFromUnit('C'); setToUnit('K'); }
                    if (c.id === 'pressure') { setFromUnit('atm'); setToUnit('Pa'); }
                    if (c.id === 'energy') { setFromUnit('kJ'); setToUnit('J'); }
                    if (c.id === 'length') { setFromUnit('km'); setToUnit('m'); }
                    if (c.id === 'mass') { setFromUnit('kg'); setToUnit('g'); }
                  }}
                  className={`px-2.5 py-1 rounded-lg font-semibold whitespace-nowrap transition-all ${
                    unitCat === c.id
                      ? 'bg-[var(--accent-primary)] text-[var(--surface-primary)]'
                      : 'bg-[var(--surface-secondary)] text-[var(--text-muted)]'
                  }`}
                >
                  {c.label}
                </button>
              ))}
            </div>

            {/* Input & Unit Pickers */}
            <div className="grid grid-cols-2 gap-3 text-xs">
              <div className="space-y-1.5">
                <label className="text-[10px] text-[var(--text-muted)] uppercase font-bold">From:</label>
                <input
                  type="number"
                  value={fromValue}
                  onChange={(e) => setFromValue(Number(e.target.value))}
                  className="w-full text-base font-mono font-bold p-2.5 rounded-xl bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] focus:outline-none"
                />
                <select
                  value={fromUnit}
                  onChange={(e) => setFromUnit(e.target.value)}
                  className="w-full p-2 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] text-xs focus:outline-none"
                >
                  {unitCat === 'temperature' && (
                    <>
                      <option value="C">Celsius (°C)</option>
                      <option value="K">Kelvin (K)</option>
                      <option value="F">Fahrenheit (°F)</option>
                    </>
                  )}
                  {unitCat === 'pressure' && (
                    <>
                      <option value="atm">Atmosphere (atm)</option>
                      <option value="Pa">Pascal (Pa)</option>
                      <option value="bar">Bar (bar)</option>
                      <option value="torr">Torr / mmHg</option>
                    </>
                  )}
                  {unitCat === 'energy' && (
                    <>
                      <option value="kJ">Kilojoules (kJ)</option>
                      <option value="J">Joules (J)</option>
                      <option value="cal">Calories (cal)</option>
                      <option value="eV">Electron-Volts (eV)</option>
                    </>
                  )}
                  {unitCat === 'length' && (
                    <>
                      <option value="m">Meters (m)</option>
                      <option value="km">Kilometers (km)</option>
                      <option value="cm">Centimeters (cm)</option>
                      <option value="mm">Millimeters (mm)</option>
                    </>
                  )}
                  {unitCat === 'mass' && (
                    <>
                      <option value="kg">Kilograms (kg)</option>
                      <option value="g">Grams (g)</option>
                      <option value="mg">Milligrams (mg)</option>
                      <option value="lb">Pounds (lb)</option>
                    </>
                  )}
                </select>
              </div>

              <div className="space-y-1.5">
                <label className="text-[10px] text-[var(--text-muted)] uppercase font-bold">To:</label>
                <div className="w-full text-base font-mono font-bold p-2.5 rounded-xl bg-[var(--surface-secondary)] border border-[#65E6C4]/40 text-[#65E6C4] truncate">
                  {convertUnits()}
                </div>
                <select
                  value={toUnit}
                  onChange={(e) => setToUnit(e.target.value)}
                  className="w-full p-2 rounded-lg bg-[var(--surface-secondary)] border border-[var(--border-subtle)] text-[var(--text-dominant)] text-xs focus:outline-none"
                >
                  {unitCat === 'temperature' && (
                    <>
                      <option value="K">Kelvin (K)</option>
                      <option value="C">Celsius (°C)</option>
                      <option value="F">Fahrenheit (°F)</option>
                    </>
                  )}
                  {unitCat === 'pressure' && (
                    <>
                      <option value="Pa">Pascal (Pa)</option>
                      <option value="atm">Atmosphere (atm)</option>
                      <option value="bar">Bar (bar)</option>
                      <option value="torr">Torr / mmHg</option>
                    </>
                  )}
                  {unitCat === 'energy' && (
                    <>
                      <option value="J">Joules (J)</option>
                      <option value="kJ">Kilojoules (kJ)</option>
                      <option value="cal">Calories (cal)</option>
                      <option value="eV">Electron-Volts (eV)</option>
                    </>
                  )}
                  {unitCat === 'length' && (
                    <>
                      <option value="m">Meters (m)</option>
                      <option value="km">Kilometers (km)</option>
                      <option value="cm">Centimeters (cm)</option>
                      <option value="mm">Millimeters (mm)</option>
                    </>
                  )}
                  {unitCat === 'mass' && (
                    <>
                      <option value="g">Grams (g)</option>
                      <option value="kg">Kilograms (kg)</option>
                      <option value="mg">Milligrams (mg)</option>
                      <option value="lb">Pounds (lb)</option>
                    </>
                  )}
                </select>
              </div>
            </div>
          </div>
        )}
      </GlassSurface>
    </div>
  );
};
