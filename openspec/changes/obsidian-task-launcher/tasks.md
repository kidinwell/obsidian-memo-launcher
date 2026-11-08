# Tasks

## 1. Project Scaffolding & Setup

- [x] 1.1 Tauri v2 プロジェクトの初期化（Rust バックエンド + TypeScript + Tailwind CSS）を行い、ビルドおよび開発サーバーが正常に起動することを確認する
- [x] 1.2 ウィンドウ設定（フレームレス、最前面表示、ホットキー待機用の非表示初期状態）を構成し、ウィンドウの表示/非表示制御が機能することを検証する

## 2. Configuration Management

- [x] 2.1 設定管理モジュール（Vaultパス、タスクフォルダ名、初期モード、作業ログ走査見出し、ホットキー、タスク拡張プロパティ）を Rust 側で実装し、シリアライズ/デシリアライズの単体テストをパスすることを確認する
- [x] 2.2 フロントエンドから設定値を取得・保存・検証する Tauri コマンドを実装し、テスト呼び出しで設定値が正常に読み書きできることを検証する
- [x] 2.3 設定モデルおよび設定UIにアンカーコメントオプション（`worklog_use_anchor: bool`, `worklog_anchor_comment: String`）を追加する

## 3. Global Hotkey & Window Management

- [x] 3.1 グローバルショートカット登録（デフォルト: `Win+Shift+Q`）を実装し、別アプリがアクティブな状態からホットキー押下でランチャーが前面表示されることを確認する
- [x] 3.2 `Escape` キー押下、フォーカス喪失（blur）、および送信完了時の自動非表示・フォーカス復帰処理を実装し、スムーズに元のアプリへ戻ることを確認する
- [x] 3.3 タスクトレイ（システムトレイ）アイコンおよびトレイメニュー（表示、設定、終了）を実装し、トレイからの操作を確認する

## 4. Task Capture Implementation

- [x] 4.1 Obsidian タスクノート生成ロジック（ファイル名サニタイズ、重複回避、YAMLフロントマター生成、本文書き込み）を Rust 側で実装し、単体テストで正しい Markdown ファイルが出力されることを検証する
- [x] 4.2 タスク作成 UI（タイトル、依頼者、締め切り、本文、および拡張プロパティ入力フォーム）を実装し、`Ctrl+Enter` で Vault のタスクフォルダへノートが生成されることを確認する

## 5. Worklog Capture Implementation

- [x] 5.1 Obsidian コアプラグイン設定（`.obsidian/daily-notes.json`）の読み込みおよび本日分デイリーノートのテンプレート適用ロジックを実装し、設定が存在する場合・未設定時のフォールバック双方の単体テストをパスすることを確認する
- [x] 5.2 設定された走査見出し（`worklog_section_heading`）に基づくデイリーノートへの時系列追記ロジック（`hh:mm` タイムスタンプ + 複数行メモの WebVTT 類似フォーマット）を実装し、該当セクション直下または末尾へ正確に挿入される単体テストをパスすることを確認する
- [x] 5.3 作業実績入力 UI（オートフォーカスの複数行メモ入力欄）を実装し、`Ctrl+Enter` 押下でデイリーノートへ追記されることを確認する
- [x] 5.4 アンカーオプション有効時のアンカー直前挿入ロジック（およびアンカー未検出時の見出しフォールバック）の実装と単体テスト

## 6. UI Polish & Keyboard Navigation

- [x] 6.1 デフォルト起動モード（作業実績入力画面の自動選択およびメモ欄オートフォーカス）とタスク入力への切り替え機能（タブUIおよび `Ctrl+Tab` / `Alt+1` / `Alt+2`）を実装し、入力保持とフォーカス移動を確認する
- [x] 6.2 送信成功時の軽量フィードバック（トーストやインジケータ表示）と自動ウィンドウ非表示を実装し、操作感を検証する
- [x] 6.3 Vault パス選択およびフォルダ設定画面を実装し、設定変更が即座に反映されることを確認する

## 7. End-to-End Verification

- [x] 7.1 テスト用 Vault 環境を用意し、ホットキー起動からタスク作成を行い、Obsidian 側でプロパティと本文が正常に認識されることを検証する
- [x] 7.2 テスト用 Vault 環境でデイリーノートへの複数回の作業実績追記を行い、時系列に沿って設定見出し直下へきれいに追記されることを検証する
- [x] 7.3 `%% worklog %%` アンカーコメントを含むデイリーノートテンプレートへの作業ログ追記E2Eテスト

## 8. Remove Redundant Daily Note Settings

- [x] 8.1 ランチャー設定モデル（`AppConfig`）から `daily_notes_folder`, `daily_note_format`, `daily_note_template_path` を削除し、デイリーノート解決ロジックが `.obsidian/daily-notes.json` および Obsidian コアプラグインのデフォルト値（`YYYY-MM-DD`、Vault ルート）のみを参照するよう Rust 実装と単体テストを更新する
- [x] 8.2 フロントエンド設定画面（`SettingsModal.tsx`）および設定サービス・型定義からデイリーノート関連の入力項目（保存先フォルダ、日付書式、テンプレート）を削除し、UI を整理する
- [x] 8.3 デイリーノート自動生成・追記および設定保存の E2E / 統合テストを更新し、Obsidian コア設定のみで正しく動作することを検証する

## 9. Moment.js Daily Note Date Format & Weekday Support

- [x] 9.1 Moment.js 互換の日付フォーマッタ（最長一致トークン置換、`[...]` エスケープ対応、`ddd` / `dddd` / `dd` などのトークン衝突回避、およびロケールに応じた曜日置換）を Rust 側で実装し、単体テストを追加する
- [x] 9.2 デイリーノート解決処理（`ensure_daily_note_exists`）でロケール（`.obsidian/app.json` またはシステムロケール）に基づくファイル名（例: `YYYY-MM-DD(ddd)` → `2026-11-08(日).md`）が正しく認識され、既存ノートの重複作成を防ぐことを検証する

## 10. Heading-based Worklog Entry Formatting

- [x] 10.1 Rust バックエンドのログ整形処理（`format_worklog_entry`）を小見出し形式（`### hh:mm\n{memo}`）に改修し、単体テストを更新・パスさせる
- [x] 10.2 セクション挿入（`insert_entry_into_content`）およびアンカー挿入（`insert_entry_before_anchor`）で `### hh:mm` 形式のエントリが正しく配置されることを検証する
- [x] 10.3 フロントエンドのプレースホルダーやヘルプテキスト等を微調整し、E2E での記録結果を確認する

## Workflow follow-up

- Archive the change after the project's review requirements are satisfied.
- Verify the archived result.
