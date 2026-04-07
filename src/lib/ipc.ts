import { invoke } from "@tauri-apps/api/core";
import type {
  SerialConfig,
  NetworkConfig,
  ModbusRequest,
  ModbusResponse,
  ModbusRegister,
  ConnectionStatus,
} from "@/types";

export type ConnectionConfig = SerialConfig | NetworkConfig;

export async function connect(config: ConnectionConfig): Promise<{ success: boolean; error?: string }> {
  return invoke("connect", { config });
}

export async function disconnect(): Promise<{ success: boolean; error?: string }> {
  return invoke("disconnect");
}

export async function getConnectionStatus(): Promise<ConnectionStatus> {
  return invoke("get_connection_status");
}

export async function readCoils(address: number, quantity: number): Promise<ModbusResponse> {
  return invoke("read_coils", { address, quantity });
}

export async function readDiscreteInputs(address: number, quantity: number): Promise<ModbusResponse> {
  return invoke("read_discrete_inputs", { address, quantity });
}

export async function readHoldingRegisters(address: number, quantity: number): Promise<ModbusResponse> {
  return invoke("read_holding_registers", { address, quantity });
}

export async function readInputRegisters(address: number, quantity: number): Promise<ModbusResponse> {
  return invoke("read_input_registers", { address, quantity });
}

export async function writeSingleCoil(address: number, value: boolean): Promise<ModbusResponse> {
  return invoke("write_single_coil", { address, value });
}

export async function writeSingleRegister(address: number, value: number): Promise<ModbusResponse> {
  return invoke("write_single_register", { address, value });
}

export async function writeMultipleCoils(address: number, values: boolean[]): Promise<ModbusResponse> {
  return invoke("write_multiple_coils", { address, values });
}

export async function writeMultipleRegisters(address: number, values: number[]): Promise<ModbusResponse> {
  return invoke("write_multiple_registers", { address, values });
}

export async function getRegisterValue(address: number): Promise<ModbusRegister | null> {
  return invoke("get_register_value", { address });
}

export async function setRegisterValue(address: number, value: number): Promise<boolean> {
  return invoke("set_register_value", { address, value });
}

export async function listSerialPorts(): Promise<string[]> {
  return invoke("list_serial_ports");
}

export async function sendModbusRequest(request: ModbusRequest): Promise<ModbusResponse> {
  return invoke("send_modbus_request", { request });
}

export interface AppSettings {
  theme: "light" | "dark" | "system";
  language: "en" | "zh-CN";
  autoReconnect: boolean;
  maxReconnectAttempts: number;
  logLevel: "debug" | "info" | "warn" | "error";
}

export async function getSettings(): Promise<AppSettings> {
  return invoke("get_settings");
}

export async function saveSettings(settings: AppSettings): Promise<boolean> {
  return invoke("save_settings", { settings });
}
