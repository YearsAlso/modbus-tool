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
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts";
import { Plus, Trash2, Play, Pause } from "lucide-react";
import { useMonitorStore, useConnectionStore } from "@/stores";
import { useModbus } from "@/hooks";
import type { MonitoredRegister, DataFormat } from "@/types";

const dataFormats: { value: DataFormat; label: string }[] = [
  { value: "UINT16", label: "UINT16 (Unsigned 16-bit)" },
  { value: "INT16", label: "INT16 (Signed 16-bit)" },
  { value: "UINT32", label: "UINT32 (Unsigned 32-bit)" },
  { value: "INT32", label: "INT32 (Signed 32-bit)" },
  { value: "FLOAT32", label: "FLOAT32 (32-bit Float)" },
  { value: "BCD", label: "BCD (Binary Coded Decimal)" },
  { value: "HEX", label: "HEX (Hexadecimal)" },
  { value: "STRING", label: "STRING (ASCII String)" },
];

const colors = ["#3b82f6", "#ef4444", "#22c55e", "#f59e0b", "#8b5cf6", "#ec4899"];

export function MonitorPage() {
  const { status } = useConnectionStore();
  const { points, addPoint, removePoint, clearAll, isMonitoring, setMonitoring, interval } = useMonitorStore();
  const { readRegisters } = useModbus();

  const [newRegister, setNewRegister] = useState<Partial<MonitoredRegister>>({
    address: 0,
    name: "",
    format: "UINT16",
    color: colors[0],
  });

  const handleAddPoint = () => {
    if (!newRegister.address || !newRegister.name) return;

    const register: MonitoredRegister = {
      address: newRegister.address as number,
      name: newRegister.name as string,
      format: newRegister.format as DataFormat,
      color: newRegister.color as string,
    };

    addPoint(register);
    setNewRegister({
      address: 0,
      name: "",
      format: "UINT16",
      color: colors[(points.length + 1) % colors.length],
    });
  };

  const handleStartMonitoring = async () => {
    if (status !== "connected") return;
    setMonitoring(true);

    while (isMonitoring && status === "connected") {
      for (const point of points) {
        try {
          const response = await readRegisters(0x03, point.register.address, 1);
          if (response.success && response.data) {
            useMonitorStore.getState().updatePointValue(point.id, response.data[0]);
          }
        } catch (err) {
          console.error("Monitor read error:", err);
        }
      }
      await new Promise((resolve) => setTimeout(resolve, interval));
    }
  };

  const handleStopMonitoring = () => {
    setMonitoring(false);
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Add Monitor Point</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-5 gap-4">
            <div className="space-y-2">
              <Label>Address</Label>
              <Input
                type="number"
                min={0}
                max={65535}
                placeholder="0"
                value={newRegister.address || ""}
                onChange={(e) =>
                  setNewRegister({ ...newRegister, address: parseInt(e.target.value) })
                }
              />
            </div>
            <div className="space-y-2">
              <Label>Name</Label>
              <Input
                placeholder="Register name"
                value={newRegister.name || ""}
                onChange={(e) =>
                  setNewRegister({ ...newRegister, name: e.target.value })
                }
              />
            </div>
            <div className="space-y-2">
              <Label>Format</Label>
              <Select
                value={newRegister.format}
                onValueChange={(value: DataFormat) =>
                  setNewRegister({ ...newRegister, format: value })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {dataFormats.map((f) => (
                    <SelectItem key={f.value} value={f.value}>
                      {f.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label>Color</Label>
              <Select
                value={newRegister.color}
                onValueChange={(value) =>
                  setNewRegister({ ...newRegister, color: value })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {colors.map((c) => (
                    <SelectItem key={c} value={c}>
                      <div className="flex items-center gap-2">
                        <div
                          className="h-4 w-4 rounded"
                          style={{ backgroundColor: c }}
                        />
                        {c}
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="flex items-end">
              <Button onClick={handleAddPoint} disabled={status !== "connected"}>
                <Plus className="mr-2 h-4 w-4" />
                Add
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <span>Monitor Points ({points.length})</span>
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={handleStartMonitoring}
                disabled={status !== "connected" || points.length === 0 || isMonitoring}
              >
                <Play className="mr-2 h-4 w-4" />
                Start
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={handleStopMonitoring}
                disabled={!isMonitoring}
              >
                <Pause className="mr-2 h-4 w-4" />
                Stop
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={clearAll}
                disabled={points.length === 0}
              >
                <Trash2 className="mr-2 h-4 w-4" />
                Clear All
              </Button>
            </div>
          </CardTitle>
        </CardHeader>
        <CardContent>
          {points.length === 0 ? (
            <p className="text-center text-muted-foreground py-8">
              No monitor points added yet. Add a point above to start monitoring.
            </p>
          ) : (
            <div className="space-y-4">
              <div className="h-64 w-full">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="timestamp" tickFormatter={(v) => new Date(v).toLocaleTimeString()} />
                    <YAxis />
                    <Tooltip />
                    {points.map((point) => (
                      <Line
                        key={point.id}
                        type="monotone"
                        data={point.history}
                        dataKey="value"
                        stroke={point.register.color}
                        name={point.register.name}
                      />
                    ))}
                  </LineChart>
                </ResponsiveContainer>
              </div>

              <div className="border rounded-lg">
                <table className="w-full">
                  <thead className="bg-muted">
                    <tr>
                      <th className="px-4 py-2 text-left">Name</th>
                      <th className="px-4 py-2 text-left">Address</th>
                      <th className="px-4 py-2 text-left">Format</th>
                      <th className="px-4 py-2 text-left">Value</th>
                      <th className="px-4 py-2 text-left">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {points.map((point) => (
                      <tr key={point.id} className="border-t">
                        <td className="px-4 py-2">
                          <div className="flex items-center gap-2">
                            <div
                              className="h-3 w-3 rounded-full"
                              style={{ backgroundColor: point.register.color }}
                            />
                            {point.register.name}
                          </div>
                        </td>
                        <td className="px-4 py-2">{point.register.address}</td>
                        <td className="px-4 py-2">{point.register.format}</td>
                        <td className="px-4 py-2 font-mono">
                          {point.value !== null ? point.value.toString() : "-"}
                        </td>
                        <td className="px-4 py-2">
                          <Button
                            variant="ghost"
                            size="icon"
                            onClick={() => removePoint(point.id)}
                          >
                            <Trash2 className="h-4 w-4" />
                          </Button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
