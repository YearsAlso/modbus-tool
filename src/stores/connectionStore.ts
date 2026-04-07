import { create } from "zustand";
import type { ConnectionConfig, ConnectionStatus, ModbusMessage } from "@/types";

interface ConnectionState {
  status: ConnectionStatus;
  config: ConnectionConfig | null;
  error: string | null;
  messages: ModbusMessage[];
  addMessage: (message: ModbusMessage) => void;
  clearMessages: () => void;
  setStatus: (status: ConnectionStatus) => void;
  setConfig: (config: ConnectionConfig | null) => void;
  setError: (error: string | null) => void;
}

export const useConnectionStore = create<ConnectionState>((set) => ({
  status: "disconnected",
  config: null,
  error: null,
  messages: [],
  
  addMessage: (message) =>
    set((state) => ({
      messages: [...state.messages, message].slice(-1000), // Keep last 1000 messages
    })),
    
  clearMessages: () => set({ messages: [] }),
  
  setStatus: (status) => set({ status }),
  
  setConfig: (config) => set({ config }),
  
  setError: (error) => set({ error }),
}));
