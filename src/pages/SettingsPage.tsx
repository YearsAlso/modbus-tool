import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Moon, Sun, Monitor, Save, RotateCcw } from "lucide-react";
import { useSettingsStore } from "@/stores";
import { useTheme } from "@/hooks";

export function SettingsPage() {
  const { settings, updateSettings, resetSettings } = useSettingsStore();
  useTheme();

  const handleSave = () => {
    // Settings are auto-persisted via zustand/persist
    // This is just for showing feedback
    console.log("Settings saved:", settings);
  };

  return (
    <div className="mx-auto max-w-2xl space-y-6">
      {/* Appearance */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Sun className="h-5 w-5" />
            Appearance
          </CardTitle>
          <CardDescription>
            Customize the look and feel of the application
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Theme</Label>
            <Select
              value={settings.theme}
              onValueChange={(value: "light" | "dark" | "system") =>
                updateSettings({ theme: value })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="light">
                  <div className="flex items-center gap-2">
                    <Sun className="h-4 w-4" />
                    Light
                  </div>
                </SelectItem>
                <SelectItem value="dark">
                  <div className="flex items-center gap-2">
                    <Moon className="h-4 w-4" />
                    Dark
                  </div>
                </SelectItem>
                <SelectItem value="system">
                  <div className="flex items-center gap-2">
                    <Monitor className="h-4 w-4" />
                    System
                  </div>
                </SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label>Language</Label>
            <Select
              value={settings.language}
              onValueChange={(value: "en" | "zh-CN") =>
                updateSettings({ language: value })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="en">English</SelectItem>
                <SelectItem value="zh-CN">简体中文</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Connection */}
      <Card>
        <CardHeader>
          <CardTitle>Connection</CardTitle>
          <CardDescription>
            Default connection settings for new sessions
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Default Protocol</Label>
              <Select
                value={settings.defaultProtocol}
                onValueChange={(value: "RTU" | "ASCII" | "TCP" | "UDP") =>
                  updateSettings({ defaultProtocol: value })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="RTU">Modbus RTU</SelectItem>
                  <SelectItem value="ASCII">Modbus ASCII</SelectItem>
                  <SelectItem value="TCP">Modbus TCP</SelectItem>
                  <SelectItem value="UDP">Modbus UDP</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Default Slave ID</Label>
              <Input
                type="number"
                min={1}
                max={255}
                value={settings.defaultSlaveId}
                onChange={(e) =>
                  updateSettings({ defaultSlaveId: parseInt(e.target.value) })
                }
              />
            </div>

            <div className="space-y-2">
              <Label>Default Timeout (ms)</Label>
              <Input
                type="number"
                min={100}
                max={30000}
                value={settings.defaultTimeout}
                onChange={(e) =>
                  updateSettings({ defaultTimeout: parseInt(e.target.value) })
                }
              />
            </div>
          </div>

          <div className="space-y-2">
            <Label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={settings.autoReconnect}
                onChange={(e) =>
                  updateSettings({ autoReconnect: e.target.checked })
                }
                className="rounded"
              />
              Auto-reconnect on connection loss
            </Label>
          </div>

          {settings.autoReconnect && (
            <div className="space-y-2">
              <Label>Max Reconnect Attempts</Label>
              <Input
                type="number"
                min={1}
                max={10}
                value={settings.maxReconnectAttempts}
                onChange={(e) =>
                  updateSettings({
                    maxReconnectAttempts: parseInt(e.target.value),
                  })
                }
              />
            </div>
          )}
        </CardContent>
      </Card>

      {/* Logging */}
      <Card>
        <CardHeader>
          <CardTitle>Logging</CardTitle>
          <CardDescription>
            Control the level of detail in application logs
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Log Level</Label>
            <Select
              value={settings.logLevel}
              onValueChange={(value: "debug" | "info" | "warn" | "error") =>
                updateSettings({ logLevel: value })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="debug">Debug (Most verbose)</SelectItem>
                <SelectItem value="info">Info</SelectItem>
                <SelectItem value="warn">Warning</SelectItem>
                <SelectItem value="error">Error (Least verbose)</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Actions */}
      <div className="flex gap-4">
        <Button onClick={handleSave}>
          <Save className="mr-2 h-4 w-4" />
          Save Settings
        </Button>
        <Button variant="outline" onClick={resetSettings}>
          <RotateCcw className="mr-2 h-4 w-4" />
          Reset to Defaults
        </Button>
      </div>
    </div>
  );
}
