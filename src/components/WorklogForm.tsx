import React, { useEffect, useRef, useState } from "react";
import { appendWorklog } from "../services/worklogService";

interface WorklogFormProps {
  memo: string;
  setMemo: (val: string) => void;
  onSuccess: (msg: string) => void;
  onError: (msg: string) => void;
}

export const WorklogForm: React.FC<WorklogFormProps> = ({
  memo,
  setMemo,
  onSuccess,
  onError,
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
    textareaRef.current?.focus();
  }, []);

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
      <div className="flex items-center justify-between px-1 py-1 mb-2 text-xs text-zinc-400">
        <span className="flex items-center gap-1.5 font-medium text-zinc-300">
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
          記録見出し: <span className="font-mono text-zinc-200 font-semibold">### {currentTime}</span>
        </span>
        <span className="text-[11px] text-zinc-500">
          <kbd className="px-1 py-0.5 rounded bg-zinc-800 text-zinc-300 font-mono text-[10px]">Ctrl+Enter</kbd> で送信
        </span>
      </div>

      <div className="relative flex-1 min-h-[180px]">
        <textarea
          ref={textareaRef}
          value={memo}
          onChange={(e) => setMemo(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="作業実績・メモを入力 (例: レビュー依頼対応、- PR #123 の確認 など)..."
          disabled={submitting}
          className="w-full h-full p-3.5 text-sm bg-zinc-900/80 border border-zinc-800/80 rounded-xl resize-none text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/70 focus:border-indigo-500/70 transition-all font-sans leading-relaxed"
        />
      </div>

      <div className="flex items-center justify-between pt-3 mt-2 border-t border-zinc-800/50">
        <div className="text-[11px] text-zinc-500 flex gap-2">
          <span><kbd className="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[10px]">Esc</kbd> 閉じる</span>
          <span><kbd className="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[10px]">Ctrl+Tab</kbd> 切替</span>
        </div>
        <button
          onClick={handleSubmit}
          disabled={submitting || !memo.trim()}
          className="px-4 py-1.5 text-xs font-semibold rounded-lg bg-indigo-600 hover:bg-indigo-500 active:bg-indigo-700 disabled:opacity-40 disabled:hover:bg-indigo-600 text-white shadow-sm transition-all flex items-center gap-1.5 cursor-pointer"
        >
          {submitting ? "保存中..." : "追記する (Ctrl+Enter)"}
        </button>
      </div>
    </div>
  );
};
