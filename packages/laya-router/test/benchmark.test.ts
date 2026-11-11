/**
 * MindSet OS — Laya Router & Multilingual Benchmark Test Suite
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { LayaRouter, DEFAULT_LAYA_MODEL } from '../src/LayaRouter';
import { PERSONA_CONFIGS, getPersonaConfig } from '../src/personas';
import { LayaBenchmarkSuite } from '../src/benchmark';
import type { UserProfile } from '@mindset/db-schema';

async function runTests() {
  console.log('🧪 Starting Laya Router & 8-Point Benchmark Test Suite...\n');

  let passed = 0;
  let failed = 0;

  function assert(condition: boolean, msg: string) {
    if (condition) {
      console.log(`  ✅ PASS: ${msg}`);
      passed++;
    } else {
      console.error(`  ❌ FAIL: ${msg}`);
      failed++;
    }
  }

  // 1. Router State & Model Configuration
  console.log('--- 1. Router Initialization & Model State ---');
  const router = new LayaRouter();
  assert(router.getActiveModel() === DEFAULT_LAYA_MODEL, 'Default model matches gemma2:9b-instruct-q4_K_M');
  
  router.setActiveModel('qwen2.5:7b-instruct-q4_K_M');
  assert(router.getActiveModel() === 'qwen2.5:7b-instruct-q4_K_M', 'Active model updated dynamically');

  router.setActivePersona('PlannerAgent');
  assert(router.getActivePersona() === 'PlannerAgent', 'Active persona switched to PlannerAgent');

  // 2. Persona Base Configurations
  console.log('\n--- 2. Persona Base Configurations ---');
  const personas = ['StudyAgent', 'PlannerAgent', 'HabitAgent', 'MemoryAgent', 'CoachAgent'] as const;
  for (const p of personas) {
    const config = getPersonaConfig(p);
    assert(config.id === p, `Persona ${p} config matches ID`);
    assert(config.recommendedChips.length >= 4, `Persona ${p} has >= 4 recommended chips`);
    assert(config.systemBasePrompt.length > 100, `Persona ${p} has robust system prompt`);
  }

  // 3. System Prompt Synthesis & Guardrail Injection
  console.log('\n--- 3. System Prompt & Context Guardrails ---');
  const mockProfile: UserProfile = {
    user_id: 'user-007',
    display_name: 'Aditya',
    role: 'student',
    interface_lang: 'en',
    ai_lang: 'ta',
    conversation_style: 'tanglish',
    goals_json: JSON.stringify([{ id: 'g1', title: 'Ace NEET 2027', priority: 1, completed: false }]),
    focus_areas_json: JSON.stringify(['Organic Chemistry', 'Plant Physiology']),
    routine_json: '{}',
    preferences_json: '{}',
    interests_json: '[]',
    permissions_json: '{}',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  };

  const synthesized = router.buildSystemPrompt('StudyAgent', {
    persona: 'StudyAgent',
    messages: [],
    contextData: {
      profile: mockProfile,
      energyScore: 85,
    },
  });

  assert(synthesized.includes('Laya StudyAgent'), 'Includes StudyAgent base prompt');
  assert(synthesized.includes('<user_context_data'), 'Includes protected <user_context_data delimiter');
  assert(synthesized.includes('Aditya'), 'Includes user display name in context block');
  assert(synthesized.includes('Ace NEET 2027'), 'Includes user primary goal');
  assert(synthesized.includes('Tanglish') || synthesized.includes('tanglish'), 'Includes Tanglish language rules');

  // 4. Fallback Mock Generation & Tool Call Execution
  console.log('\n--- 4. Tool Execution & Streaming Fallback ---');
  const streamResult = await router.streamChat({
    persona: 'StudyAgent',
    messages: [{ role: 'user', content: 'What is momentum?' }],
  });
  assert(streamResult.length > 10, 'StreamChat returns coherent content');

  const jsonResult = await router.executeToolCall<{ status: string; plan?: any[] }>(
    'create_timetable',
    'Plan a study block'
  );
  assert(typeof jsonResult === 'object' && jsonResult !== null, 'Structured tool call returns valid object');

  // 5. Run the 8-Point Multilingual Benchmark Suite
  console.log('\n--- 5. 8-Point Multilingual & Persona Benchmark Suite ---');
  const benchSuite = new LayaBenchmarkSuite({ router: new LayaRouter(), verbose: true });
  const scorecard = await benchSuite.runAll();

  console.log(`\n  🏆 Total Score: ${scorecard.totalScore}/100 [Grade: ${scorecard.grade}]`);
  console.log(`  ⏱️ Average Latency: ${scorecard.averageLatencyMs}ms\n`);

  for (const r of scorecard.results) {
    assert(r.score >= 70, `Benchmark: [${r.criterion}] ${r.title} (Score: ${r.score}/100)`);
  }

  assert(scorecard.totalScore >= 80, 'Composite Benchmark Score >= 80 (Grade A or A+)');

  console.log(`\n========================================`);
  console.log(`Results: ${passed} passed, ${failed} failed`);
  console.log(`========================================\n`);

  if (failed > 0) {
    process.exit(1);
  }
}

runTests().catch((e) => {
  console.error('Fatal test error:', e);
  process.exit(1);
});
