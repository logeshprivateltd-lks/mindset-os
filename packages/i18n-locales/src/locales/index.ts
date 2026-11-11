/**
 * MindSet OS — Locales Resource Registry
 * All 8 Indic & English Contemporary Translation Resources
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import { en } from './en';
import { ta } from './ta';
import { hi } from './hi';
import { te } from './te';
import { ml } from './ml';
import { kn } from './kn';
import { bn } from './bn';
import { mr } from './mr';
import type { SupportedLocale, TranslationDictionary } from '../types';

export const LOCALES: Record<SupportedLocale, TranslationDictionary> = {
  en,
  ta,
  hi,
  te,
  ml,
  kn,
  bn,
  mr,
};

export const I18N_RESOURCES = {
  en: { translation: en },
  ta: { translation: ta },
  hi: { translation: hi },
  te: { translation: te },
  ml: { translation: ml },
  kn: { translation: kn },
  bn: { translation: bn },
  mr: { translation: mr },
};

export { en, ta, hi, te, ml, kn, bn, mr };
