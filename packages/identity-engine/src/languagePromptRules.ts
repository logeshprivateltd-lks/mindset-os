/**
 * MindSet OS — AI Language & Tanglish Style Formatter
 * Generates explicit linguistic directives for PersonalContextBuilder
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

export interface LanguagePromptOptions {
  aiLanguage: string;
  conversationStyle: string;
  learningStyle?: string;
  aiTone?: string;
}

export interface SampleAiPreview {
  query: string;
  response: string;
  languageLabel: string;
  styleLabel: string;
}

/**
 * Builds explicit linguistic instruction blocks for the LLM system prompt
 */
export function buildLanguagePromptRules(options: LanguagePromptOptions): string {
  const { aiLanguage, conversationStyle } = options;

  // 1. TAMIL (ta)
  if (aiLanguage === 'ta') {
    if (conversationStyle === 'tanglish') {
      return `CONVERSATION STYLE: TANGLISH (Natural Tamil + English Code-Switching)
- Converse like a warm, smart Tamil peer/mentor using natural Tanglish (conversational Tamil in Latin script + English terms).
- Keep scientific, mathematical, and technical terms in standard English (e.g., "Electrostatics", "Derivation", "Integration").
- Use authentic Tamil conversational markers naturally (e.g., "Bro", "kandippa", "namma paarkalaam", "adhukku munnadi", "marakkadheenga", "romba easy thaan").
- Example: "Bro, innaiku evening 6 PM-ku Physics Electrostatics revise pannalam, adhukku munnadi 30 mins walk poitu vaanga."
- Avoid archaic textbook Tamil; sound natural, encouraging, and empathetic.`;
    }

    if (conversationStyle === 'mixed') {
      return `CONVERSATION STYLE: BILINGUAL MIXED (TAMIL + ENGLISH)
- Freely code-switch between English and Tamil script (தமிழ்) where concepts are best understood.
- Retain core technical definitions and formulas in English.`;
    }

    // Pure Tamil
    return `CONVERSATION STYLE: PURE TAMIL (தமிழ்)
- Respond in natural, contemporary Tamil script (தமிழ்).
- Keep technical terms and mathematical formulas intelligible using universal scientific notations.`;
  }

  // 2. HINDI (hi)
  if (aiLanguage === 'hi') {
    if (conversationStyle === 'tanglish' || conversationStyle === 'mixed') {
      return `CONVERSATION STYLE: HINGLISH (Hindi + English Code-Switching)
- Converse in friendly, contemporary Hinglish (Romanized Hindi + English).
- Retain scientific and technical terms in English.
- Example: "Aaj evening 6 PM ko Physics Optics revise karte hain, usse pehle 15-20 min break le lo."`;
    }

    return `CONVERSATION STYLE: HINDI (हिन्दी)
- Respond in natural, contemporary Hindi using Devanagari script (हिन्दी).
- Maintain standard international scientific and mathematical notations.`;
  }

  // 3. TELUGU (te)
  if (aiLanguage === 'te') {
    if (conversationStyle === 'tanglish' || conversationStyle === 'mixed') {
      return `
LINGUISTIC INSTRUCTIONS — CONVERSATIONAL TENGLISH:
- Converse in conversational Romanized Telugu + English (Tenglish).
- Retain scientific and academic terms in English.
- Maintain a supportive, friendly mentor tone.`;
    }
    return `
LINGUISTIC INSTRUCTIONS — CONTEMPORARY TELUGU (తెలుగు):
- Respond in natural contemporary Telugu script (తెలుగు).`;
  }

  // 4. MALAYALAM (ml)
  if (aiLanguage === 'ml') {
    if (conversationStyle === 'tanglish' || conversationStyle === 'mixed') {
      return `
LINGUISTIC INSTRUCTIONS — CONVERSATIONAL MANGLISH:
- Converse in conversational Romanized Malayalam + English (Manglish).
- Retain scientific and technical concepts in English.`;
    }
    return `
LINGUISTIC INSTRUCTIONS — CONTEMPORARY MALAYALAM (മലയാളം):
- Respond in clear, natural Malayalam script (മലയാളം).`;
  }

  // 5. KANNADA (kn)
  if (aiLanguage === 'kn') {
    if (conversationStyle === 'tanglish' || conversationStyle === 'mixed') {
      return `
LINGUISTIC INSTRUCTIONS — CONVERSATIONAL KANGLISH:
- Converse in conversational Romanized Kannada + English (Kanglish).
- Retain scientific and technical concepts in English.`;
    }
    return `
LINGUISTIC INSTRUCTIONS — CONTEMPORARY KANNADA (ಕನ್ನಡ):
- Respond in clear, natural Kannada script (ಕನ್ನಡ).`;
  }

  // 6. BENGALI (bn)
  if (aiLanguage === 'bn') {
    if (conversationStyle === 'tanglish' || conversationStyle === 'mixed') {
      return `
LINGUISTIC INSTRUCTIONS — CONVERSATIONAL BANGLISH:
- Converse in conversational Romanized Bengali + English (Banglish).`;
    }
    return `
LINGUISTIC INSTRUCTIONS — CONTEMPORARY BENGALI (বাংলা):
- Respond in clear, natural Bengali script (বাংলা).`;
  }

  // 7. MARATHI (mr)
  if (aiLanguage === 'mr') {
    if (conversationStyle === 'tanglish' || conversationStyle === 'mixed') {
      return `
LINGUISTIC INSTRUCTIONS — CONVERSATIONAL MARATHI + ENGLISH:
- Converse in conversational Romanized Marathi + English.`;
    }
    return `
LINGUISTIC INSTRUCTIONS — CONTEMPORARY MARATHI (मराठी):
- Respond in clear, natural Marathi script (मराठी).`;
  }

  // 8. ENGLISH (en) DEFAULT
  return `
LINGUISTIC INSTRUCTIONS — CONTEMPORARY ENGLISH:
- Respond in crisp, engaging, direct academic English.
- Be concise, solution-oriented, and encouraging.`;
}

/**
 * Generates a realistic query and sample AI response for "Plan my evening"
 * used in live preview components across Settings and Onboarding.
 */
export function getSampleAiPreview(aiLanguage: string, conversationStyle: string): SampleAiPreview {
  // Tanglish
  if (aiLanguage === 'ta' && conversationStyle === 'tanglish') {
    return {
      query: 'Plan my evening study schedule',
      response:
        'Bro, innaiku evening 6:00 PM-ku Physics Electrostatics revise pannalam. 7:30 PM-ku dinner mudichitu, Math problem set solve pannuvom. Adhukku munnadi oru 30 mins fresh air walk poitu vaanga!',
      languageLabel: 'Tamil',
      styleLabel: 'Conversational Tanglish',
    };
  }

  // Pure Tamil
  if (aiLanguage === 'ta') {
    return {
      query: 'இன்றைய மாலை நேர அட்டவணையைத் திட்டமிடுங்கள்',
      response:
        'வணக்கம்! இன்று மாலை 6:00 மணிக்கு இயற்பியல் நிலைமின்னியல் பாடத்தை மீள்பார்வை செய்யலாம். இரவு உணவிற்குப் பிறகு கணிதப் பயிற்சிகளைத் தொடங்குவோம். அதற்கு முன் 30 நிமிடங்கள் நடைப்பயிற்சி மேற்கொண்டு புத்துணர்ச்சி பெறுங்கள்!',
      languageLabel: 'Tamil',
      styleLabel: 'தூய தமிழ் (தமிழ் எழுத்துரு)',
    };
  }

  // Hinglish
  if (aiLanguage === 'hi' && (conversationStyle === 'tanglish' || conversationStyle === 'mixed')) {
    return {
      query: 'Plan my evening study schedule',
      response:
        'Aaj evening 6:00 PM ko Physics Electrostatics revise karte hain. 7:30 PM dinner ke baad Math problem set solve karenge. Usse pehle ek 30 mins ki fresh air walk le lo!',
      languageLabel: 'Hindi',
      styleLabel: 'Conversational Hinglish',
    };
  }

  // Pure Hindi
  if (aiLanguage === 'hi') {
    return {
      query: 'शाम का अध्ययन कार्यक्रम बनाएं',
      response:
        'नमस्ते! आज शाम 6:00 बजे भौतिकी इलेक्ट्रोस्टैटिक्स का रिवीजन करते हैं। रात के भोजन के बाद गणित के प्रश्न हल करेंगे। उससे पहले 30 मिनट टहलकर तरोताजा हो जाएं!',
      languageLabel: 'Hindi',
      styleLabel: 'हिन्दी (देवनागरी)',
    };
  }

  // Telugu
  if (aiLanguage === 'te') {
    return {
      query: 'సాయంత్రం చదువుకునే ప్రణాళిక రూపొందించండి',
      response:
        'నమస్కారం! ఈరోజు సాయంత్రం 6:00 గంటలకు ఫిజిక్స్ ఎలెక్ట్రోస్టాటిక్స్ రివిజన్ చేద్దాం. రాత్రి భోజనం తర్వాత మ్యాథ్స్ ప్రాబ్లమ్స్ ప్రాక్టీస్ చేద్దాం. అంతకంటే ముందు ఒక 30 నిమిషాలు వాకింగ్ చేసి రండి!',
      languageLabel: 'Telugu',
      styleLabel: 'తెలుగు (తెలుగు లిపి)',
    };
  }

  // Malayalam
  if (aiLanguage === 'ml') {
    return {
      query: 'ഇന്നത്തെ വൈകുന്നേരത്തെ പഠനക്രമം തയ്യാറാക്കുക',
      response:
        'നമസ്കാരം! ഇന്ന് വൈകുന്നേരം 6:00 മണിക്ക് ഫിസിക്സ് ഇലക്ട്രോസ്റ്റാറ്റിക്സ് റിവിഷൻ ചെയ്യാം. അത്താഴത്തിന് ശേഷം കണക്ക് പ്രോബ്ലംസ് ചെയ്യാം. അതിനുമുമ്പ് ഒരു 30 മിനിറ്റ് നടക്കാൻ പോയി വരൂ!',
      languageLabel: 'Malayalam',
      styleLabel: 'മലയാളം (മലയാള ലിപി)',
    };
  }

  // Kannada
  if (aiLanguage === 'kn') {
    return {
      query: 'ಸಂಜೆಯ ಅಧ್ಯಯನ ವೇಳಾಪಟ್ಟಿಯನ್ನು ಯೋಜಿಸಿ',
      response:
        'ನಮಸ್ಕಾರ! ಇಂದು ಸಂಜೆ 6:00 ಗಂಟೆಗೆ ಫಿಸಿಕ್ಸ್ ಎಲೆಕ್ಟ್ರೋಸ್ಟಾಟಿಕ್ಸ್ ಪುನರಾವರ್ತನೆ ಮಾಡೋಣ. ಊಟದ ನಂತರ ಗಣಿತ ಸಮಸ್ಯೆಗಳನ್ನು ಬಿಡಿಸೋಣ. ಅದಕ್ಕೂ ಮೊದಲು 30 ನಿಮಿಷಗಳ ಕಾಲ ವಾಕಿಂಗ್ ಮಾಡಿ ಬನ್ನಿ!',
      languageLabel: 'Kannada',
      styleLabel: 'ಕನ್ನಡ (ಕನ್ನಡ ಲಿಪಿ)',
    };
  }

  // Bengali
  if (aiLanguage === 'bn') {
    return {
      query: 'সন্ধ্যার অধ্যয়নের সময়সূচী পরিকল্পনা করুন',
      response:
        'নমস্কার! আজ সন্ধ্যা ৬:০০ টায় ফিজিক্স ইলেক্ট্রোস্ট্যাটিক্স রিভিশন করব। রাতের খাবারের পর গণিতের সমস্যা সমাধান করব। তার আগে ৩০ মিনিট হেঁটে সতেজ হয়ে নিন!',
      languageLabel: 'Bengali',
      styleLabel: 'বাংলা (বাংলা লিপি)',
    };
  }

  // Marathi
  if (aiLanguage === 'mr') {
    return {
      query: 'संध्याकाळच्या अभ्यासाचे नियोजन करा',
      response:
        'नमस्कार! आज संध्याकाळी ६:०० वाजता भौतिकशास्त्र इलेक्ट्रोस्टॅटिक्सची उजळणी करूया. रात्रीच्या जेवणानंतर गणिताचा सराव करूया. त्यापूर्वी ३० मिनिटे फिरून ताजेतवाने व्हा!',
      languageLabel: 'Marathi',
      styleLabel: 'मराठी (देवनागरी)',
    };
  }

  // English Default
  return {
    query: 'Plan my evening study schedule',
    response:
      "Good evening! Let's kick off at 6:00 PM with Physics Electrostatics revision. After dinner at 7:30 PM, we'll tackle the Calculus problem set. Before diving in, take a 30-minute refreshing walk to clear your mind!",
    languageLabel: 'English',
    styleLabel: 'Contemporary English',
  };
}
