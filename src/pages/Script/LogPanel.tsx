/**
 * Log Panel Component
 * Displays script trigger execution history with filtering
 */

import { useState } from "react";
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
import { useScriptStore, type LogEntry } from "./store";
import { getTriggerDescription } from "./data";
import {
  History,
  Trash2,
  Search,
  Zap,
  CheckCircle2,
  XCircle,
  ChevronDown,
  ChevronRight,
} from "lucide-react";

export function LogPanel() {
  const { logs, clearLogs, scripts } = useScriptStore();
  const [filterScript, setFilterScript] = useState<string>("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [expandedId, setExpandedId] = useState<string | null>(null);

  // Filter logs
  const filteredLogs = logs.filter((log) => {
    const matchesScript = filterScript === "all" || log.scriptId === filterScript;
    const query = searchQuery.toLowerCase();
    const matchesSearch =
      !query ||
      log.scriptName.toLowerCase().includes(query) ||
      log.triggerCondition.toLowerCase().includes(query) ||
      log.actions.some((a) => a.toLowerCase().includes(query));
    return matchesScript && matchesSearch;
  });

  return (
    <Card className="w-full">
      <CardHeader className="pb-3 bg-gradient-to-r from-primary/5 to-transparent border-b dark:border-border/50">
        <CardTitle className="flex items-center justify-between text-base">
          <span className="flex items-center gap-2">
            <div className="flex h-7 w-7 items-center justify-center rounded-md bg-blue-100 dark:bg-blue-900/40">
              <History className="h-4 w-4 text-blue-600 dark:text-blue-400" />
            </div>
            <span>Execution Log</span>
            <span className="text-xs font-normal text-muted-foreground">
              ({filteredLogs.length} {filteredLogs.length === 1 ? "entry" : "entries"})
            </span>
          </span>
          <Button
            variant="ghost"
            size="sm"
            onClick={clearLogs}
            disabled={logs.length === 0}
            className="gap-1 text-destructive/70 hover:text-destructive"
          >
            <Trash2 className="h-3.5 w-3.5" />
            Clear
          </Button>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-3 pt-4">
        {/* Filters */}
        <div className="flex gap-2 flex-wrap">
          <div className="flex-1 min-w-[150px]">
            <Select
              value={filterScript}
              onValueChange={setFilterScript}
            >
              <SelectTrigger className="h-8 text-xs">
                <SelectValue placeholder="All scripts" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All scripts</SelectItem>
                {scripts.map((s) => (
                  <SelectItem key={s.id} value={s.id}>
                    {s.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="flex-1 min-w-[150px] relative">
            <Search className="absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="Search logs..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="h-8 pl-8 text-xs"
            />
          </div>
        </div>

        {/* Log entries */}
        {filteredLogs.length === 0 ? (
          <div className="flex flex-col items-center gap-2 py-8 text-center">
            <History className="h-8 w-8 text-muted-foreground/50" />
            <p className="text-sm text-muted-foreground">
              {logs.length === 0
                ? "No log entries yet"
                : "No entries match your filters"}
            </p>
          </div>
        ) : (
          <div className="max-h-80 overflow-y-auto space-y-2 pr-2">
              {filteredLogs.map((log) => (
                <LogEntryRow
                  key={log.id}
                  log={log}
                  expanded={expandedId === log.id}
                  onToggle={() =>
                    setExpandedId(expandedId === log.id ? null : log.id)
                  }
                />
              ))}
          </div>
        )}
      </CardContent>
    </Card>
  );
}

function LogEntryRow({
  log,
  expanded,
  onToggle,
}: {
  log: LogEntry;
  expanded: boolean;
  onToggle: () => void;
}) {
  const time = new Date(log.triggeredAt);
  const timeStr = time.toLocaleTimeString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
  const dateStr = time.toLocaleDateString("zh-CN", {
    month: "short",
    day: "numeric",
  });

  return (
    <div
      className={`
        rounded-md border text-xs transition-colors
        ${log.success
          ? "border-border bg-card hover:bg-muted/30"
          : "border-destructive/30 bg-destructive/5"
        }
      `}
    >
      {/* Header row */}
      <button
        type="button"
        className="flex w-full items-center gap-2 p-2.5 text-left"
        onClick={onToggle}
      >
        {/* Expand icon */}
        {expanded ? (
          <ChevronDown className="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
        ) : (
          <ChevronRight className="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
        )}

        {/* Status icon */}
        {log.success ? (
          <CheckCircle2 className="h-3.5 w-3.5 shrink-0 text-green-500" />
        ) : (
          <XCircle className="h-3.5 w-3.5 shrink-0 text-destructive" />
        )}

        {/* Script name */}
        <span className="shrink-0 font-medium flex items-center gap-1">
          <Zap className="h-3 w-3 text-yellow-500" />
          {log.scriptName}
        </span>

        {/* Trigger condition */}
        <span className="text-muted-foreground truncate">
          {log.triggerCondition}
        </span>

        {/* Timestamp */}
        <span className="ml-auto shrink-0 text-muted-foreground pl-2">
          {dateStr} {timeStr}
        </span>
      </button>

      {/* Expanded details */}
      {expanded && (
        <div className="border-t px-3 py-2 space-y-1.5 bg-muted/20">
          <div className="text-muted-foreground">
            <span className="font-medium text-foreground">Trigger:</span>{" "}
            {log.triggerCondition}
          </div>
          {log.actions.length > 0 && (
            <div>
              <div className="font-medium text-foreground mb-0.5">Actions:</div>
              <ul className="ml-2 space-y-0.5 text-muted-foreground">
                {log.actions.map((action, i) => (
                  <li key={i} className="flex items-start gap-1">
                    <span className="text-primary">•</span>
                    {action}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
