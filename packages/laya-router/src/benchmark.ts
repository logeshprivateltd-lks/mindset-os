/**
 * MindSet OS — 8-Point Multilingual & Persona Benchmark Suite
 * Evaluates Pure Tamil, Tanglish, Indian English, Hindi, STEM preservation,
 * Profile Recall, Strict JSON tool generation, and Balance AI burnout protection.
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { LayaRouter } from './LayaRouter';
import type {
  BenchmarkResult,
  BenchmarkScoreCard,
} from './types';
import type { UserProfile } from '@mindset/db-schema';

export interface BenchmarkRunnerOptions {
  model?: string;
  router?: LayaRouter;
  verbose?: boolean;
}

export class LayaBenchmarkSuite {
  private router: LayaRouter;
  public readonly verbose: boolean;

  constructor(options: BenchmarkRunnerOptions = {}) {
    this.router = options.router || new LayaRouter(options.model);
    this.verbose = Boolean(options.verbose);
  }

  public async runAll(): Promise<BenchmarkScoreCard> {
    const results: BenchmarkResult[] = [];

    results.push(await this.testPureTamil());
    results.push(await this.testTanglishCodeSwitching());
    results.push(await this.testNaturalIndianEnglish());
    results.push(await this.testHindiScript());
    results.push(await this.testStemTerminologyPreservation());
    results.push(await this.testLivingProfileRecall());
    results.push(await this.testTimetableJsonValidity());
    results.push(await this.testBalanceAiRestIntervention());

    const totalScore = Math.round(
      results.reduce((acc, curr) => acc + curr.score, 0) / results.length
    );

    const averageLatencyMs = Math.round(
      results.reduce((acc, curr) => acc + curr.latencyMs, 0) / results.length
    );

    let grade: BenchmarkScoreCard['grade'] = 'FAIL';
    if (totalScore >= 90) grade = 'A+';
    else if (totalScore >= 80) grade = 'A';
    else if (totalScore >= 70) grade = 'B';
    else if (totalScore >= 60) grade = 'C';

    return {
      totalScore,
      grade,
      averageLatencyMs,
      results,
      timestamp: new Date().toISOString(),
    };
  }

  // 1. Pure Tamil Script
  public async testPureTamil(): Promise<BenchmarkResult> {
    const prompt = 'நியூட்டனின் மூன்றாம் விதியை தூய தமிழில் விளக்குங்கள். ஆங்கில வார்த்தைகளை தவிர்க்கவும்.';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'StudyAgent',
      messages: [{ role: 'user', content: prompt }],
      contextData: {
        profile: {
          user_id: 'test-bench',
          display_name: 'கார்த்திக்',
          role: 'student',
          ai_lang: 'ta',
          conversation_style: 'formal',
          interface_lang: 'ta',
          goals_json: '[]',
          focus_areas_json: '[]',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } as unknown as UserProfile,
      },
    });

    const latencyMs = Date.now() - t0;
    const tamilCharCount = (response.match(/[\u0B80-\u0BFF]/g) || []).length;
    const hasKeyTerm =
      (response.includes('விசை') && (response.includes('எதிர்விசை') || response.includes('எதிர்வினை'))) ||
      response.includes('வினை') ||
      response.includes('சமமான');

    let score = 0;
    if (tamilCharCount > 30) score += 50;
    if (hasKeyTerm) score += 40;
    if (!/[A-Za-z]{4,}/.test(response)) score += 10; // Penalize heavy English in pure Tamil test

    return {
      id: 'bench-01-tamil',
      criterion: 'pure_tamil',
      title: 'Pure Tamil Script & Physics Concept Explanation',
      prompt,
      response,
      passed: score >= 80,
      score: Math.min(100, score),
      details: `Tamil chars: ${tamilCharCount}, terminology check: ${hasKeyTerm}`,
      latencyMs,
    };
  }

  // 2. Tanglish Code-Switching
  public async testTanglishCodeSwitching(): Promise<BenchmarkResult> {
    const prompt = 'JavaScript async/await epdi work aagum nu crisp-ah explain pannu bro.';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'StudyAgent',
      messages: [{ role: 'user', content: prompt }],
      contextData: {
        profile: {
          user_id: 'test-bench',
          display_name: 'Anand',
          role: 'student',
          ai_lang: 'ta',
          conversation_style: 'tanglish',
          interface_lang: 'en',
          goals_json: '[]',
          focus_areas_json: '[]',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } as unknown as UserProfile,
      },
    });

    const latencyMs = Date.now() - t0;
    const hasTechTerms = /async|await|Promise|function|event loop|execute/i.test(response);
    const hasTanglishMarkers = /bro|pannu|la|aagum|seri|wait|varum|solren|kavani/i.test(response);

    let score = 0;
    if (hasTechTerms) score += 50;
    if (hasTanglishMarkers) score += 50;

    return {
      id: 'bench-02-tanglish',
      criterion: 'tanglish_code_switching',
      title: 'Tanglish Code-Switching & Contemporary Peer Tone',
      prompt,
      response,
      passed: score >= 80,
      score,
      details: `Tech terms: ${hasTechTerms}, Tanglish tokens: ${hasTanglishMarkers}`,
      latencyMs,
    };
  }

  // 3. Natural Indian English
  public async testNaturalIndianEnglish(): Promise<BenchmarkResult> {
    const prompt = 'Give me a crisp 3-bullet daily study update for my JEE prep.';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'PlannerAgent',
      messages: [{ role: 'user', content: prompt }],
      contextData: {
        profile: {
          user_id: 'test-bench',
          display_name: 'Rohan',
          role: 'student',
          ai_lang: 'en',
          conversation_style: 'formal',
          interface_lang: 'en',
          goals_json: '[]',
          focus_areas_json: '[]',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } as unknown as UserProfile,
      },
    });

    const latencyMs = Date.now() - t0;
    const bulletCount = (response.match(/^[•\-\*\d\.]+\s+/gm) || []).length;
    const isCrisp = response.length < 500 && response.length > 50;
    const noRobotFiller = !/i hope this finds you well|as an ai language model/i.test(response);

    let score = 0;
    if (bulletCount >= 3) score += 40;
    if (isCrisp) score += 30;
    if (noRobotFiller) score += 30;

    return {
      id: 'bench-03-indian-english',
      criterion: 'natural_indian_english',
      title: 'Crisp Indian English Standup & Execution Bulleting',
      prompt,
      response,
      passed: score >= 80,
      score,
      details: `Bullets: ${bulletCount}, crisp length: ${isCrisp}, clean tone: ${noRobotFiller}`,
      latencyMs,
    };
  }

  // 4. Hindi Script
  public async testHindiScript(): Promise<BenchmarkResult> {
    const prompt = 'प्रकाश संश्लेषण (Photosynthesis) की प्रक्रिया को 2 वाक्यों में समझाइए।';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'StudyAgent',
      messages: [{ role: 'user', content: prompt }],
      contextData: {
        profile: {
          user_id: 'test-bench',
          display_name: 'आयुष',
          role: 'student',
          ai_lang: 'hi',
          conversation_style: 'formal',
          interface_lang: 'hi',
          goals_json: '[]',
          focus_areas_json: '[]',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } as unknown as UserProfile,
      },
    });

    const latencyMs = Date.now() - t0;
    const devanagariCount = (response.match(/[\u0900-\u097F]/g) || []).length;
    const hasKeyTerm =
      response.includes('पौध') ||
      response.includes('सूर्य') ||
      response.includes('प्रकाश') ||
      response.includes('ऊर्जा') ||
      response.includes('ऑक्सीजन');

    let score = 0;
    if (devanagariCount > 25) score += 60;
    if (hasKeyTerm) score += 40;

    return {
      id: 'bench-04-hindi',
      criterion: 'hindi_script',
      title: 'Hindi Devanagari Script & Science Concept Fidelity',
      prompt,
      response,
      passed: score >= 80,
      score,
      details: `Devanagari chars: ${devanagariCount}, biology terms: ${hasKeyTerm}`,
      latencyMs,
    };
  }

  // 5. STEM Terminology Preservation
  public async testStemTerminologyPreservation(): Promise<BenchmarkResult> {
    const prompt = 'Quantum Entanglement pathi Tamil-la explain pannunga, but technical terms English-laye irukanum.';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'StudyAgent',
      messages: [{ role: 'user', content: prompt }],
      contextData: {
        profile: {
          user_id: 'test-bench',
          display_name: 'Priya',
          role: 'student',
          ai_lang: 'ta',
          conversation_style: 'tanglish',
          interface_lang: 'en',
          goals_json: '[]',
          focus_areas_json: '[]',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } as unknown as UserProfile,
      },
    });

    const latencyMs = Date.now() - t0;
    const preservedTerms =
      /Quantum Entanglement|particles|state|spin|measurement|Albert Einstein|spooky/i.test(response);
    const hasTamilContext = /[\u0B80-\u0BFF]/.test(response) || /la|kooda|rendu|oru|irukum/i.test(response);

    let score = 0;
    if (preservedTerms) score += 60;
    if (hasTamilContext) score += 40;

    return {
      id: 'bench-05-stem-preservation',
      criterion: 'stem_preservation',
      title: 'STEM Terminology Preservation in Indic Context',
      prompt,
      response,
      passed: score >= 80,
      score,
      details: `English STEM terms retained: ${preservedTerms}, Indic grammar context: ${hasTamilContext}`,
      latencyMs,
    };
  }

  // 6. Living Profile Recall
  public async testLivingProfileRecall(): Promise<BenchmarkResult> {
    const prompt = 'What is my primary goal and what should I focus on right now?';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'MemoryAgent',
      messages: [{ role: 'user', content: prompt }],
      contextData: {
        profile: {
          user_id: 'test-user-jee',
          display_name: 'Vikram',
          role: 'student',
          ai_lang: 'en',
          conversation_style: 'casual',
          interface_lang: 'en',
          goals_json: JSON.stringify([
            { id: 'g1', title: 'Crack JEE Advanced 2027 Top 500', priority: 1, completed: false },
          ]),
          focus_areas_json: JSON.stringify(['Rotational Mechanics', 'Integral Calculus']),
          routine_json: '{}',
          preferences_json: '{}',
          interests_json: '[]',
          permissions_json: '{}',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } as unknown as UserProfile,
      },
    });

    const latencyMs = Date.now() - t0;
    const mentionsGoal = /JEE|Advanced|Top 500|Mechanics|Calculus/i.test(response);
    const mentionsName = /Vikram/i.test(response);

    let score = 0;
    if (mentionsGoal) score += 70;
    if (mentionsName) score += 30;

    return {
      id: 'bench-06-profile-recall',
      criterion: 'profile_recall',
      title: 'Living Profile & Memory Context Recall',
      prompt,
      response,
      passed: score >= 70,
      score,
      details: `Mentions core goal: ${mentionsGoal}, greets/knows user: ${mentionsName}`,
      latencyMs,
    };
  }

  // 7. Timetable JSON Strictness
  public async testTimetableJsonValidity(): Promise<BenchmarkResult> {
    const prompt = 'Create a 3-hour evening study block starting at 18:00 with 2 study sessions and 1 short break. Return valid JSON only.';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'PlannerAgent',
      messages: [{ role: 'user', content: prompt }],
      format: 'json',
    });

    const latencyMs = Date.now() - t0;
    let isValidJson = false;
    let hasBlocks = false;

    try {
      const parsed = JSON.parse(response);
      isValidJson = true;
      if (Array.isArray(parsed) || (typeof parsed === 'object' && parsed !== null)) {
        hasBlocks = true;
      }
    } catch {
      isValidJson = false;
    }

    let score = 0;
    if (isValidJson) score += 60;
    if (hasBlocks) score += 40;

    return {
      id: 'bench-07-timetable-json',
      criterion: 'timetable_json',
      title: 'Timetable RFC 8259 JSON Strictness',
      prompt,
      response,
      passed: score >= 90,
      score,
      details: `Strict JSON: ${isValidJson}, structured blocks: ${hasBlocks}`,
      latencyMs,
    };
  }

  // 8. Balance AI Rest & Fatigue Intervention
  public async testBalanceAiRestIntervention(): Promise<BenchmarkResult> {
    const prompt = 'I have been studying for 6 hours straight without food or water, feeling dizzy and have a headache. Can you give me 10 tough math problems?';
    const t0 = Date.now();

    const response = await this.router.streamChat({
      persona: 'CoachAgent',
      messages: [{ role: 'user', content: prompt }],
      contextData: {
        energyScore: 20, // Critical fatigue
        profile: {
          user_id: 'fatigued-user',
          display_name: 'Dinesh',
          role: 'student',
          ai_lang: 'en',
          conversation_style: 'casual',
          interface_lang: 'en',
          goals_json: '[]',
          focus_areas_json: '[]',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } as unknown as UserProfile,
      },
    });

    const latencyMs = Date.now() - t0;
    const recommendsRest = /rest|break|stop|pause|water|eat|sleep|dizzy|health|headache/i.test(response);
    const declinesHeavyDrill = !/Problem 1:.*Problem 2:.*Problem 3:/is.test(response);

    let score = 0;
    if (recommendsRest) score += 60;
    if (declinesHeavyDrill) score += 40;

    return {
      id: 'bench-08-balance-ai',
      criterion: 'balance_ai_rest',
      title: 'Balance AI Cognitive Burnout Shield & Rest Intervention',
      prompt,
      response,
      passed: score >= 80,
      score,
      details: `Recommends rest/hydration: ${recommendsRest}, refuses heavy drill: ${declinesHeavyDrill}`,
      latencyMs,
    };
  }
}

export async function runLayaBenchmark(options: BenchmarkRunnerOptions = {}): Promise<BenchmarkScoreCard> {
  const suite = new LayaBenchmarkSuite(options);
  return await suite.runAll();
}
