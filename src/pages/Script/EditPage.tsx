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
  const [hasChanges, setHasChanges] = useState(false);

  // Sync with store when script changes
  useEffect(() => {
    if (script) {
      setName(script.name);
      setDescription(script.description);
      setEnabled(script.enabled);
      setTrigger(script.trigger);
      setActions(script.actions);
      setHasChanges(false);
    }
  }, [script]);

  const handleSave = async () => {
    if (!script) return;
    setSaving(true);
    try {
      const updated: Script = {
        ...script,
        name,
        description,
        enabled,
        trigger,
        actions,
        updated_at: new Date().toISOString(),
      };
      await updateScript(updated);
      setHasChanges(false);
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
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="icon" onClick={() => navigate("/scripts")}>
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div>
            <h2 className="text-xl font-semibold">Edit Script</h2>
            <p className="text-sm text-muted-foreground">
              {hasChanges ? "Unsaved changes" : "All changes saved"}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            onClick={handleSave}
            disabled={!hasChanges || saving}
          >
            <Save className="h-4 w-4 mr-2" />
            {saving ? "Saving..." : "Save"}
          </Button>
        </div>
      </div>

      {/* Script Info */}
      <Card>
        <CardHeader>
          <CardTitle>Script Settings</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="name">Script Name</Label>
              <Input
                id="name"
                value={name}
                onChange={(e) => {
                  setName(e.target.value);
                  setHasChanges(true);
                }}
              />
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
                <Label htmlFor="enabled">Enabled</Label>
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
      <div className="rounded-lg border-2 border-dashed border-primary/20 bg-primary/5 p-4">
        <TriggerEditor
          trigger={trigger}
          onChange={(t) => {
            setTrigger(t);
            setHasChanges(true);
          }}
          disabled={saving}
        />
      </div>

      {/* Arrow indicator */}
      <div className="flex justify-center">
        <div className="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-primary-foreground">
          <ListFilter className="h-4 w-4" />
        </div>
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
          <div className="space-y-4">
            {actions.map((action, index) => (
              <div key={index} className="flex items-start gap-4">
                {/* Connector line */}
                {index > 0 && (
                  <div className="flex w-8 flex-col items-center">
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
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Test Button */}
      <Card className="border-dashed">
        <CardContent className="flex items-center justify-center gap-4 p-4">
          <Button variant="outline" disabled={saving}>
            <Play className="h-4 w-4 mr-2" />
            Test Script
          </Button>
          <p className="text-sm text-muted-foreground">
            Test the trigger condition without executing actions
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
