/**
 * MindSet OS — Telugu (te) Translation Dictionary
 * Natural contemporary Telugu script (తెలుగు)
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import type { TranslationDictionary } from '../types';

export const te: TranslationDictionary = {
  shell: {
    title: 'MindSet OS — LKS PVT LTD సమర్పణ',
    subtitle: 'అరోరా కోర్ v0.1.0',
    ramUsage: 'ర్యామ్ వినియోగం',
    kernelStatus: 'కెర్నల్ స్థితి',
    locked: 'సిస్టమ్ లాక్ చేయబడింది',
    unlock: 'సిస్టమ్‌ను అన్‌లాక్ చేయండి',
    privacyBlurActive: 'ప్రైవసీ బ్లర్ గార్డ్ యాక్టివ్‌గా ఉంది (Alt + B)',
    activities: {
      deepStudy: 'లోతైన అధ్యయనం & పుస్తకాలు',
      codingProjects: 'కోడింగ్ & సిస్టమ్స్',
      planningReflection: 'ప్రణాళిక & ఆలోచన',
      relaxation: 'మానసిక ప్రశాంతత & విశ్రాంతి',
    },
  },
  taskbar: {
    start: 'ప్రారంభం',
    searchPlaceholder: 'సిలబస్, యాప్‌లు, నోట్స్ శోధించండి...',
    focusSession: 'ఫోకస్ సెషన్',
    memoryLabel: 'మెమరీ',
    statusLabel: 'కెర్నల్ స్థితి',
    lockScreen: 'లాక్ స్క్రీన్',
  },
  startMenu: {
    searchPrompt: 'యాప్‌లు, ఫైల్స్ లేదా సెట్టింగ్‌లను శోధించండి...',
    pinnedApps: 'పిన్ చేసిన యాప్‌లు',
    workspaces: 'ప్రస్తుత కార్యాచరణ స్థలం',
    memoryInspector: 'AI మెమరీ ఇన్‌స్పెక్టర్',
    rerunOnboarding: 'సెటప్ విజార్డ్‌ను మళ్లీ ప్రారంభించండి',
    diagnostics: 'కెర్నల్ విశ్లేషణలు (/status)',
    lock: 'లాక్ స్క్రీన్ (Super + L)',
  },
  commandCenter: {
    title: 'కమాండ్ సెంటర్ & టెలిమెట్రీ',
    cpuTelemetry: 'సీపీయూ వినియోగం',
    ramTelemetry: 'ర్యామ్ పరిమాణం',
    activeTasks: 'ప్రాధాన్యతా లక్ష్యాలు',
    todayFocus: 'నేటి ప్రధాన అధ్యయనం',
    startFocusSession: 'లోతైన పనిని ప్రారంభించండి',
  },
  focusGuard: {
    title: 'ఫోకస్ గార్డ్ — ఏకాగ్రత రక్షణ',
    startDeepWork: '45-నిమిషాల లోతైన ఏకాగ్రత మొదలుపెట్టండి',
    pause: 'పాజ్ చేయండి',
    resume: 'తిరిగి ప్రారంభించండి',
    complete: 'సెషన్ పూర్తి చేయండి',
    interruptionCount: 'అంతరాయాలు',
    strictMode: 'స్ట్రిక్ట్ ఫోకస్ మోడ్ యాక్టివ్',
  },
  balanceAi: {
    title: 'MindSet సమతుల్యత & ఆరోగ్యం',
    moodCheckin: 'మూడ్ & శక్తిని నమోదు చేయండి',
    energyCheckin: 'శక్తి స్థాయి',
    journalEntry: 'కృతజ్ఞతా భావాలు',
    breakSuggested: 'విశ్రాంతి సిఫార్సు చేయబడింది',
    takeBreak: '10 నిమిషాలు విశ్రాంతి తీసుకోండి',
  },
  onboarding: {
    welcome: 'MindSet OS కి స్వాగతం',
    tagline: 'సాంకేతికత మీ లక్ష్యాలను సాధించడంలో సహాయపడాలి, మీ దృష్టిని మరల్చకూడదు.',
    stepIndicator: 'దశ {{current}} / {{total}}',
    continueBtn: 'కొనసాగించండి',
    backBtn: 'వెనుకకు',
    launchBtn: 'కమాండ్ సెంటర్‌ను ప్రారంభించండి',
    themeSelectTitle: 'మీకు నచ్చిన వాతావరణాన్ని ఎంచుకోండి',
    languageTitle: 'ద్విభాషా అమరిక',
    languageSubtitle: 'సిస్టమ్ మెనూలు ఇంగ్లీష్‌లో ఉంచి, AI మెంటార్‌తో తెలుగులో మాట్లాడండి!',
    aiStyleTitle: 'AI మాట్లాడే శైలి',
    tanglishOption: 'తెంగ్లీష్ / సంభాషణ శైలి',
    pureScriptOption: 'స్వచ్ఛమైన తెలుగు (తెలుగు లిపి)',
    englishOption: 'ఇంగ్లీష్',
  },
  settings: {
    title: 'సిస్టమ్ ప్రాధాన్యతలు & సెట్టింగ్‌లు',
    languageRegion: 'భాష & సంభాషణ AI',
    osInterfaceLang: 'ఆపరేటింగ్ సిస్టమ్ భాష',
    aiResponseLang: 'AI స్పందన భాష',
    aiConversationStyle: 'AI సంభాషణ శైలి',
    previewTitle: 'ప్రత్యక్ష మునుజూపు',
    saveChanges: 'మార్పులను భద్రపరచండి',
    savedSuccess: 'భాషా ప్రాధాన్యతలు విజయవంతంగా వర్తించబడ్డాయి!',
  },
  auth: {
    unlockSystem: 'MindSet OS అన్‌లాక్ చేయండి',
    enterPin: '6-అంకెల పిన్ నమోదు చేయండి',
    enterPassword: 'పాస్‌వర్డ్ నమోదు చేయండి',
    biometricPrompt: 'ముఖం మరియు వేలిముద్ర స్కాన్ అవుతోంది...',
    privacyGuardTitle: 'ప్రైవసీ గార్డ్ యాక్టివ్‌గా ఉంది',
    uacElevateTitle: 'నిర్వాహక అనుమతి అవసరం',
  },
};
