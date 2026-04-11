/**
 * Script Module - Zustand Store
 * State management for automation scripts
 */

import { create } from "zustand";
import type { Script, ScriptStatus, Trigger, Action } from "./data";
import { getTriggerDescription } from "./data";
import * as service from "./service";
import { showNotification as showDesktopNotification } from "./service";

// ========== Log Entry Types ==========

export interface LogEntry {
  id: string;
  scriptId: string;
  scriptName: string;
  triggeredAt: string; // ISO timestamp
  triggerCondition: string; // human-readable
  actions: string[]; // human-readable action descriptions
  success: boolean;
}

interface ScriptState {
  // Data
  scripts: Script[];
  statuses: Record<string, ScriptStatus>;
  logs: LogEntry[];

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
  executeScript: (id: string) => Promise<void>;
  clearError: () => void;

  // Log actions
  addLog: (entry: Omit<LogEntry, "id">) => void;
  clearLogs: () => void;
}

export const useScriptStore = create<ScriptState>((set, get) => ({
  scripts: [],
  statuses: {},
  logs: [],
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

  executeScript: async (id: string) => {
    const script = get().scripts.find((s) => s.id === id);
    if (!script) return;

    // Log the execution
    get().addLog({
      scriptId: script.id,
      scriptName: script.name,
      triggeredAt: new Date().toISOString(),
      triggerCondition: getTriggerDescription(script.trigger),
      actions: script.actions.map((a) => {
        switch (a.type) {
          case "write_value": return `Write ${a.value} → ${a.register}`;
          case "write_on": return `Write ON → ${a.register}`;
          case "write_off": return `Write OFF → ${a.register}`;
          case "toggle": return `Toggle ${a.register}`;
          case "show_notification": return `Notify: ${a.title}`;
          case "play_sound": return `Play sound: ${a.sound}`;
          case "log": return `Log: ${a.message}`;
          case "run_script": return `Run script ${a.script_id}`;
          case "stop_script": return `Stop script ${a.script_id}`;
          case "delay": return `Delay ${a.seconds}s`;
        }
      }),
      success: true,
    });

    // Execute notification actions with desktop notifications
    for (const action of script.actions) {
      if (action.type === "show_notification") {
        try {
          await showDesktopNotification(action.title, action.body);
        } catch (err) {
          console.error("Failed to show notification:", err);
        }
      }
    }
  },

  clearError: () => {
    set({ error: null });
  },

  addLog: (entry) => {
    set((state) => ({
      logs: [
        { ...entry, id: crypto.randomUUID() },
        ...state.logs,
      ].slice(0, 200), // Keep max 200 entries
    }));
  },

  clearLogs: () => {
    set({ logs: [] });
  },
}));
