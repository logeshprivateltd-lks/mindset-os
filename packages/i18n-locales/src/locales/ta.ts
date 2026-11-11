/**
 * MindSet OS — Tamil (ta) Translation Dictionary
 * Natural contemporary Tamil script (தமிழ்) for modern students
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { TranslationDictionary } from '../types';

export const ta: TranslationDictionary = {
  shell: {
    title: 'மைண்ட்செட் இயங்குதளம் — LKS PVT LTD',
    subtitle: 'அரோரா கோர் v0.1.0',
    ramUsage: 'நினைவக பயன்பாடு',
    kernelStatus: 'கர்னல் நிலை',
    locked: 'அமைப்பு பூட்டப்பட்டது',
    unlock: 'அமைப்பைத் திறக்கவும்',
    privacyBlurActive: 'தனியுரிமை பாதுகாப்பு திரையிடல் செயலில் உள்ளது (Alt + B)',
    activities: {
      deepStudy: 'ஆழ்ந்த படிப்பு & பாடப்புத்தகம்',
      codingProjects: 'நிரலாக்கம் & சிஸ்டம்ஸ்',
      planningReflection: 'திட்டமிடல் & சுய சிந்தனை',
      relaxation: 'மன அமைதி & ஓய்வு',
    },
  },
  taskbar: {
    start: 'தொடங்கு',
    searchPlaceholder: 'பாடத்திட்டம், செயலிகள், குறிப்புகளைத் தேடுங்கள்...',
    focusSession: 'கவன அமர்வு',
    memoryLabel: 'நினைவகம்',
    statusLabel: 'கர்னல் நிலை',
    lockScreen: 'திரைப் பூட்டு',
  },
  startMenu: {
    searchPrompt: 'பாடங்கள், செயலிகள், கோப்புகள் அல்லது அமைப்புகளைத் தேடுங்கள்...',
    pinnedApps: 'முக்கிய பணிநிலை செயலிகள்',
    workspaces: 'தற்போதைய செயல்பாட்டு தளம்',
    memoryInspector: 'AI நினைவக ஆய்வாளர்',
    rerunOnboarding: 'அமைப்பு வழிகாட்டியை மீண்டும் இயக்கு',
    diagnostics: 'கர்னல் கண்டறிதல் (/status)',
    lock: 'திரையைப் பூட்டு (Super + L)',
  },
  commandCenter: {
    title: 'கட்டளை மையம் & அளவீடுகள்',
    cpuTelemetry: 'செயலி இயக்கம்',
    ramTelemetry: 'நினைவக அளவு',
    activeTasks: 'முன்னுரிமை இலக்குகள்',
    todayFocus: 'இன்றைய முக்கிய படிப்பு',
    startFocusSession: 'ஆழ்ந்த கவன அமர்வு தொடங்கு',
  },
  focusGuard: {
    title: 'கவனப் பாதுகாப்பு அரண்',
    startDeepWork: '45-நிமிட கவன அமர்வு தொடங்கு',
    pause: 'இடைநிறுத்து',
    resume: 'மீண்டும் தொடர்',
    complete: 'அமர்வை முடி',
    interruptionCount: 'கவனச்சிதறல்கள்',
    strictMode: 'கடுமையான கவனக்கட்டுப்பாடு செயலில் உள்ளது',
  },
  balanceAi: {
    title: 'மைண்ட்செட் நல்வாழ்வு & சமநிலை',
    moodCheckin: 'மனநிலை & ஆற்றலைப் பதிவு செய்',
    energyCheckin: 'ஆற்றல் அளவு',
    journalEntry: 'நன்றியுணர்வு பகிர்வு',
    breakSuggested: 'ஓய்வெடுக்க பரிந்துரைக்கப்படுகிறது',
    takeBreak: '10 நிமிடம் சிறு ஓய்வெடுங்கள்',
  },
  onboarding: {
    welcome: 'மைண்ட்செட் இயங்குதளத்திற்கு நல்வரவு',
    tagline: 'தொழில்நுட்பம் உங்கள் இலக்குகளை அடைய உதவ வேண்டும், உங்கள் கவனத்தை சிதறடிக்கக் கூடாது.',
    stepIndicator: 'படி {{current}} / {{total}}',
    continueBtn: 'தொடரவும்',
    backBtn: 'பின்செல்க',
    launchBtn: 'கட்டளை மையத்தைத் தொடங்கு',
    themeSelectTitle: 'உங்கள் காட்சி சூழலைத் தேர்ந்தெடுக்கவும்',
    languageTitle: 'இரட்டை மொழி வடிவமைப்பு',
    languageSubtitle: 'மெனுக்களை ஆங்கிலத்திலும், AI வழிகாட்டலை இயல்பான தமிழிலும் வைத்துக் கொள்ளலாம்!',
    aiStyleTitle: 'AI பேசும் நடை',
    tanglishOption: 'உரையாடல் தங்கிலீஷ் (தமிழ் + ஆங்கிலம்)',
    pureScriptOption: 'தூய தமிழ் (தமிழ் எழுத்துரு)',
    englishOption: 'ஆங்கிலம்',
  },
  settings: {
    title: 'அமைப்பு விருப்பங்கள் & அமைப்புகள்',
    languageRegion: 'மொழி & உரையாடல் AI',
    osInterfaceLang: 'இயங்குதள இடைமுக மொழி',
    aiResponseLang: 'மைண்ட்செட் AI பதில் மொழி',
    aiConversationStyle: 'AI பேசும் நடை & பாணி',
    previewTitle: 'நேரடி ஒத்திசைவு முன்னோட்டம்',
    saveChanges: 'மொழி அமைப்புகளைச் சேமி',
    savedSuccess: 'மொழி மாற்றங்கள் உடனடியாக நடைமுறைக்கு வந்தன!',
  },
  auth: {
    unlockSystem: 'மைண்ட்செட் இயங்குதளத்தைத் திற',
    enterPin: '6-இலக்க பாதுகாப்பு பின்னை உள்ளிடவும்',
    enterPassword: 'முதன்மை கடவுச்சொல்லை உள்ளிடவும்',
    biometricPrompt: 'முகம் மற்றும் கைரேகை சரிபார்க்கப்படுகிறது...',
    privacyGuardTitle: 'தனியுரிமை மங்கலாக்கல் செயலில் உள்ளது',
    uacElevateTitle: 'நிர்வாக அனுமதி தேவைப்படுகிறது',
  },
};

