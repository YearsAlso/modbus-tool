/**
 * Script Module - Tauri IPC Service
 * Wrapper for Tauri invoke commands for script management
 */

import { invoke } from "@tauri-apps/api/core";
import type { Script, ScriptStatus, CommandResponse } from "./data";

/**
 * List all scripts
 */
export async function listScripts(): Promise<Script[]> {
  const response = await invoke<CommandResponse<Script[]>>("script_list");
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to list scripts");
  }
  return response.data ?? [];
}

/**
 * Get a single script by ID
 */
export async function getScript(id: string): Promise<Script | null> {
  const response = await invoke<CommandResponse<Script | null>>("script_get", { id });
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to get script");
  }
  return response.data ?? null;
}

/**
 * Save (create or update) a script
 */
export async function saveScript(script: Script): Promise<Script> {
  const response = await invoke<CommandResponse<Script>>("script_save", { script });
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to save script");
  }
  return response.data!;
}

/**
 * Delete a script by ID
 */
export async function deleteScript(id: string): Promise<boolean> {
  const response = await invoke<CommandResponse<boolean>>("script_delete", { id });
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to delete script");
  }
  return response.data ?? false;
}

/**
 * Get script execution status
 */
export async function getScriptStatus(id: string): Promise<ScriptStatus | null> {
  const response = await invoke<CommandResponse<ScriptStatus | null>>("script_status", { id });
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to get script status");
  }
  return response.data ?? null;
}

/**
 * Start a script
 */
export async function startScript(id: string): Promise<boolean> {
  const response = await invoke<CommandResponse<boolean>>("script_start", { id });
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to start script");
  }
  return response.data ?? false;
}

/**
 * Stop a script
 */
export async function stopScript(id: string): Promise<boolean> {
  const response = await invoke<CommandResponse<boolean>>("script_stop", { id });
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to stop script");
  }
  return response.data ?? false;
}

/**
 * Get all script statuses
 */
export async function listScriptStatuses(): Promise<Record<string, ScriptStatus>> {
  const response = await invoke<CommandResponse<Record<string, ScriptStatus>>>("script_list_statuses");
  if (!response.success) {
    throw new Error(response.error?.message ?? "Failed to list script statuses");
  }
  return response.data ?? {};
}
