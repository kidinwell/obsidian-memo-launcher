import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, act } from "@testing-library/react";
import App from "./App";
import * as configService from "./services/configService";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockImplementation(() => Promise.resolve()),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockImplementation(() => Promise.resolve(() => {})),
}));

describe("App Theme Synchronization Integration", () => {
  let mediaListeners: ((e: any) => void)[] = [];
  let matchesDark = false;

  beforeEach(() => {
    mediaListeners = [];
    matchesDark = false;
    document.documentElement.className = "";

    window.matchMedia = vi.fn().mockImplementation((query: string) => ({
      matches: matchesDark,
      media: query,
      onchange: null,
      addListener: vi.fn((fn) => mediaListeners.push(fn)),
      removeListener: vi.fn((fn) => {
        mediaListeners = mediaListeners.filter((l) => l !== fn);
      }),
      addEventListener: vi.fn((_, fn) => mediaListeners.push(fn)),
      removeEventListener: vi.fn((_, fn) => {
        mediaListeners = mediaListeners.filter((l) => l !== fn);
      }),
      dispatchEvent: vi.fn(),
    }));

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

  afterEach(() => {
    document.documentElement.className = "";
  });

  it("synchronizes document theme class dynamically with OS color scheme changes", async () => {
    // Initial state: Light mode
    matchesDark = false;
    render(<App />);

    await act(async () => {
      await new Promise((r) => setTimeout(r, 60));
    });

    expect(document.documentElement.classList.contains("dark")).toBe(false);

    // OS switches to Dark mode
    matchesDark = true;
    await act(async () => {
      mediaListeners.forEach((l) => l({ matches: true }));
      await new Promise((r) => setTimeout(r, 60));
    });

    expect(document.documentElement.classList.contains("dark")).toBe(true);

    // OS switches back to Light mode
    matchesDark = false;
    await act(async () => {
      mediaListeners.forEach((l) => l({ matches: false }));
      await new Promise((r) => setTimeout(r, 60));
    });

    expect(document.documentElement.classList.contains("dark")).toBe(false);
  });
});
