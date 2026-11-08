import React, { useEffect, useState } from "react";
import { AppConfig, CustomProperty, VaultValidationResult } from "../types/config";
import { getConfig, setConfig, validateVault } from "../services/configService";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "./ui/input";
import { Checkbox } from "./ui/checkbox";
import { Label } from "./ui/label";
import { Button } from "./ui/button";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "./ui/select";
import { Settings, X, Trash2, Plus } from "lucide-react";

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSaved: (cfg: AppConfig) => void;
  showToast: (msg: string, type?: "success" | "error" | "info") => void;
}

export const SettingsModal: React.FC<SettingsModalProps> = ({
  isOpen,
  onClose,
  onSaved,
  showToast,
}) => {
  const [config, setLocalConfig] = useState<AppConfig | null>(null);
  const [vaultStatus, setVaultStatus] = useState<VaultValidationResult | null>(null);
  const [validating, setValidating] = useState(false);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadInitialConfig();
    }
  }, [isOpen]);

  const loadInitialConfig = async () => {
    try {
      const cfg = await getConfig();
      setLocalConfig(cfg);
      if (cfg.vault_path) {
        checkVault(cfg.vault_path);
      }
    } catch (e: any) {
      showToast("設定の読み込みに失敗しました", "error");
    }
  };

  const checkVault = async (path: string) => {
    if (!path.trim()) {
      setVaultStatus(null);
      return;
    }
    setValidating(true);
    try {
      const res = await validateVault(path);
      setVaultStatus(res);
    } catch (e) {
      setVaultStatus({ is_valid: false, has_obsidian_folder: false, message: "検証エラー" });
    } finally {
      setValidating(false);
    }
  };

  const handleChange = (field: keyof AppConfig, value: any) => {
    if (!config) return;
    setLocalConfig({ ...config, [field]: value });
    if (field === "vault_path") {
      checkVault(value);
    }
  };

  const handleAddCustomProperty = () => {
    if (!config) return;
    const newProp: CustomProperty = {
      key: `custom_${Date.now().toString().slice(-4)}`,
      label: "新規プロパティ",
      type: "text",
      default: null,
    };
    setLocalConfig({
      ...config,
      task_custom_properties: [...config.task_custom_properties, newProp],
    });
  };

  const handleRemoveCustomProperty = (index: number) => {
    if (!config) return;
    const updated = config.task_custom_properties.filter((_, i) => i !== index);
    setLocalConfig({ ...config, task_custom_properties: updated });
  };

  const handleCustomPropChange = (index: number, key: keyof CustomProperty, val: any) => {
    if (!config) return;
    const updated = [...config.task_custom_properties];
    updated[index] = { ...updated[index], [key]: val };
    setLocalConfig({ ...config, task_custom_properties: updated });
  };

  const handleSave = async () => {
    if (!config || saving) return;
    setSaving(true);
    try {
      await setConfig(config);
      // Re-register hotkey
      try {
        await invoke("update_hotkey", { shortcut: config.global_shortcut });
      } catch (hotkeyErr: any) {
        console.warn("Hotkey update warning:", hotkeyErr);
      }

      onSaved(config);
      showToast("✓ 設定を保存しました", "success");
      onClose();
    } catch (e: any) {
      showToast(typeof e === "string" ? e : "設定の保存に失敗しました", "error");
    } finally {
      setSaving(false);
    }
  };

  if (!isOpen || !config) return null;

  return (
    <div className="fixed inset-0 z-40 bg-black/50 backdrop-blur-xs flex items-center justify-center p-4">
      <div className="w-full max-w-lg bg-card border text-card-foreground rounded-2xl shadow-2xl overflow-hidden flex flex-col max-h-[92vh]">
        {/* Header */}
        <div className="flex items-center justify-between px-5 py-3.5 border-b bg-muted/40">
          <div className="flex items-center gap-2">
            <Settings className="size-4 text-primary" />
            <h2 className="text-sm font-semibold">ランチャー設定</h2>
          </div>
          <Button
            variant="ghost"
            size="icon-xs"
            onClick={onClose}
          >
            <X />
          </Button>
        </div>

        {/* Scrollable Form Body */}
        <div className="flex-1 overflow-y-auto p-5 space-y-4 text-xs">
          {/* Obsidian Vault Path */}
          <div>
            <Label className="text-xs mb-1">
              Obsidian Vault パス <span className="text-destructive">*</span>
            </Label>
            <div className="flex gap-2">
              <Input
                type="text"
                value={config.vault_path}
                onChange={(e) => handleChange("vault_path", e.target.value)}
                placeholder="C:/Users/name/Documents/ObsidianVault"
                className="flex-1 text-xs"
              />
            </div>
            {/* Vault Status Indicator */}
            <div className="mt-1.5 flex items-center gap-1.5 text-[11px]">
              {validating ? (
                <span className="text-muted-foreground">検証中...</span>
              ) : vaultStatus?.is_valid ? (
                <span className="text-emerald-500 font-medium flex items-center gap-1">
                  ✓ 有効なVaultディレクトリ {vaultStatus.has_obsidian_folder && "(.obsidian 検出済み)"}
                </span>
              ) : vaultStatus ? (
                <span className="text-destructive flex items-center gap-1">
                  ⚠ {vaultStatus.message || "無効なディレクトリです"}
                </span>
              ) : (
                <span className="text-muted-foreground">Vaultフォルダの絶対パスを指定してください</span>
              )}
            </div>
          </div>

          {/* Task Folder */}
          <div>
            <Label className="text-xs mb-1">
              タスク保存先フォルダ
            </Label>
            <Input
              type="text"
              value={config.tasks_folder}
              onChange={(e) => handleChange("tasks_folder", e.target.value)}
              placeholder="Tasks"
              className="text-xs"
            />
          </div>

          {/* Worklog Section Heading */}
          <div>
            <Label className="text-xs mb-1">
              作業ログ走査見出し
            </Label>
            <Input
              type="text"
              value={config.worklog_section_heading}
              onChange={(e) => handleChange("worklog_section_heading", e.target.value)}
              placeholder="## 作業ログ"
              className="text-xs"
            />
            <p className="mt-1 text-[10px] text-muted-foreground">
              ※ デイリーノート内のこの見出し配下に「### hh:mm」の小見出し形式で時系列に追記されます。デイリーノートの保存先フォルダ・日付書式・テンプレートは Obsidian コアプラグイン設定 (.obsidian/daily-notes.json) に自動連携します。
            </p>
          </div>

          {/* Anchor Settings */}
          <div className="p-3 bg-muted/30 border rounded-xl space-y-2.5">
            <div className="flex items-center gap-2">
              <Checkbox
                id="anchor-toggle"
                checked={config.worklog_use_anchor}
                onCheckedChange={(checked) => handleChange("worklog_use_anchor", !!checked)}
              />
              <Label htmlFor="anchor-toggle" className="text-xs cursor-pointer">
                アンカーコメント位置への追記を優先する
              </Label>
            </div>
            <div>
              <Label className="text-xs mb-1 text-muted-foreground">
                アンカーコメント文字列
              </Label>
              <Input
                type="text"
                value={config.worklog_anchor_comment}
                onChange={(e) => handleChange("worklog_anchor_comment", e.target.value)}
                placeholder="%% worklog %%"
                disabled={!config.worklog_use_anchor}
                className="text-xs font-mono"
              />
              <p className="mt-1 text-[10px] text-muted-foreground">
                有効時、デイリーノート内のこの行の直前に作業ログを挿入します（未検出時は上記見出しへフォールバック）。
              </p>
            </div>
          </div>

          {/* Shortcut & Behavior */}
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label className="text-xs mb-1">
                起動ホットキー (OSグローバル)
              </Label>
              <Input
                type="text"
                value={config.global_shortcut}
                onChange={(e) => handleChange("global_shortcut", e.target.value)}
                placeholder="Win+Shift+Q"
                className="text-xs font-mono"
              />
            </div>
            <div>
              <Label className="text-xs mb-1">
                デフォルト起動モード
              </Label>
              <Select
                value={config.default_mode}
                onValueChange={(val) => handleChange("default_mode", val)}
              >
                <SelectTrigger className="w-full text-xs">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="worklog">作業実績 (Worklog)</SelectItem>
                  <SelectItem value="task">タスク作成 (Task)</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          {/* Blur auto hide toggle */}
          <div className="flex items-center gap-2 pt-1">
            <Checkbox
              id="blur-toggle"
              checked={config.auto_hide_on_blur}
              onCheckedChange={(checked) => handleChange("auto_hide_on_blur", !!checked)}
            />
            <Label htmlFor="blur-toggle" className="text-xs cursor-pointer">
              フォーカス喪失時 (ウィンドウ外クリック) に自動で非表示にする
            </Label>
          </div>

          {/* Custom Task Properties */}
          <div className="pt-3 border-t">
            <div className="flex items-center justify-between mb-2">
              <Label className="text-xs font-semibold">
                タスク拡張プロパティ設定 (フロントマター)
              </Label>
              <Button
                type="button"
                variant="ghost"
                size="xs"
                onClick={handleAddCustomProperty}
                className="text-primary hover:text-primary/80"
              >
                <Plus /> プロパティ追加
              </Button>
            </div>

            <div className="space-y-2">
              {config.task_custom_properties.map((prop, idx) => (
                <div key={idx} className="flex items-center gap-2 bg-muted/20 p-2 rounded-lg border">
                  <Input
                    type="text"
                    value={prop.key}
                    onChange={(e) => handleCustomPropChange(idx, "key", e.target.value)}
                    placeholder="キー (例: estimate)"
                    className="flex-1 text-xs"
                  />
                  <Input
                    type="text"
                    value={prop.label}
                    onChange={(e) => handleCustomPropChange(idx, "label", e.target.value)}
                    placeholder="ラベル (例: 見積もり)"
                    className="flex-1 text-xs"
                  />
                  <Select
                    value={prop.type}
                    onValueChange={(val) => handleCustomPropChange(idx, "type", val)}
                  >
                    <SelectTrigger size="sm" className="w-24 text-xs">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="text">テキスト</SelectItem>
                      <SelectItem value="number">数値</SelectItem>
                    </SelectContent>
                  </Select>
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon-xs"
                    onClick={() => handleRemoveCustomProperty(idx)}
                    className="text-muted-foreground hover:text-destructive"
                  >
                    <Trash2 />
                  </Button>
                </div>
              ))}
              {config.task_custom_properties.length === 0 && (
                <div className="text-[11px] text-muted-foreground py-1 text-center">
                  追加されたカスタムプロパティはありません
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-2 px-5 py-3 border-t bg-muted/40">
          <Button
            variant="ghost"
            size="sm"
            onClick={onClose}
          >
            キャンセル
          </Button>
          <Button
            size="sm"
            onClick={handleSave}
            disabled={saving}
          >
            {saving ? "保存中..." : "設定を保存"}
          </Button>
        </div>
      </div>
    </div>
  );
};
