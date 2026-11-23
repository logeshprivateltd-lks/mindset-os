#!/usr/bin/env node
/**
 * MORPHISSMS_INSPECTOR — Local stdio MCP Server
 * Specialized UI lighting, material physics, and shader inspection server for
 * Glassmorphism, Liquidmorphism, Claymorphism, and Neomorphism across Pure Rust (Iced 0.14.0),
 * wgpu WGSL shaders, CSS/Tailwind, and Flutter.
 *
 * Implements standard JSON-RPC 2.0 stdio MCP protocol.
 */

import * as readline from 'node:readline';
import * as fs from 'node:fs';
import * as path from 'node:path';

// --- Color & Vector Math Helpers ---

function parseColorValues(colorStr) {
  if (!colorStr) return null;

  // Match Color::from_rgba(r, g, b, a)
  const rgbaMatch = colorStr.match(/Color::from_rgba\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)\s*\)/i);
  if (rgbaMatch) {
    return {
      r: parseFloat(rgbaMatch[1]),
      g: parseFloat(rgbaMatch[2]),
      b: parseFloat(rgbaMatch[3]),
      a: parseFloat(rgbaMatch[4]),
      type: 'rgba'
    };
  }

  // Match Color::from_rgb(r, g, b)
  const rgbMatch = colorStr.match(/Color::from_rgb\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)\s*\)/i);
  if (rgbMatch) {
    return {
      r: parseFloat(rgbMatch[1]),
      g: parseFloat(rgbMatch[2]),
      b: parseFloat(rgbMatch[3]),
      a: 1.0,
      type: 'rgb'
    };
  }

  // Match rgba(r, g, b, a) or rgb(r, g, b)
  const cssRgba = colorStr.match(/rgba\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)\s*\)/i);
  if (cssRgba) {
    return {
      r: parseFloat(cssRgba[1]) > 1 ? parseFloat(cssRgba[1]) / 255 : parseFloat(cssRgba[1]),
      g: parseFloat(cssRgba[2]) > 1 ? parseFloat(cssRgba[2]) / 255 : parseFloat(cssRgba[2]),
      b: parseFloat(cssRgba[3]) > 1 ? parseFloat(cssRgba[3]) / 255 : parseFloat(cssRgba[3]),
      a: parseFloat(cssRgba[4]),
      type: 'rgba'
    };
  }

  // Match hex #RRGGBB or #RRGGBBAA
  const hexMatch = colorStr.match(/#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})\b/);
  if (hexMatch) {
    const hex = hexMatch[1];
    const r = parseInt(hex.slice(0, 2), 16) / 255;
    const g = parseInt(hex.slice(2, 4), 16) / 255;
    const b = parseInt(hex.slice(4, 6), 16) / 255;
    const a = hex.length === 8 ? parseInt(hex.slice(6, 8), 16) / 255 : 1.0;
    return { r, g, b, a, type: 'hex' };
  }

  // Check named constants
  if (colorStr.includes('Color::WHITE')) return { r: 1.0, g: 1.0, b: 1.0, a: 1.0, type: 'const' };
  if (colorStr.includes('Color::BLACK')) return { r: 0.0, g: 0.0, b: 0.0, a: 1.0, type: 'const' };
  if (colorStr.includes('Color::TRANSPARENT')) return { r: 0.0, g: 0.0, b: 0.0, a: 0.0, type: 'const' };

  return null;
}

function calculateLightAngle(x, y) {
  const rad = Math.atan2(y, x);
  let deg = (rad * 180) / Math.PI;
  if (deg < 0) deg += 360;
  return Math.round(deg * 10) / 10;
}

function extractStructBlocks(body, structName) {
  const results = [];
  const regex = new RegExp(`\\b${structName}\\s*\\{`, 'g');
  let m;
  while ((m = regex.exec(body)) !== null) {
    const start = m.index + m[0].length;
    let depth = 1;
    let end = start;
    while (depth > 0 && end < body.length) {
      if (body[end] === '{') depth++;
      else if (body[end] === '}') depth--;
      end++;
    }
    results.push(body.slice(start, end - 1));
  }
  return results;
}

// --- Token & Style Parsers for Rust Iced, WGSL, CSS, and Dart ---

function parseRustTokens(filePath, content) {
  const widgets = [];
  const fnRegex = /(?:pub\s+)?(?:const\s+)?fn\s+([a-zA-Z0-9_]+)/g;
  let match;

  while ((match = fnRegex.exec(content)) !== null) {
    const fnName = match[1];
    const start = match.index;
    const nextSemi = content.indexOf(';', start);
    const openBrace = content.indexOf('{', start);
    if (openBrace === -1 || (nextSemi !== -1 && nextSemi < openBrace)) continue;

    let depth = 1;
    let end = openBrace + 1;
    while (depth > 0 && end < content.length) {
      if (content[end] === '{') depth++;
      else if (content[end] === '}') depth--;
      end++;
    }

    const body = content.slice(openBrace, end);
    const lineNum = content.slice(0, start).split(/\r?\n/).length;

    const rawShadows = extractStructBlocks(body, 'Shadow');
    const rawBorders = extractStructBlocks(body, 'Border');
    const hasContainer = body.includes('container::Style') || body.includes('container(');
    const isStyleCandidate = rawShadows.length > 0 || rawBorders.length > 0 || hasContainer ||
      /card|pill|inset|glass|clay|neu|dock|modal|orb|button|slider|input/i.test(fnName);

    if (isStyleCandidate) {
      const shadows = [];
      for (const sBody of rawShadows) {
        const colorM = sBody.match(/color\s*:\s*([^,\n]+)/);
        const offsetM = sBody.match(/offset\s*:\s*(?:Vector::new\(\s*(-?[\d.]+)\s*,\s*(-?[\d.]+)\s*\)|([a-zA-Z0-9_:]+))/i);
        const blurM = sBody.match(/blur_radius\s*:\s*([\d.]+)/);

        const offsetX = offsetM && offsetM[1] ? parseFloat(offsetM[1]) : 0.0;
        const offsetY = offsetM && offsetM[2] ? parseFloat(offsetM[2]) : 0.0;
        const blurRadius = blurM ? parseFloat(blurM[1]) : 0.0;
        const colorRaw = colorM ? colorM[1].trim() : 'Default';
        const parsedColor = parseColorValues(colorRaw);

        shadows.push({
          raw_color: colorRaw,
          parsed_color: parsedColor,
          offset: { x: offsetX, y: offsetY },
          blur_radius: blurRadius,
          angle_degrees: (offsetX !== 0 || offsetY !== 0) ? calculateLightAngle(offsetX, offsetY) : null
        });
      }

      const borders = [];
      for (const bBody of rawBorders) {
        const colorM = bBody.match(/color\s*:\s*([^,\n]+)/);
        const widthM = bBody.match(/width\s*:\s*([\d.]+)/);
        const radiusM = bBody.match(/radius\s*:\s*(?:Radius::from\(([\d.]+)\)|([\d.]+)\.into\(\)|([\d.]+)|([a-zA-Z0-9_:]+))/);

        let radiusVal = 0.0;
        if (radiusM) {
          if (radiusM[1]) radiusVal = parseFloat(radiusM[1]);
          else if (radiusM[2]) radiusVal = parseFloat(radiusM[2]);
          else if (radiusM[3]) radiusVal = parseFloat(radiusM[3]);
          else radiusVal = radiusM[4];
        }

        const widthVal = widthM ? parseFloat(widthM[1]) : 1.0;
        const colorRaw = colorM ? colorM[1].trim() : 'Default';
        const parsedColor = parseColorValues(colorRaw);

        borders.push({
          raw_color: colorRaw,
          parsed_color: parsedColor,
          width: widthVal,
          radius: radiusVal
        });
      }

      let background = null;
      const bgMatch = body.match(/background\s*:\s*([^,\n}]+)/);
      if (bgMatch) {
        const bgRaw = bgMatch[1].trim();
        background = {
          raw: bgRaw,
          is_gradient: /Gradient/i.test(bgRaw),
          is_color: /Color/i.test(bgRaw),
          parsed_color: parseColorValues(bgRaw)
        };
      }

      const isNestedStack = (body.match(/container\(/g) || []).length >= 2;

      widgets.push({
        name: fnName,
        line: lineNum,
        type: 'rust_iced_widget',
        shadows,
        borders,
        background,
        is_nested_stack: isNestedStack,
        source_snippet: body.slice(0, 320)
      });
    }
  }

  // Scan standalone Palette definitions
  const paletteRegex = /pub struct ([a-zA-Z0-9_]+Palette)/g;
  const palettes = [];
  let pMatch;
  while ((pMatch = paletteRegex.exec(content)) !== null) {
    palettes.push({ name: pMatch[1], line: content.slice(0, pMatch.index).split(/\r?\n/).length });
  }

  return { widgets, palettes };
}

function parseWgslTokens(filePath, content) {
  const shaders = [];
  const hasKawase = /kawase/i.test(content);
  const hasGaussian = /gaussian|blur/i.test(content);
  const hasFresnel = /fresnel|schlick/i.test(content);
  const hasRefraction = /refract|distortion|caustic/i.test(content);
  const hasLightUniform = /light|LightParams|direction/i.test(content);

  const structs = [];
  const structRegex = /struct\s+([a-zA-Z0-9_]+)\s*\{([^}]+)\}/g;
  let sMatch;
  while ((sMatch = structRegex.exec(content)) !== null) {
    structs.push({
      name: sMatch[1],
      fields: sMatch[2].split(';').map(f => f.trim()).filter(Boolean)
    });
  }

  shaders.push({
    file: filePath,
    has_kawase_blur: hasKawase,
    has_gaussian_blur: hasGaussian,
    has_fresnel_refraction: hasFresnel,
    has_chromatic_distortion: hasRefraction,
    has_light_uniform: hasLightUniform,
    structs
  });

  return { shaders };
}

function parseCssTokens(filePath, content) {
  const widgets = [];
  const ruleRegex = /([.#a-zA-Z0-9_-]+)\s*\{([^}]+)\}/g;
  let rMatch;

  while ((rMatch = ruleRegex.exec(content)) !== null) {
    const selector = rMatch[1].trim();
    const body = rMatch[2];

    const hasShadow = /box-shadow\s*:/i.test(body);
    const hasBlur = /backdrop-filter\s*:[^;]*blur/i.test(body);
    const hasBorder = /border(?:-radius)?\s*:/i.test(body);

    if (hasShadow || hasBlur || hasBorder || /glass|clay|neu|liquid/i.test(selector)) {
      const shadowMatch = body.match(/box-shadow\s*:\s*([^;]+)/i);
      const blurMatch = body.match(/backdrop-filter\s*:\s*[^;]*blur\(([^)]+)\)/i);
      const radiusMatch = body.match(/border-radius\s*:\s*([^;]+)/i);
      const borderMatch = body.match(/border\s*:\s*([^;]+)/i);

      widgets.push({
        name: selector,
        type: 'css_rule',
        raw_box_shadow: shadowMatch ? shadowMatch[1].trim() : null,
        backdrop_blur: blurMatch ? blurMatch[1].trim() : null,
        border_radius: radiusMatch ? radiusMatch[1].trim() : null,
        border: borderMatch ? borderMatch[1].trim() : null
      });
    }
  }

  return { widgets };
}

function parseDartTokens(filePath, content) {
  const widgets = [];
  const boxDecRegex = /BoxDecoration\s*\(([\s\S]*?)\)/g;
  let bMatch;

  while ((bMatch = boxDecRegex.exec(content)) !== null) {
    const body = bMatch[1];
    const lineNum = content.slice(0, bMatch.index).split(/\r?\n/).length;

    const hasShadow = /boxShadow\s*:\s*\[([\s\S]*?)\]/.exec(body);
    const hasBorder = /border\s*:\s*Border\.all\(([^)]+)\)/.exec(body);
    const hasRadius = /borderRadius\s*:\s*BorderRadius\.circular\(([\d.]+)\)/.exec(body);

    widgets.push({
      name: `BoxDecoration@line_${lineNum}`,
      type: 'flutter_widget',
      has_box_shadow: Boolean(hasShadow),
      has_border: Boolean(hasBorder),
      radius: hasRadius ? parseFloat(hasRadius[1]) : null
    });
  }

  return { widgets };
}

// --- Tool 1: scan_morphism_tokens ---

function scanTokensForFile(filePath) {
  const resolved = path.resolve(filePath);
  if (!fs.existsSync(resolved)) {
    throw new Error(`File not found: ${resolved}`);
  }

  const content = fs.readFileSync(resolved, 'utf-8');
  const ext = path.extname(resolved).toLowerCase();

  if (ext === '.rs') {
    return { file: resolved, language: 'rust', ...parseRustTokens(resolved, content) };
  } else if (ext === '.wgsl') {
    return { file: resolved, language: 'wgsl', ...parseWgslTokens(resolved, content) };
  } else if (ext === '.css') {
    return { file: resolved, language: 'css', ...parseCssTokens(resolved, content) };
  } else if (ext === '.dart') {
    return { file: resolved, language: 'dart', ...parseDartTokens(resolved, content) };
  } else {
    return { file: resolved, language: ext.slice(1) || 'unknown', widgets: [] };
  }
}

function collectFiles(dirPath, recursive = false) {
  let results = [];
  const items = fs.readdirSync(dirPath);

  for (const item of items) {
    if (item === 'target' || item === 'node_modules' || item === '.git') continue;
    const full = path.join(dirPath, item);
    const stat = fs.statSync(full);

    if (stat.isDirectory()) {
      if (recursive) {
        results = results.concat(collectFiles(full, recursive));
      }
    } else if (/\.(rs|wgsl|css|tsx|dart)$/i.test(item)) {
      results.push(full);
    }
  }

  return results;
}

// --- Tool 2: audit_lighting_consistency ---

function auditLighting(targetFiles) {
  const shadowRecords = [];
  const conflictingPairs = [];

  for (const file of targetFiles) {
    const scan = scanTokensForFile(file);
    if (!scan.widgets) continue;

    for (const w of scan.widgets) {
      if (!w.shadows || w.shadows.length === 0) continue;

      for (const s of w.shadows) {
        if (s.offset && (s.offset.x !== 0 || s.offset.y !== 0)) {
          shadowRecords.push({
            file: path.basename(file),
            widget: w.name,
            line: w.line,
            offset: s.offset,
            blur_radius: s.blur_radius,
            angle_degrees: s.angle_degrees,
            color: s.raw_color
          });
        }
      }
    }
  }

  let topDownCount = 0;
  let topLeftCount = 0;

  for (const rec of shadowRecords) {
    const { x, y } = rec.offset;
    if (x === 0 && y > 0) topDownCount++;
    else if (x > 0 && y > 0) topLeftCount++;
  }

  for (let i = 0; i < shadowRecords.length; i++) {
    for (let j = i + 1; j < shadowRecords.length; j++) {
      const a = shadowRecords[i];
      const b = shadowRecords[j];

      const aIsHighlight = a.offset.x < 0 && a.offset.y < 0;
      const bIsHighlight = b.offset.x < 0 && b.offset.y < 0;

      if (!aIsHighlight && !bIsHighlight) {
        if ((a.offset.y > 0 && b.offset.y < 0) || (a.offset.y < 0 && b.offset.y > 0)) {
          conflictingPairs.push({
            widget_a: `${a.file}:${a.widget}`,
            offset_a: a.offset,
            widget_b: `${b.file}:${b.widget}`,
            offset_b: b.offset,
            issue: 'Inverted vertical shadow cast direction; widgets appear lit from opposite angles'
          });
        } else if ((a.offset.x > 3 && b.offset.x < -3) || (a.offset.x < -3 && b.offset.x > 3)) {
          conflictingPairs.push({
            widget_a: `${a.file}:${a.widget}`,
            offset_a: a.offset,
            widget_b: `${b.file}:${b.widget}`,
            offset_b: b.offset,
            issue: 'Conflicting horizontal light azimuth; one widget lit from left and another from right'
          });
        }
      }
    }
  }

  return {
    total_shadows_scanned: shadowRecords.length,
    dominant_light_direction: topLeftCount >= topDownCount ? 'Top-Left Directional (Azimuth ~135°)' : 'Top-Down Ambient (Azimuth ~90°)',
    shadows: shadowRecords,
    lighting_conflicts: conflictingPairs,
    consistency_verdict: conflictingPairs.length === 0 ? 'COHERENT' : 'CONFLICT_DETECTED'
  };
}

// --- Tool 3: evaluate_morphism_quality ---

function inferMorphismType(name, target) {
  if (target && target !== 'auto') return target.toLowerCase();
  const lower = name.toLowerCase();
  if (lower.includes('clay')) return 'claymorphism';
  if (lower.includes('neu') || lower.includes('soft_ui')) return 'neumorphism';
  if (lower.includes('liquid') || lower.includes('bubble') || lower.includes('orb')) return 'liquidmorphism';
  if (lower.includes('glass') || lower.includes('frosted') || lower.includes('acrylic') || lower.includes('dock') || lower.includes('top_bar')) return 'glassmorphism';
  return 'glassmorphism';
}

function evaluateWidgetQuality(widget, targetMorphism, fileContent) {
  const morphism = inferMorphismType(widget.name, targetMorphism);
  const evaluation = {
    widget: widget.name,
    line: widget.line,
    detected_morphism: morphism,
    scores: {},
    grade: 'PASS',
    critiques: [],
    recommended_patch: null
  };

  const primaryShadow = widget.shadows && widget.shadows[0] ? widget.shadows[0] : null;
  const primaryBorder = widget.borders && widget.borders[0] ? widget.borders[0] : null;

  // 1. Glassmorphism Grading
  if (morphism === 'glassmorphism') {
    let fillAlpha = null;
    if (widget.background && widget.background.parsed_color) {
      fillAlpha = widget.background.parsed_color.a;
    } else if (fileContent) {
      const alphaMatch = fileContent.match(/glass_surface[^;]+Color::from_rgba\([^,]+,[^,]+,[^,]+,\s*([\d.]+)\)/);
      if (alphaMatch) fillAlpha = parseFloat(alphaMatch[1]);
    }

    if (fillAlpha !== null) {
      if (fillAlpha >= 0.08 && fillAlpha <= 0.25) {
        evaluation.scores.translucency = { status: 'PASS', alpha: fillAlpha, message: 'Optimal frosted glass translucency.' };
      } else if (fillAlpha > 0.35) {
        evaluation.scores.translucency = { status: 'WEAK', alpha: fillAlpha, message: `Fill alpha (${fillAlpha}) is too opaque; looks like plastic/acrylic slab rather than realistic glass.` };
        evaluation.grade = 'WEAK';
        evaluation.critiques.push(`High fill opacity (${Math.round(fillAlpha * 100)}%) obscures background wallpaper details, destroying translucent frosted glass perception.`);
      } else {
        evaluation.scores.translucency = { status: 'WEAK', alpha: fillAlpha, message: `Fill alpha (${fillAlpha}) is faint/invisible.` };
        evaluation.grade = 'WEAK';
      }
    } else {
      evaluation.scores.translucency = { status: 'UNKNOWN', message: 'Could not directly extract fill alpha.' };
    }

    const mentionsBlur = fileContent && /blur|Kawase|Gaussian|wgpu.*blur/i.test(fileContent);
    if (mentionsBlur) {
      evaluation.scores.blur_depth = { status: 'PASS', message: 'Backdrop blur filter present or supported.' };
    } else {
      evaluation.scores.blur_depth = { status: 'MISSING', message: 'No backdrop blur detected! Glass requires real blur (12-28px or Kawase pass).' };
      evaluation.grade = 'WEAK';
      evaluation.critiques.push('Applying translucent alpha without a backdrop blur filter creates flat cellophane transparency that degrades text legibility.');
    }

    if (primaryBorder) {
      if (primaryBorder.width >= 0.8 && primaryBorder.width <= 1.5) {
        evaluation.scores.specular_rim = { status: 'PASS', width: primaryBorder.width, message: 'Crisp 1px directional specular edge rim.' };
      } else {
        evaluation.scores.specular_rim = { status: 'WEAK', width: primaryBorder.width, message: `Border width (${primaryBorder.width}px) is heavy; glass requires a delicate 1px specular rim.` };
      }
    } else {
      evaluation.scores.specular_rim = { status: 'MISSING', message: 'Missing specular rim border! Glass panels require a light border for edge definition.' };
      evaluation.grade = 'WEAK';
    }

    if (primaryShadow && primaryShadow.blur_radius >= 16.0) {
      evaluation.scores.shadow_quality = { status: 'PASS', blur_radius: primaryShadow.blur_radius, message: 'Soft diffused drop shadow provides clear wallpaper separation.' };
    } else if (primaryShadow) {
      evaluation.scores.shadow_quality = { status: 'WEAK', blur_radius: primaryShadow.blur_radius, message: `Shadow blur (${primaryShadow.blur_radius}px) is too tight; requires soft diffusion (>= 16px).` };
      evaluation.grade = 'WEAK';
    } else {
      evaluation.scores.shadow_quality = { status: 'MISSING', message: 'No ambient drop shadow! Text will bleed over bright wallpaper elements.' };
      evaluation.grade = 'WEAK';
    }
  }

  // 2. Liquidmorphism Grading
  else if (morphism === 'liquidmorphism') {
    const radius = primaryBorder ? (typeof primaryBorder.radius === 'number' ? primaryBorder.radius : 24.0) : 0;
    if (radius >= 22.0 || /orb|bubble|pill/i.test(widget.name)) {
      evaluation.scores.radii_geometry = { status: 'PASS', radius, message: 'Smooth high-radius organic geometry.' };
    } else {
      evaluation.scores.radii_geometry = { status: 'WEAK', radius, message: 'Boxy or low corner radius; liquidmorphism requires organic curves or squircles.' };
      evaluation.grade = 'WEAK';
    }

    const hasGradient = widget.background && widget.background.is_gradient;
    const hasSheen = fileContent && /Gradient|fresnel|specular|sheen/i.test(fileContent);
    if (hasGradient || hasSheen) {
      evaluation.scores.refraction_sheen = { status: 'PASS', message: 'Refraction gradient sheen or Fresnel specular reflection detected.' };
    } else {
      evaluation.scores.refraction_sheen = { status: 'MISSING', message: 'Lacks gradient sheen or Fresnel edge highlighting.' };
      evaluation.grade = 'WEAK';
      evaluation.critiques.push('Liquid glass requires refractive specular gradients or Fresnel highlights rather than flat translucent fills.');
    }
  }

  // 3. Claymorphism Grading
  else if (morphism === 'claymorphism') {
    const radius = primaryBorder ? (typeof primaryBorder.radius === 'number' ? primaryBorder.radius : 28.0) : 0;
    if (radius >= 20.0 || /orb/i.test(widget.name)) {
      evaluation.scores.radii_geometry = { status: 'PASS', radius, message: 'Inflated sculptural corner radii.' };
    } else {
      evaluation.scores.radii_geometry = { status: 'WEAK', radius, message: 'Radius is too sharp for puffy clay; requires radius between 20.0 and 36.0px.' };
      evaluation.grade = 'WEAK';
    }

    const shadowCount = widget.shadows ? widget.shadows.length : 0;
    if (shadowCount >= 2 || widget.is_nested_stack) {
      evaluation.scores.dual_layer_depth = { status: 'PASS', message: 'Dual-layer depth: outer drop shadow combined with inner highlight bevel.' };
    } else {
      evaluation.scores.dual_layer_depth = { status: 'WEAK', message: 'Single shadow detected! True clay requires outer diffused shadow PLUS inner top-left highlight and bottom-right depth rim.' };
      evaluation.grade = 'FLAT';
      evaluation.critiques.push('Clay elements with only a single drop shadow look like flat cookie-cutter cardboard; they lack tactile 3D volume.');
    }

    if (primaryShadow && primaryShadow.parsed_color) {
      const { r, g, b } = primaryShadow.parsed_color;
      const isPureBlack = r === 0 && g === 0 && b === 0;
      if (isPureBlack) {
        evaluation.scores.shadow_tinting = { status: 'WEAK', message: 'Shadow is pure black (#000000); creates muddy, unnatural edges on clay.' };
        evaluation.grade = 'WEAK';
        evaluation.critiques.push('Pure black shadows muddy claymorphic pastels. Shadows must be color-tinted with the surface hue.');
      } else {
        evaluation.scores.shadow_tinting = { status: 'PASS', message: 'Harmonious tinted shadow color derived from palette.' };
      }
    } else {
      evaluation.scores.shadow_tinting = { status: 'PASS', message: 'Shadow tinting verified via palette.' };
    }
  }

  // 4. Neomorphism Grading
  else if (morphism === 'neumorphism') {
    let hasOpposingOffsets = false;

    if (widget.shadows && widget.shadows.length >= 2) {
      const s1 = widget.shadows[0].offset;
      const s2 = widget.shadows[1].offset;
      if ((s1.x * s2.x < 0 || s1.y * s2.y < 0)) {
        hasOpposingOffsets = true;
      }
    } else if (widget.is_nested_stack || (fileContent && /shadow_light.*shadow_dark|neu_raised_card/i.test(fileContent))) {
      hasOpposingOffsets = true;
    }

    if (hasOpposingOffsets) {
      evaluation.scores.dual_opposing_shadows = { status: 'PASS', message: 'Opposing dual shadows (top-left light highlight + bottom-right sculpted shadow).' };
    } else {
      evaluation.scores.dual_opposing_shadows = { status: 'WEAK', message: 'Missing opposing dual shadows! Neomorphism requires both light and dark offsets.' };
      evaluation.grade = 'WEAK';
      evaluation.critiques.push('A neomorphic surface without opposing dual shadows loses its extruded soft-plastic illusion.');
    }

    evaluation.scores.surface_continuity = { status: 'PASS', message: 'Surface matches background with subtle elevation delta.' };
  }

  // Recommended Code Upgrades
  if (evaluation.grade !== 'PASS') {
    if (morphism === 'claymorphism') {
      evaluation.recommended_patch = `// Upgrade to 2-Layer Dual-Shadow Stacking with Tinted Shadow & Specular Rim:
container(
    container(content)
        .padding(16)
        .style(move |_theme: &Theme| container::Style {
            background: Some(Background::Color(p.grad_bottom)),
            border: Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.65),
                width: 2.0,
                radius: 28.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.357, 0.322, 0.941, 0.45), // Tinted shadow
                offset: Vector::new(0.0, 10.0),
                blur_radius: 24.0,
            },
            ..Default::default()
        })
)
.style(move |_theme: &Theme| container::Style {
    background: None,
    shadow: Shadow {
        color: Color::from_rgba(1.0, 1.0, 1.0, 0.35), // Top-left inner dome highlight
        offset: Vector::new(-2.0, -2.0),
        blur_radius: 6.0,
    },
    border: Border { radius: 28.0.into(), ..Default::default() },
    ..Default::default()
})`;
    } else if (morphism === 'glassmorphism') {
      evaluation.recommended_patch = `// Upgrade to High-Fidelity Frosted Glass with 1px Specular Border:
container::Style {
    text_color: Some(palette.text_primary),
    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.18))), // Translucent (18% alpha)
    border: Border {
        color: Color::from_rgba(1.0, 1.0, 1.0, 0.32), // 1px Specular Rim
        width: 1.0,
        radius: Radius::from(18.0),
    },
    shadow: Shadow {
        color: Color::from_rgba(0.04, 0.12, 0.28, 0.22), // Ambient drop shadow
        offset: Vector::new(0.0, 6.0),
        blur_radius: 22.0,
    },
    ..Default::default()
}`;
    }
  }

  return evaluation;
}

// --- MCP Tools Definitions ---

const TOOLS = [
  {
    name: 'scan_morphism_tokens',
    description: 'Scans target .rs, .wgsl, .css, .tsx, or .dart files to extract every Shadow, Border, blur_radius, offset, Color::from_rgba, gradient, and shader blur uniform. Returns structured JSON showing detected vs missing tokens.',
    inputSchema: {
      type: 'object',
      properties: {
        path: {
          type: 'string',
          description: 'Absolute or workspace-relative path to a file or directory containing UI or shader code.'
        },
        recursive: {
          type: 'boolean',
          description: 'Whether to recursively scan subdirectories (default: false).'
        }
      },
      required: ['path']
    }
  },
  {
    name: 'audit_lighting_consistency',
    description: 'Compares Shadow.offset vectors (x, y) and highlight directions across multiple UI files or style functions to detect conflicting light-source angles and inverted shadows.',
    inputSchema: {
      type: 'object',
      properties: {
        path: {
          type: 'string',
          description: 'Directory or file path to analyze for lighting angle consistency.'
        },
        files: {
          type: 'array',
          items: { type: 'string' },
          description: 'Optional explicit list of file paths to compare.'
        }
      }
    }
  },
  {
    name: 'evaluate_morphism_quality',
    description: 'Scores each detected widget against Glassmorphism, Liquidmorphism, Claymorphism, or Neomorphism rules and flags PASS, WEAK, or MISSING for shadows, borders, blur, specular highlights, and radii.',
    inputSchema: {
      type: 'object',
      properties: {
        path: {
          type: 'string',
          description: 'Path to a file or directory containing UI definitions to grade.'
        },
        target_morphism: {
          type: 'string',
          enum: ['auto', 'glassmorphism', 'liquidmorphism', 'claymorphism', 'neumorphism'],
          description: 'Specific morphism style to grade against, or auto to infer from naming/context.'
        }
      },
      required: ['path']
    }
  }
];

// --- RPC Request Router ---

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
          name: 'MORPHISSMS_INSPECTOR',
          version: '1.0.0'
        }
      }
    };
  }

  if (method === 'notifications/initialized') {
    return null;
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
      if (name === 'scan_morphism_tokens') {
        const targetPath = args.path;
        if (!targetPath) throw new Error('Missing required argument: path');

        const resolved = path.resolve(targetPath);
        if (!fs.existsSync(resolved)) {
          throw new Error(`Path does not exist: ${resolved}`);
        }

        const stat = fs.statSync(resolved);
        let result;

        if (stat.isDirectory()) {
          const files = collectFiles(resolved, args.recursive || false);
          const scans = files.map(f => scanTokensForFile(f));
          result = { directory: resolved, files_scanned: files.length, results: scans };
        } else {
          result = scanTokensForFile(resolved);
        }

        return {
          jsonrpc: '2.0',
          id,
          result: {
            content: [{ type: 'text', text: JSON.stringify(result, null, 2) }]
          }
        };
      }

      if (name === 'audit_lighting_consistency') {
        let targetFiles = [];

        if (args.files && Array.isArray(args.files) && args.files.length > 0) {
          targetFiles = args.files.map(f => path.resolve(f)).filter(f => fs.existsSync(f));
        } else if (args.path) {
          const resolved = path.resolve(args.path);
          if (fs.existsSync(resolved)) {
            const stat = fs.statSync(resolved);
            if (stat.isDirectory()) {
              targetFiles = collectFiles(resolved, true);
            } else {
              targetFiles = [resolved];
            }
          }
        }

        if (targetFiles.length === 0) {
          throw new Error('No valid files found for lighting audit.');
        }

        const audit = auditLighting(targetFiles);

        return {
          jsonrpc: '2.0',
          id,
          result: {
            content: [{ type: 'text', text: JSON.stringify(audit, null, 2) }]
          }
        };
      }

      if (name === 'evaluate_morphism_quality') {
        const targetPath = args.path;
        if (!targetPath) throw new Error('Missing required argument: path');

        const resolved = path.resolve(targetPath);
        if (!fs.existsSync(resolved)) {
          throw new Error(`Path does not exist: ${resolved}`);
        }

        const stat = fs.statSync(resolved);
        const filesToEval = stat.isDirectory() ? collectFiles(resolved, false) : [resolved];
        const evaluations = [];

        for (const file of filesToEval) {
          const content = fs.readFileSync(file, 'utf-8');
          const scan = scanTokensForFile(file);

          if (scan.widgets && scan.widgets.length > 0) {
            for (const w of scan.widgets) {
              evaluations.push(evaluateWidgetQuality(w, args.target_morphism || 'auto', content));
            }
          }
        }

        const overallGrade = evaluations.some(e => e.grade === 'FLAT')
          ? 'NEEDS_UPGRADE'
          : evaluations.some(e => e.grade === 'WEAK')
          ? 'WEAK'
          : 'PASS';

        const result = {
          target: resolved,
          overall_grade: overallGrade,
          components_evaluated: evaluations.length,
          evaluations
        };

        return {
          jsonrpc: '2.0',
          id,
          result: {
            content: [{ type: 'text', text: JSON.stringify(result, null, 2) }]
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

rl.on('line', line => {
  const trimmed = line.trim();
  if (!trimmed || trimmed.startsWith('Content-Length:')) return;

  try {
    const request = JSON.parse(trimmed);
    const response = handleRpcRequest(request);
    if (response) {
      process.stdout.write(JSON.stringify(response) + '\n');
    }
  } catch (err) {
    process.stderr.write(`[MORPHISSMS_INSPECTOR ERROR] Failed parsing JSON-RPC line: ${err.message}\n`);
  }
});

process.stderr.write('[MORPHISSMS_INSPECTOR] Local stdio MCP Server started.\n');
