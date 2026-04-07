// Modbus Protocol Types

export type ModbusProtocol = "RTU" | "ASCII" | "TCP" | "UDP";

export type ModbusFunctionCode =
  | 0x01  // Read Coils
  | 0x02  // Read Discrete Inputs
  | 0x03  // Read Holding Registers
  | 0x04  // Read Input Registers
  | 0x05  // Write Single Coil
  | 0x06  // Write Single Register
  | 0x0F  // Write Multiple Coils
  | 0x10; // Write Multiple Registers

export interface ConnectionConfig {
  protocol: ModbusProtocol;
  // Serial settings (RTU/ASCII)
  port?: string;
  baudRate?: number;
  dataBits?: 5 | 6 | 7 | 8;
  stopBits?: 1 | 2;
  parity?: "none" | "even" | "odd";
  // TCP/UDP settings
  host?: string;
  port?: number;
  // Common
  slaveId: number;
  timeout: number;
}

export interface ModbusRequest {
  id: string;
  functionCode: ModbusFunctionCode;
  address: number;
  quantity: number;
  data?: number[];
  timestamp: number;
}

export interface ModbusResponse {
  id: string;
  requestId: string;
  success: boolean;
  data?: number[];
  errorCode?: number;
  errorMessage?: string;
  timestamp: number;
  duration: number; // ms
}

export interface ModbusRegister {
  address: number;
  value: number;
  type: "coil" | "discrete" | "holding" | "input";
  timestamp: number;
}

export interface ModbusMessage {
  id: string;
  direction: "request" | "response";
  raw: string; // hex string
  parsed: ModbusRequest | ModbusResponse;
  timestamp: number;
}

// Connection state
export type ConnectionStatus = 
  | "disconnected"
  | "connecting"
  | "connected"
  | "error";

// Data format types
export type DataFormat = "INT16" | "UINT16" | "INT32" | "UINT32" | "FLOAT32" | "BCD" | "HEX" | "STRING";

export interface DataConverterOptions {
  format: DataFormat;
  byteOrder: "AB" | "BA" | "ABCD" | "CDAB" | "BADC" | "DCBA";
  signed: boolean;
}

// Monitoring types
export interface MonitoredRegister {
  address: number;
  name: string;
  format: DataFormat;
  color: string;
}

export interface MonitorPoint {
  id: string;
  register: MonitoredRegister;
  value: number | null;
  history: { timestamp: number; value: number }[];
}
