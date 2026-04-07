import { useCallback } from "react";
import { useConnectionStore } from "@/stores";
import * as ipc from "@/lib/ipc";
import type { ConnectionConfig, ModbusFunctionCode, ModbusMessage } from "@/types";

export function useModbus() {
  const { status, setStatus, setError, addMessage } = useConnectionStore();

  const connect = useCallback(async (config: ConnectionConfig) => {
    try {
      setStatus("connecting");
      setError(null);
      
      const result = await ipc.connect(config);
      
      if (result.success) {
        setStatus("connected");
        return true;
      } else {
        setStatus("error");
        setError(result.error || "Connection failed");
        return false;
      }
    } catch (err) {
      setStatus("error");
      setError(err instanceof Error ? err.message : "Unknown error");
      return false;
    }
  }, [setStatus, setError]);

  const disconnect = useCallback(async () => {
    try {
      const result = await ipc.disconnect();
      
      if (result.success) {
        setStatus("disconnected");
        return true;
      } else {
        setError(result.error || "Disconnect failed");
        return false;
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
      return false;
    }
  }, [setStatus, setError]);

  const readRegisters = useCallback(async (
    functionCode: ModbusFunctionCode,
    address: number,
    quantity: number
  ) => {
    try {
      let response;
      
      switch (functionCode) {
        case 0x01:
          response = await ipc.readCoils(address, quantity);
          break;
        case 0x02:
          response = await ipc.readDiscreteInputs(address, quantity);
          break;
        case 0x03:
          response = await ipc.readHoldingRegisters(address, quantity);
          break;
        case 0x04:
          response = await ipc.readInputRegisters(address, quantity);
          break;
        default:
          throw new Error(`Unsupported function code: ${functionCode}`);
      }

      // Log the message
      const message: ModbusMessage = {
        id: crypto.randomUUID(),
        direction: "response",
        raw: response.data?.map((v) => v.toString(16).padStart(2, "0")).join(" ") || "",
        parsed: response,
        timestamp: response.timestamp,
      };
      addMessage(message);

      return response;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Read failed");
      throw err;
    }
  }, [setError, addMessage]);

  const writeRegister = useCallback(async (
    functionCode: ModbusFunctionCode,
    address: number,
    value: number | boolean | number[]
  ) => {
    try {
      let response;
      
      switch (functionCode) {
        case 0x05:
          response = await ipc.writeSingleCoil(address, value as boolean);
          break;
        case 0x06:
          response = await ipc.writeSingleRegister(address, value as number);
          break;
        case 0x0F:
          response = await ipc.writeMultipleCoils(address, value as boolean[]);
          break;
        case 0x10:
          response = await ipc.writeMultipleRegisters(address, value as number[]);
          break;
        default:
          throw new Error(`Unsupported function code: ${functionCode}`);
      }

      // Log the message
      const message: ModbusMessage = {
        id: crypto.randomUUID(),
        direction: "response",
        raw: response.data?.map((v) => v.toString(16).padStart(2, "0")).join(" ") || "",
        parsed: response,
        timestamp: response.timestamp,
      };
      addMessage(message);

      return response;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Write failed");
      throw err;
    }
  }, [setError, addMessage]);

  return {
    status,
    connect,
    disconnect,
    readRegisters,
    writeRegister,
  };
}
