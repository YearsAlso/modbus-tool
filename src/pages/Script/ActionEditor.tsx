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
import { Trash2 } from "lucide-react";

interface ActionEditorProps {
  action: Action;
  index: number;
  onChange: (action: Action) => void;
  onRemove: () => void;
  disabled?: boolean;
}

export function ActionEditor({ action, index, onChange, onRemove, disabled }: ActionEditorProps) {
  const actionTypes = [
    { value: "write_value", label: "Write Value", icon: "⚡" },
    { value: "write_on", label: "Turn ON", icon: "🔌" },
    { value: "write_off", label: "Turn OFF", icon: "⭕" },
    { value: "toggle", label: "Toggle", icon: "🔄" },
    { value: "show_notification", label: "Show Notification", icon: "🔔" },
    { value: "play_sound", label: "Play Sound", icon: "🔊" },
    { value: "log", label: "Log Message", icon: "📝" },
    { value: "delay", label: "Delay", icon: "⏱️" },
  ] as const;

  return (
    <Card className="w-full">
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center justify-between text-base">
          <span className="flex items-center gap-2">
            <span className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-xs text-primary-foreground">
              {index + 1}
            </span>
            Action
          </span>
          <Button
            variant="ghost"
            size="sm"
            onClick={onRemove}
            disabled={disabled}
            className="text-destructive hover:text-destructive"
          >
            <Trash2 className="h-4 w-4" />
          </Button>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Action Type */}
        <div className="space-y-2">
          <label className="text-sm font-medium">Action Type</label>
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
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {actionTypes.map((t) => (
                <SelectItem key={t.value} value={t.value}>
                  {t.icon} {t.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Register-based actions */}
        {["write_value", "write_on", "write_off", "toggle"].includes(action.type) && (
          <div className="space-y-2">
            <label className="text-sm font-medium">Register Address</label>
            <Input
              value={action.register}
              onChange={(e) => {
                onChange({ ...action, register: e.target.value } as Action);
              }}
              placeholder="e.g. 40001, 00001"
              disabled={disabled}
            />
          </div>
        )}

        {/* Write Value specific */}
        {action.type === "write_value" && (
          <div className="space-y-2">
            <label className="text-sm font-medium">Value</label>
            <Input
              type="number"
              value={action.value}
              onChange={(e) => {
                onChange({ ...action, value: parseInt(e.target.value) || 0 });
              }}
              disabled={disabled}
            />
          </div>
        )}

        {/* Show Notification specific */}
        {action.type === "show_notification" && (
          <>
            <div className="space-y-2">
              <label className="text-sm font-medium">Title</label>
              <Input
                value={action.title}
                onChange={(e) => {
                  onChange({ ...action, title: e.target.value });
                }}
                placeholder="Alert title"
                disabled={disabled}
              />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium">Message</label>
              <Input
                value={action.message}
                onChange={(e) => {
                  onChange({ ...action, message: e.target.value });
                }}
                placeholder="Notification message"
                disabled={disabled}
              />
            </div>
          </>
        )}

        {/* Play Sound specific */}
        {action.type === "play_sound" && (
          <div className="space-y-2">
            <label className="text-sm font-medium">Sound Type</label>
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
          <div className="space-y-2">
            <label className="text-sm font-medium">Message</label>
            <Input
              value={action.message}
              onChange={(e) => {
                onChange({ ...action, message: e.target.value });
              }}
              placeholder="Message to log"
              disabled={disabled}
            />
          </div>
        )}

        {/* Delay specific */}
        {action.type === "delay" && (
          <div className="space-y-2">
            <label className="text-sm font-medium">Seconds</label>
            <Input
              type="number"
              value={action.seconds}
              onChange={(e) => {
                onChange({ ...action, seconds: parseInt(e.target.value) || 0 });
              }}
              disabled={disabled}
            />
          </div>
        )}
      </CardContent>
    </Card>
  );
}
