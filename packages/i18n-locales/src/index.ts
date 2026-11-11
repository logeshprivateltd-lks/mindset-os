export type SupportedLocale = 'en' | 'ta' | 'hi' | 'te' | 'ml' | 'kn' | 'bn' | 'mr' | 'tanglish';

export const LOCALE_METADATA: Record<SupportedLocale, { name: string; nativeName: string }> = {
  en: { name: 'English', nativeName: 'English' },
  ta: { name: 'Tamil', nativeName: 'தமிழ்' },
  hi: { name: 'Hindi', nativeName: 'हिन्दी' },
  te: { name: 'Telugu', nativeName: 'తెలుగు' },
  ml: { name: 'Malayalam', nativeName: 'മലയാളം' },
  kn: { name: 'Kannada', nativeName: 'ಕನ್ನಡ' },
  bn: { name: 'Bengali', nativeName: 'বাংলা' },
  mr: { name: 'Marathi', nativeName: 'मराठी' },
  tanglish: { name: 'Tanglish', nativeName: 'Tanglish (Tamil + English)' },
};

export const COMMON_TRANSLATIONS: Record<string, Record<string, string>> = {
  en: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'Aurora Focus Lock',
    'lock.unlock': 'Unlock System',
    'lock.biometric': 'Scan Face / Fingerprint',
    'kernel.status': 'Kernel Status',
    'focus.start': 'Start Deep Work',
  },
  tanglish: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'Aurora Focus Lock',
    'lock.unlock': 'System Unlock Pannunga',
    'lock.biometric': 'Face illa Fingerprint Scan Pannunga',
    'kernel.status': 'Kernel Nilavaram',
    'focus.start': 'Deep Work Start Pannunga',
  },
  ta: {
    'app.title': 'MindSet OS — Powered by LKS PVT LTD',
    'lock.title': 'அரோரா ஃபோகஸ் பூட்டு',
    'lock.unlock': 'அமைப்பைத் திறக்கவும்',
    'lock.biometric': 'முகம் / கைரேகை சரிபார்ப்பு',
    'kernel.status': 'கர்னல் நிலை',
    'focus.start': 'ஆழ்ந்த கவன அமர்வு தொடங்கு',
  }
};

