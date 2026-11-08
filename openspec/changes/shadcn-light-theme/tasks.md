# Tasks

## 1. テーマ管理基盤とトークンの整備

- [ ] 1.1 テーマ管理ユーティリティ（`src/lib/theme.ts` または `src/hooks/useTheme.ts`）を実装し、`window.matchMedia('(prefers-color-scheme: dark)')` のリスナー登録および `document.documentElement` への `.dark` クラス切り替え動作を検証する
- [ ] 1.2 `src/lib/utils.ts` の `cn` ヘルパー（`clsx` と `tailwind-merge`）の型定義とエクスポートを確認・整備する
- [ ] 1.3 `src/App.css` のライトテーマ用CSS変数（`:root`）を調整し、ライトモードでのカード背景、ボーダー、入力枠、テキストのコントラストと視認性を検証する

## 2. インストール済み shadcn UI コンポーネントの動作確認と調整

- [ ] 2.1 `input.tsx` および `textarea.tsx` が ref（フォーカス制御・選択範囲設定）を正常に受け渡せることをテストで検証する
- [ ] 2.2 `button.tsx`, `card.tsx`, `checkbox.tsx`, `label.tsx`, `tabs.tsx` のライトテーマでの表示スタイルとコントラストを検証する
- [ ] 2.3 `select.tsx`, `kbd.tsx`, `toast.tsx` の動作およびライトテーマでの表示スタイルを検証する

## 3. 各画面コンポーネントの shadcn 化とライトテーマ対応

- [ ] 3.1 `src/App.tsx` にテーマ初期化フックを適用し、メインランチャーコンテナを `Card` コンポーネント / セマンティッククラス（`bg-card/95 border border-border shadow-2xl`）に刷新して表示を検証する
- [ ] 3.2 `src/components/Header.tsx` のタブボタンを `Tabs` / `Kbd`、設定・閉じるボタンを `Button` (ghost) に刷新し、ライトテーマでの表示と `data-testid` 属性を検証する
- [ ] 3.3 `src/components/WorklogForm.tsx` のメモ入力欄を `Textarea`、送信ボタンを `Button`、キー案内を `Kbd` に置き換え、Ctrl+Enter 送信、オートフォーカス、ライトテーマでの表示を検証する
- [ ] 3.4 `src/components/TaskForm.tsx` のタイトル、依頼者、締切日、カスタムプロパティ、本文入力欄、キー案内を shadcn コンポーネント（`Input`, `Textarea`, `Button`, `Kbd`）に置き換え、入力イベント・ショートカット・フォーカス復帰を検証する
- [ ] 3.5 `src/components/SettingsModal.tsx` の各入力欄を `Input`、トグルを `Checkbox`、ドロップダウンを `Select` に刷新し、モーダル開閉、設定保存、ライトテーマでの可読性を検証する
- [ ] 3.6 `src/components/Toast.tsx` を `toast.tsx` またはセマンティックカラートークンに刷新し、成功・エラー時のトーストがライトテーマ上で明瞭に視認できることを検証する

## 4. 統合検証とリグレッションテスト

- [ ] 4.1 既存テスト（`pnpm test`）を実行し、`App.focus.test.tsx` のオートフォーカステストがすべてパスすることを検証する
- [ ] 4.2 TypeScript 型チェックおよびプロダクションビルド（`pnpm run build`）を実行し、コンパイルエラーがないことを検証する
- [ ] 4.3 OS システム設定のダーク/ライト切り替えに伴い、画面全体の配色がスムーズに追従することをブラウザまたはアプリプレビューで検証する
