import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useConnectionStore, useSettingsStore } from "@/stores";
import { useModbus, useSerialPorts } from "@/hooks";
import { Plug, PlugZap, RefreshCw } from "lucide-react";
import type { ConnectionConfig, ModbusProtocol } from "@/types";

const protocols: { value: ModbusProtocol; label: string }[] = [
  { value: "RTU", label: "Modbus RTU (Serial)" },
  { value: "ASCII", label: "Modbus ASCII (Serial)" },
  { value: "TCP", label: "Modbus TCP (Ethernet)" },
  { value: "UDP", label: "Modbus UDP (Ethernet)" },
];

const baudRates = [9600, 19200, 38400, 57600, 115200];
const dataBitsOptions = [5, 6, 7, 8] as const;
const stopBitsOptions = [1, 2] as const;
const parityOptions = ["none", "even", "odd"] as const;

export function ConnectionPanel() {
  const { status, error } = useConnectionStore();
  const { settings } = useSettingsStore();
  const { connect, disconnect } = useModbus();
  const { ports, loading: portsLoading, refreshPorts } = useSerialPorts();

  const [config, setConfig] = useState<ConnectionConfig>({
    protocol: settings.defaultProtocol,
    port: "",
    baudRate: 9600,
    dataBits: 8,
    stopBits: 1,
    parity: "none",
    host: "192.168.1.1",
    port: 502,
    slaveId: settings.defaultSlaveId,
    timeout: settings.defaultTimeout,
  });

  const handleConnect = async () => {
    await connect(config);
  };

  const handleDisconnect = async () => {
    await disconnect();
  };

  const isSerial = config.protocol === "RTU" || config.protocol === "ASCII";
  const isConnected = status === "connected";
  const isConnecting = status === "connecting";

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Plug className="h-5 w-5" />
          Connection Settings
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Protocol Selection */}
        <div className="space-y-2">
          <Label>Protocol</Label>
          <Select
            value={config.protocol}
            onValueChange={(value: ModbusProtocol) =>
              setConfig({ ...config, protocol: value })
            }
          >
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {protocols.map((p) => (
                <SelectItem key={p.value} value={p.value}>
                  {p.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Serial Settings */}
        {isSerial && (
          <>
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label>Serial Port</Label>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={refreshPorts}
                  disabled={portsLoading}
                >
                  <RefreshCw
                    className={`h-4 w-4 ${portsLoading ? "animate-spin" : ""}`}
                  />
                </Button>
              </div>
              <Select
                value={config.port}
                onValueChange={(value) => setConfig({ ...config, port: value })}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select port..." />
                </SelectTrigger>
                <SelectContent>
                  {ports.map((port) => (
                    <SelectItem key={port} value={port}>
                      {port}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label>Baud Rate</Label>
                <Select
                  value={config.baudRate?.toString()}
                  onValueChange={(value) =>
                    setConfig({ ...config, baudRate: parseInt(value) })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {baudRates.map((rate) => (
                      <SelectItem key={rate} value={rate.toString()}>
                        {rate}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Data Bits</Label>
                <Select
                  value={config.dataBits?.toString()}
                  onValueChange={(value) =>
                    setConfig({ ...config, dataBits: parseInt(value) as 5 | 6 | 7 | 8 })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {dataBitsOptions.map((bits) => (
                      <SelectItem key={bits} value={bits.toString()}>
                        {bits}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Stop Bits</Label>
                <Select
                  value={config.stopBits?.toString()}
                  onValueChange={(value) =>
                    setConfig({ ...config, stopBits: parseInt(value) as 1 | 2 })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {stopBitsOptions.map((bits) => (
                      <SelectItem key={bits} value={bits.toString()}>
                        {bits}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Parity</Label>
                <Select
                  value={config.parity}
                  onValueChange={(value) =>
                    setConfig({ ...config, parity: value as "none" | "even" | "odd" })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {parityOptions.map((p) => (
                      <SelectItem key={p} value={p}>
                        {p.charAt(0).toUpperCase() + p.slice(1)}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
          </>
        )}

        {/* TCP/UDP Settings */}
        {!isSerial && (
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Host</Label>
              <Input
                placeholder="192.168.1.1"
                value={config.host || ""}
                onChange={(e) => setConfig({ ...config, host: e.target.value })}
              />
            </div>
            <div className="space-y-2">
              <Label>Port</Label>
              <Input
                type="number"
                placeholder="502"
                value={config.port || ""}
                onChange={(e) =>
                  setConfig({ ...config, port: parseInt(e.target.value) })
                }
              />
            </div>
          </div>
        )}

        {/* Common Settings */}
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label>Slave ID</Label>
            <Input
              type="number"
              min={1}
              max={255}
              value={config.slaveId}
              onChange={(e) =>
                setConfig({ ...config, slaveId: parseInt(e.target.value) })
              }
            />
          </div>
          <div className="space-y-2">
            <Label>Timeout (ms)</Label>
            <Input
              type="number"
              min={100}
              max={30000}
              value={config.timeout}
              onChange={(e) =>
                setConfig({ ...config, timeout: parseInt(e.target.value) })
              }
            />
          </div>
        </div>

        {/* Error Message */}
        {error && (
          <div className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
            {error}
          </div>
        )}

        {/* Connect/Disconnect Button */}
        <div className="flex gap-2">
          {!isConnected ? (
            <Button
              className="flex-1"
              onClick={handleConnect}
              disabled={isConnecting}
            >
              <PlugZap className="mr-2 h-4 w-4" />
              {isConnecting ? "Connecting..." : "Connect"}
            </Button>
          ) : (
            <Button
              variant="destructive"
              className="flex-1"
              onClick={handleDisconnect}
            >
              Disconnect
            </Button>
          )}
        </div>

        {/* Connection Status */}
        <div className="flex items-center gap-2 text-sm">
          <span
            className={`h-2 w-2 rounded-full ${
              status === "connected"
                ? "bg-green-500"
                : status === "connecting"
                ? "bg-yellow-500 animate-pulse"
                : status === "error"
                ? "bg-red-500"
                : "bg-gray-400"
            }`}
          />
          <span className="text-muted-foreground">
            {status.charAt(0).toUpperCase() + status.slice(1)}
          </span>
        </div>
      </CardContent>
    </Card>
  );
}
