export type Theme = "system" | "light" | "dark";

export interface ThemeConfig {
  theme?: Theme;
}

export const THEME_STORAGE_KEY = "obsidian_launcher_theme";

/**
 * Returns the current resolved theme ("light" or "dark") based on theme setting and system preference.
 */
export function getSystemTheme(): "light" | "dark" {
  if (typeof window === "undefined" || !window.matchMedia) {
    return "light";
  }
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

/**
 * Applies the resolved theme class to document.documentElement.
 */
export function applyTheme(theme: Theme = "system"): "light" | "dark" {
  if (typeof document === "undefined") {
    return "light";
  }

  const resolved = theme === "system" ? getSystemTheme() : theme;
  if (resolved === "dark") {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
  return resolved;
}

/**
 * Sets up a listener for OS color scheme changes and synchronizes document.documentElement.
 * Returns an unlisten cleanup function.
 */
export function initThemeListener(
  getThemeSetting: () => Theme = () => "system",
  onResolvedChange?: (resolved: "light" | "dark") => void
): () => void {
  if (typeof window === "undefined" || !window.matchMedia) {
    return () => {};
  }

  const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

  const update = () => {
    const currentTheme = getThemeSetting();
    const resolved = applyTheme(currentTheme);
    onResolvedChange?.(resolved);
  };

  // Initial apply
  update();

  const listener = () => update();

  if (typeof mediaQuery.addEventListener === "function") {
    mediaQuery.addEventListener("change", listener);
    return () => mediaQuery.removeEventListener("change", listener);
  } else if (typeof (mediaQuery as any).addListener === "function") {
    (mediaQuery as any).addListener(listener);
    return () => (mediaQuery as any).removeListener(listener);
  }

  return () => {};
}
