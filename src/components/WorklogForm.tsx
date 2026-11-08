import React, { useEffect, useRef, useState } from "react";
import { appendWorklog } from "../services/worklogService";
import { Textarea } from "./ui/textarea";
import { Button } from "./ui/button";
import { Kbd } from "./ui/kbd";

interface WorklogFormProps {
  memo: string;
  setMemo: (val: string) => void;
  onSuccess: (msg: string) => void;
  onError: (msg: string) => void;
  autoFocusTrigger?: number;
}

export const WorklogForm: React.FC<WorklogFormProps> = ({
  memo,
  setMemo,
  onSuccess,
  onError,
  autoFocusTrigger,
}) => {
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [submitting, setSubmitting] = useState(false);
  const [currentTime, setCurrentTime] = useState("");

  useEffect(() => {
    const updateTime = () => {
      const now = new Date();
      const hh = String(now.getHours()).padStart(2, "0");
      const mm = String(now.getMinutes()).padStart(2, "0");
      setCurrentTime(`${hh}:${mm}`);
    };
    updateTime();
    const timer = setInterval(updateTime, 10000);
    return () => clearInterval(timer);
  }, []);

  useEffect(() => {
    const timer = setTimeout(() => {
      if (textareaRef.current) {
        textareaRef.current.focus();
        const len = textareaRef.current.value.length;
        textareaRef.current.setSelectionRange(len, len);
      }
    }, 20);
    return () => clearTimeout(timer);
  }, [autoFocusTrigger]);

  const handleSubmit = async () => {
    if (submitting) return;
    if (!memo.trim()) {
      onError("メモを入力してください");
      return;
    }

    setSubmitting(true);
    try {
      const result = await appendWorklog({ memo });
      if (result.success) {
        setMemo("");
        onSuccess("✓ デイリーノートに作業ログを追記しました");
      }
    } catch (e: any) {
      onError(typeof e === "string" ? e : e?.message || "作業ログの追記に失敗しました");
    } finally {
      setSubmitting(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-1 py-1 mb-2 text-xs text-muted-foreground">
        <span className="flex items-center gap-1.5 font-medium text-foreground">
          <span className="size-2 rounded-full bg-emerald-500 animate-pulse" />
          記録見出し: <span className="font-mono font-semibold">### {currentTime}</span>
        </span>
        <span className="text-[11px] flex items-center gap-1">
          <Kbd>Ctrl+Enter</Kbd> で送信
        </span>
      </div>

      <div className="flex-1 min-h-[180px]">
        <Textarea
          ref={textareaRef}
          data-testid="worklog-memo-input"
          value={memo}
          onChange={(e) => setMemo(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="作業実績・メモを入力 (例: レビュー依頼対応、- PR #123 の確認 など)..."
          disabled={submitting}
          className="h-full resize-none p-3 text-sm leading-relaxed"
        />
      </div>

      <div className="flex items-center justify-between pt-3 mt-2 border-t">
        <div className="text-[11px] text-muted-foreground flex gap-3 items-center">
          <span className="flex items-center gap-1">
            <Kbd>Esc</Kbd> 閉じる
          </span>
          <span className="flex items-center gap-1">
            <Kbd>Ctrl+Tab</Kbd> 切替
          </span>
        </div>
        <Button
          onClick={handleSubmit}
          disabled={submitting || !memo.trim()}
          size="sm"
        >
          {submitting ? "保存中..." : "追記する (Ctrl+Enter)"}
        </Button>
      </div>
    </div>
  );
};
