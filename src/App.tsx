import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { AppConfig } from "./types/config";
import { getConfig } from "./services/configService";
import { Header } from "./components/Header";
import { WorklogForm } from "./components/WorklogForm";
import { TaskForm } from "./components/TaskForm";
import { SettingsModal } from "./components/SettingsModal";
import { Toast, ToastProps } from "./components/Toast";
import { Card } from "./components/ui/card";
import { useTheme } from "./hooks/useTheme";
import "./App.css";

function App() {
  useTheme();
  const [config, setConfigState] = useState<AppConfig | null>(null);
  const [activeTab, setActiveTab] = useState<"worklog" | "task">("worklog");
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [focusCounter, setFocusCounter] = useState(0);

  const isSettingsOpenRef = useRef(isSettingsOpen);

  const triggerFocus = () => setFocusCounter((c) => c + 1);

  const switchTab = (tab: "worklog" | "task") => {
    setActiveTab(tab);
    triggerFocus();
  };

  // Re-focus active input when settings modal is closed
  useEffect(() => {
    if (isSettingsOpenRef.current && !isSettingsOpen) {
      triggerFocus();
    }
    isSettingsOpenRef.current = isSettingsOpen;
  }, [isSettingsOpen]);

  // Trigger focus when activeTab changes
  useEffect(() => {
    triggerFocus();
  }, [activeTab]);

  // Form persistence across tabs
  const [worklogMemo, setWorklogMemo] = useState("");
  const [taskData, setTaskData] = useState({
    title: "",
    requester: "",
    due: "",
    body: "",
    custom_properties: {} as Record<string, any>,
  });

  const [toast, setToast] = useState<ToastProps>({
    message: "",
    type: "success",
    visible: false,
  });

  const showToast = (message: string, type: "success" | "error" | "info" = "success") => {
    setToast({ message, type, visible: true });
    setTimeout(() => {
      setToast((prev) => ({ ...prev, visible: false }));
    }, 2800);
  };

  const handleClose = async () => {
    try {
      await invoke("hide_launcher");
    } catch (e) {
      console.error("Failed to hide launcher", e);
    }
  };

  const handleSuccess = (msg: string) => {
    showToast(msg, "success");
    setTimeout(() => {
      handleClose();
    }, 350);
  };

  const handleError = (msg: string) => {
    showToast(msg, "error");
  };

  // Load configuration on startup and register window listeners
  useEffect(() => {
    getConfig().then((cfg) => {
      setConfigState(cfg);
      if (cfg.default_mode === "task" || cfg.default_mode === "worklog") {
        setActiveTab(cfg.default_mode);
      }
    }).catch(console.error);

    // Listen for launcher opened event from backend / tray / shortcut
    const unlistenOpenPromise = listen("launcher-opened", () => {
      setIsSettingsOpen(false);
      triggerFocus();
      getConfig().then((cfg) => {
        setConfigState(cfg);
        if (cfg.default_mode === "task" || cfg.default_mode === "worklog") {
          setActiveTab(cfg.default_mode);
        }
      }).catch(console.error);
    });

    // Listen for open settings event from tray menu
    const unlistenSettingsPromise = listen("open-settings", () => {
      setIsSettingsOpen(true);
    });

    // Listen for window focus to restore input focus on window activation
    const handleWindowFocus = (e: FocusEvent) => {
      if (e.target && e.target instanceof HTMLElement) {
        return;
      }
      if (!isSettingsOpenRef.current) {
        triggerFocus();
      }
    };
    window.addEventListener("focus", handleWindowFocus);

    return () => {
      unlistenOpenPromise.then((unlisten) => unlisten());
      unlistenSettingsPromise.then((unlisten) => unlisten());
      window.removeEventListener("focus", handleWindowFocus);
    };
  }, []);

  // Keyboard navigation & global window dismiss
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Escape
      if (e.key === "Escape") {
        e.preventDefault();
        if (isSettingsOpen) {
          setIsSettingsOpen(false);
        } else {
          handleClose();
        }
        return;
      }

      // Tab switching: Ctrl+Tab
      if (e.ctrlKey && e.key === "Tab") {
        e.preventDefault();
        setActiveTab((prev) => (prev === "worklog" ? "task" : "worklog"));
        triggerFocus();
        return;
      }

      // Alt+1 / Alt+2
      if (e.altKey && e.key === "1") {
        e.preventDefault();
        switchTab("worklog");
        return;
      }
      if (e.altKey && e.key === "2") {
        e.preventDefault();
        switchTab("task");
        return;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isSettingsOpen]);

  return (
    <div className="w-screen h-screen flex items-center justify-center p-2 box-border">
      {/* Toast Notification */}
      <Toast message={toast.message} type={toast.type} visible={toast.visible} />

      {/* Main Container */}
      <Card className="w-full h-full max-w-[620px] max-h-[400px] p-4 gap-0 shadow-2xl relative">
        {/* Top Header & Tab Controls */}
        <Header
          activeTab={activeTab}
          onTabChange={switchTab}
          onOpenSettings={() => setIsSettingsOpen(true)}
          onClose={handleClose}
        />

        {/* Tab Content Body */}
        <div className="flex-1 min-h-0">
          {activeTab === "worklog" ? (
            <WorklogForm
              memo={worklogMemo}
              setMemo={setWorklogMemo}
              autoFocusTrigger={focusCounter}
              onSuccess={handleSuccess}
              onError={handleError}
            />
          ) : (
            <TaskForm
              formData={taskData}
              setFormData={setTaskData}
              customPropertiesConfig={config?.task_custom_properties || []}
              autoFocusTrigger={focusCounter}
              onSuccess={handleSuccess}
              onError={handleError}
            />
          )}
        </div>

        {/* Settings Modal */}
        <SettingsModal
          isOpen={isSettingsOpen}
          onClose={() => setIsSettingsOpen(false)}
          onSaved={(newCfg) => setConfigState(newCfg)}
          showToast={showToast}
        />
      </Card>
    </div>
  );
}

export default App;
