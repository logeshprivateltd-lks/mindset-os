/**
 * MindSet OS — Laya AI Client
 * Frontend IPC bridge to Tauri AI Subsystem & Single-Model Orchestrator
 * Copyright (c) 2026 LKS PVT LTD. All rights reserved.
 */

import {
  LayaRouter,
  DEFAULT_LAYA_MODEL,
  PERSONA_CONFIGS,
  getPersonaConfig,
  runLayaBenchmark,
  type AgentPersona,
  type PersonaConfig,
  type LayaMessage,
  type LayaStreamChunk,
  type LocalModelInfo,
  type BenchmarkScoreCard,
} from '@mindset/laya-router';
import { dbClient } from './dbClient';

async function invokeTauri<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[layaClient] Tauri invoke failed for command '${cmd}':`, err);
    throw err;
  }
}

export const sharedLayaRouter = new LayaRouter();

export const layaClient = {
  // ---------------------------------------------------------------------------
  // 1. Streaming Chat & Persona Interactions
  // ---------------------------------------------------------------------------
  chat: {
    stream: async (
      persona: AgentPersona,
      messages: LayaMessage[],
      onChunk?: (chunk: LayaStreamChunk) => void
    ): Promise<string> => {
      // Gather active profile and tasks context from local SQLite
      let profile = null;
      let tasks: any[] = [];

      try {
        profile = await dbClient.profile.get();
        tasks = await dbClient.tasks.list();
      } catch {
        // Fallback to offline memory
      }

      return sharedLayaRouter.streamChat({
        persona,
        messages,
        contextData: {
          profile,
          tasks,
        },
        onChunk,
      });
    },

    quickAsk: async (
      persona: AgentPersona,
      userPrompt: string,
      onToken?: (token: string) => void
    ): Promise<string> => {
      return layaClient.chat.stream(
        persona,
        [{ role: 'user', content: userPrompt }],
        onToken
          ? (chunk) => {
              if (chunk.token) onToken(chunk.token);
            }
          : undefined
      );
    },
  },

  // ---------------------------------------------------------------------------
  // 2. Structured JSON Tool Execution
  // ---------------------------------------------------------------------------
  tools: {
    execute: async <T = Record<string, unknown>>(
      toolName: string,
      userPrompt: string,
      jsonSchema?: Record<string, unknown>,
      persona: AgentPersona = 'PlannerAgent'
    ): Promise<T> => {
      try {
        return await invokeTauri<T>('ai_execute_structured_tool_call', {
          model: sharedLayaRouter.getActiveModel(),
          systemPrompt: `You are Laya ${persona} operating as a structured tool generator for '${toolName}'. Respond ONLY with valid JSON.`,
          userPrompt,
          jsonSchema: jsonSchema ? JSON.stringify(jsonSchema) : undefined,
        });
      } catch {
        return sharedLayaRouter.executeToolCall<T>(toolName, userPrompt, jsonSchema, persona);
      }
    },
  },

  // ---------------------------------------------------------------------------
  // 3. Local Model State & RAM Discipline
  // ---------------------------------------------------------------------------
  models: {
    list: async (): Promise<LocalModelInfo[]> => {
      try {
        return await invokeTauri<LocalModelInfo[]>('ai_list_local_models');
      } catch {
        return sharedLayaRouter.listAvailableModels();
      }
    },

    getActive: async (): Promise<string> => {
      try {
        return await invokeTauri<string>('ai_get_active_model');
      } catch {
        return sharedLayaRouter.getActiveModel();
      }
    },

    setActive: async (modelName: string): Promise<string> => {
      sharedLayaRouter.setActiveModel(modelName);
      try {
        return await invokeTauri<string>('ai_set_active_model', { modelName });
      } catch {
        return modelName;
      }
    },

    unload: async (modelName?: string): Promise<boolean> => {
      const target = modelName || sharedLayaRouter.getActiveModel();
      try {
        return await invokeTauri<boolean>('ai_unload_model', { modelName: target });
      } catch {
        return sharedLayaRouter.unloadActiveModel();
      }
    },
  },

  // ---------------------------------------------------------------------------
  // 4. Multilingual 8-Point Benchmark Suite
  // ---------------------------------------------------------------------------
  benchmark: {
    run: async (): Promise<BenchmarkScoreCard> => {
      return runLayaBenchmark({ router: sharedLayaRouter });
    },
  },

  // ---------------------------------------------------------------------------
  // 5. Persona Configurations
  // ---------------------------------------------------------------------------
  personas: {
    get: (persona: AgentPersona): PersonaConfig => {
      return getPersonaConfig(persona);
    },
    list: (): PersonaConfig[] => {
      return Object.values(PERSONA_CONFIGS);
    },
  },
};

export { DEFAULT_LAYA_MODEL };

