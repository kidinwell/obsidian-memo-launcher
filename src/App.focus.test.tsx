import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, act } from "@testing-library/react";
import App from "./App";
import * as configService from "./services/configService";

// Event listener registry for mocked tauri listen
type Listener = (event: any) => void;
const listeners: Record<string, Listener[]> = {};

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockImplementation((cmd: string) => {
    if (cmd === "get_config") {
      return Promise.resolve({
        vault_path: "/test/vault",
        tasks_folder: "Tasks",
        default_mode: "worklog",
        worklog_section_heading: "## 作業ログ",
        worklog_use_anchor: false,
        worklog_anchor_comment: "%% worklog %%",
        global_shortcut: "Super+Shift+Q",
        auto_hide_on_blur: true,
        task_custom_properties: [],
      });
    }
    return Promise.resolve();
  }),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockImplementation((event: string, handler: Listener) => {
    if (!listeners[event]) {
      listeners[event] = [];
    }
    listeners[event].push(handler);
    return Promise.resolve(() => {
      listeners[event] = listeners[event].filter((h) => h !== handler);
    });
  }),
}));

describe("Launcher Input Auto-focus Reliability", () => {
  const emitTauriEvent = async (event: string, payload: any = {}) => {
    const handlers = listeners[event] || [];
    for (const h of handlers) {
      act(() => {
        h({ event, payload });
      });
    }
    await act(async () => {
      await new Promise((r) => setTimeout(r, 100));
    });
  };

  beforeEach(() => {
    vi.clearAllMocks();
    for (const key of Object.keys(listeners)) {
      delete listeners[key];
    }
    vi.spyOn(configService, "getConfig").mockResolvedValue({
      vault_path: "/test/vault",
      tasks_folder: "Tasks",
      default_mode: "worklog",
      worklog_section_heading: "## 作業ログ",
      worklog_use_anchor: false,
      worklog_anchor_comment: "%% worklog %%",
      global_shortcut: "Super+Shift+Q",
      auto_hide_on_blur: true,
      task_custom_properties: [],
    });
  });

  it("11.1: automatically focuses active input on initial load and launcher-opened event", async () => {
    vi.spyOn(configService, "getConfig").mockResolvedValue({
      vault_path: "/test/vault",
      tasks_folder: "Tasks",
      default_mode: "worklog",
      worklog_section_heading: "## 作業ログ",
      worklog_use_anchor: false,
      worklog_anchor_comment: "%% worklog %%",
      global_shortcut: "Super+Shift+Q",
      auto_hide_on_blur: true,
      task_custom_properties: [],
    });

    render(<App />);

    // Wait for initial render and focus timer
    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    const worklogInput = screen.getByTestId("worklog-memo-input");
    expect(document.activeElement).toBe(worklogInput);

    // Simulate clicking away or blurring
    worklogInput.blur();
    expect(document.activeElement).not.toBe(worklogInput);

    // Re-open launcher event
    await emitTauriEvent("launcher-opened");

    expect(document.activeElement).toBe(worklogInput);
  });

  it("11.1: automatically focuses active input on window focus event", async () => {
    render(<App />);
    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    const worklogInput = screen.getByTestId("worklog-memo-input");
    worklogInput.blur();
    expect(document.activeElement).not.toBe(worklogInput);

    // Trigger window focus event
    await act(async () => {
      window.dispatchEvent(new Event("focus"));
      await new Promise((r) => setTimeout(r, 60));
    });

    expect(document.activeElement).toBe(worklogInput);
  });

  it("11.2: moves focus to primary input on tab switch via button click", async () => {
    render(<App />);
    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    const taskTabBtn = screen.getByTestId("tab-task-btn");

    await act(async () => {
      fireEvent.click(taskTabBtn);
      await new Promise((r) => setTimeout(r, 60));
    });

    const taskTitleInput = screen.getByTestId("task-title-input");
    expect(document.activeElement).toBe(taskTitleInput);

    // Click worklog tab button
    const worklogTabBtn = screen.getByTestId("tab-worklog-btn");
    await act(async () => {
      fireEvent.click(worklogTabBtn);
      await new Promise((r) => setTimeout(r, 60));
    });

    const worklogInput = screen.getByTestId("worklog-memo-input");
    expect(document.activeElement).toBe(worklogInput);
  });

  it("11.2: moves focus on keyboard mode switching (Ctrl+Tab, Alt+1, Alt+2)", async () => {
    render(<App />);
    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    // Start in Worklog mode -> switch to Task via Ctrl+Tab
    await act(async () => {
      window.dispatchEvent(new KeyboardEvent("keydown", { key: "Tab", ctrlKey: true }));
      await new Promise((r) => setTimeout(r, 60));
    });
    const taskInput = screen.getByTestId("task-title-input");
    expect(document.activeElement).toBe(taskInput);

    // Switch back to Worklog via Alt+1
    await act(async () => {
      window.dispatchEvent(new KeyboardEvent("keydown", { key: "1", altKey: true }));
      await new Promise((r) => setTimeout(r, 60));
    });
    const worklogInput = screen.getByTestId("worklog-memo-input");
    expect(document.activeElement).toBe(worklogInput);

    // Switch to Task via Alt+2
    await act(async () => {
      window.dispatchEvent(new KeyboardEvent("keydown", { key: "2", altKey: true }));
      await new Promise((r) => setTimeout(r, 60));
    });
    const taskInput2 = screen.getByTestId("task-title-input");
    expect(document.activeElement).toBe(taskInput2);
  });

  it("11.2 & 11.3: returns focus to active input when settings modal is closed", async () => {
    render(<App />);
    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    const worklogInput = screen.getByTestId("worklog-memo-input");
    expect(document.activeElement).toBe(worklogInput);

    // Open settings modal
    const settingsBtn = screen.getByTestId("open-settings-btn");
    await act(async () => {
      fireEvent.click(settingsBtn);
      await new Promise((r) => setTimeout(r, 60));
    });

    // Modal is open, worklog input is no longer activeElement
    expect(screen.getByText("ランチャー設定")).toBeDefined();

    // Close settings modal via Escape
    await act(async () => {
      window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape" }));
      await new Promise((r) => setTimeout(r, 60));
    });

    // Settings modal closed, worklog input regained focus
    expect(screen.queryByText("ランチャー設定")).toBeNull();
    expect(document.activeElement).toBe(screen.getByTestId("worklog-memo-input"));
  });

  it("11.1 & 11.3: automatically focuses task title input in task mode on window focus, and when default_mode is task on re-open", async () => {
    render(<App />);
    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    // Switch to task mode
    const taskTabBtn = screen.getByTestId("tab-task-btn");
    await act(async () => {
      fireEvent.click(taskTabBtn);
      await new Promise((r) => setTimeout(r, 60));
    });

    const taskTitleInput = screen.getByTestId("task-title-input");
    expect(document.activeElement).toBe(taskTitleInput);

    // Blur
    taskTitleInput.blur();
    expect(document.activeElement).not.toBe(taskTitleInput);

    // Window focus in Task mode restores focus to task title
    await act(async () => {
      window.dispatchEvent(new Event("focus"));
      await new Promise((r) => setTimeout(r, 60));
    });
    expect(document.activeElement).toBe(taskTitleInput);

    // If default_mode is configured to "task", re-opening launcher focuses task input
    vi.spyOn(configService, "getConfig").mockResolvedValue({
      vault_path: "/test/vault",
      tasks_folder: "Tasks",
      default_mode: "task",
      worklog_section_heading: "## 作業ログ",
      worklog_use_anchor: false,
      worklog_anchor_comment: "%% worklog %%",
      global_shortcut: "Super+Shift+Q",
      auto_hide_on_blur: true,
      task_custom_properties: [],
    });

    taskTitleInput.blur();
    expect(document.activeElement).not.toBe(taskTitleInput);

    await emitTauriEvent("launcher-opened");
    expect(document.activeElement).toBe(screen.getByTestId("task-title-input"));
  });

  it("11.3: does not steal focus when settings modal is open upon window focus", async () => {
    render(<App />);
    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    // Open settings modal
    const settingsBtn = screen.getByTestId("open-settings-btn");
    await act(async () => {
      fireEvent.click(settingsBtn);
      await new Promise((r) => setTimeout(r, 60));
    });

    expect(screen.getByText("ランチャー設定")).toBeDefined();

    const worklogInput = screen.getByTestId("worklog-memo-input");
    worklogInput.blur();
    expect(document.activeElement).not.toBe(worklogInput);

    // Trigger window focus event while modal is open
    await act(async () => {
      window.dispatchEvent(new Event("focus"));
      await new Promise((r) => setTimeout(r, 60));
    });

    // Worklog input should NOT have stolen focus while modal is open
    expect(document.activeElement).not.toBe(worklogInput);
  });
});
