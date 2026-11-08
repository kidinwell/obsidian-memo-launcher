import React, { useEffect, useState } from "react";
import { AppConfig, CustomProperty, VaultValidationResult } from "../types/config";
import { getConfig, setConfig, validateVault } from "../services/configService";
import { invoke } from "@tauri-apps/api/core";

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
    <div className="fixed inset-0 z-40 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
      <div className="w-full max-w-lg bg-zinc-900 border border-zinc-800 rounded-2xl shadow-2xl overflow-hidden flex flex-col max-h-[92vh]">
        {/* Header */}
        <div className="flex items-center justify-between px-5 py-3.5 border-b border-zinc-800 bg-zinc-900/50">
          <div className="flex items-center gap-2">
            <svg className="w-4 h-4 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <h2 className="text-sm font-semibold text-zinc-100">ランチャー設定</h2>
          </div>
          <button
            onClick={onClose}
            className="text-zinc-400 hover:text-zinc-200 p-1 rounded-lg hover:bg-zinc-800 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Scrollable Form Body */}
        <div className="flex-1 overflow-y-auto p-5 space-y-4 text-xs">
          {/* Obsidian Vault Path */}
          <div>
            <label className="block text-[11px] font-medium text-zinc-300 mb-1">
              Obsidian Vault パス <span className="text-rose-400">*</span>
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={config.vault_path}
                onChange={(e) => handleChange("vault_path", e.target.value)}
                placeholder="C:/Users/name/Documents/ObsidianVault"
                className="flex-1 px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              />
            </div>
            {/* Vault Status Indicator */}
            <div className="mt-1.5 flex items-center gap-1.5 text-[11px]">
              {validating ? (
                <span className="text-zinc-400">検証中...</span>
              ) : vaultStatus?.is_valid ? (
                <span className="text-emerald-400 flex items-center gap-1">
                  ✓ 有効なVaultディレクトリ {vaultStatus.has_obsidian_folder && "(.obsidian 検出済み)"}
                </span>
              ) : vaultStatus ? (
                <span className="text-rose-400 flex items-center gap-1">
                  ⚠ {vaultStatus.message || "無効なディレクトリです"}
                </span>
              ) : (
                <span className="text-zinc-500">Vaultフォルダの絶対パスを指定してください</span>
              )}
            </div>
          </div>

          {/* Folders */}
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-[11px] font-medium text-zinc-300 mb-1">
                タスク保存先フォルダ
              </label>
              <input
                type="text"
                value={config.tasks_folder}
                onChange={(e) => handleChange("tasks_folder", e.target.value)}
                placeholder="Tasks"
                className="w-full px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              />
            </div>
            <div>
              <label className="block text-[11px] font-medium text-zinc-300 mb-1">
                デイリーノート保存先フォルダ
              </label>
              <input
                type="text"
                value={config.daily_notes_folder}
                onChange={(e) => handleChange("daily_notes_folder", e.target.value)}
                placeholder="Daily"
                className="w-full px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              />
            </div>
          </div>

          {/* Daily Note Settings */}
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-[11px] font-medium text-zinc-300 mb-1">
                日付書式 (Format)
              </label>
              <input
                type="text"
                value={config.daily_note_format}
                onChange={(e) => handleChange("daily_note_format", e.target.value)}
                placeholder="YYYY-MM-DD"
                className="w-full px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              />
            </div>
            <div>
              <label className="block text-[11px] font-medium text-zinc-300 mb-1">
                作業ログ走査見出し
              </label>
              <input
                type="text"
                value={config.worklog_section_heading}
                onChange={(e) => handleChange("worklog_section_heading", e.target.value)}
                placeholder="## 作業ログ"
                className="w-full px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              />
            </div>
          </div>

          <div>
            <label className="block text-[11px] font-medium text-zinc-300 mb-1">
              デイリーノートテンプレート (相対パス)
            </label>
            <input
              type="text"
              value={config.daily_note_template_path}
              onChange={(e) => handleChange("daily_note_template_path", e.target.value)}
              placeholder="Templates/Daily Note.md (未設定時は自動検出)"
              className="w-full px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-indigo-500"
            />
          </div>

          {/* Shortcut & Behavior */}
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-[11px] font-medium text-zinc-300 mb-1">
                起動ホットキー (OSグローバル)
              </label>
              <input
                type="text"
                value={config.global_shortcut}
                onChange={(e) => handleChange("global_shortcut", e.target.value)}
                placeholder="Win+Shift+Q"
                className="w-full px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-indigo-500 font-mono"
              />
            </div>
            <div>
              <label className="block text-[11px] font-medium text-zinc-300 mb-1">
                デフォルト起動モード
              </label>
              <select
                value={config.default_mode}
                onChange={(e) => handleChange("default_mode", e.target.value)}
                className="w-full px-3 py-1.5 bg-zinc-950 border border-zinc-800 rounded-lg text-zinc-200 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option value="worklog">作業実績 (Worklog)</option>
                <option value="task">タスク作成 (Task)</option>
              </select>
            </div>
          </div>

          {/* Blur auto hide toggle */}
          <div className="flex items-center gap-2 pt-1">
            <input
              type="checkbox"
              id="blur-toggle"
              checked={config.auto_hide_on_blur}
              onChange={(e) => handleChange("auto_hide_on_blur", e.target.checked)}
              className="w-4 h-4 rounded text-indigo-600 bg-zinc-950 border-zinc-800 focus:ring-indigo-500 cursor-pointer"
            />
            <label htmlFor="blur-toggle" className="text-xs text-zinc-300 select-none cursor-pointer">
              フォーカス喪失時 (ウィンドウ外クリック) に自動で非表示にする
            </label>
          </div>

          {/* Custom Task Properties */}
          <div className="pt-3 border-t border-zinc-800">
            <div className="flex items-center justify-between mb-2">
              <label className="text-[11px] font-semibold text-zinc-300">
                タスク拡張プロパティ設定 (フロントマター)
              </label>
              <button
                type="button"
                onClick={handleAddCustomProperty}
                className="text-[11px] text-indigo-400 hover:text-indigo-300 flex items-center gap-1 cursor-pointer"
              >
                + プロパティ追加
              </button>
            </div>

            <div className="space-y-2">
              {config.task_custom_properties.map((prop, idx) => (
                <div key={idx} className="flex items-center gap-2 bg-zinc-950/60 p-2 rounded-lg border border-zinc-800/80">
                  <input
                    type="text"
                    value={prop.key}
                    onChange={(e) => handleCustomPropChange(idx, "key", e.target.value)}
                    placeholder="キー (例: estimate)"
                    className="flex-1 px-2 py-1 bg-zinc-900 border border-zinc-800 rounded text-xs text-zinc-200 placeholder-zinc-600"
                  />
                  <input
                    type="text"
                    value={prop.label}
                    onChange={(e) => handleCustomPropChange(idx, "label", e.target.value)}
                    placeholder="ラベル (例: 見積もり)"
                    className="flex-1 px-2 py-1 bg-zinc-900 border border-zinc-800 rounded text-xs text-zinc-200 placeholder-zinc-600"
                  />
                  <select
                    value={prop.type}
                    onChange={(e) => handleCustomPropChange(idx, "type", e.target.value)}
                    className="px-2 py-1 bg-zinc-900 border border-zinc-800 rounded text-xs text-zinc-200"
                  >
                    <option value="text">テキスト</option>
                    <option value="number">数値</option>
                  </select>
                  <button
                    type="button"
                    onClick={() => handleRemoveCustomProperty(idx)}
                    className="p-1 text-zinc-500 hover:text-rose-400 cursor-pointer"
                  >
                    <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              ))}
              {config.task_custom_properties.length === 0 && (
                <div className="text-[11px] text-zinc-500 py-1 text-center">
                  追加されたカスタムプロパティはありません
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-2 px-5 py-3 border-t border-zinc-800 bg-zinc-900/50">
          <button
            onClick={onClose}
            className="px-3 py-1.5 text-xs text-zinc-400 hover:text-zinc-200 rounded-lg hover:bg-zinc-800 transition-colors cursor-pointer"
          >
            キャンセル
          </button>
          <button
            onClick={handleSave}
            disabled={saving}
            className="px-4 py-1.5 text-xs font-semibold rounded-lg bg-indigo-600 hover:bg-indigo-500 active:bg-indigo-700 disabled:opacity-50 text-white shadow transition-all cursor-pointer"
          >
            {saving ? "保存中..." : "設定を保存"}
          </button>
        </div>
      </div>
    </div>
  );
};
