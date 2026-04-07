// Modbus Protocol Types
export type ModbusProtocol = "RTU" | "ASCII" | "TCP" | "UDP";
export type ModbusFunctionCode = 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x0F | 0x10;
export type ConnectionStatus = "disconnected" | "connecting" | "connected" | "error";
export type DataFormat = "INT16" | "UINT16" | "INT32" | "UINT32" | "FLOAT32" | "BCD" | "HEX" | "STRING";
export type ParityType = "none" | "even" | "odd";
export type DataBitsType = 5 | 6 | 7 | 8;
export type StopBitsType = 1 | 2;
export interface SerialConfig {
  type: "serial";
  protocol: "RTU" | "ASCII";
  port: string;
  baudRate: number;
  dataBits: DataBitsType;
  stopBits: StopBitsType;
  parity: ParityType;
  slaveId: number;
  timeout: number;
}
export interface NetworkConfig {
  type: "network";
  protocol: "TCP" | "UDP";
  host: string;
  port: number;
  slaveId: number;
  timeout: number;
}
export type ConnectionConfig = SerialConfig | NetworkConfig;
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
  duration: number;
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
  raw: string;
  parsed: ModbusRequest | ModbusResponse;
  timestamp: number;
}
export interface DataConverterOptions {
  format: DataFormat;
  byteOrder: "AB" | "BA" | "ABCD" | "CDAB" | "BADC" | "DCBA";
  signed: boolean;
}
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
