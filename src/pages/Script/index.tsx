/**
 * Script List Page
 * Main page showing all automation scripts with status and controls
 */

import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Card, CardContent } from "@/components/ui/card";
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
import { getTriggerDescription, type Trigger } from "./data";
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
  ChevronRight,
} from "lucide-react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";

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
  const [deletingId, setDeletingId] = useState<string | null>(null);

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

  const confirmDelete = async () => {
    if (!deletingId) return;
    await removeScript(deletingId);
    setDeletingId(null);
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
        <div className="grid gap-3">
          {filteredScripts.map((script) => {
            const status = statuses[script.id];
            const isRunning = status?.running ?? false;

            return (
              <div key={script.id} className="group relative">
                <Card
                  className={`transition-all duration-200 hover:shadow-md hover:border-primary/30 ${
                    isRunning ? "border-green-500 dark:border-green-500" : ""
                  }`}
                >
                  <CardContent className="flex items-center gap-3 p-4">
                    {/* Status indicator */}
                    <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-muted transition-colors">
                      {isRunning ? (
                        <div className="h-3 w-3 rounded-full bg-green-500 shadow-sm shadow-green-500/50 animate-pulse" />
                      ) : (
                        <div className="h-3 w-3 rounded-full bg-muted-foreground" />
                      )}
                    </div>

                    {/* Script Info */}
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        <h3 className="font-medium truncate">{script.name}</h3>
                        {script.enabled ? (
                          <span className="rounded bg-green-100 px-1.5 py-0.5 text-xs text-green-700 dark:bg-green-900/50 dark:text-green-400 shrink-0">
                            Active
                          </span>
                        ) : (
                          <span className="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground shrink-0">
                            Disabled
                          </span>
                        )}
                      </div>
                      <div className="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm text-muted-foreground">
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
                    <div className="flex items-center gap-1 opacity-80 group-hover:opacity-100 transition-opacity">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleToggleRunning(script.id, isRunning)}
                        title={isRunning ? "Stop script" : "Start script"}
                        className="transition-colors"
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
                        className="transition-colors hover:text-primary"
                      >
                        <Pencil className="h-4 w-4" />
                      </Button>
                      <AlertDialog>
                        <AlertDialogTrigger asChild>
                          <Button
                            variant="ghost"
                            size="icon"
                            className="text-destructive/70 hover:text-destructive transition-colors"
                            title="Delete script"
                            onClick={() => setDeletingId(script.id)}
                          >
                            <Trash2 className="h-4 w-4" />
                          </Button>
                        </AlertDialogTrigger>
                        <AlertDialogContent>
                          <AlertDialogHeader>
                            <AlertDialogTitle>Delete Script?</AlertDialogTitle>
                            <AlertDialogDescription>
                              This action cannot be undone. The script "{script.name}" will be permanently deleted.
                            </AlertDialogDescription>
                          </AlertDialogHeader>
                          <AlertDialogFooter>
                            <AlertDialogCancel onClick={() => setDeletingId(null)}>Cancel</AlertDialogCancel>
                            <AlertDialogAction
                              onClick={confirmDelete}
                              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                            >
                              Delete
                            </AlertDialogAction>
                          </AlertDialogFooter>
                        </AlertDialogContent>
                      </AlertDialog>
                    </div>
                  </CardContent>
                </Card>

                {/* Edit chevron hint */}
                <Button
                  variant="ghost"
                  size="icon"
                  className="absolute right-3 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none group-hover:pointer-events-auto"
                  onClick={() => navigate(`/scripts/${script.id}`)}
                  title="Edit script"
                >
                  <ChevronRight className="h-4 w-4 text-muted-foreground" />
                </Button>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
