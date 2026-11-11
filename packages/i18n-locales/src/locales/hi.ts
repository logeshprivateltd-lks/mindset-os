/**
 * MindSet OS — Hindi (hi) Translation Dictionary
 * Natural contemporary Hindi script (हिन्दी)
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { TranslationDictionary } from '../types';

export const hi: TranslationDictionary = {
  shell: {
    title: 'MindSet OS — LKS PVT LTD द्वारा संचालित',
    subtitle: 'अरोरा कोर v0.1.0',
    ramUsage: 'रैम उपयोग',
    kernelStatus: 'कर्नेल स्थिति',
    locked: 'सिस्टम लॉक है',
    unlock: 'सिस्टम अनलॉक करें',
    privacyBlurActive: 'प्राइवेसी ब्लर गार्ड सक्रिय है (Alt + B)',
    activities: {
      deepStudy: 'गहन अध्ययन और पाठ्यपुस्तक',
      codingProjects: 'सिस्टम्स और कोडिंग',
      planningReflection: 'योजना और चिंतन',
      relaxation: 'मानसिक शांति और आराम',
    },
  },
  taskbar: {
    start: 'प्रारंभ',
    searchPlaceholder: 'पाठ्यक्रम, ऐप्स, नोट्स खोजें...',
    focusSession: 'फोकस सत्र',
    memoryLabel: 'मेमोरी',
    statusLabel: 'कर्नेल स्थिति',
    lockScreen: 'स्क्रीन लॉक',
  },
  startMenu: {
    searchPrompt: 'पाठ्यक्रम, ऐप्स, फाइलें या सेटिंग्स खोजें...',
    pinnedApps: 'पिन किए गए ऐप्स',
    workspaces: 'वर्तमान कार्यक्षेत्र',
    memoryInspector: 'AI मेमोरी निरीक्षक',
    rerunOnboarding: 'सेटअप विज़ार्ड दोबारा चलाएं',
    diagnostics: 'कर्नेल डायग्नोस्टिक्स (/status)',
    lock: 'स्क्रीन लॉक करें (Super + L)',
  },
  commandCenter: {
    title: 'कमांड सेंटर और टेलीमेट्री',
    cpuTelemetry: 'सीपीयू गतिविधि',
    ramTelemetry: 'रैम उपयोग',
    activeTasks: 'प्राथमिकता वाले लक्ष्य',
    todayFocus: 'आज का मुख्य अध्ययन',
    startFocusSession: 'गहन अध्ययन शुरू करें',
  },
  focusGuard: {
    title: 'फोकस गार्ड — ध्यान ढाल',
    startDeepWork: '45-मिनट का गहरा फोकस शुरू करें',
    pause: 'रोकें',
    resume: 'फिर से शुरू करें',
    complete: 'सत्र पूरा करें',
    interruptionCount: 'रुकावटें',
    strictMode: 'सख्त फोकस मोड सक्रिय है',
  },
  balanceAi: {
    title: 'MindSet संतुलन और खुशहाली',
    moodCheckin: 'मूड और ऊर्जा दर्ज करें',
    energyCheckin: 'ऊर्जा स्तर',
    journalEntry: 'आभार डायरी',
    breakSuggested: 'विश्राम की सलाह',
    takeBreak: '10 मिनट का ब्रेक लें',
  },
  onboarding: {
    welcome: 'MindSet OS में आपका स्वागत है',
    tagline: 'आपकी तकनीक आपके लक्ष्यों को हासिल करने में मदद करे, आपका ध्यान न भटकाए।',
    stepIndicator: 'चरण {{current}} / {{total}}',
    continueBtn: 'आगे बढ़ें',
    backBtn: 'पीछे',
    launchBtn: 'कमांड सेंटर शुरू करें',
    themeSelectTitle: 'अपना पसंदीदा माहौल चुनें',
    languageTitle: 'दोहरी भाषा व्यवस्था',
    languageSubtitle: 'ओएस मेनू अंग्रेजी में रखें और अपने एआई मेंटर से अपनी भाषा में बात करें!',
    aiStyleTitle: 'एआई मेंटर की बातचीत शैली',
    tanglishOption: 'हिंग्लिश / बोलचाल की भाषा',
    pureScriptOption: 'शुद्ध हिन्दी (देवनागरी)',
    englishOption: 'अंग्रेजी (English)',
  },
  settings: {
    title: 'सिस्टम प्राथमिकताएं और सेटिंग्स',
    languageRegion: 'भाषा और एआई मेंटर',
    osInterfaceLang: 'ऑपरेटिंग सिस्टम भाषा',
    aiResponseLang: 'एआई मेंटर भाषा',
    aiConversationStyle: 'एआई बातचीत और कोड-स्विचिंग शैली',
    previewTitle: 'लाइव पूर्वावलोकन',
    saveChanges: 'सेटिंग्स सहेजें',
    savedSuccess: 'भाषा प्राथमिकताएं तुरंत लागू हो गईं!',
  },
  auth: {
    unlockSystem: 'MindSet OS अनलॉक करें',
    enterPin: '6-अंकीय सुरक्षा पिन दर्ज करें',
    enterPassword: 'मास्टर पासवर्ड दर्ज करें',
    biometricPrompt: 'चेहरा और फिंगरप्रिंट स्कैन हो रहा है...',
    privacyGuardTitle: 'प्राइवेसी गार्ड सक्रिय है',
    uacElevateTitle: 'प्रशासक अनुमति आवश्यक है',
  },
};
