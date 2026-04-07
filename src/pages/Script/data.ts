/**
 * Script Module - Type Definitions
 * TypeScript types for automation scripts matching Rust backend
 */

// ========== Trigger Types ==========

/** Comparison operators for trigger conditions */
export type CompareOp = "gt" | "lt" | "eq" | "neq" | "gte" | "lte";

export const CompareOpLabels: Record<CompareOp, { label: string; symbol: string }> = {
  gt: { label: "Greater than", symbol: ">" },
  lt: { label: "Less than", symbol: "<" },
  eq: { label: "Equal", symbol: "=" },
  neq: { label: "Not equal", symbol: "≠" },
  gte: { label: "Greater or equal", symbol: "≥" },
  lte: { label: "Less or equal", symbol: "≤" },
};

/** Trigger condition types */
export type Trigger =
  | { type: "compare"; register: string; operator: CompareOp; value: number }
  | { type: "changed"; register: string }
  | { type: "became_on"; register: string }
  | { type: "became_off"; register: string }
  | { type: "stable"; register: string; seconds: number };

export function getTriggerRegister(trigger: Trigger): string {
  switch (trigger.type) {
    case "compare":
      return trigger.register;
    case "changed":
      return trigger.register;
    case "became_on":
      return trigger.register;
    case "became_off":
      return trigger.register;
    case "stable":
      return trigger.register;
  }
}

export function getTriggerDescription(trigger: Trigger): string {
  switch (trigger.type) {
    case "compare":
      return `${trigger.register} ${CompareOpLabels[trigger.operator].symbol} ${trigger.value}`;
    case "changed":
      return `${trigger.register} changed`;
    case "became_on":
      return `${trigger.register} became ON`;
    case "became_off":
      return `${trigger.register} became OFF`;
    case "stable":
      return `${trigger.register} stable for ${trigger.seconds}s`;
  }
}

// ========== Action Types ==========

export type SoundType = "alert" | "success" | "warning";

/** Action types that can be executed when trigger fires */
export type Action =
  | { type: "write_value"; register: string; value: number }
  | { type: "write_on"; register: string }
  | { type: "write_off"; register: string }
  | { type: "toggle"; register: string }
  | { type: "show_notification"; title: string; message: string }
  | { type: "play_sound"; sound: SoundType }
  | { type: "log"; message: string }
  | { type: "run_script"; script_id: string }
  | { type: "stop_script"; script_id: string }
  | { type: "delay"; seconds: number };

export function getActionDescription(action: Action): string {
  switch (action.type) {
    case "write_value":
      return `Write ${action.value} to ${action.register}`;
    case "write_on":
      return `Turn ON ${action.register}`;
    case "write_off":
      return `Turn OFF ${action.register}`;
    case "toggle":
      return `Toggle ${action.register}`;
    case "show_notification":
      return `Notify: ${action.title} - ${action.message}`;
    case "play_sound":
      return `Play sound: ${action.sound}`;
    case "log":
      return `Log: ${action.message}`;
    case "run_script":
      return `Run script: ${action.script_id}`;
    case "stop_script":
      return `Stop script: ${action.script_id}`;
    case "delay":
      return `Wait ${action.seconds}s`;
  }
}

export function actionHasRegister(action: Action): boolean {
  return ["write_value", "write_on", "write_off", "toggle"].includes(action.type);
}

// ========== Script Types ==========

export interface Script {
  id: string;
  name: string;
  description: string;
  trigger: Trigger;
  actions: Action[];
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface ScriptStatus {
  script_id: string;
  running: boolean;
  last_triggered: string | null;
  last_error: string | null;
}

// ========== Command Response ==========

export interface CommandResponse<T> {
  success: boolean;
  data?: T;
  error?: {
    code: number;
    message: string;
  };
}

// ========== UI State Types ==========

export interface ScriptWithStatus extends Script {
  status: ScriptStatus | null;
}
