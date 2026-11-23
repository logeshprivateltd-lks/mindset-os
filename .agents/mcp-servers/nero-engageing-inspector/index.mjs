#!/usr/bin/env node
/**
 * NERO-ENGAGEING-INSPECTOR — Local stdio MCP Server
 * Audits UI layouts, widgets, themes, and views for student cognitive engagement,
 * attention anchoring, dopamine micro-rewards, distraction shielding, eye strain ergonomics,
 * and emotional resonance during long study sessions.
 *
 * Implements standard JSON-RPC 2.0 stdio MCP protocol.
 */

import * as readline from 'node:readline';
import * as fs from 'node:fs';
import * as path from 'node:path';

// --- Color & Contrast Math Helpers ---

function sRGBtoLin(c) {
  return c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
}

function getRelativeLuminance(r, g, b) {
  // r, g, b in [0, 1]
  return 0.2126 * sRGBtoLin(r) + 0.7152 * sRGBtoLin(g) + 0.0722 * sRGBtoLin(b);
}

function calculateContrastRatio(lum1, lum2) {
  const l1 = Math.max(lum1, lum2);
  const l2 = Math.min(lum1, lum2);
  return (l1 + 0.05) / (l2 + 0.05);
}

function hexToRgb(hex) {
  let clean = hex.replace('#', '').trim();
  if (clean.length === 3) {
    clean = clean.split('').map(c => c + c).join('');
  }
  if (clean.length >= 6) {
    const num = parseInt(clean.slice(0, 6), 16);
    return {
      r: ((num >> 16) & 255) / 255,
      g: ((num >> 8) & 255) / 255,
      b: (num & 255) / 255,
      hex: '#' + clean.slice(0, 6).toUpperCase()
    };
  }
  return null;
}

// --- Tool 1: scan_ui_engagement ---

function scanFileEngagement(filePath) {
  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split(/\r?\n/);
  const ext = path.extname(filePath).toLowerCase();

  const findings = [];
  let containerDepth = 0;
  let maxContainerDepth = 0;
  let maxDepthLine = 1;
  const buttonActions = [];
  const primaryButtons = [];
  const fontSizes = [];
  const hardcodedColors = [];
  const interactiveStateHandlers = [];
  const progressCues = [];
  const textBlocks = [];
  const unchunkedTexts = [];
  const negativeMicrocopy = [];
  const encouragingMicrocopy = [];
  const spacingCalls = [];
  const paddingCalls = [];

  // Track primary CTAs and nesting
  for (let i = 0; i < lines.length; i++) {
    const lineNum = i + 1;
    const line = lines[i];
    const trimmed = line.trim();

    // Container / Layout nesting in Rust/Iced, TSX, Flutter
    const containerOpens = (line.match(/\b(container|column!|row!|scrollable|Container|Column|Row|div|section)\b/g) || []).length;
    const containerCloses = (line.match(/[\]\)\}]/g) || []).length;
    if (containerOpens > 0) {
      containerDepth += containerOpens;
      if (containerDepth > maxContainerDepth) {
        maxContainerDepth = containerDepth;
        maxDepthLine = lineNum;
      }
    }
    // Approximate depth decrement
    if (trimmed.startsWith(']') || trimmed.startsWith(')') || trimmed.startsWith('}')) {
      containerDepth = Math.max(0, containerDepth - 1);
    }

    // Button / Actions
    if (/\b(button\s*\(|Button\s*\(|<button|<Button|ElevatedButton|TextButton)\b/.test(line)) {
      buttonActions.push({ line: lineNum, text: trimmed });
      if (/\b(primary_button|ButtonVariant::Primary|style\s*:\s*primary|variant\s*=\s*["']primary["'])\b/i.test(line)) {
        primaryButtons.push({ line: lineNum, text: trimmed });
      }
    }
    if (/\.on_press\(|onPress=|onClick=|onPressed:/.test(line)) {
      interactiveStateHandlers.push({ line: lineNum, text: trimmed });
    }

    // Spacing / Padding
    if (/\.spacing\s*\(|spacing:\s*\d+/.test(line)) {
      spacingCalls.push({ line: lineNum });
    }
    if (/\.padding\s*\(|padding:\s*/.test(line)) {
      paddingCalls.push({ line: lineNum });
    }

    // Font Sizes
    const sizeMatches = line.matchAll(/\.size\s*\(\s*(\d+)\s*\)|fontSize:\s*(\d+)|font-size:\s*(\d+)px/g);
    for (const match of sizeMatches) {
      const sz = parseInt(match[1] || match[2] || match[3], 10);
      fontSizes.push({ line: lineNum, size: sz });
    }

    // Hardcoded colors
    const rgbMatch = line.matchAll(/Color::from_rgb\s*\(\s*([\d\.]+)\s*,\s*([\d\.]+)\s*,\s*([\d\.]+)\s*\)/g);
    for (const match of rgbMatch) {
      hardcodedColors.push({
        line: lineNum,
        type: 'Color::from_rgb',
        r: parseFloat(match[1]),
        g: parseFloat(match[2]),
        b: parseFloat(match[3]),
        text: match[0]
      });
    }
    const rgbaMatch = line.matchAll(/Color::from_rgba\s*\(\s*([\d\.]+)\s*,\s*([\d\.]+)\s*,\s*([\d\.]+)\s*,\s*([\d\.]+)\s*\)/g);
    for (const match of rgbaMatch) {
      hardcodedColors.push({
        line: lineNum,
        type: 'Color::from_rgba',
        r: parseFloat(match[1]),
        g: parseFloat(match[2]),
        b: parseFloat(match[3]),
        a: parseFloat(match[4]),
        text: match[0]
      });
    }
    const hexMatch = line.matchAll(/#([0-9a-fA-F]{6}|[0-9a-fA-F]{3})\b/g);
    for (const match of hexMatch) {
      hardcodedColors.push({
        line: lineNum,
        type: 'hex',
        hex: match[0],
        text: match[0]
      });
    }

    // Progress, Streak, Dopamine cues
    if (/\b(progress_bar|streak|xp|matches_found|total_pairs|completed|milestone|badge|score|level|bonus)\b/i.test(line)) {
      progressCues.push({ line: lineNum, cue: trimmed });
    }

    // Microcopy analysis in string literals
    const textLiteralMatch = line.matchAll(/text\s*\(\s*["']([^"']+)["']\s*\)|<Text[^>]*>([^<]+)<\/Text>|["']([^"']{10,})["']/g);
    for (const match of textLiteralMatch) {
      const str = (match[1] || match[2] || match[3] || '').trim();
      if (!str) continue;
      textBlocks.push({ line: lineNum, text: str });

      // Unchunked dense text (> 90 chars in single string without linebreaks)
      if (str.length > 90 && !str.includes('\n')) {
        unchunkedTexts.push({ line: lineNum, text: str, length: str.length });
      }

      // Check tone: negative vs empowering
      if (/\b(error|failed|invalid|wrong|forbidden|denied|timeout|syntax error)\b/i.test(str)) {
        negativeMicrocopy.push({ line: lineNum, text: str });
      }
      if (/\b(reset|match|streak|xp|level|well done|great job|keep going|focus|all pairs|break|complete|congratulations|mastery)\b/i.test(str)) {
        encouragingMicrocopy.push({ line: lineNum, text: str });
      }
    }
  }

  // Auditing Cognitive Load & Hick's Law
  if (primaryButtons.length > 3) {
    findings.push({
      pillar: 'Cognitive Load & Hick\'s Law',
      severity: 'FAIL',
      line: primaryButtons[3].line,
      widget: 'primary_button',
      issue: `Hick's Law Violation: Found ${primaryButtons.length} competing primary CTAs in view. Limit to 1-2 primary actions per study screen to avoid decision fatigue.`
    });
  } else if (primaryButtons.length > 2) {
    findings.push({
      pillar: 'Cognitive Load & Hick\'s Law',
      severity: 'WARN',
      line: primaryButtons[2].line,
      widget: 'primary_button',
      issue: `Elevated Cognitive Choice: ${primaryButtons.length} primary CTAs detected. Ensure single clear next step for the student.`
    });
  }

  if (maxContainerDepth > 6) {
    findings.push({
      pillar: 'Cognitive Load & Hick\'s Law',
      severity: 'WARN',
      line: maxDepthLine,
      widget: 'container_nesting',
      issue: `Excessive layout nesting depth (${maxContainerDepth} levels). High cognitive density and visual noise risk.`
    });
  }

  if (unchunkedTexts.length > 0) {
    findings.push({
      pillar: 'Cognitive Load & Hick\'s Law',
      severity: 'WARN',
      line: unchunkedTexts[0].line,
      widget: 'text',
      issue: `Unchunked Dense Text Block (${unchunkedTexts[0].length} chars). Long study sessions require chunking into 1-2 sentence bites with bullet points or bold anchors.`
    });
  }

  if (buttonActions.length > 6 && spacingCalls.length === 0) {
    findings.push({
      pillar: 'Cognitive Load & Hick\'s Law',
      severity: 'WARN',
      line: buttonActions[0].line,
      widget: 'spacing',
      issue: `Crowded action row/column: Multiple buttons detected without explicit spacing tokens between elements.`
    });
  }

  // Auditing Visual Hierarchy & Attention Anchoring
  const uniqueSizes = [...new Set(fontSizes.map(f => f.size))];
  if (uniqueSizes.length < 2 && fontSizes.length > 2) {
    findings.push({
      pillar: 'Attention Anchoring & Visual Hierarchy',
      severity: 'FAIL',
      line: fontSizes[0].line,
      widget: 'text_size',
      issue: `Flat Typography Hierarchy: Only ${uniqueSizes.length} font size (${uniqueSizes.join(', ')}px) used across multiple text elements. Header, metrics, and body copy lack distinct size contrast.`
    });
  } else if (uniqueSizes.length > 6) {
    findings.push({
      pillar: 'Attention Anchoring & Visual Hierarchy',
      severity: 'WARN',
      line: fontSizes[0].line,
      widget: 'text_size',
      issue: `Inconsistent Type Scale: ${uniqueSizes.length} different font sizes (${uniqueSizes.sort((a,b)=>a-b).join(', ')}px) used in single view. Unify to a 4-tier token scale (e.g. 11/12px caption, 14px body, 18px subhead, 24px metric).`
    });
  }

  // Auditing Dopamine & Micro-Reward Feedback Loops
  if (progressCues.length === 0 && buttonActions.length > 0) {
    findings.push({
      pillar: 'Dopamine & Micro-Reward Feedback Loops',
      severity: 'WARN',
      line: buttonActions[0].line,
      widget: 'progress_momentum',
      issue: `Missing Progress Momentum: Interactive view lacks visible progression cues (streaks, XP gain, progress bar, or milestone counter). Students experience study fatigue without incremental micro-feedback.`
    });
  }

  // Auditing Distraction Shielding & Flow Protection
  const dangerColors = hardcodedColors.filter(c => {
    if (c.r !== undefined) return c.r > 0.85 && c.g < 0.35 && c.b < 0.35;
    if (c.hex) {
      const rgb = hexToRgb(c.hex);
      return rgb && rgb.r > 0.85 && rgb.g < 0.35 && rgb.b < 0.35;
    }
    return false;
  });
  if (dangerColors.length > 2) {
    findings.push({
      pillar: 'Distraction Shielding & Flow Protection',
      severity: 'WARN',
      line: dangerColors[0].line,
      widget: 'danger_accents',
      issue: `Multiple high-contrast alarm red accents (${dangerColors.length}) detected. High-contrast alert saturation triggers cortisol and breaks deep focus. Reserve intense red strictly for destructive or urgent actions.`
    });
  }

  // Auditing Emotional Resonance & Student Agency
  if (negativeMicrocopy.length > 0) {
    findings.push({
      pillar: 'Emotional Resonance & Student Agency',
      severity: 'WARN',
      line: negativeMicrocopy[0].line,
      widget: 'microcopy',
      issue: `Robotic or Punitive Microcopy: "${negativeMicrocopy[0].text}". Reframe error states into constructive growth cues (e.g. "Let's review this concept together" vs "Incorrect/Failed").`
    });
  }

  return {
    filePath,
    lineCount: lines.length,
    metrics: {
      maxContainerDepth,
      maxDepthLine,
      totalButtons: buttonActions.length,
      primaryButtonsCount: primaryButtons.length,
      interactiveHandlersCount: interactiveStateHandlers.length,
      fontSizesUsed: uniqueSizes.sort((a, b) => a - b),
      hardcodedColorsCount: hardcodedColors.length,
      progressCuesCount: progressCues.length,
      textBlocksCount: textBlocks.length,
      unchunkedTextsCount: unchunkedTexts.length,
      encouragingMicrocopyCount: encouragingMicrocopy.length,
      negativeMicrocopyCount: negativeMicrocopy.length
    },
    findings
  };
}

// --- Tool 2: check_contrast_ergonomics ---

function parseColorsAndCheckContrast(filePath, inputColors = null) {
  let content = '';
  if (filePath && fs.existsSync(filePath)) {
    content = fs.readFileSync(filePath, 'utf-8');
  }

  const lines = content ? content.split(/\r?\n/) : [];
  const extractedColors = [];

  if (content) {
    for (let i = 0; i < lines.length; i++) {
      const lineNum = i + 1;
      const line = lines[i];

      // Match Color::from_rgb(r, g, b)
      for (const m of line.matchAll(/Color::from_rgb\s*\(\s*([\d\.]+)\s*,\s*([\d\.]+)\s*,\s*([\d\.]+)\s*\)/g)) {
        const r = parseFloat(m[1]), g = parseFloat(m[2]), b = parseFloat(m[3]);
        extractedColors.push({
          line: lineNum,
          name: line.split(':')[0]?.trim() || 'color',
          r, g, b, a: 1.0,
          raw: m[0],
          hex: '#' + [r, g, b].map(x => Math.round(x * 255).toString(16).padStart(2, '0')).join('').toUpperCase()
        });
      }

      // Match Color::from_rgba(r, g, b, a)
      for (const m of line.matchAll(/Color::from_rgba\s*\(\s*([\d\.]+)\s*,\s*([\d\.]+)\s*,\s*([\d\.]+)\s*,\s*([\d\.]+)\s*\)/g)) {
        const r = parseFloat(m[1]), g = parseFloat(m[2]), b = parseFloat(m[3]), a = parseFloat(m[4]);
        extractedColors.push({
          line: lineNum,
          name: line.split(':')[0]?.trim() || 'color',
          r, g, b, a,
          raw: m[0],
          hex: '#' + [r, g, b].map(x => Math.round(x * 255).toString(16).padStart(2, '0')).join('').toUpperCase()
        });
      }

      // Match hex codes
      for (const m of line.matchAll(/#([0-9a-fA-F]{6})\b/g)) {
        const rgb = hexToRgb(m[0]);
        if (rgb) {
          extractedColors.push({
            line: lineNum,
            name: line.split(':')[0]?.trim() || 'hex',
            r: rgb.r, g: rgb.g, b: rgb.b, a: 1.0,
            raw: m[0],
            hex: rgb.hex
          });
        }
      }
    }
  }

  // Combine with explicit input colors if provided
  if (Array.isArray(inputColors)) {
    for (const c of inputColors) {
      if (typeof c === 'string' && c.startsWith('#')) {
        const rgb = hexToRgb(c);
        if (rgb) extractedColors.push({ line: 1, name: 'input', r: rgb.r, g: rgb.g, b: rgb.b, a: 1.0, raw: c, hex: rgb.hex });
      }
    }
  }

  // Analyze contrast evaluations
  const contrastEvaluations = [];
  const ergonomicFlags = [];

  // Group by background and text roles if theme file
  const backgrounds = extractedColors.filter(c => /background|surface|glass_surface/i.test(c.name));
  const foregrounds = extractedColors.filter(c => /text|foreground|accent/i.test(c.name));

  // If no labeled roles found, check pairwise against darkest/lightest
  let darkBg = extractedColors.find(c => c.r < 0.2 && c.g < 0.2 && c.b < 0.2);
  let lightFg = extractedColors.find(c => c.r > 0.8 && c.g > 0.8 && c.b > 0.8);

  // Check 1: Harsh Pure White on Pure Black Glare (Halation Eye-Strain)
  const pureBlack = extractedColors.find(c => c.r === 0.0 && c.g === 0.0 && c.b === 0.0);
  const pureWhite = extractedColors.find(c => c.r === 1.0 && c.g === 1.0 && c.b === 1.0);
  if (pureBlack && pureWhite) {
    ergonomicFlags.push({
      type: 'GLARE_HAZARD',
      severity: 'WARN',
      issue: 'Harsh Pure-White (#FFFFFF) on Pure-Black (#000000) glare detected.',
      recommendation: 'In nocturnal or extended study modes, maximum 21:1 pure contrast causes astigmatic halation and pupil fatigue. Soften pure black to deep obsidian (#0D1117 / Color::from_rgb(0.05, 0.07, 0.10)) and text to gentle off-white (#E6EDF3 / Color::from_rgb(0.90, 0.93, 0.95)).'
    });
  }

  // Check 2: Low-alpha frosted glass readability risk
  const lowAlphaGlass = extractedColors.filter(c => c.a !== undefined && c.a > 0.0 && c.a < 0.70 && /glass|surface/i.test(c.name));
  for (const glass of lowAlphaGlass) {
    ergonomicFlags.push({
      type: 'TRANSLUCENCY_LEGIBILITY_RISK',
      severity: 'WARN',
      line: glass.line,
      issue: `Translucent surface opacity too low (${Math.round(glass.a * 100)}%). Opacity below 70% allows desktop wallpapers to bleed through, causing severe text legibility degradation.`,
      recommendation: `Increase glass surface alpha to >= 0.74 (e.g. Color::from_rgba(..., 0.75)) and add a 1px scrim border.`
    });
  }

  // Check contrast ratios between identified pairs
  if (backgrounds.length > 0 && foregrounds.length > 0) {
    for (const bg of backgrounds) {
      const bgLum = getRelativeLuminance(bg.r, bg.g, bg.b);
      for (const fg of foregrounds) {
        const fgLum = getRelativeLuminance(fg.r, fg.g, fg.b);
        const ratio = calculateContrastRatio(bgLum, fgLum);
        const passNormal = ratio >= 4.5;
        const passLarge = ratio >= 3.0;
        const passAAA = ratio >= 7.0;

        const evalItem = {
          bgName: bg.name,
          bgHex: bg.hex,
          fgName: fg.name,
          fgHex: fg.hex,
          ratio: parseFloat(ratio.toFixed(2)),
          wcag_AA_normal: passNormal ? 'PASS' : 'FAIL',
          wcag_AA_large: passLarge ? 'PASS' : 'FAIL',
          wcag_AAA: passAAA ? 'PASS' : 'FAIL'
        };
        contrastEvaluations.push(evalItem);

        if (!passNormal && /text_primary|text_secondary|body/i.test(fg.name)) {
          ergonomicFlags.push({
            type: 'CONTRAST_DEFICIT',
            severity: 'FAIL',
            line: fg.line,
            issue: `Low Contrast Ratio (${evalItem.ratio}:1) between ${fg.name} (${fg.hex}) and ${bg.name} (${bg.hex}). Fails WCAG AA minimum 4.5:1.`,
            recommendation: `Lighten or darken ${fg.name} to achieve at least 4.5:1 for body copy to prevent student eye strain during marathon study sessions.`
          });
        }
      }
    }
  }

  return {
    filePath,
    totalColorsParsed: extractedColors.length,
    colors: extractedColors.slice(0, 20),
    contrastEvaluations: contrastEvaluations.slice(0, 15),
    ergonomicFlags
  };
}

// --- Tool 3: generate_engagement_scorecard ---

function generateScorecard(scanData, contrastData, viewName = 'Target UI View') {
  const metrics = scanData?.metrics || {};
  const findings = scanData?.findings || [];
  const contrastFlags = contrastData?.ergonomicFlags || [];

  // Pillar 1: Cognitive Load & Hick's Law (20 pts)
  let p1Score = 20;
  const p1Deductions = [];
  if (metrics.primaryButtonsCount > 3) {
    p1Score -= 8;
    p1Deductions.push(`-8: Hick's Law violation with ${metrics.primaryButtonsCount} competing primary CTAs`);
  } else if (metrics.primaryButtonsCount > 2) {
    p1Score -= 4;
    p1Deductions.push(`-4: ${metrics.primaryButtonsCount} primary CTAs competing for student focus`);
  }
  if (metrics.maxContainerDepth > 6) {
    p1Score -= 4;
    p1Deductions.push(`-4: High container nesting depth (${metrics.maxContainerDepth} levels)`);
  }
  if (metrics.unchunkedTextsCount > 0) {
    p1Score -= 4;
    p1Deductions.push(`-4: ${metrics.unchunkedTextsCount} unchunked dense text blocks (>90 chars)`);
  }
  p1Score = Math.max(0, p1Score);

  // Pillar 2: Attention Anchoring & Visual Hierarchy (20 pts)
  let p2Score = 20;
  const p2Deductions = [];
  const fontSizesCount = metrics.fontSizesUsed?.length || 0;
  if (fontSizesCount === 1) {
    p2Score -= 8;
    p2Deductions.push('-8: Flat typography scale (only 1 font size detected)');
  } else if (fontSizesCount > 6) {
    p2Score -= 4;
    p2Deductions.push(`-4: Fragmented typography scale (${fontSizesCount} different font sizes)`);
  }
  p2Score = Math.max(0, p2Score);

  // Pillar 3: Dopamine & Micro-Reward Feedback Loops (20 pts)
  let p3Score = 20;
  const p3Deductions = [];
  if (metrics.progressCuesCount === 0) {
    p3Score -= 9;
    p3Deductions.push('-9: Zero visible progress cues (XP, streak, milestone, progress bar)');
  } else if (metrics.progressCuesCount < 2) {
    p3Score -= 3;
    p3Deductions.push('-3: Minimal progress feedback during task execution');
  }
  if (metrics.interactiveHandlersCount === 0 && metrics.totalButtons > 0) {
    p3Score -= 6;
    p3Deductions.push('-6: Dead-end button elements with missing interactive feedback');
  }
  p3Score = Math.max(0, p3Score);

  // Pillar 4: Distraction Shielding & Flow Protection (15 pts)
  let p4Score = 15;
  const p4Deductions = [];
  const dangerFlags = findings.filter(f => f.widget === 'danger_accents');
  if (dangerFlags.length > 0) {
    p4Score -= 5;
    p4Deductions.push('-5: Excessive alert red accents in focus zone triggering cognitive stress');
  }
  p4Score = Math.max(0, p4Score);

  // Pillar 5: Eye-Strain & Long-Session Ergonomics (15 pts)
  let p5Score = 15;
  const p5Deductions = [];
  const failContrast = contrastFlags.filter(f => f.severity === 'FAIL');
  const warnContrast = contrastFlags.filter(f => f.severity === 'WARN');
  if (failContrast.length > 0) {
    p5Score -= 7;
    p5Deductions.push(`-7: ${failContrast.length} color pair(s) failing WCAG AA (4.5:1) body contrast`);
  }
  if (warnContrast.some(w => w.type === 'GLARE_HAZARD')) {
    p5Score -= 3;
    p5Deductions.push('-3: Pure white on pure black high-halation glare hazard');
  }
  if (warnContrast.some(w => w.type === 'TRANSLUCENCY_LEGIBILITY_RISK')) {
    p5Score -= 3;
    p5Deductions.push('-3: Low-opacity frosted glass risking wallpaper text washout');
  }
  p5Score = Math.max(0, p5Score);

  // Pillar 6: Emotional Resonance & Student Agency (10 pts)
  let p6Score = 10;
  const p6Deductions = [];
  if (metrics.negativeMicrocopyCount > 0) {
    p6Score -= 5;
    p6Deductions.push(`-5: ${metrics.negativeMicrocopyCount} punitive or discouraging microcopy string(s)`);
  }
  if (metrics.encouragingMicrocopyCount === 0 && metrics.totalButtons > 0) {
    p6Score -= 3;
    p6Deductions.push('-3: Lack of encouraging or empowering student guidance copy');
  }
  p6Score = Math.max(0, p6Score);

  const totalScore = p1Score + p2Score + p3Score + p4Score + p5Score + p6Score;
  const status = totalScore >= 85 ? 'PASS' : totalScore >= 70 ? 'WARN' : 'FAIL';

  function pillarStatus(score, max) {
    const pct = score / max;
    return pct >= 0.85 ? 'PASS' : pct >= 0.70 ? 'WARN' : 'FAIL';
  }

  const pillars = [
    {
      name: '1. Cognitive Load & Hick\'s Law',
      weight: 20,
      score: p1Score,
      status: pillarStatus(p1Score, 20),
      deductions: p1Deductions,
      summary: p1Deductions.length === 0 ? 'Optimal chunking & micro-focus' : p1Deductions.join('; ')
    },
    {
      name: '2. Attention Anchoring & Visual Hierarchy',
      weight: 20,
      score: p2Score,
      status: pillarStatus(p2Score, 20),
      deductions: p2Deductions,
      summary: p2Deductions.length === 0 ? 'Distinct focal anchoring & clear typography scale' : p2Deductions.join('; ')
    },
    {
      name: '3. Dopamine & Micro-Reward Feedback Loops',
      weight: 20,
      score: p3Score,
      status: pillarStatus(p3Score, 20),
      deductions: p3Deductions,
      summary: p3Deductions.length === 0 ? 'High momentum with streaks, XP, & milestone markers' : p3Deductions.join('; ')
    },
    {
      name: '4. Distraction Shielding & Flow Protection',
      weight: 15,
      score: p4Score,
      status: pillarStatus(p4Score, 15),
      deductions: p4Deductions,
      summary: p4Deductions.length === 0 ? 'Calm study sanctuary with zero alert noise' : p4Deductions.join('; ')
    },
    {
      name: '5. Eye-Strain & Long-Session Ergonomics',
      weight: 15,
      score: p5Score,
      status: pillarStatus(p5Score, 15),
      deductions: p5Deductions,
      summary: p5Deductions.length === 0 ? 'Ergonomic contrast & glare-shielded palette' : p5Deductions.join('; ')
    },
    {
      name: '6. Emotional Resonance & Student Agency',
      weight: 10,
      score: p6Score,
      status: pillarStatus(p6Score, 10),
      deductions: p6Deductions,
      summary: p6Deductions.length === 0 ? 'Empowering, growth-oriented microcopy' : p6Deductions.join('; ')
    }
  ];

  return {
    viewName,
    totalScore,
    maxScore: 100,
    status,
    pillars,
    allFindings: [...findings, ...contrastFlags]
  };
}

// --- Standard JSON-RPC 2.0 stdio MCP Dispatcher ---

const TOOLS = [
  {
    name: 'scan_ui_engagement',
    description: 'Scans Rust/Iced (.rs), WGSL (.wgsl), CSS/TSX, or Dart UI files, extracts layout density metrics (container depth, CTA counts, font scales, hardcoded colors, dopamine cues), and returns structured findings.',
    inputSchema: {
      type: 'object',
      properties: {
        path: {
          type: 'string',
          description: 'Absolute or relative path to the UI file or directory to scan.'
        }
      },
      required: ['path']
    }
  },
  {
    name: 'check_contrast_ergonomics',
    description: 'Parses color definitions (Color::from_rgb, Color::from_rgba, hex, rgba) from a UI file or list and audits WCAG AA/AAA contrast ratios, frosted-glass opacity, and eye-strain glare risks.',
    inputSchema: {
      type: 'object',
      properties: {
        path: {
          type: 'string',
          description: 'Path to the UI or theme file to parse colors from.'
        },
        colors: {
          type: 'array',
          items: { type: 'string' },
          description: 'Optional array of hex colors to test against backgrounds.'
        }
      }
    }
  },
  {
    name: 'generate_engagement_scorecard',
    description: 'Accepts scan results and contrast audits, computes a 0-100 weighted Neuro-Engagement Scorecard across 6 cognitive pillars with PASS/WARN/FAIL statuses and actionable patches.',
    inputSchema: {
      type: 'object',
      properties: {
        scan_data: {
          type: 'object',
          description: 'The JSON result from scan_ui_engagement.'
        },
        contrast_data: {
          type: 'object',
          description: 'The JSON result from check_contrast_ergonomics.'
        },
        view_name: {
          type: 'string',
          description: 'Name of the audited view/screen (e.g. Break Arcade, Study Capsule).'
        }
      },
      required: ['scan_data']
    }
  }
];

function handleRpcRequest(request) {
  const { id, method, params } = request;

  if (method === 'initialize') {
    return {
      jsonrpc: '2.0',
      id,
      result: {
        protocolVersion: '2024-11-05',
        capabilities: {
          tools: {}
        },
        serverInfo: {
          name: 'NERO-ENGAGEING-INSPECTOR',
          version: '1.0.0'
        }
      }
    };
  }

  if (method === 'notifications/initialized') {
    return null; // Notifications produce no response
  }

  if (method === 'ping') {
    return { jsonrpc: '2.0', id, result: {} };
  }

  if (method === 'tools/list') {
    return {
      jsonrpc: '2.0',
      id,
      result: {
        tools: TOOLS
      }
    };
  }

  if (method === 'tools/call') {
    const { name, arguments: args = {} } = params || {};

    try {
      if (name === 'scan_ui_engagement') {
        const targetPath = args.path;
        if (!targetPath) throw new Error('Missing required argument: path');

        const resolved = path.resolve(targetPath);
        if (!fs.existsSync(resolved)) {
          throw new Error(`File or directory not found: ${resolved}`);
        }

        const stats = fs.statSync(resolved);
        let scanResult;
        if (stats.isDirectory()) {
          const files = fs.readdirSync(resolved)
            .filter(f => /\.(rs|wgsl|tsx|jsx|css|dart)$/i.test(f))
            .map(f => path.join(resolved, f));
          const subScans = files.map(f => scanFileEngagement(f));
          scanResult = { directory: resolved, filesScanned: files.length, results: subScans };
        } else {
          scanResult = scanFileEngagement(resolved);
        }

        return {
          jsonrpc: '2.0',
          id,
          result: {
            content: [
              {
                type: 'text',
                text: JSON.stringify(scanResult, null, 2)
              }
            ]
          }
        };
      }

      if (name === 'check_contrast_ergonomics') {
        const targetPath = args.path ? path.resolve(args.path) : null;
        const contrastResult = parseColorsAndCheckContrast(targetPath, args.colors);

        return {
          jsonrpc: '2.0',
          id,
          result: {
            content: [
              {
                type: 'text',
                text: JSON.stringify(contrastResult, null, 2)
              }
            ]
          }
        };
      }

      if (name === 'generate_engagement_scorecard') {
        const scorecard = generateScorecard(args.scan_data, args.contrast_data, args.view_name);

        return {
          jsonrpc: '2.0',
          id,
          result: {
            content: [
              {
                type: 'text',
                text: JSON.stringify(scorecard, null, 2)
              }
            ]
          }
        };
      }

      throw new Error(`Unknown tool: ${name}`);
    } catch (err) {
      return {
        jsonrpc: '2.0',
        id,
        error: {
          code: -32603,
          message: err.message || 'Internal tool execution error'
        }
      };
    }
  }

  return {
    jsonrpc: '2.0',
    id,
    error: {
      code: -32601,
      message: `Method not found: ${method}`
    }
  };
}

// --- Stdio transport setup ---

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let buffer = '';

rl.on('line', line => {
  const trimmed = line.trim();
  if (!trimmed) return;

  // Handle optional Content-Length headers if sent by some MCP clients
  if (trimmed.startsWith('Content-Length:')) {
    return;
  }

  try {
    const request = JSON.parse(trimmed);
    const response = handleRpcRequest(request);
    if (response) {
      process.stdout.write(JSON.stringify(response) + '\n');
    }
  } catch (err) {
    process.stderr.write(`[NERO-INSPECTOR ERROR] Failed parsing JSON-RPC line: ${err.message}\n`);
  }
});

process.stderr.write('[NERO-ENGAGEING-INSPECTOR] Local stdio MCP Server started.\n');
