import { BrowserRouter, Routes, Route } from "react-router-dom";
import { AppLayout } from "@/components";
import { ConnectionPage, MonitorPage, SettingsPage, ScriptListPage, ScriptEditPage } from "@/pages";
import { useTheme } from "@/hooks";

function App() {
  useTheme();

  return (
    <BrowserRouter>
      <AppLayout>
        <Routes>
          <Route path="/" element={<ConnectionPage />} />
          <Route path="/connection" element={<ConnectionPage />} />
          <Route path="/monitor" element={<MonitorPage />} />
          <Route path="/settings" element={<SettingsPage />} />
          <Route path="/scripts" element={<ScriptListPage />} />
          <Route path="/scripts/:id" element={<ScriptEditPage />} />
        </Routes>
      </AppLayout>
    </BrowserRouter>
  );
}

export default App;
