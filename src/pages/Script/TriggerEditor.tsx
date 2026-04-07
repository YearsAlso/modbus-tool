/**
 * Trigger Editor Component
 * Editor for trigger conditions (register + operator + value)
 */

import { useState, useEffect } from "react";
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
    <Card className="w-full">
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2 text-base">
          <Zap className="h-4 w-4 text-yellow-500" />
          Trigger Condition
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Trigger Type */}
        <div className="space-y-2">
          <label className="text-sm font-medium">Trigger Type</label>
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
            <SelectTrigger>
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
          <label className="text-sm font-medium">Register Address</label>
          <Input
            value={trigger.register}
            onChange={(e) => {
              onChange({ ...trigger, register: e.target.value });
            }}
            placeholder="e.g. 40001, 00001"
            disabled={disabled}
          />
          <p className="text-xs text-muted-foreground">
            Use format: 0XXXX for coils, 1XXXX for discrete inputs, 4XXXX for holding registers, 3XXXX for input registers
          </p>
        </div>

        {/* Compare-specific fields */}
        {trigger.type === "compare" && (
          <>
            <div className="space-y-2">
              <label className="text-sm font-medium">Operator</label>
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
                      {CompareOpLabels[op].symbol} - {CompareOpLabels[op].label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Value</label>
              <Input
                type="number"
                value={trigger.value}
                onChange={(e) => {
                  onChange({ ...trigger, value: parseInt(e.target.value) || 0 });
                }}
                disabled={disabled}
              />
            </div>
          </>
        )}

        {/* Stable-specific fields */}
        {trigger.type === "stable" && (
          <div className="space-y-2">
            <label className="text-sm font-medium">Seconds</label>
            <Input
              type="number"
              value={trigger.seconds}
              onChange={(e) => {
                onChange({ ...trigger, seconds: parseInt(e.target.value) || 0 });
              }}
              disabled={disabled}
            />
          </div>
        )}
      </CardContent>
    </Card>
  );
}
