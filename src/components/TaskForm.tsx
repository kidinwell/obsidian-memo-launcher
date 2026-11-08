import React, { useEffect, useRef, useState } from "react";
import { createTask } from "../services/taskService";
import { CustomProperty } from "../types/config";

interface TaskFormData {
  title: string;
  requester: string;
  due: string;
  body: string;
  custom_properties: Record<string, any>;
}

interface TaskFormProps {
  formData: TaskFormData;
  setFormData: React.Dispatch<React.SetStateAction<TaskFormData>>;
  customPropertiesConfig: CustomProperty[];
  onSuccess: (msg: string) => void;
  onError: (msg: string) => void;
}

export const TaskForm: React.FC<TaskFormProps> = ({
  formData,
  setFormData,
  customPropertiesConfig,
  onSuccess,
  onError,
}) => {
  const titleInputRef = useRef<HTMLInputElement>(null);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    titleInputRef.current?.focus();
  }, []);

  const handleChange = (field: keyof TaskFormData, value: any) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  const handleCustomPropertyChange = (key: string, value: any) => {
    setFormData((prev) => ({
      ...prev,
      custom_properties: {
        ...prev.custom_properties,
        [key]: value,
      },
    }));
  };

  const handleSubmit = async () => {
    if (submitting) return;
    if (!formData.title.trim()) {
      onError("タスクタイトルを入力してください");
      titleInputRef.current?.focus();
      return;
    }

    setSubmitting(true);
    try {
      const res = await createTask({
        title: formData.title,
        requester: formData.requester,
        due: formData.due,
        body: formData.body,
        custom_properties: formData.custom_properties,
      });

      if (res.success) {
        setFormData({
          title: "",
          requester: "",
          due: "",
          body: "",
          custom_properties: {},
        });
        onSuccess(`✓ タスクノートを作成しました: ${res.filename}`);
      }
    } catch (e: any) {
      onError(typeof e === "string" ? e : e?.message || "タスクの作成に失敗しました");
    } finally {
      setSubmitting(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div className="flex flex-col h-full" onKeyDown={handleKeyDown}>
      <div className="flex-1 overflow-y-auto pr-1 space-y-2.5 max-h-[260px]">
        {/* Title */}
        <div>
          <label className="block text-[11px] font-medium text-zinc-400 mb-1">
            タスクタイトル <span className="text-rose-400">*</span>
          </label>
          <input
            ref={titleInputRef}
            type="text"
            value={formData.title}
            onChange={(e) => handleChange("title", e.target.value)}
            placeholder="例: 新規機能の設計レビュー"
            disabled={submitting}
            className="w-full px-3 py-1.5 text-xs bg-zinc-900/80 border border-zinc-800 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/70"
          />
        </div>

        {/* Requester & Due */}
        <div className="grid grid-cols-2 gap-2">
          <div>
            <label className="block text-[11px] font-medium text-zinc-400 mb-1">
              依頼者
            </label>
            <input
              type="text"
              value={formData.requester}
              onChange={(e) => handleChange("requester", e.target.value)}
              placeholder="例: 田中"
              disabled={submitting}
              className="w-full px-3 py-1.5 text-xs bg-zinc-900/80 border border-zinc-800 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/70"
            />
          </div>
          <div>
            <label className="block text-[11px] font-medium text-zinc-400 mb-1">
              締め切り (Due)
            </label>
            <input
              type="date"
              value={formData.due}
              onChange={(e) => handleChange("due", e.target.value)}
              disabled={submitting}
              className="w-full px-3 py-1.5 text-xs bg-zinc-900/80 border border-zinc-800 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/70"
            />
          </div>
        </div>

        {/* Custom Properties */}
        {customPropertiesConfig.length > 0 && (
          <div className="grid grid-cols-2 gap-2 pt-1 border-t border-zinc-800/40">
            {customPropertiesConfig.map((prop) => (
              <div key={prop.key}>
                <label className="block text-[11px] font-medium text-zinc-400 mb-1">
                  {prop.label}
                </label>
                {prop.type === "number" ? (
                  <input
                    type="number"
                    step="any"
                    value={formData.custom_properties[prop.key] ?? ""}
                    onChange={(e) =>
                      handleCustomPropertyChange(
                        prop.key,
                        e.target.value === "" ? null : Number(e.target.value)
                      )
                    }
                    placeholder="0"
                    disabled={submitting}
                    className="w-full px-3 py-1.5 text-xs bg-zinc-900/80 border border-zinc-800 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/70"
                  />
                ) : (
                  <input
                    type="text"
                    value={formData.custom_properties[prop.key] ?? ""}
                    onChange={(e) =>
                      handleCustomPropertyChange(prop.key, e.target.value)
                    }
                    placeholder=""
                    disabled={submitting}
                    className="w-full px-3 py-1.5 text-xs bg-zinc-900/80 border border-zinc-800 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/70"
                  />
                )}
              </div>
            ))}
          </div>
        )}

        {/* Body Content */}
        <div>
          <label className="block text-[11px] font-medium text-zinc-400 mb-1">
            タスク本文・詳細
          </label>
          <textarea
            rows={3}
            value={formData.body}
            onChange={(e) => handleChange("body", e.target.value)}
            placeholder="タスクの詳細や留意事項を入力..."
            disabled={submitting}
            className="w-full p-2.5 text-xs bg-zinc-900/80 border border-zinc-800 rounded-lg resize-none text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-indigo-500/70 font-sans leading-relaxed"
          />
        </div>
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between pt-3 mt-2 border-t border-zinc-800/50">
        <div className="text-[11px] text-zinc-500 flex gap-2">
          <span><kbd className="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[10px]">Esc</kbd> 閉じる</span>
          <span><kbd className="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[10px]">Ctrl+Tab</kbd> 切替</span>
        </div>
        <button
          onClick={handleSubmit}
          disabled={submitting || !formData.title.trim()}
          className="px-4 py-1.5 text-xs font-semibold rounded-lg bg-indigo-600 hover:bg-indigo-500 active:bg-indigo-700 disabled:opacity-40 disabled:hover:bg-indigo-600 text-white shadow-sm transition-all flex items-center gap-1.5 cursor-pointer"
        >
          {submitting ? "作成中..." : "タスク作成 (Ctrl+Enter)"}
        </button>
      </div>
    </div>
  );
};
