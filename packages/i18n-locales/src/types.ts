/**
 * MindSet OS — Dual-Layer Multilingual Types & Metadata
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export type SupportedLocale = 'en' | 'ta' | 'hi' | 'te' | 'ml' | 'kn' | 'bn' | 'mr';

export type ConversationStyle =
  | 'tanglish'
  | 'pure_script'
  | 'mixed'
  | 'formal'
  | 'concise'
  | 'socratic';

export interface LocaleMetadata {
  code: SupportedLocale;
  name: string;
  nativeName: string;
  script: string;
  region: string;
  sampleGreeting: string;
}

export const SUPPORTED_LOCALES: Record<SupportedLocale, LocaleMetadata> = {
  en: {
    code: 'en',
    name: 'English',
    nativeName: 'English',
    script: 'Latin',
    region: 'India / Global',
    sampleGreeting: 'Welcome back! Ready for deep work?',
  },
  ta: {
    code: 'ta',
    name: 'Tamil',
    nativeName: 'தமிழ்',
    script: 'Tamil',
    region: 'Tamil Nadu & Puducherry',
    sampleGreeting: 'வணக்கம்! இன்றைய கவன அமர்வை தொடங்கலாமா?',
  },
  hi: {
    code: 'hi',
    name: 'Hindi',
    nativeName: 'हिन्दी',
    script: 'Devanagari',
    region: 'India',
    sampleGreeting: 'नमस्ते! आज का गहन अध्ययन शुरू करें?',
  },
  te: {
    code: 'te',
    name: 'Telugu',
    nativeName: 'తెలుగు',
    script: 'Telugu',
    region: 'Andhra Pradesh & Telangana',
    sampleGreeting: 'నమస్కారం! ఈరోజు లోతైన అధ్యయనం మొదలుపెడదామా?',
  },
  ml: {
    code: 'ml',
    name: 'Malayalam',
    nativeName: 'മലയാളം',
    script: 'Malayalam',
    region: 'Kerala',
    sampleGreeting: 'നമസ്കാരം! ഇന്നത്തെ പഠനം ആരംഭിക്കാം?',
  },
  kn: {
    code: 'kn',
    name: 'Kannada',
    nativeName: 'ಕನ್ನಡ',
    script: 'Kannada',
    region: 'Karnataka',
    sampleGreeting: 'ನಮಸ್ಕಾರ! ಇಂದಿನ ಆಳವಾದ ಅಧ್ಯಯನ ಪ್ರಾರಂಭಿಸೋಣವೇ?',
  },
  bn: {
    code: 'bn',
    name: 'Bengali',
    nativeName: 'বাংলা',
    script: 'Bengali',
    region: 'West Bengal',
    sampleGreeting: 'নমস্কার! আজকের গভীর অধ্যয়ন শুরু করবেন?',
  },
  mr: {
    code: 'mr',
    name: 'Marathi',
    nativeName: 'मराठी',
    script: 'Devanagari',
    region: 'Maharashtra',
    sampleGreeting: 'नमस्कार! आजचा सखोल अभ्यास सुरू करायचा का?',
  },
};

export interface TranslationDictionary {
  shell: {
    title: string;
    subtitle: string;
    ramUsage: string;
    kernelStatus: string;
    locked: string;
    unlock: string;
    privacyBlurActive: string;
    activities: {
      deepStudy: string;
      codingProjects: string;
      planningReflection: string;
      relaxation: string;
    };
  };
  taskbar: {
    start: string;
    searchPlaceholder: string;
    focusSession: string;
    memoryLabel: string;
    statusLabel: string;
    lockScreen: string;
  };
  startMenu: {
    searchPrompt: string;
    pinnedApps: string;
    workspaces: string;
    memoryInspector: string;
    rerunOnboarding: string;
    diagnostics: string;
    lock: string;
  };
  commandCenter: {
    title: string;
    cpuTelemetry: string;
    ramTelemetry: string;
    activeTasks: string;
    todayFocus: string;
    startFocusSession: string;
  };
  focusGuard: {
    title: string;
    startDeepWork: string;
    pause: string;
    resume: string;
    complete: string;
    interruptionCount: string;
    strictMode: string;
  };
  balanceAi: {
    title: string;
    moodCheckin: string;
    energyCheckin: string;
    journalEntry: string;
    breakSuggested: string;
    takeBreak: string;
  };
  onboarding: {
    welcome: string;
    tagline: string;
    stepIndicator: string;
    continueBtn: string;
    backBtn: string;
    launchBtn: string;
    themeSelectTitle: string;
    languageTitle: string;
    languageSubtitle: string;
    aiStyleTitle: string;
    tanglishOption: string;
    pureScriptOption: string;
    englishOption: string;
  };
  settings: {
    title: string;
    languageRegion: string;
    osInterfaceLang: string;
    aiResponseLang: string;
    aiConversationStyle: string;
    previewTitle: string;
    saveChanges: string;
    savedSuccess: string;
  };
  auth: {
    unlockSystem: string;
    enterPin: string;
    enterPassword: string;
    biometricPrompt: string;
    privacyGuardTitle: string;
    uacElevateTitle: string;
  };
}

