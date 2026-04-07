/**
 * Action Editor Component
 * Editor for automation actions (write, notify, log, etc.)
 */

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { Action, SoundType } from "./data";
import {
  Trash2,
  Zap,
  Bell,
  Volume2,
  FileText,
  Timer,
  Power,
  ToggleRight,
  ListFilter,
} from "lucide-react";

interface ActionEditorProps {
  action: Action;
  index: number;
  onChange: (action: Action) => void;
  onRemove: () => void;
  disabled?: boolean;
}

const actionTypeMeta: Record<
  Action["type"],
  { label: string; icon: React.ReactNode; color: string }
> = {
  write_value: { label: "Write Value", icon: <Zap className="h-3.5 w-3.5" />, color: "text-blue-600 dark:text-blue-400" },
  write_on: { label: "Turn ON", icon: <Power className="h-3.5 w-3.5" />, color: "text-green-600 dark:text-green-400" },
  write_off: { label: "Turn OFF", icon: <Power className="h-3.5 w-3.5" />, color: "text-orange-600 dark:text-orange-400" },
  toggle: { label: "Toggle", icon: <ToggleRight className="h-3.5 w-3.5" />, color: "text-purple-600 dark:text-purple-400" },
  show_notification: { label: "Notification", icon: <Bell className="h-3.5 w-3.5" />, color: "text-yellow-600 dark:text-yellow-400" },
  play_sound: { label: "Play Sound", icon: <Volume2 className="h-3.5 w-3.5" />, color: "text-pink-600 dark:text-pink-400" },
  log: { label: "Log Message", icon: <FileText className="h-3.5 w-3.5" />, color: "text-muted-foreground" },
  delay: { label: "Delay", icon: <Timer className="h-3.5 w-3.5" />, color: "text-cyan-600 dark:text-cyan-400" },
  run_script: { label: "Run Script", icon: <Zap className="h-3.5 w-3.5" />, color: "text-indigo-600 dark:text-indigo-400" },
  stop_script: { label: "Stop Script", icon: <Zap className="h-3.5 w-3.5" />, color: "text-red-600 dark:text-red-400" },
};

const actionTypes = [
  { value: "write_value", label: "Write Value" },
  { value: "write_on", label: "Turn ON" },
  { value: "write_off", label: "Turn OFF" },
  { value: "toggle", label: "Toggle" },
  { value: "show_notification", label: "Show Notification" },
  { value: "play_sound", label: "Play Sound" },
  { value: "log", label: "Log Message" },
  { value: "delay", label: "Delay" },
] as const;

export function ActionEditor({ action, index, onChange, onRemove, disabled }: ActionEditorProps) {
  const meta = actionTypeMeta[action.type] ?? { label: action.type, icon: <ListFilter className="h-3.5 w-3.5" />, color: "text-muted-foreground" };

  return (
    <Card className="w-full overflow-hidden transition-shadow hover:shadow-md">
      <CardHeader className="pb-3 bg-gradient-to-r from-muted/30 to-transparent border-b dark:border-border/50">
        <CardTitle className="flex items-center justify-between text-base">
          <span className="flex items-center gap-2">
            <div className="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-muted text-sm font-semibold">
              {index + 1}
            </div>
            <span className={`flex items-center gap-1.5 ${meta.color}`}>
              {meta.icon}
              <span className="text-sm">{meta.label}</span>
            </span>
          </span>
          <Button
            variant="ghost"
            size="sm"
            onClick={onRemove}
            disabled={disabled}
            className="text-muted-foreground/60 hover:text-destructive transition-colors h-8 px-2"
          >
            <Trash2 className="h-4 w-4" />
          </Button>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4 pt-4">
        {/* Action Type */}
        <div className="space-y-2">
          <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
            Action Type
          </label>
          <Select
            value={action.type}
            onValueChange={(type: Action["type"]) => {
              let newAction: Action;
              switch (type) {
                case "write_value":
                  newAction = { type: "write_value", register: "40001", value: 0 };
                  break;
                case "write_on":
                  newAction = { type: "write_on", register: "00001" };
                  break;
                case "write_off":
                  newAction = { type: "write_off", register: "00001" };
                  break;
                case "toggle":
                  newAction = { type: "toggle", register: "00001" };
                  break;
                case "show_notification":
                  newAction = { type: "show_notification", title: "Alert", message: "" };
                  break;
                case "play_sound":
                  newAction = { type: "play_sound", sound: "alert" };
                  break;
                case "log":
                  newAction = { type: "log", message: "" };
                  break;
                case "delay":
                  newAction = { type: "delay", seconds: 1 };
                  break;
                default:
                  return;
              }
              onChange(newAction);
            }}
            disabled={disabled}
          >
            <SelectTrigger className="w-full transition-colors focus:ring-2 focus:ring-primary/20">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {actionTypes.map((t) => (
                <SelectItem key={t.value} value={t.value}>
                  {actionTypeMeta[t.value]?.icon}{" "}
                  <span className="ml-1.5">{t.label}</span>
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Register-based actions */}
        {(action.type === "write_value" || action.type === "write_on" || action.type === "write_off" || action.type === "toggle") && (
          <div className="space-y-2">
            <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Register Address
            </label>
            <Input
              value={(action as Action & { register: string }).register}
              onChange={(e) => {
                const reg = (action as Action & { register: string });
                onChange({ ...reg, register: e.target.value } as Action);
              }}
              placeholder="e.g. 40001, 00001"
              disabled={disabled}
              className="font-mono transition-colors focus:ring-2 focus:ring-primary/20"
            />
          </div>
        )}

        {/* Write Value specific */}
        {action.type === "write_value" && (
          <div className="space-y-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Value
            </label>
            <Input
              type="number"
              value={action.value}
              onChange={(e) => {
                onChange({ ...action, value: parseInt(e.target.value) || 0 });
              }}
              disabled={disabled}
              className="transition-colors focus:ring-2 focus:ring-primary/20"
            />
          </div>
        )}

        {/* Show Notification specific */}
        {action.type === "show_notification" && (
          <div className="space-y-3 animate-in fade-in slide-in-from-top-2 duration-200">
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Title
              </label>
              <Input
                value={action.title}
                onChange={(e) => {
                  onChange({ ...action, title: e.target.value });
                }}
                placeholder="Alert title"
                disabled={disabled}
                className="transition-colors focus:ring-2 focus:ring-primary/20"
              />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Message
              </label>
              <Input
                value={action.message}
                onChange={(e) => {
                  onChange({ ...action, message: e.target.value });
                }}
                placeholder="Notification message"
                disabled={disabled}
                className="transition-colors focus:ring-2 focus:ring-primary/20"
              />
            </div>
          </div>
        )}

        {/* Play Sound specific */}
        {action.type === "play_sound" && (
          <div className="space-y-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Sound Type
            </label>
            <Select
              value={action.sound}
              onValueChange={(sound: SoundType) => {
                onChange({ ...action, sound });
              }}
              disabled={disabled}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="alert">🔔 Alert</SelectItem>
                <SelectItem value="success">✅ Success</SelectItem>
                <SelectItem value="warning">⚠️ Warning</SelectItem>
              </SelectContent>
            </Select>
          </div>
        )}

        {/* Log Message specific */}
        {action.type === "log" && (
          <div className="space-y-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Message
            </label>
            <Input
              value={action.message}
              onChange={(e) => {
                onChange({ ...action, message: e.target.value });
              }}
              placeholder="Message to log"
              disabled={disabled}
              className="transition-colors focus:ring-2 focus:ring-primary/20"
            />
          </div>
        )}

        {/* Delay specific */}
        {action.type === "delay" && (
          <div className="space-y-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Seconds
            </label>
            <Input
              type="number"
              value={action.seconds}
              onChange={(e) => {
                onChange({ ...action, seconds: parseInt(e.target.value) || 0 });
              }}
              disabled={disabled}
              min={0.1}
              step={0.5}
              className="transition-colors focus:ring-2 focus:ring-primary/20"
            />
          </div>
        )}
      </CardContent>
    </Card>
  );
}
