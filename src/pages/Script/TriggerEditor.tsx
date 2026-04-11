/**
 * Trigger Editor Component
 * Editor for trigger conditions with tabbed interface:
 * - Compare Value tab: numeric comparison triggers
 * - Change Detection tab: BecameOn / BecameOff / Changed / StableN
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
import { Button } from "@/components/ui/button";
import type { Trigger, CompareOp } from "./data";
import { CompareOpLabels } from "./data";
import { Zap, ArrowRightLeft, Timer, Bell, BellOff, RefreshCw } from "lucide-react";
import { useState } from "react";

interface TriggerEditorProps {
  trigger: Trigger;
  onChange: (trigger: Trigger) => void;
  disabled?: boolean;
}

/** Top-level trigger categories */
type TriggerCategory = "compare" | "change";

/** Change detection sub-types */
type ChangeType = "became_on" | "became_off" | "changed" | "stable";

export function TriggerEditor({ trigger, onChange, disabled }: TriggerEditorProps) {
  // Determine which top-level category the current trigger belongs to
  const isCompareTrigger = trigger.type === "compare";
  const isChangeTrigger = ["changed", "became_on", "became_off", "stable"].includes(trigger.type);

  const [category, setCategory] = useState<TriggerCategory>(
    isCompareTrigger ? "compare" : "change"
  );

  /** Called when user switches top-level category */
  function handleCategoryChange(cat: TriggerCategory) {
    setCategory(cat);
    if (cat === "compare") {
      onChange({ type: "compare", register: "40001", operator: "gt", value: 0 });
    } else {
      // Default to "changed" when switching to change detection tab
      onChange({ type: "changed", register: "40001" });
    }
  }

  /** Called when user changes the specific change-detection sub-type */
  function handleChangeType(type: ChangeType) {
    switch (type) {
      case "changed":
        onChange({ type: "changed", register: "40001" });
        break;
      case "became_on":
        onChange({ type: "became_on", register: "00001" });
        break;
      case "became_off":
        onChange({ type: "became_off", register: "00001" });
        break;
      case "stable":
        onChange({ type: "stable", register: "40001", seconds: 5 });
        break;
    }
  }

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
        {/* Category Tabs */}
        <div className="flex gap-1 p-1 bg-muted rounded-lg w-fit">
          <Button
            size="sm"
            variant={category === "compare" ? "default" : "ghost"}
            onClick={() => handleCategoryChange("compare")}
            disabled={disabled}
            className="gap-1.5"
          >
            <Zap className="h-3.5 w-3.5" />
            Compare Value
          </Button>
          <Button
            size="sm"
            variant={category === "change" ? "default" : "ghost"}
            onClick={() => handleCategoryChange("change")}
            disabled={disabled}
            className="gap-1.5"
          >
            <RefreshCw className="h-3.5 w-3.5" />
            Change Detection
          </Button>
        </div>

        {/* Change Detection sub-options (shown when category === 'change') */}
        {category === "change" && (
          <div className="animate-in fade-in slide-in-from-top-2 duration-200">
            <label className="text-sm font-medium leading-none mb-2 block">
              Detection Mode
            </label>
            <div className="grid grid-cols-2 gap-2">
              <ChangeTypeButton
                active={trigger.type === "became_on"}
                onClick={() => handleChangeType("became_on")}
                disabled={disabled}
                title="Rising edge (0 → 1)"
              >
                <Bell className="h-4 w-4 mr-1.5" />
                Became ON
              </ChangeTypeButton>
              <ChangeTypeButton
                active={trigger.type === "became_off"}
                onClick={() => handleChangeType("became_off")}
                disabled={disabled}
                title="Falling edge (1 → 0)"
              >
                <BellOff className="h-4 w-4 mr-1.5" />
                Became OFF
              </ChangeTypeButton>
              <ChangeTypeButton
                active={trigger.type === "changed"}
                onClick={() => handleChangeType("changed")}
                disabled={disabled}
                title="Any value change"
              >
                <ArrowRightLeft className="h-4 w-4 mr-1.5" />
                Changed
              </ChangeTypeButton>
              <ChangeTypeButton
                active={trigger.type === "stable"}
                onClick={() => handleChangeType("stable")}
                disabled={disabled}
                title="Value stable for N periods"
              >
                <Timer className="h-4 w-4 mr-1.5" />
                Stable N
              </ChangeTypeButton>
            </div>
          </div>
        )}

        {/* Register Address */}
        <div className="space-y-2">
          <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
            Register Address
          </label>
          <Input
            value={trigger.register}
            onChange={(e) => {
              onChange({ ...trigger, register: e.target.value } as Trigger);
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
        {category === "compare" && (
          <div className="grid gap-4 sm:grid-cols-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Operator
              </label>
              <Select
                value={trigger.type === "compare" ? trigger.operator : "gt"}
                onValueChange={(op: CompareOp) => {
                  onChange({ type: "compare", register: trigger.register, operator: op, value: trigger.type === "compare" ? trigger.value : 0 });
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
                value={trigger.type === "compare" ? trigger.value : 0}
                onChange={(e) => {
                  if (trigger.type === "compare") {
                    onChange({ ...trigger, value: parseInt(e.target.value) || 0 });
                  }
                }}
                disabled={disabled}
                className="transition-colors focus:ring-2 focus:ring-primary/20"
              />
            </div>
          </div>
        )}

        {/* Stable-specific: period input */}
        {trigger.type === "stable" && (
          <div className="space-y-2 animate-in fade-in slide-in-from-top-2 duration-200">
            <label className="text-sm font-medium leading-none">
              Stable Periods (N)
            </label>
            <Input
              type="number"
              value={trigger.seconds}
              onChange={(e) => {
                if (trigger.type === "stable") {
                  onChange({ ...trigger, seconds: parseInt(e.target.value) || 1 });
                }
              }}
              disabled={disabled}
              min={1}
              className="transition-colors focus:ring-2 focus:ring-primary/20"
            />
            <p className="text-xs text-muted-foreground">
              Trigger fires after the value remains unchanged for N polling cycles
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

/** Small button for change-detection sub-type selection */
function ChangeTypeButton({
  active,
  onClick,
  disabled,
  children,
  title,
}: {
  active: boolean;
  onClick: () => void;
  disabled?: boolean;
  title?: string;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled}
      title={title}
      className={`
        flex items-center justify-center px-3 py-2 rounded-md text-sm font-medium
        transition-all duration-150 border
        ${active
          ? "bg-primary text-primary-foreground border-primary shadow-sm"
          : "bg-background text-muted-foreground border-border hover:border-primary/40 hover:text-foreground"
        }
        disabled:opacity-50 disabled:cursor-not-allowed
      `}
    >
      {children}
    </button>
  );
}
