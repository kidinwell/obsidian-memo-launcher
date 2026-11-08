import React, { useEffect, useRef, useState } from "react";
import { createTask } from "../services/taskService";
import { CustomProperty } from "../types/config";
import { Input } from "./ui/input";
import { Textarea } from "./ui/textarea";
import { Button } from "./ui/button";
import { Label } from "./ui/label";
import { Kbd } from "./ui/kbd";

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
  autoFocusTrigger?: number;
}

export const TaskForm: React.FC<TaskFormProps> = ({
  formData,
  setFormData,
  customPropertiesConfig,
  onSuccess,
  onError,
  autoFocusTrigger,
}) => {
  const titleInputRef = useRef<HTMLInputElement>(null);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      if (titleInputRef.current) {
        titleInputRef.current.focus();
        const len = titleInputRef.current.value.length;
        titleInputRef.current.setSelectionRange(len, len);
      }
    }, 20);
    return () => clearTimeout(timer);
  }, [autoFocusTrigger]);

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
          <Label className="text-xs mb-1">
            タスクタイトル <span className="text-destructive">*</span>
          </Label>
          <Input
            ref={titleInputRef}
            data-testid="task-title-input"
            type="text"
            value={formData.title}
            onChange={(e) => handleChange("title", e.target.value)}
            placeholder="例: 新規機能の設計レビュー"
            disabled={submitting}
            className="text-xs"
          />
        </div>

        {/* Requester & Due */}
        <div className="grid grid-cols-2 gap-2">
          <div>
            <Label className="text-xs mb-1">
              依頼者
            </Label>
            <Input
              type="text"
              value={formData.requester}
              onChange={(e) => handleChange("requester", e.target.value)}
              placeholder="例: 田中"
              disabled={submitting}
              className="text-xs"
            />
          </div>
          <div>
            <Label className="text-xs mb-1">
              締め切り (Due)
            </Label>
            <Input
              type="date"
              value={formData.due}
              onChange={(e) => handleChange("due", e.target.value)}
              disabled={submitting}
              className="text-xs"
            />
          </div>
        </div>

        {/* Custom Properties */}
        {customPropertiesConfig.length > 0 && (
          <div className="grid grid-cols-2 gap-2 pt-1 border-t">
            {customPropertiesConfig.map((prop) => (
              <div key={prop.key}>
                <Label className="text-xs mb-1">
                  {prop.label}
                </Label>
                {prop.type === "number" ? (
                  <Input
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
                    className="text-xs"
                  />
                ) : (
                  <Input
                    type="text"
                    value={formData.custom_properties[prop.key] ?? ""}
                    onChange={(e) =>
                      handleCustomPropertyChange(prop.key, e.target.value)
                    }
                    placeholder=""
                    disabled={submitting}
                    className="text-xs"
                  />
                )}
              </div>
            ))}
          </div>
        )}

        {/* Body Content */}
        <div>
          <Label className="text-xs mb-1">
            タスク本文・詳細
          </Label>
          <Textarea
            rows={3}
            value={formData.body}
            onChange={(e) => handleChange("body", e.target.value)}
            placeholder="タスクの詳細や留意事項を入力..."
            disabled={submitting}
            className="text-xs resize-none min-h-[60px]"
          />
        </div>
      </div>

      {/* Footer */}
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
          disabled={submitting || !formData.title.trim()}
          size="sm"
        >
          {submitting ? "作成中..." : "タスク作成 (Ctrl+Enter)"}
        </Button>
      </div>
    </div>
  );
};
