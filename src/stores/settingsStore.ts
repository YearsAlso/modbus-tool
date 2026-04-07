import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface Settings {
  theme: "light" | "dark" | "system";
  language: "en" | "zh-CN";
  autoReconnect: boolean;
  maxReconnectAttempts: number;
  logLevel: "debug" | "info" | "warn" | "error";
  defaultProtocol: "RTU" | "ASCII" | "TCP" | "UDP";
  defaultTimeout: number;
  defaultSlaveId: number;
}

interface SettingsState {
  settings: Settings;
  updateSettings: (settings: Partial<Settings>) => void;
  resetSettings: () => void;
}

const defaultSettings: Settings = {
  theme: "system",
  language: "en",
  autoReconnect: true,
  maxReconnectAttempts: 3,
  logLevel: "info",
  defaultProtocol: "TCP",
  defaultTimeout: 3000,
  defaultSlaveId: 1,
};

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      settings: defaultSettings,
      
      updateSettings: (newSettings) =>
        set((state) => ({
          settings: { ...state.settings, ...newSettings },
        })),
        
      resetSettings: () => set({ settings: defaultSettings }),
    }),
    {
      name: "modbus-tool-settings",
    }
  )
);
