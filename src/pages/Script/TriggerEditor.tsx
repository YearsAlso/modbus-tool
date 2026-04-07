/**
 * Trigger Editor Component
 * Editor for trigger conditions (register + operator + value)
 */

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { Trigger, CompareOp } from "./data";
import { CompareOpLabels } from "./data";
import { Zap } from "lucide-react";

interface TriggerEditorProps {
  trigger: Trigger;
  onChange: (trigger: Trigger) => void;
  disabled?: boolean;
}

export function TriggerEditor({ trigger, onChange, disabled }: TriggerEditorProps) {
  const triggerTypes = [
    { value: "compare", label: "Compare Value" },
    { value: "changed", label: "Value Changed" },
    { value: "became_on", label: "Became ON" },
    { value: "became_off", label: "Became OFF" },
    { value: "stable", label: "Stable For" },
  ] as const;

  return (
    <Card className="w-full overflow-hidden">
      <CardHeader className="pb-3 bg-gradient-to-r from-primary/5 to-transparent border-b dark:border-border/50">
        <CardTitle className="flex items-center gap-2 text-base">
          <div className="flex h-7 w-7 items-center justify-center rounded-md bg-yellow-100 dark:bg-yellow-900/40">
            <Zap className="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
          </div>
          <span>Trigger Condition</span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4 pt-4">
        {/* Trigger Type */}
        <div className="space-y-2">
          <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
            Trigger Type
          </label>
          <Select
            value={trigger.type}
            onValueChange={(type) => {
              const t = triggerTypes.find((t) => t.value === type);
              if (!t) return;

              let newTrigger: Trigger;
              switch (type) {
                case "compare":
                  newTrigger = { type: "compare", register: "40001", operator: "gt", value: 0 };
                  break;
                case "changed":
                  newTrigger = { type: "changed", register: "40001" };
                  break;
                case "became_on":
                  newTrigger = { type: "became_on", register: "00001" };
                  break;
                case "became_off":
                  newTrigger = { type: "became_off", register: "00001" };
                  break;
                case "stable":
                  newTrigger = { type: "stable", register: "40001", seconds: 5 };
                  break;
                default:
                  return;
              }
              onChange(newTrigger);
            }}
            disabled={disabled}
          >
            <SelectTrigger className="w-full transition-colors focus:ring-2 focus:ring-primary/20">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {triggerTypes.map((t) => (
                <SelectItem key={t.value} value={t.value}>
                  {t.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Register Address */}
        <div className="space-y-2">
          <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
            Register Address
          </label>
          <Input
            value={trigger.register}
            onChange={(e) => {
              onChange({ ...trigger, register: e.target.value });
            }}
            placeholder="e.g. 40001, 00001"
            disabled={disabled}
            className="font-mono transition-colors focus:ring-2 focus:ring-primary/20"
          />
          <p className="text-xs text-muted-foreground">
            0XXXX=coils · 1XXXX=discrete inputs · 4XXXX=holding registers · 3XXXX=input registers
          </p>
        </div>

        {/* Compare-specific fields */}
        {trigger.type === "compare" && (
          <div className="grid gap-4 sm:grid-cols-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Operator
              </label>
              <Select
                value={trigger.operator}
                onValueChange={(op: CompareOp) => {
                  onChange({ ...trigger, operator: op });
                }}
                disabled={disabled}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {(Object.keys(CompareOpLabels) as CompareOp[]).map((op) => (
                    <SelectItem key={op} value={op}>
                      {CompareOpLabels[op].symbol} — {CompareOpLabels[op].label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Value
              </label>
              <Input
                type="number"
                value={trigger.value}
                onChange={(e) => {
                  onChange({ ...trigger, value: parseInt(e.target.value) || 0 });
                }}
                disabled={disabled}
                className="transition-colors focus:ring-2 focus:ring-primary/20"
              />
            </div>
          </div>
        )}

        {/* Stable-specific fields */}
        {trigger.type === "stable" && (
          <div className="space-y-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Seconds
            </label>
            <Input
              type="number"
              value={trigger.seconds}
              onChange={(e) => {
                onChange({ ...trigger, seconds: parseInt(e.target.value) || 0 });
              }}
              disabled={disabled}
              min={1}
              className="transition-colors focus:ring-2 focus:ring-primary/20"
            />
          </div>
        )}
      </CardContent>
    </Card>
  );
}
