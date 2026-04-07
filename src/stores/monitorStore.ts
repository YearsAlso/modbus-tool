import { create } from "zustand";
import type { MonitorPoint, MonitoredRegister } from "@/types";

interface MonitorState {
  points: MonitorPoint[];
  isMonitoring: boolean;
  interval: number; // ms
  addPoint: (register: MonitoredRegister) => void;
  removePoint: (id: string) => void;
  updatePointValue: (id: string, value: number) => void;
  clearHistory: (id: string) => void;
  clearAll: () => void;
  setMonitoring: (isMonitoring: boolean) => void;
  setPollInterval: (interval: number) => void;
}

export const useMonitorStore = create<MonitorState>((set) => ({
  points: [],
  isMonitoring: false,
  interval: 1000,
  
  addPoint: (register) =>
    set((state) => ({
      points: [
        ...state.points,
        {
          id: crypto.randomUUID(),
          register,
          value: null,
          history: [],
        },
      ],
    })),
    
  removePoint: (id) =>
    set((state) => ({
      points: state.points.filter((p) => p.id !== id),
    })),
    
  updatePointValue: (id, value) =>
    set((state) => ({
      points: state.points.map((p) =>
        p.id === id
          ? {
              ...p,
              value,
              history: [
                ...p.history.slice(-99), // Keep last 100 points
                { timestamp: Date.now(), value },
              ],
            }
          : p
      ),
    })),
    
  clearHistory: (id) =>
    set((state) => ({
      points: state.points.map((p) =>
        p.id === id ? { ...p, history: [] } : p
      ),
    })),
    
  clearAll: () => set({ points: [] }),
  
  setMonitoring: (isMonitoring) => set({ isMonitoring }),
  
  setPollInterval: (interval) => set({ interval }),
}));
