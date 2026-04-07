import { useState, useEffect, useCallback } from "react";
import * as ipc from "@/lib/ipc";

export function useSerialPorts() {
  const [ports, setPorts] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refreshPorts = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const portList = await ipc.listSerialPorts();
      setPorts(portList);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to list ports");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refreshPorts();
  }, [refreshPorts]);

  return {
    ports,
    loading,
    error,
    refreshPorts,
  };
}
