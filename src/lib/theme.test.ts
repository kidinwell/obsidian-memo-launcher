import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { applyTheme, getSystemTheme, initThemeListener } from "./theme";

describe("Theme Management Utility", () => {
  let listeners: ((e: any) => void)[] = [];
  let matchesDark = false;

  beforeEach(() => {
    listeners = [];
    matchesDark = false;
    document.documentElement.className = "";

    window.matchMedia = vi.fn().mockImplementation((query: string) => {
      return {
        matches: matchesDark,
        media: query,
        onchange: null,
        addListener: vi.fn((fn) => listeners.push(fn)),
        removeListener: vi.fn((fn) => {
          listeners = listeners.filter((l) => l !== fn);
        }),
        addEventListener: vi.fn((_, fn) => listeners.push(fn)),
        removeEventListener: vi.fn((_, fn) => {
          listeners = listeners.filter((l) => l !== fn);
        }),
        dispatchEvent: vi.fn(),
      };
    });
  });

  afterEach(() => {
    document.documentElement.className = "";
  });

  it("getSystemTheme returns 'light' when prefers-color-scheme: dark is false", () => {
    matchesDark = false;
    expect(getSystemTheme()).toBe("light");
  });

  it("getSystemTheme returns 'dark' when prefers-color-scheme: dark is true", () => {
    matchesDark = true;
    expect(getSystemTheme()).toBe("dark");
  });

  it("applyTheme('system') removes .dark in light mode", () => {
    matchesDark = false;
    document.documentElement.classList.add("dark");
    const resolved = applyTheme("system");
    expect(resolved).toBe("light");
    expect(document.documentElement.classList.contains("dark")).toBe(false);
  });

  it("applyTheme('system') adds .dark in dark mode", () => {
    matchesDark = true;
    const resolved = applyTheme("system");
    expect(resolved).toBe("dark");
    expect(document.documentElement.classList.contains("dark")).toBe(true);
  });

  it("applyTheme('light') explicitly forces light mode even if system is dark", () => {
    matchesDark = true;
    const resolved = applyTheme("light");
    expect(resolved).toBe("light");
    expect(document.documentElement.classList.contains("dark")).toBe(false);
  });

  it("applyTheme('dark') explicitly forces dark mode even if system is light", () => {
    matchesDark = false;
    const resolved = applyTheme("dark");
    expect(resolved).toBe("dark");
    expect(document.documentElement.classList.contains("dark")).toBe(true);
  });

  it("initThemeListener responds dynamically to OS scheme change", () => {
    matchesDark = false;
    const onResolved = vi.fn();
    const cleanup = initThemeListener(() => "system", onResolved);

    expect(document.documentElement.classList.contains("dark")).toBe(false);
    expect(onResolved).toHaveBeenCalledWith("light");

    // Simulate OS switched to dark mode
    matchesDark = true;
    listeners.forEach((l) => l({ matches: true }));

    expect(document.documentElement.classList.contains("dark")).toBe(true);
    expect(onResolved).toHaveBeenCalledWith("dark");

    cleanup();
  });
});
