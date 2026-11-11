/**
 * MindSet OS Balance Engine Test Suite
 * Validates FreeTimeDetector, ActivityRecommender, and SmartFreeTimePlanner
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import {
  FreeTimeDetector,
  ActivityRecommender,
  SmartFreeTimePlanner,
  ScheduledBlock,
} from '../src';

function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(`Assertion failed: ${message}`);
  }
}

async function runTests() {
  console.log('--- [1/3] Testing FreeTimeDetector ---');
  const detector = new FreeTimeDetector({
    dayStartTime: '08:00',
    sleepStartTime: '22:00',
    defaultBufferMinutes: 15,
  });

  const sampleBlocks: ScheduledBlock[] = [
    { id: '1', title: 'Physics Lecture', startTime: '09:00', endTime: '11:00' },
    { id: '2', title: 'Maths Lab', startTime: '12:00', endTime: '13:30' },
    { id: '3', title: 'Evening Study', startTime: '17:00', endTime: '19:00' },
  ];

  const slots = detector.detectFreeSlots(sampleBlocks);
  console.log(`Detected ${slots.length} free-time slots.`);
  assert(slots.length >= 3, 'Should detect at least 3 gaps');

  // Slot 1: 08:00 to 09:00 (60 mins, buffer 15, usable 45)
  const slot1 = slots[0]!;
  assert(slot1.startTime === '08:00' && slot1.endTime === '09:00', 'Slot 1 should be 08:00-09:00');
  assert(slot1.usableMinutes === 45, `Slot 1 usable minutes should be 45, got ${slot1.usableMinutes}`);
  assert(!slot1.isWindDownWindow, 'Slot 1 morning should not be wind-down window');

  // Sleep Protection: slot after 19:00 should end at 22:00
  const eveningSlot = slots[slots.length - 1]!;
  assert(eveningSlot.endTime === '22:00', 'Last slot must be capped at 22:00 sleep routine');
  assert(eveningSlot.isWindDownWindow, 'Evening slot (19:00-22:00) must be flagged as wind-down');
  console.log('✓ FreeTimeDetector correctly enforces gap calculation & sleep routine.');

  console.log('\n--- [2/3] Testing ActivityRecommender ---');
  const recommender = new ActivityRecommender();

  // Test energy & mood matching
  const tiredRecs = recommender.recommendActivities({
    availableMinutes: 30,
    energyTier: 'VERY_LOW',
    mood: 'Tired',
  });
  assert(tiredRecs.length > 0, 'Should find activities for VERY_LOW energy');
  assert(
    tiredRecs[0]!.category === 'rest' || tiredRecs[0]!.category === 'mental',
    'Top recommendation for tired user should be rest or gentle mental'
  );

  // Test wind-down filter
  const windDownRecs = recommender.recommendActivities({
    availableMinutes: 60,
    isWindDown: true,
  });
  const hasHighEnergyInWindDown = windDownRecs.some((a) => a.energyTier === 'HIGH');
  assert(!hasHighEnergyInWindDown, 'Wind-down window must never recommend HIGH energy stimulation');

  // Test Hobby Challenges
  const challenges = recommender.getHobbyChallenges();
  assert(challenges.length === 12, 'Must provide 12 beginner hobby challenges');
  console.log('✓ ActivityRecommender correctly pairs energy/mood and prevents evening over-stimulation.');

  console.log('\n--- [3/3] Testing SmartFreeTimePlanner ---');
  const planner = new SmartFreeTimePlanner(recommender);

  // 60-minute window
  const plan60 = planner.generatePlan(slot1, 'MEDIUM', 'Calm');
  assert(plan60.phases.length >= 3, '45m usable time should generate a multi-phase plan');
  assert(plan60.phases[0]!.category === 'rest', 'First phase should be decompression/rest');
  console.log(`Generated plan: "${plan60.title}" with ${plan60.phases.length} phases.`);
  console.log('✓ SmartFreeTimePlanner successfully generates non-stressful micro-break sequences.');

  console.log('\n========================================');
  console.log('All Balance Engine Unit Tests Passed Cleanly!');
  console.log('========================================');
}

runTests().catch((err) => {
  console.error('Test failed:', err);
  process.exit(1);
});
