import { describe, it, expect } from "vitest";
import { useRef, useEffect } from "react";
import { render, screen } from "@testing-library/react";
import { Input } from "./input";
import { Textarea } from "./textarea";

describe("Input and Textarea ref forwarding and focus control", () => {
  it("Input correctly forwards ref, supports .focus() and .setSelectionRange()", () => {
    const TestComponent = () => {
      const inputRef = useRef<HTMLInputElement>(null);

      useEffect(() => {
        if (inputRef.current) {
          inputRef.current.focus();
          inputRef.current.setSelectionRange(2, 4);
        }
      }, []);

      return <Input ref={inputRef} defaultValue="Hello World" data-testid="test-input" />;
    };

    render(<TestComponent />);
    const el = screen.getByTestId("test-input") as HTMLInputElement;

    expect(document.activeElement).toBe(el);
    expect(el.selectionStart).toBe(2);
    expect(el.selectionEnd).toBe(4);
  });

  it("Textarea correctly forwards ref, supports .focus() and .setSelectionRange()", () => {
    const TestComponent = () => {
      const textareaRef = useRef<HTMLTextAreaElement>(null);

      useEffect(() => {
        if (textareaRef.current) {
          textareaRef.current.focus();
          textareaRef.current.setSelectionRange(3, 5);
        }
      }, []);

      return <Textarea ref={textareaRef} defaultValue="Multi line note" data-testid="test-textarea" />;
    };

    render(<TestComponent />);
    const el = screen.getByTestId("test-textarea") as HTMLTextAreaElement;

    expect(document.activeElement).toBe(el);
    expect(el.selectionStart).toBe(3);
    expect(el.selectionEnd).toBe(5);
  });
});
