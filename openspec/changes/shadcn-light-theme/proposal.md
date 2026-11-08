# Proposal

## Why

現在のランチャーUIは Tailwind CSS の特定カラークラス（`bg-zinc-950`, `border-zinc-800` 等のダーク固定クラス）が全コンポーネントにベタ書きされており、ライトテーマやOSのシステム設定連動に対応できていません。
コンポーネントを shadcn UI を基盤とした再利用可能なパーツへ置き換え、CSS変数を介したセマンティックトークン設計に統一することで、保守性とアクセシビリティを向上させます。また、システム設定（`prefers-color-scheme`）に応じてテーマを切り替えられる設計基盤を導入し、まずはライトテーマの表示を確実に整備します。

## What Changes

- **shadcn UI コンポーネントへの刷新**:
  - ベタ書きされている原生 HTML 要素や Tailwind クラスを、shadcn/ui コンポーネント（`Button`, `Input`, `Textarea`, `Dialog` / `Modal`, `Tabs`, `Toast`, `Label` 等）およびセマンティックカラー（`bg-background`, `text-foreground`, `border-border`, `bg-card` 等）に置き換え。
- **システム設定連動のテーマ管理基盤の導入**:
  - OS の `prefers-color-scheme` を検知・監視し、HTML/ルート要素へのテーマクラス適用を管理するテーマプロバイダー/フックを設計・導入。
  - 将来的な手動切り替え（Light / Dark / System）への拡張性を持った設計としつつ、まずはライトテーマが快適に機能するようトークン・スタイリングを最適化。
- **主要画面のライトテーマ対応**:
  - ランチャーメインウィンドウ、ヘッダー（タブ切り替え部）、作業実績入力フォーム（WorklogForm）、タスク追加フォーム（TaskForm）、設定モーダル（SettingsModal）、トースト通知（Toast）のライトテーマ対応。
- **既存操作性・機能の維持**:
  - グローバルショートカット、Esc による閉じる動作、Ctrl+Tab / Alt+1 / Alt+2 によるタブ切り替え、Ctrl+Enter による保存、および起動・復帰時のオートフォーカス挙動をすべて維持。

## Capabilities

### New Capabilities
- `theme-management`: OSのシステムカラー設定（`prefers-color-scheme`）の監視・同期を行い、セマンティックなCSS変数に基づくテーマ適用（ライト/ダーク）を制御する基盤。

### Modified Capabilities
- `quick-launcher`: ランチャーウィンドウ、各入力フォーム、モーダルが shadcn UI コンポーネントとセマンティックトークンでレンダリングされ、システム設定に応じたライトテーマ表示に対応する要件を追加。

## Impact
- **コンポーネント & スタイル**:
  - `src/App.css`: shadcn テーマ変数と Tailwind v4 のカラーバインディング調整
  - `src/App.tsx`: テーマ基盤の適用とメインラッパーのセマンティッククラス化
  - `src/components/Header.tsx`, `TaskForm.tsx`, `WorklogForm.tsx`, `SettingsModal.tsx`, `Toast.tsx`: shadcn UI コンポーネント化とカラーベタ書きの排除
  - `src/components/ui/*`: 必要な shadcn コンポーネント群（`input.tsx`, `textarea.tsx`, `dialog.tsx` など）の整備
- **テスト**:
  - `src/App.focus.test.tsx` などの既存テストのセレクタ整合性（`data-testid` などの保持）と回帰防止
