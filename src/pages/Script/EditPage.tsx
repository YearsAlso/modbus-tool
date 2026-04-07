/**
 * Script Edit Page
 * Block-style editor with trigger + action list
 */

import { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { useScriptStore } from "./store";
import { TriggerEditor } from "./TriggerEditor";
import { ActionEditor } from "./ActionEditor";
import type { Script, Action } from "./data";
import {
  ArrowLeft,
  Save,
  Play,
  Plus,
  ListFilter,
  AlertCircle,
  CheckCircle2,
  Loader2,
  ArrowDown,
} from "lucide-react";

export function ScriptEditPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { scripts, updateScript } = useScriptStore();

  const script = scripts.find((s) => s.id === id);

  const [name, setName] = useState(script?.name ?? "New Script");
  const [description, setDescription] = useState(script?.description ?? "");
  const [enabled, setEnabled] = useState(script?.enabled ?? true);
  const [trigger, setTrigger] = useState(script?.trigger ?? {
    type: "changed" as const,
    register: "40001",
  });
  const [actions, setActions] = useState<Action[]>(script?.actions ?? []);
  const [saving, setSaving] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);
  const [nameError, setNameError] = useState("");

  // Sync with store when script changes
  useEffect(() => {
    if (script) {
      setName(script.name);
      setDescription(script.description);
      setEnabled(script.enabled);
      setTrigger(script.trigger);
      setActions(script.actions);
      setHasChanges(false);
      setNameError("");
    }
  }, [script]);

  const validateForm = () => {
    if (!name.trim()) {
      setNameError("Script name is required");
      return false;
    }
    if (name.trim().length > 100) {
      setNameError("Script name must be less than 100 characters");
      return false;
    }
    setNameError("");
    return true;
  };

  const handleNameChange = (value: string) => {
    setName(value);
    setHasChanges(true);
    if (nameError) validateForm();
  };

  const handleSave = async () => {
    if (!script || !validateForm()) return;
    setSaving(true);
    setSaveSuccess(false);
    try {
      const updated: Script = {
        ...script,
        name: name.trim(),
        description,
        enabled,
        trigger,
        actions,
        updated_at: new Date().toISOString(),
      };
      await updateScript(updated);
      setHasChanges(false);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 2500);
    } finally {
      setSaving(false);
    }
  };

  const handleAddAction = () => {
    const newAction: Action = {
      type: "log",
      message: "New action",
    };
    setActions([...actions, newAction]);
    setHasChanges(true);
  };

  const handleUpdateAction = (index: number, action: Action) => {
    const updated = [...actions];
    updated[index] = action;
    setActions(updated);
    setHasChanges(true);
  };

  const handleRemoveAction = (index: number) => {
    setActions(actions.filter((_, i) => i !== index));
    setHasChanges(true);
  };

  if (!script) {
    return (
      <div className="flex h-full items-center justify-center">
        <Card className="w-full max-w-md">
          <CardContent className="flex flex-col items-center gap-4 p-8">
            <AlertCircle className="h-12 w-12 text-muted-foreground" />
            <p className="text-lg font-medium">Script not found</p>
            <Button variant="outline" onClick={() => navigate("/scripts")}>
              <ArrowLeft className="h-4 w-4 mr-2" />
              Back to Scripts
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="space-y-6 max-w-3xl mx-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Button variant="ghost" size="icon" onClick={() => navigate("/scripts")}>
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div>
            <h2 className="text-xl font-semibold">Edit Script</h2>
            <div className="flex items-center gap-1.5 text-sm text-muted-foreground min-h-[20px]">
              <AnimatePresence mode="wait" initial={false}>
                {saving ? (
                  <motion.span
                    key="saving"
                    initial={{ opacity: 0, y: -4 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 4 }}
                    className="flex items-center gap-1 text-primary"
                  >
                    <Loader2 className="h-3 w-3 animate-spin" /> Saving...
                  </motion.span>
                ) : saveSuccess ? (
                  <motion.span
                    key="success"
                    initial={{ opacity: 0, y: -4 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 4 }}
                    className="flex items-center gap-1 text-green-600 dark:text-green-400"
                  >
                    <CheckCircle2 className="h-3 w-3" /> Saved
                  </motion.span>
                ) : hasChanges ? (
                  <motion.span
                    key="unsaved"
                    initial={{ opacity: 0, y: -4 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 4 }}
                    className="flex items-center gap-1"
                  >
                    <span className="h-1.5 w-1.5 rounded-full bg-yellow-500" /> Unsaved changes
                  </motion.span>
                ) : (
                  <motion.span
                    key="saved"
                    initial={{ opacity: 0, y: -4 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 4 }}
                    className="flex items-center gap-1"
                  >
                    <CheckCircle2 className="h-3 w-3 text-green-500" /> All changes saved
                  </motion.span>
                )}
              </AnimatePresence>
            </div>
          </div>
        </div>
        <Button
          variant="outline"
          onClick={handleSave}
          disabled={!hasChanges || saving || !name.trim()}
          className="transition-all"
        >
          <Save className="h-4 w-4 mr-2" />
          Save
        </Button>
      </div>

      {/* Script Info */}
      <Card>
        <CardHeader>
          <CardTitle>Script Settings</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="name" className={nameError ? "text-destructive" : ""}>
                Script Name
              </Label>
              <Input
                id="name"
                value={name}
                onChange={(e) => handleNameChange(e.target.value)}
                onBlur={validateForm}
                placeholder="My Automation Script"
                className={`transition-colors ${nameError ? "border-destructive focus-visible:ring-destructive" : ""}`}
              />
              {nameError && (
                <p className="text-xs text-destructive flex items-center gap-1">
                  <AlertCircle className="h-3 w-3" />
                  {nameError}
                </p>
              )}
            </div>
            <div className="flex items-center gap-4">
              <div className="flex items-center gap-2">
                <Switch
                  id="enabled"
                  checked={enabled}
                  onCheckedChange={(checked) => {
                    setEnabled(checked);
                    setHasChanges(true);
                  }}
                />
                <Label htmlFor="enabled">{enabled ? "Enabled" : "Disabled"}</Label>
              </div>
            </div>
          </div>
          <div className="space-y-2">
            <Label htmlFor="description">Description (optional)</Label>
            <Input
              id="description"
              value={description}
              onChange={(e) => {
                setDescription(e.target.value);
                setHasChanges(true);
              }}
              placeholder="What does this script do?"
            />
          </div>
        </CardContent>
      </Card>

      {/* Trigger Block */}
      <motion.div
        initial={{ opacity: 0, y: 8 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.3 }}
        className="rounded-xl border-2 border-dashed border-primary/20 bg-primary/[0.03] dark:bg-primary/5 p-1"
      >
        <TriggerEditor
          trigger={trigger}
          onChange={(t) => {
            setTrigger(t);
            setHasChanges(true);
          }}
          disabled={saving}
        />
      </motion.div>

      {/* Arrow indicator */}
      <div className="flex justify-center">
        <motion.div
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          transition={{ duration: 0.3, delay: 0.1 }}
          className="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-sm"
        >
          <ArrowDown className="h-4 w-4" />
        </motion.div>
      </div>

      {/* Actions List */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="text-lg font-medium">Actions ({actions.length})</h3>
          <Button variant="outline" size="sm" onClick={handleAddAction} disabled={saving}>
            <Plus className="h-4 w-4 mr-2" />
            Add Action
          </Button>
        </div>

        {actions.length === 0 ? (
          <Card className="border-dashed">
            <CardContent className="flex flex-col items-center justify-center gap-2 p-8 text-muted-foreground">
              <ListFilter className="h-8 w-8" />
              <p>No actions yet</p>
              <p className="text-sm">Add actions to execute when the trigger fires</p>
            </CardContent>
          </Card>
        ) : (
          <AnimatePresence initial={false}>
            <div className="space-y-4">
              {actions.map((action, index) => (
                <motion.div
                  key={`${action.type}-${index}`}
                  initial={{ opacity: 0, height: 0, scale: 0.95 }}
                  animate={{ opacity: 1, height: "auto", scale: 1 }}
                  exit={{ opacity: 0, height: 0, scale: 0.95 }}
                  transition={{ duration: 0.2, ease: "easeInOut" }}
                  className="flex items-start gap-4"
                >
                  {/* Connector line */}
                  {index > 0 && (
                    <div className="flex w-8 flex-col items-center pt-2">
                      <div className="h-4 w-0.5 bg-border" />
                      <div className="h-4 w-0.5 bg-border" />
                    </div>
                  )}
                  <div className="flex-1">
                    <ActionEditor
                      action={action}
                      index={index}
                      onChange={(a) => handleUpdateAction(index, a)}
                      onRemove={() => handleRemoveAction(index)}
                      disabled={saving}
                    />
                  </div>
                </motion.div>
              ))}
            </div>
          </AnimatePresence>
        )}
      </div>

      {/* Test Button */}
      <Card className="border-dashed bg-muted/30 dark:bg-muted/10">
        <CardContent className="flex flex-col sm:flex-row items-center justify-center gap-3 p-4">
          <Button
            variant="outline"
            disabled={saving || !name.trim()}
            className="w-full sm:w-auto"
          >
            <Play className="h-4 w-4 mr-2" />
            Test Script
          </Button>
          <p className="text-sm text-muted-foreground text-center">
            Test the trigger condition without executing actions
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
