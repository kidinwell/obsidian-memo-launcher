import { describe, it, expect } from "vitest";
import { cn } from "./utils";

describe("cn helper utility", () => {
  it("merges tailwind class names properly", () => {
    expect(cn("px-2 py-1", "bg-red-500")).toBe("px-2 py-1 bg-red-500");
  });

  it("handles tailwind class precedence correctly", () => {
    expect(cn("p-2", "p-4")).toBe("p-4");
    expect(cn("text-red-500", "text-blue-500")).toBe("text-blue-500");
  });

  it("handles conditional classes, arrays, and undefined values", () => {
    const isTrue = true;
    const isFalse = false;
    expect(cn("base", isTrue && "active", isFalse && "inactive", undefined, null)).toBe("base active");
  });
});
