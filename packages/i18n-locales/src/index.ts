/**
 * MindSet OS — Dual-Layer Multilingual Localization Core
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export type SupportedLocale = 'en' | 'ta' | 'hi' | 'te' | 'ml' | 'kn' | 'bn' | 'mr' | 'tanglish';

export interface LocaleInfo {
  code: SupportedLocale;
  name: string;
  nativeName: string;
  script: string;
}

export const LOCALE_METADATA: Record<SupportedLocale, LocaleInfo> = {
  en: { code: 'en', name: 'English', nativeName: 'English', script: 'Latin' },
  ta: { code: 'ta', name: 'Tamil', nativeName: 'தமிழ்', script: 'Tamil' },
  tanglish: { code: 'tanglish', name: 'Tanglish', nativeName: 'Tanglish (தமிழ் + Eng)', script: 'Latin' },
  hi: { code: 'hi', name: 'Hindi', nativeName: 'हिन्दी', script: 'Devanagari' },
  te: { code: 'te', name: 'Telugu', nativeName: 'తెలుగు', script: 'Telugu' },
  ml: { code: 'ml', name: 'Malayalam', nativeName: 'മലയാളം', script: 'Malayalam' },
  kn: { code: 'kn', name: 'Kannada', nativeName: 'ಕನ್ನಡ', script: 'Kannada' },
  bn: { code: 'bn', name: 'Bengali', nativeName: 'বাংলা', script: 'Bengali' },
  mr: { code: 'mr', name: 'Marathi', nativeName: 'मराठी', script: 'Devanagari' },
};

export const COMMON_TRANSLATIONS: Record<string, Record<string, string>> = {
  en: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'Aurora Focus Lock',
    'lock.unlock': 'Unlock System',
    'lock.biometric': 'Scan Face / Fingerprint',
    'kernel.status': 'Kernel Status',
    'focus.start': 'Start Deep Work',
    'onboarding.welcome': 'Welcome to MindSet OS',
    'onboarding.tagline': 'Your technology should help you achieve your goals, not compete for your attention.',
    'onboarding.next': 'Continue',
    'onboarding.back': 'Back',
    'onboarding.launch': 'Launch MindSet Command Center',
    'ai.sample_greeting': 'Hello! I am your MindSet AI mentor. Ready to master your syllabus with zero distractions?',
  },
  tanglish: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'Aurora Focus Lock',
    'lock.unlock': 'System Unlock Pannunga',
    'lock.biometric': 'Face illa Fingerprint Scan Pannunga',
    'kernel.status': 'Kernel Nilavaram',
    'focus.start': 'Deep Work Start Pannunga',
    'onboarding.welcome': 'MindSet OS-ku Ungalai Varaverkirom',
    'onboarding.tagline': 'Ungaloda technology goals achieve panna udhavanum, attention-ah distract panna koodadhu.',
    'onboarding.next': 'Munnadi Ponga',
    'onboarding.back': 'Pinnadi',
    'onboarding.launch': 'MindSet Command Center Start Pannunga',
    'ai.sample_greeting': 'Vanakkam! Naan unga MindSet AI mentor. Innikku daily target mudikkalaama?',
  },
  ta: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'அரோரா ஃபோகஸ் பூட்டு',
    'lock.unlock': 'அமைப்பைத் திறக்கவும்',
    'lock.biometric': 'முகம் / கைரேகை சரிபார்ப்பு',
    'kernel.status': 'கர்னல் நிலை',
    'focus.start': 'ஆழ்ந்த கவன அமர்வு தொடங்கு',
    'onboarding.welcome': 'மைண்ட்செட் இயங்குதளத்திற்கு நல்வரவு',
    'onboarding.tagline': 'தொழில்நுட்பம் உங்கள் இலக்குகளை அடைய உதவ வேண்டும், உங்கள் கவனத்தை சிதறடிக்கக் கூடாது.',
    'onboarding.next': 'தொடரவும்',
    'onboarding.back': 'பின்செல்க',
    'onboarding.launch': 'கட்டளை மையத்தைத் தொடங்கு',
    'ai.sample_greeting': 'வணக்கம்! நான் உங்கள் மைண்ட்செட் வழிகாட்டி. இன்றைய பாடத்திட்டத்தைத் தொடங்கலாமா?',
  },
  hi: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'अरोरा फोकस लॉक',
    'lock.unlock': 'सिस्टम अनलॉक करें',
    'lock.biometric': 'चेहरा / फिंगरप्रिंट स्कैन करें',
    'kernel.status': 'कर्नेल स्थिति',
    'focus.start': 'गहन अध्ययन शुरू करें',
    'onboarding.welcome': 'MindSet OS में आपका स्वागत है',
    'onboarding.tagline': 'आपकी तकनीक आपके लक्ष्यों को हासिल करने में मदद करे, आपका ध्यान न भटकाए।',
    'onboarding.next': 'आगे बढ़ें',
    'onboarding.back': 'पीछे जाएं',
    'onboarding.launch': 'कमांड सेंटर शुरू करें',
    'ai.sample_greeting': 'नमस्ते! मैं आपका MindSet AI मेंटर हूँ। आज का अध्ययन लक्ष्य शुरू करें?',
  },
  te: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'అరోరా ఫోకస్ లాక్',
    'lock.unlock': 'సిస్టమ్‌ను అన్‌లాక్ చేయండి',
    'lock.biometric': 'ఫేస్ / ఫింగర్‌ప్రింట్ స్కాన్ చేయండి',
    'kernel.status': 'కెర్నల్ స్థితి',
    'focus.start': 'లోతైన పనిని ప్రారంభించండి',
    'onboarding.welcome': 'MindSet OS కి స్వాగతం',
    'onboarding.tagline': 'సాంకేతికత మీ లక్ష్యాలను సాధించడంలో సహాయపడాలి.',
    'onboarding.next': 'కొనసాగించండి',
    'onboarding.back': 'వెనుకకు',
    'onboarding.launch': 'కమాండ్ సెంటర్‌ను ప్రారంభించండి',
    'ai.sample_greeting': 'నమస్కారం! నేను మీ MindSet AI మార్గదర్శిని.',
  },
  ml: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'അറോറ ഫോക്കസ് ലോക്ക്',
    'lock.unlock': 'സിസ്റ്റം അൺലോക്ക് ചെയ്യുക',
    'lock.biometric': 'ഫെയ്സ് / വിരലടയാളം സ്കാൻ ചെയ്യുക',
    'kernel.status': 'കേർണൽ നില',
    'focus.start': 'പഠനം ആരംഭിക്കുക',
    'onboarding.welcome': 'MindSet OS-ലേക്ക് സ്വാഗതം',
    'onboarding.tagline': 'സാങ്കേതികവിദ്യ നിങ്ങളുടെ ലക്ഷ്യങ്ങൾ നേടാൻ സഹായിക്കണം.',
    'onboarding.next': 'തുടരുക',
    'onboarding.back': 'തിരികെ',
    'onboarding.launch': 'കമാൻഡ് സെന്റർ ആരംഭിക്കുക',
    'ai.sample_greeting': 'നമസ്കാരം! ഞാൻ നിങ്ങളുടെ MindSet AI ഉപദേഷ്ടാവാണ്.',
  },
  kn: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'ಅರೋರಾ ಫೋಕಸ್ ಲಾಕ್',
    'lock.unlock': 'ಸಿಸ್ಟಮ್ ಅನ್‌ಲಾಕ್ ಮಾಡಿ',
    'lock.biometric': 'ಮುಖ / ಫಿಂಗರ್‌ಪ್ರಿಂಟ್ ಸ್ಕ್ಯಾನ್ ಮಾಡಿ',
    'kernel.status': 'ಕರ್ನಲ್ ಸ್ಥಿತಿ',
    'focus.start': 'ಆಳವಾದ ಅಧ್ಯಯನ ಪ್ರಾರಂಭಿಸಿ',
    'onboarding.welcome': 'MindSet OS ಗೆ ಸುಸ್ವಾಗತ',
    'onboarding.tagline': 'ತಂತ್ರಜ್ಞಾನವು ನಿಮ್ಮ ಗುರಿಗಳನ್ನು ಸಾಧಿಸಲು ಸಹಾಯ ಮಾಡಬೇಕು.',
    'onboarding.next': 'ಮುಂದುವರಿಯಿರಿ',
    'onboarding.back': 'ಹಿಂದೆ',
    'onboarding.launch': 'ಕಮಾಂಡ್ ಸೆಂಟರ್ ಪ್ರಾರಂಭಿಸಿ',
    'ai.sample_greeting': 'ನಮಸ್ಕಾರ! ನಾನು ನಿಮ್ಮ MindSet AI ಮಾರ್ಗದರ್ಶಕ.',
  },
  bn: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'অরোরা ফোকাস লক',
    'lock.unlock': 'সিস্টেম আনলক করুন',
    'lock.biometric': 'ফেস / ফিঙ্গারপ্রিন্ট স্ক্যান করুন',
    'kernel.status': 'কার্নেল স্থিতি',
    'focus.start': 'গভীর অধ্যয়ন শুরু করুন',
    'onboarding.welcome': 'MindSet OS-এ স্বাগতম',
    'onboarding.tagline': 'প্রযুক্তি আপনার লক্ষ্য অর্জনে সাহায্য করবে।',
    'onboarding.next': 'এগিয়ে যান',
    'onboarding.back': 'পেছনে',
    'onboarding.launch': 'কমান্ড সেন্টার চালু করুন',
    'ai.sample_greeting': 'নমস্কার! আমি আপনার MindSet AI পরামর্শদাতা।',
  },
  mr: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'अरोरा फोकस लॉक',
    'lock.unlock': 'सिस्टम अनलॉक करा',
    'lock.biometric': 'चेहरा / फिंगरप्रिंट स्कॅन करा',
    'kernel.status': 'कर्नेल स्थिती',
    'focus.start': 'सखोल अभ्यास सुरू करा',
    'onboarding.welcome': 'MindSet OS मध्ये आपले स्वागत आहे',
    'onboarding.tagline': 'तंत्रज्ञान तुमचे ध्येय साध्य करण्यास मदत करणारे असावे.',
    'onboarding.next': 'पुढे जा',
    'onboarding.back': 'मागे',
    'onboarding.launch': 'कमांड सेंटर सुरू करा',
    'ai.sample_greeting': 'नमस्कार! मी आपला MindSet AI मार्गदर्शक आहे.',
  },
};

export function t(locale: SupportedLocale | string, key: string, fallback?: string): string {
  const langDict = COMMON_TRANSLATIONS[locale] || COMMON_TRANSLATIONS['en'];
  if (langDict && langDict[key]) {
    return langDict[key];
  }
  const enDict = COMMON_TRANSLATIONS['en'];
  if (enDict && enDict[key]) {
    return enDict[key];
  }
  return fallback || key;
}
