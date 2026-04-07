/**
 * Script Module - Zustand Store
 * State management for automation scripts
 */

import { create } from "zustand";
import type { Script, ScriptStatus, Trigger, Action } from "./data";
import * as service from "./service";

interface ScriptState {
  // Data
  scripts: Script[];
  statuses: Record<string, ScriptStatus>;
  
  // UI State
  loading: boolean;
  error: string | null;
  selectedScriptId: string | null;
  
  // Actions
  loadScripts: () => Promise<void>;
  createScript: (name: string, trigger: Trigger) => Promise<Script>;
  updateScript: (script: Script) => Promise<void>;
  removeScript: (id: string) => Promise<void>;
  selectScript: (id: string | null) => void;
  startScript: (id: string) => Promise<void>;
  stopScript: (id: string) => Promise<void>;
  addAction: (scriptId: string, action: Action) => Promise<void>;
  removeAction: (scriptId: string, index: number) => Promise<void>;
  updateTrigger: (scriptId: string, trigger: Trigger) => Promise<void>;
  clearError: () => void;
}

export const useScriptStore = create<ScriptState>((set, get) => ({
  scripts: [],
  statuses: {},
  loading: false,
  error: null,
  selectedScriptId: null,

  loadScripts: async () => {
    set({ loading: true, error: null });
    try {
      const scripts = await service.listScripts();
      const statuses = await service.listScriptStatuses();
      set({ scripts, statuses, loading: false });
    } catch (err) {
      set({ error: err instanceof Error ? err.message : "Failed to load scripts", loading: false });
    }
  },

  createScript: async (name: string, trigger: Trigger) => {
    const now = new Date().toISOString();
    const newScript: Script = {
      id: crypto.randomUUID(),
      name,
      description: "",
      trigger,
      actions: [],
      enabled: true,
      created_at: now,
      updated_at: now,
    };
    const saved = await service.saveScript(newScript);
    set((state) => ({
      scripts: [...state.scripts, saved],
      selectedScriptId: saved.id,
    }));
    return saved;
  },

  updateScript: async (script: Script) => {
    const saved = await service.saveScript(script);
    set((state) => ({
      scripts: state.scripts.map((s) => (s.id === saved.id ? saved : s)),
    }));
  },

  removeScript: async (id: string) => {
    await service.deleteScript(id);
    set((state) => ({
      scripts: state.scripts.filter((s) => s.id !== id),
      selectedScriptId: state.selectedScriptId === id ? null : state.selectedScriptId,
    }));
  },

  selectScript: (id: string | null) => {
    set({ selectedScriptId: id });
  },

  startScript: async (id: string) => {
    await service.startScript(id);
    const status = await service.getScriptStatus(id);
    set((state) => ({
      statuses: { ...state.statuses, [id]: status ?? { script_id: id, running: true, last_triggered: null, last_error: null } },
    }));
  },

  stopScript: async (id: string) => {
    await service.stopScript(id);
    const status = await service.getScriptStatus(id);
    set((state) => ({
      statuses: { ...state.statuses, [id]: status ?? { script_id: id, running: false, last_triggered: null, last_error: null } },
    }));
  },

  addAction: async (scriptId: string, action: Action) => {
    const script = get().scripts.find((s) => s.id === scriptId);
    if (!script) return;
    const updated: Script = {
      ...script,
      actions: [...script.actions, action],
      updated_at: new Date().toISOString(),
    };
    const saved = await service.saveScript(updated);
    set((state) => ({
      scripts: state.scripts.map((s) => (s.id === saved.id ? saved : s)),
    }));
  },

  removeAction: async (scriptId: string, index: number) => {
    const script = get().scripts.find((s) => s.id === scriptId);
    if (!script) return;
    const updated: Script = {
      ...script,
      actions: script.actions.filter((_, i) => i !== index),
      updated_at: new Date().toISOString(),
    };
    const saved = await service.saveScript(updated);
    set((state) => ({
      scripts: state.scripts.map((s) => (s.id === saved.id ? saved : s)),
    }));
  },

  updateTrigger: async (scriptId: string, trigger: Trigger) => {
    const script = get().scripts.find((s) => s.id === scriptId);
    if (!script) return;
    const updated: Script = {
      ...script,
      trigger,
      updated_at: new Date().toISOString(),
    };
    const saved = await service.saveScript(updated);
    set((state) => ({
      scripts: state.scripts.map((s) => (s.id === saved.id ? saved : s)),
    }));
  },

  clearError: () => {
    set({ error: null });
  },
}));
