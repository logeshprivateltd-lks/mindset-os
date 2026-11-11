import type { UserProfile } from '@mindset/db-schema';

export interface StudentContext {
  userId: string;
  displayName: string;
  role: string;
  interfaceLang: string;
  aiLang: string;
  conversationStyle: string;
  topGoals: string[];
  activeFocusAreas: string[];
}

export class PersonalContextBuilder {
  static buildContext(profile: UserProfile): StudentContext {
    let parsedGoals: string[] = [];
    let parsedFocusAreas: string[] = [];

    try {
      parsedGoals = JSON.parse(profile.goals_json);
    } catch {
      parsedGoals = [];
    }

    try {
      parsedFocusAreas = JSON.parse(profile.focus_areas_json);
    } catch {
      parsedFocusAreas = [];
    }

    return {
      userId: profile.user_id,
      displayName: profile.display_name,
      role: profile.role,
      interfaceLang: profile.interface_lang,
      aiLang: profile.ai_lang,
      conversationStyle: profile.conversation_style,
      topGoals: parsedGoals,
      activeFocusAreas: parsedFocusAreas,
    };
  }
}

