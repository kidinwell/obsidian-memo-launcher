# Design

## Context

現在の UI は Tailwind CSS のハードコードされたダークカラー（`bg-zinc-950`, `bg-zinc-900`, `border-zinc-800`, `text-zinc-100` 等）で直接スタイルされています。
リポジトリには既に `components.json` および Tailwind v4 ベースの `src/App.css` が導入され、`:root` (ライトテーマ用CSS変数) と `.dark` (ダークテーマ用CSS変数) が定義されています。
また、`src/components/ui/` 配下には、`@base-ui/react` ベースの以下の 10 種類の shadcn コンポーネントがインストール済みです：
- `button.tsx`, `card.tsx`, `checkbox.tsx`, `input.tsx`, `kbd.tsx`, `label.tsx`, `select.tsx`, `tabs.tsx`, `textarea.tsx`, `toast.tsx`

本設計では、これらの UI 部品を活用して全画面をセマンティッククラスへ移行し、OSのシステム設定（`prefers-color-scheme`）に連動してテーマを切り替えられる設計基盤を構築した上で、まずはライトテーマの表示品質を完成させます。

## Goals / Non-Goals

**Goals:**
- **システム設定連動テーマアーキテクチャ**:
  - `prefers-color-scheme: dark` / `light` を検知・動的監視し、ルート要素の `.dark` クラスを制御するテーマ管理機構（`useTheme` / `theme.ts`）を導入。
  - 将来的な手動切り替え（System / Light / Dark）を容易にサポートできる設計とする。
- **shadcn UI コンポーネントの全面活用**:
  - インストール済みの 10 種類のコンポーネント（`Card`, `Tabs`, `Button`, `Input`, `Textarea`, `Checkbox`, `Select`, `Label`, `Kbd`, `Toast`）を各画面に適用。
  - `src/lib/utils.ts` の `cn` ユーティリティを活用。
- **ライトテーマの徹底対応**:
  - `App`, `Header`, `WorklogForm`, `TaskForm`, `SettingsModal`, `Toast` の全画面から Tailwind のダークベタ書きを排除し、セマンティックトークン（`bg-background`, `text-foreground`, `bg-card`, `border-border`, `focus-visible:ring-ring` 等）に置換。
  - ライトテーマにおけるテキストの可読性、入力枠の視認性、フォーカスリングの明瞭さを確保。
- **機能と操作性の維持**:
  - オートフォーカス（起動時・タブ切替時・モーダル閉時）、キーボードショートカット（Esc, Ctrl+Tab, Ctrl+Enter, Alt+1, Alt+2）、Obsidian 連携処理の互換性を完全維持。

**Non-Goals:**
- 設定モーダル内での手動テーマ切り替えセレクター（System / Light / Dark）の UI 項目追加（アーキテクチャ上は切り替え関数を受け入れ可能にするが、今回のスコープはシステム連動の基盤導入とライトテーマの見た目対応）。
- ダークテーマの細部カラー調整（CSS変数定義は維持しつつ、今回はライトテーマ表示を最優先で仕上げる）。

## Decisions

### 1. テーマ管理基盤: `useTheme` フックと `prefers-color-scheme` 監視
- **決定**: クライアントサイドで `window.matchMedia('(prefers-color-scheme: dark)')` を監視し、`document.documentElement.classList.toggle('dark', matches)` を制御する軽量なテーマ管理（`useTheme` または `initThemeListener`）を導入。
- **根拠**:
  - Tailwind v4 の `@custom-variant dark (&:is(.dark *));` と直接整合。
  - OS設定変更イベント（`change` イベント）を即時反映可能。
  - Tauri バックエンドへの依存を増やさず、Web標準 API だけでテストも容易。
- **代替案**:
  - Tauri のプラグインや Rust バックエンド経由で OS テーマを取得する方法: Rust側の依存関係と非同期通信オーバーヘッドが増えるため不採用。

### 2. インストール済み shadcn コンポーネントのマッピング方針
- **決定**:
  - **メインウィンドウ (`App.tsx`)**: `Card` コンポーネントおよびセマンティッククラス（`bg-card/95 border border-border shadow-2xl`）でラップ。
  - **ヘッダー (`Header.tsx`)**: タブ切り替えを `Tabs` / `TabsList` / `TabsTrigger`、キー番号を `Kbd`、設定・閉じるボタンを `Button` (ghost, icon) に置換。
  - **作業実績・タスク入力 (`WorklogForm`, `TaskForm`)**: `Textarea`, `Input`, `Label`, `Button` を適用し、ショートカットガイドに `Kbd` を適用。
  - **設定画面 (`SettingsModal.tsx`)**: 各入力欄に `Input`、トグルに `Checkbox`、ドロップダウンに `Select` を適用。
  - **トースト通知 (`Toast.tsx`)**: `toast.tsx` またはセマンティックトークン対応。
- **根拠**:
  - 10個のコンポーネントがすべて `@base-ui/react` に揃っており、統一感のあるデザインシステムとアクセシビリティが保証される。

### 3. ライトテーマでの視認性とウィンドウ境界の確保
- **決定**:
  - ランチャーメインコンテナ（`App.tsx`）を `bg-card/95 border border-border shadow-2xl backdrop-blur-xl` に変更。
  - タブやボタン、入力フォームの各フィールド（`Input`, `Textarea`）に `bg-muted/50` または `bg-background`、`border-input` を適用し、十分なコントラスト比を確保。
- **根拠**:
  - デスクトップの背景が明るい場合でも、影（shadow）とボーダー（border）によってモーダルウィンドウの境界がクリアに認識できる。

## Risks / Trade-offs

- **[Risk] 入力フィールドのオートフォーカスが shadcn コンポーネント化で阻害される可能性**
  → **Mitigation**: `@base-ui/react` ベースの `Input`, `Textarea` が ref を正しく透過することを確認し、既存の `titleInputRef` や `memoTextareaRef` の focus 処理および `setSelectionRange` を維持する。
- **[Risk] 既存テストのセレクタ破壊**
  → **Mitigation**: `data-testid` 属性（`tab-worklog-btn`, `tab-task-btn`, `task-title-input`, `worklog-memo-input`, `open-settings-btn`, `close-launcher-btn` 等）をすべて維持。
