/**
 * Script List Page
 * Main page showing all automation scripts with status and controls
 */

import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useScriptStore } from "./store";
import { getTriggerDescription, getActionDescription, type Trigger, type Action } from "./data";
import {
  Plus,
  Pencil,
  Trash2,
  Play,
  Square,
  Search,
  Loader2,
  AlertCircle,
  Zap,
  ListFilter,
} from "lucide-react";

export function ScriptListPage() {
  const navigate = useNavigate();
  const {
    scripts,
    statuses,
    loading,
    error,
    loadScripts,
    createScript,
    removeScript,
    startScript,
    stopScript,
    clearError,
  } = useScriptStore();

  const [searchQuery, setSearchQuery] = useState("");
  const [filterStatus, setFilterStatus] = useState<"all" | "running" | "stopped">("all");
  const [creating, setCreating] = useState(false);

  useEffect(() => {
    loadScripts();
  }, [loadScripts]);

  const handleCreateScript = async () => {
    setCreating(true);
    try {
      const trigger: Trigger = { type: "changed", register: "40001" };
      const script = await createScript("New Script", trigger);
      navigate(`/scripts/${script.id}`);
    } finally {
      setCreating(false);
    }
  };

  const handleDelete = async (id: string, name: string) => {
    if (!confirm(`Delete "${name}"?`)) return;
    await removeScript(id);
  };

  const handleToggleRunning = async (id: string, currentlyRunning: boolean) => {
    if (currentlyRunning) {
      await stopScript(id);
    } else {
      await startScript(id);
    }
  };

  // Filter scripts
  const filteredScripts = scripts.filter((script) => {
    const matchesSearch = script.name.toLowerCase().includes(searchQuery.toLowerCase());
    const status = statuses[script.id];
    const isRunning = status?.running ?? false;
    
    let matchesFilter = true;
    if (filterStatus === "running") matchesFilter = isRunning;
    else if (filterStatus === "stopped") matchesFilter = !isRunning;
    
    return matchesSearch && matchesFilter;
  });

  if (error) {
    return (
      <div className="flex h-full items-center justify-center">
        <Card className="w-full max-w-md">
          <CardContent className="flex flex-col items-center gap-4 p-8">
            <AlertCircle className="h-12 w-12 text-destructive" />
            <p className="text-lg font-medium text-destructive">Error</p>
            <p className="text-sm text-muted-foreground">{error}</p>
            <Button variant="outline" onClick={clearError}>
              Dismiss
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
        <div>
          <h2 className="text-2xl font-bold">Automation Scripts</h2>
          <p className="text-sm text-muted-foreground">
            Create trigger-action automations for your Modbus devices
          </p>
        </div>
        <Button onClick={handleCreateScript} disabled={creating}>
          {creating ? (
            <Loader2 className="h-4 w-4 mr-2 animate-spin" />
          ) : (
            <Plus className="h-4 w-4 mr-2" />
          )}
          New Script
        </Button>
      </div>

      {/* Filters */}
      <Card>
        <CardContent className="p-4">
          <div className="flex gap-4">
            <div className="flex-1">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  placeholder="Search scripts..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-9"
                />
              </div>
            </div>
            <Select
              value={filterStatus}
              onValueChange={(v) => setFilterStatus(v as typeof filterStatus)}
            >
              <SelectTrigger className="w-40">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All</SelectItem>
                <SelectItem value="running">Running</SelectItem>
                <SelectItem value="stopped">Stopped</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Script List */}
      {loading ? (
        <Card>
          <CardContent className="flex items-center justify-center gap-2 p-8">
            <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
            <span className="text-muted-foreground">Loading scripts...</span>
          </CardContent>
        </Card>
      ) : filteredScripts.length === 0 ? (
        <Card className="border-dashed">
          <CardContent className="flex flex-col items-center gap-4 p-12 text-center">
            <div className="rounded-full bg-muted p-4">
              <Zap className="h-8 w-8 text-muted-foreground" />
            </div>
            <div>
              <p className="text-lg font-medium">No scripts found</p>
              <p className="text-sm text-muted-foreground">
                {scripts.length === 0
                  ? "Create your first automation script"
                  : "Try adjusting your filters"}
              </p>
            </div>
            {scripts.length === 0 && (
              <Button onClick={handleCreateScript} disabled={creating}>
                <Plus className="h-4 w-4 mr-2" />
                Create Script
              </Button>
            )}
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-4">
          {filteredScripts.map((script) => {
            const status = statuses[script.id];
            const isRunning = status?.running ?? false;

            return (
              <Card key={script.id} className={isRunning ? "border-green-500" : ""}>
                <CardContent className="flex items-center gap-4 p-4">
                  {/* Status indicator */}
                  <div className="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
                    {isRunning ? (
                      <div className="h-3 w-3 rounded-full bg-green-500 animate-pulse" />
                    ) : (
                      <div className="h-3 w-3 rounded-full bg-muted-foreground" />
                    )}
                  </div>

                  {/* Script Info */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <h3 className="font-medium truncate">{script.name}</h3>
                      {script.enabled ? (
                        <span className="rounded bg-green-100 px-1.5 py-0.5 text-xs text-green-700 dark:bg-green-900 dark:text-green-300">
                          Active
                        </span>
                      ) : (
                        <span className="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">
                          Disabled
                        </span>
                      )}
                    </div>
                    <div className="flex items-center gap-4 text-sm text-muted-foreground">
                      <span className="flex items-center gap-1">
                        <Zap className="h-3 w-3" />
                        {getTriggerDescription(script.trigger)}
                      </span>
                      <span className="flex items-center gap-1">
                        <ListFilter className="h-3 w-3" />
                        {script.actions.length} action{script.actions.length !== 1 ? "s" : ""}
                      </span>
                    </div>
                  </div>

                  {/* Actions */}
                  <div className="flex items-center gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleToggleRunning(script.id, isRunning)}
                      title={isRunning ? "Stop script" : "Start script"}
                    >
                      {isRunning ? (
                        <Square className="h-4 w-4 text-destructive" />
                      ) : (
                        <Play className="h-4 w-4 text-green-500" />
                      )}
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => navigate(`/scripts/${script.id}`)}
                      title="Edit script"
                    >
                      <Pencil className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => handleDelete(script.id, script.name)}
                      className="text-destructive hover:text-destructive"
                      title="Delete script"
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                </CardContent>
              </Card>
            );
          })}
        </div>
      )}
    </div>
  );
}
