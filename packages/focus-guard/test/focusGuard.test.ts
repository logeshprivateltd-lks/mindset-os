/**
 * MindSet OS Focus Guard Unit Test Suite
 * Validates Focus Session Lifecycle, Distraction Journaling, and XP Calculations
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import {
  FocusSessionManager,
  DistractionJournal,
  NotificationGuardManager,
  XpEngine,
} from '../src';

console.log('🧪 Starting Focus Guard Test Suite...');

// --- 1. Focus Session Manager Tests ---
console.log('\n--- 1. Testing FocusSessionManager ---');
const sessionMgr = new FocusSessionManager();

// Test start
const session = sessionMgr.startSession('Study Physics Chapter 4', 25 * 60 * 1000);
if (!session.id || session.status !== 'active') {
  throw new Error('FocusSessionManager failed to initialize active session');
}
if (session.taskTitle !== 'Study Physics Chapter 4') {
  throw new Error('Task title was not preserved');
}
console.log('  ✔ Start session verified');

// Test pause and resume
const paused = sessionMgr.pauseSession(session);
if (paused.status !== 'paused' || paused.pauseCount !== 1) {
  throw new Error('FocusSessionManager failed to pause session correctly');
}
console.log('  ✔ Pause session verified');

const resumed = sessionMgr.resumeSession(paused);
if (resumed.status !== 'active') {
  throw new Error('FocusSessionManager failed to resume session');
}
console.log('  ✔ Resume session verified');

// Test end session and XP reward calculation
const ended = sessionMgr.endSession(resumed, false, Date.now() + 25 * 60 * 1000);
if (ended.status !== 'completed' || ended.xpEarned === undefined || ended.xpEarned < 25) {
  throw new Error(`FocusSessionManager unexpected XP calculation: ${ended.xpEarned}`);
}
console.log(`  ✔ End session verified (Earned ${ended.xpEarned} XP)`);

// Test progress calculation
const progress = sessionMgr.calculateProgress(session, session.startTime + 10 * 60 * 1000);
if (progress.percentComplete < 39 || progress.percentComplete > 41) {
  throw new Error(`Unexpected progress calculation: ${progress.percentComplete}%`);
}
console.log(`  ✔ Progress calculation verified (${progress.percentComplete}%)`);

// --- 2. Distraction Journal Tests ---
console.log('\n--- 2. Testing DistractionJournal ---');
const journal = new DistractionJournal();

const entry1 = journal.log(session.id, 'Discord', true);
if (!entry1.id || entry1.appName !== 'Discord' || !entry1.wasBlocked) {
  throw new Error('DistractionJournal failed to log distraction entry');
}

const entry2 = journal.log(session.id, 'Browser Tab', false);
journal.recordReturn(entry2.id);

const sessionLogs = journal.getForSession(session.id);
if (sessionLogs.length !== 2) {
  throw new Error(`Expected 2 distraction entries, found ${sessionLogs.length}`);
}
console.log('  ✔ Distraction logging verified (2 entries)');

const stats = journal.getStats(session.id);
if (stats.totalDistractions !== 2 || stats.blockedCount !== 1) {
  throw new Error('Distraction stats aggregation mismatch');
}
console.log('  ✔ Distraction stats aggregation verified');

// --- 3. Notification Guard Manager Tests ---
console.log('\n--- 3. Testing NotificationGuardManager ---');
const guardMgr = new NotificationGuardManager();

guardMgr.addRule({
  appId: 'com.discord',
  allow: false,
  alwaysAllowEmergency: true,
});

if (guardMgr.getRules().length !== 1) {
  throw new Error('Failed to register notification guard rule');
}
console.log('  ✔ Notification rule registration verified');

guardMgr.suppressAll(true);
if (!guardMgr.isSuppressionActive()) {
  throw new Error('Suppression status should be active');
}
console.log('  ✔ System-wide notification suppression verified');

const shouldAllowDiscord = guardMgr.shouldAllow('com.discord', false);
if (shouldAllowDiscord) {
  throw new Error('Discord notification should be suppressed during active focus');
}
console.log('  ✔ Target notification suppression verified');

const shouldAllowEmergency = guardMgr.shouldAllow('com.discord', true);
if (!shouldAllowEmergency) {
  throw new Error('Emergency contacts must always bypass suppression');
}
console.log('  ✔ Emergency contact bypass safeguard verified');

guardMgr.restoreAll();
if (guardMgr.isSuppressionActive()) {
  throw new Error('Suppression should be false after restoreAll');
}
console.log('  ✔ Notification restoration verified');

// --- 4. XP Engine Tests ---
console.log('\n--- 4. Testing XpEngine ---');
const level1 = XpEngine.getLevel(50);
if (level1.level !== 1) {
  throw new Error('Expected Level 1 for 50 XP');
}
console.log('  ✔ Level calculation verified (50 XP -> Level 1)');

const level3 = XpEngine.getLevel(350);
if (level3.level !== 3) {
  throw new Error(`Expected Level 3 for 350 XP, got ${level3.level}`);
}
console.log(`  ✔ Level progression verified (350 XP -> Level ${level3.level})`);

const progressInfo = XpEngine.getProgress(350);
if (progressInfo.currentLevel !== 3 || progressInfo.xpIntoLevel !== 50) {
  throw new Error('XP progress breakdown mismatch');
}
console.log('  ✔ XP progress breakdown verified');

const awardResult = XpEngine.awardXp(280, 50);
if (awardResult.newTotalXp !== 330 || !awardResult.leveledUp) {
  throw new Error('XP award level-up detection failed');
}
console.log('  ✔ XP award and Level-Up trigger verified (280 + 50 -> Level Up!)');

console.log('\n========================================');
console.log('🎉 ALL FOCUS GUARD UNIT TESTS PASSED CLEANLY!');
console.log('========================================\n');
