/**
 * MindSet OS — i18next Core Initialization Engine
 * Zero-reload instant language switching (<16ms) with fallback to English
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import { I18N_RESOURCES } from './locales';
import type { SupportedLocale } from './types';

const STORAGE_KEY = 'mindset_interface_language';

export function getInitialLanguage(): SupportedLocale {
  if (typeof window !== 'undefined') {
    const saved = localStorage.getItem(STORAGE_KEY) as SupportedLocale | null;
    if (saved && saved in I18N_RESOURCES) {
      return saved;
    }
  }
  return 'en';
}

if (!i18n.isInitialized) {
  i18n.use(initReactI18next).init({
    resources: I18N_RESOURCES,
    lng: getInitialLanguage(),
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false, // React already safeguards against XSS
    },
    react: {
      useSuspense: false,
    },
  });
}

/**
 * Changes the OS interface language immediately without page reload (<16ms)
 */
export async function setAppLanguage(locale: SupportedLocale): Promise<void> {
  if (locale in I18N_RESOURCES) {
    await i18n.changeLanguage(locale);
    if (typeof window !== 'undefined') {
      localStorage.setItem(STORAGE_KEY, locale);
      document.documentElement.setAttribute('lang', locale);
    }
  }
}

/**
 * Gets the current active interface language
 */
export function getCurrentLanguage(): SupportedLocale {
  return (i18n.language as SupportedLocale) || 'en';
}

export default i18n;
export { i18n };
export { useTranslation, Trans, I18nextProvider } from 'react-i18next';
