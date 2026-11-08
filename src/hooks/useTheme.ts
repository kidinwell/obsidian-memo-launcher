import { useEffect, useState } from "react";
import { applyTheme, initThemeListener, Theme } from "../lib/theme";

export function useTheme(initialTheme: Theme = "system") {
  const [theme, setTheme] = useState<Theme>(initialTheme);
  const [resolvedTheme, setResolvedTheme] = useState<"light" | "dark">(() => {
    return applyTheme(initialTheme);
  });

  useEffect(() => {
    const unlisten = initThemeListener(
      () => theme,
      (resolved) => setResolvedTheme(resolved)
    );
    return unlisten;
  }, [theme]);

  return {
    theme,
    resolvedTheme,
    setTheme,
  };
}
