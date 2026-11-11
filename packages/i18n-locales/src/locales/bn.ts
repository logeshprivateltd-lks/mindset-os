/**
 * MindSet OS — Bengali (bn) Translation Dictionary
 * Natural contemporary Bengali script (বাংলা)
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { TranslationDictionary } from '../types';

export const bn: TranslationDictionary = {
  shell: {
    title: 'MindSet OS — LKS PVT LTD পরিবেশিত',
    subtitle: 'অরোরা কোর v0.1.0',
    ramUsage: 'র‍্যাম ব্যবহার',
    kernelStatus: 'কার্নেল স্থিতি',
    locked: 'সিস্টেম লক করা হয়েছে',
    unlock: 'সিস্টেম আনলক করুন',
    privacyBlurActive: 'গোপনীয়তা ব্লার গার্ড সক্রিয় (Alt + B)',
    activities: {
      deepStudy: 'গভীর অধ্যয়ন ও পাঠ্যবই',
      codingProjects: 'কোডিং ও সিস্টেম',
      planningReflection: 'পরিকল্পনা ও প্রতিফলন',
      relaxation: 'মানসিক প্রশান্তি ও বিশ্রাম',
    },
  },
  taskbar: {
    start: 'শুরু',
    searchPlaceholder: 'সিলেবাস, অ্যাপস, নোটস খুঁজুন...',
    focusSession: 'ফোকাস সেশন',
    memoryLabel: 'মেমরি',
    statusLabel: 'কার্নেল স্থিতি',
    lockScreen: 'স্ক্রিন লক',
  },
  startMenu: {
    searchPrompt: 'অ্যাপস, ফাইল বা সেটিংস অনুসন্ধান করুন...',
    pinnedApps: 'পিন করা অ্যাপ্লিকেশন',
    workspaces: 'বর্তমান কাজের ক্ষেত্র',
    memoryInspector: 'AI মেমরি ইন্সপেক্টর',
    rerunOnboarding: 'সেটআপ উইজার্ড পুনরায় চালান',
    diagnostics: 'কার্নেল ডায়াগনস্টিকস (/status)',
    lock: 'স্ক্রিন লক করুন (Super + L)',
  },
  commandCenter: {
    title: 'কমান্ড সেন্টার ও টেলিমেট্রি',
    cpuTelemetry: 'সিপিইউ কার্যক্রম',
    ramTelemetry: 'র‍্যাম ব্যবহার',
    activeTasks: 'অগ্রাধিকারমূলক লক্ষ্য',
    todayFocus: 'আজকের প্রধান অধ্যয়ন',
    startFocusSession: 'গভীর অধ্যয়ন শুরু করুন',
  },
  focusGuard: {
    title: 'ফোকাস গার্ড — মনোযোগ ঢাল',
    startDeepWork: '৪৫-মিনিটের গভীর ফোকাস শুরু করুন',
    pause: 'বিরতি দিন',
    resume: 'পুনরায় শুরু করুন',
    complete: 'সেশন সম্পন্ন করুন',
    interruptionCount: 'বিক্ষিপ্ততা',
    strictMode: 'কঠোর ফোকাস মোড সক্রিয়',
  },
  balanceAi: {
    title: 'MindSet ভারসাম্য ও সুস্থতা',
    moodCheckin: 'মনোভাব ও শক্তি নথিভুক্ত করুন',
    energyCheckin: 'শক্তি স্তর',
    journalEntry: 'কৃতজ্ঞতার জার্নাল',
    breakSuggested: 'বিশ্রাম সুপারিশ করা হয়েছে',
    takeBreak: '১০ মিনিটের বিরতি নিন',
  },
  onboarding: {
    welcome: 'MindSet OS-এ স্বাগতম',
    tagline: 'প্রযুক্তি আপনার লক্ষ্য অর্জনে সাহায্য করবে, আপনার মনোযোগ বিভ্রান্ত করবে না।',
    stepIndicator: 'ধাপ {{current}} / {{total}}',
    continueBtn: 'এগিয়ে যান',
    backBtn: 'পেছনে',
    launchBtn: 'কমান্ড সেন্টার চালু করুন',
    themeSelectTitle: 'আপনার পছন্দের পরিবেশ বেছে নিন',
    languageTitle: 'দ্বিভাষিক কনফিগারেশন',
    languageSubtitle: 'মেনু ইংরেজিতে রেখে AI পরামর্শ বাংলায় গ্রহণ করুন!',
    aiStyleTitle: 'AI কথনশৈলী',
    tanglishOption: 'বাংলিশ / কথোপকথনের ভাষা',
    pureScriptOption: 'বিশুদ্ধ বাংলা (বাংলা লিপি)',
    englishOption: 'ইংরেজি',
  },
  settings: {
    title: 'সিস্টেম পছন্দ ও সেটিংস',
    languageRegion: 'ভাষা ও কথোপকথন AI',
    osInterfaceLang: 'অপারেটিং সিস্টেম ভাষা',
    aiResponseLang: 'AI প্রতিক্রিয়া ভাষা',
    aiConversationStyle: 'AI কথনশৈলী',
    previewTitle: 'লাইভ পূর্বরূপ',
    saveChanges: 'পরিবর্তন সংরক্ষণ করুন',
    savedSuccess: 'ভাষা পছন্দ অবিলম্বে প্রয়োগ করা হয়েছে!',
  },
  auth: {
    unlockSystem: 'MindSet OS আনলক করুন',
    enterPin: '৬-সংখ্যার পিন লিখুন',
    enterPassword: 'মাস্টার পাসওয়ার্ড লিখুন',
    biometricPrompt: 'মুখ এবং আঙুলের ছাপ স্ক্যান করা হচ্ছে...',
    privacyGuardTitle: 'গোপনীয়তা ব্লার গার্ড সক্রিয়',
    uacElevateTitle: 'প্রশাসক অনুমোদন প্রয়োজন',
  },
};

