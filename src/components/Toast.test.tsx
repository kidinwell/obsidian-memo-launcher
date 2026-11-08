import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { Toast } from "./Toast";

describe("Toast component", () => {
  it("renders when visible is true", () => {
    render(<Toast message="タスク作成完了" type="success" visible={true} />);
    expect(screen.getByTestId("toast-notification")).toBeDefined();
    expect(screen.getByText("タスク作成完了")).toBeDefined();
  });

  it("does not render when visible is false", () => {
    render(<Toast message="非表示メッセージ" type="error" visible={false} />);
    expect(screen.queryByTestId("toast-notification")).toBeNull();
  });
});
