import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { Kbd, KbdGroup } from "./kbd";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "./select";

describe("Kbd and Select components", () => {
  it("renders Kbd and KbdGroup", () => {
    render(
      <KbdGroup data-testid="kbd-grp">
        <Kbd>Ctrl</Kbd>
        <Kbd>Enter</Kbd>
      </KbdGroup>
    );
    expect(screen.getByTestId("kbd-grp")).toBeDefined();
    expect(screen.getByText("Ctrl")).toBeDefined();
    expect(screen.getByText("Enter")).toBeDefined();
  });

  it("renders Select component with initial value", () => {
    render(
      <Select defaultValue="worklog">
        <SelectTrigger data-testid="test-select-trigger">
          <SelectValue placeholder="モード選択" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="worklog">作業実績</SelectItem>
          <SelectItem value="task">タスク作成</SelectItem>
        </SelectContent>
      </Select>
    );

    expect(screen.getByTestId("test-select-trigger")).toBeDefined();
    expect(screen.getByText("worklog")).toBeDefined();
  });
});
