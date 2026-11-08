# Obsidian Task Launcher

[![Tauri v2](https://img.shields.io/badge/Tauri-v2-24C8D8?logo=tauri&logoColor=white)](https://tauri.app/)
[![React 19](https://img.shields.io/badge/React-19-61DAFB?logo=react&logoColor=black)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tailwind CSS v4](https://img.shields.io/badge/Tailwind_CSS-v4-06B6D4?logo=tailwindcss&logoColor=white)](https://tailwindcss.com/)
[![Platform](https://img.shields.io/badge/Platform-Windows-0078D6?logo=windows&logoColor=white)](https://www.microsoft.com/windows)

**Obsidian Task Launcher** は、作業中の思考や集中を一切中断することなく、OS上のどこからでも瞬時に作業実績メモやタスクを Obsidian Vault へ直接記録できる、Windows向けの超軽量・高速デスクトップランチャーです。

Windows標準の `Win+R` のように、左手中心のグローバルショートカット（デフォルト: `Win+Shift+Q`）で即座にポップアップを呼び出し、キーボードのみで記録を完結できます。

---

## 主な特徴 (Features)

### 1. ⚡ OSグローバルホットキーによる超高速起動
- **どこからでも瞬時に呼び出し**: 他のアプリで作業中でも、`Win+Shift+Q` で即座に入力画面が前面に表示されます。
- **超軽量・低リソース設計**: バックグラウンド待機時の常駐メモリは約 30〜50MB。起動レイテンシを極限まで削減し、瞬時の入力を実現。
- **邪魔にならないウィンドウ制御**: 入力完了時（`Ctrl+Enter`）や `Escape` キー、ウィンドウ外のクリック（フォーカス喪失）で自動的に非表示となり、直前の作業へシームレスに復帰します。
- **システムトレイ常駐**: タスクトレイアイコンからいつでも設定画面の表示やアプリの終了が可能です。

### 2. 📝 作業ログ（作業実績）の即時追記 (Worklog Mode - デフォルト)
- **起動直後にすぐメモ可能**: ウィンドウ表示と同時にメモ欄へオートフォーカス。
- **Obsidianと美しく親和する小見出し形式**: `### hh:mm` の小見出しタイムスタンプ付きでデイリーノートへ時系列追記されます。Markdownのリストやチェックボックスを本文に含めても階層が壊れず、Obsidianのアウトライン（目次）や折りたたみ（Folding）に対応します。
- **Obsidianコア設定 (`.obsidian/daily-notes.json`) との自動連携**:
  - 保存先フォルダ・日付フォーマット・テンプレートを Obsidian 側の設定から自動検出して適用します。
  - Moment.js準拠の日付書式や曜日表記（例: `2026-11-08(日).md` などの `ddd`/`dddd` トークンや日本語ロケール）にも完全対応。
  - 当日分のデイリーノートが存在しない場合は、設定されたテンプレートから日付トークンを自動置換して新規生成します。
- **アンカーコメント追記オプション**: デイリーノート内に埋め込んだ `%% worklog %%` などのアンカーコメント行の直前へ優先して追記する設定もサポート。

### 3. ✅ タスクノートのクイック作成 (Task Mode)
- **個別タスクの素早い作成**: タイトル、依頼者（requester）、締め切り（due）、本文を入力し、Vault内のタスクフォルダ（デフォルト: `Tasks/`）へ独立したMarkdownノートとして保存。
- **安全なファイル名生成**: ファイル名禁止文字（`\ / : * ? " < > |`）のエスケープや同名ファイル時の自動連番付与に対応。
- **構造化されたフロントマター**: `requester`, `due`, `completed: null` 等の YAML frontmatter を自動生成。
- **カスタムプロパティ拡張**: 見積もり工数や重要度など、任意の拡張プロパティを設定画面から自由に追加可能（テキスト／数値型対応）。

### 4. ⌨️ 洗練されたキーボードナビゲーション
- すべての主要操作がキーボードショートカットで完結するように設計されています。

| ショートカット | 動作 |
|---|---|
| `Win+Shift+Q` (変更可能) | ランチャーウィンドウの表示 / 非表示トグル |
| `Ctrl+Enter` | 入力内容の保存・Obsidianへの書き込み（自動でウィンドウが閉じます） |
| `Ctrl+Tab` | 「作業実績」モード ⇔ 「タスク追加」モードの相互切り替え |
| `Alt+1` | 「作業実績」モードへ切り替え |
| `Alt+2` | 「タスク追加」モードへ切り替え |
| `Escape` | ウィンドウの非表示（設定画面が開いている場合は設定を閉じる） |

---

## 画面イメージとワークフロー (Workflow)

```
[OSグローバルホットキー (Win+Shift+Q)]
                 │
                 ▼
     ┌───────────────────────┐
     │  Obsidian Launcher   ⚙│
     │ ┌─────────┬─────────┐ │
     │ │[1]作業実績│[2]タスク │ │
     ├─┴─────────┴─────────┴─┤
     │  ### 14:35            │
     │  ┌──────────────────┐ │
     │  │レビュー依頼対応     │ │
     │  │- PR #123 の確認   │ │
     │  └──────────────────┘ │
     │ [Esc:閉じる] [Ctrl+Enter:送信] │
     └───────────────────────┘
                 │ (Ctrl+Enter)
                 ▼
    Obsidian Vault へ直接追記
  ・当日のデイリーノート (作業ログ)
  ・または Tasks フォルダ (タスク)
```

---

## 設定項目 (Configuration)

画面右上の歯車アイコン（またはタスクトレイメニューの「設定」）から設定モーダルを開けます。

- **Obsidian Vault パス**: 記録対象となる Obsidian Vault の絶対パス（`.obsidian` フォルダの存在を自動バリデーション）。
- **タスク保存先フォルダ**: タスクノートを生成するフォルダ（デフォルト: `Tasks`）。
- **作業ログ走査見出し**: デイリーノート内でログを挿入する親見出し（デフォルト: `## 作業ログ`）。見出しが存在しない場合は末尾に自動生成されます。
- **アンカーコメント追記の優先**: テンプレート内の特定コメント行（デフォルト: `%% worklog %%`）位置への追記を有効化。
- **起動ホットキー**: OS全体で有効なグローバルショートカットキー（例: `Super+Shift+Q`, `Ctrl+Alt+Space` 等）。
- **デフォルト起動モード**: 起動時に初期表示するモード（作業実績 または タスク追加）。
- **フォーカス喪失時の自動非表示**: ウィンドウ外をクリックした際に自動で閉じるかどうか。
- **タスク拡張プロパティ**: タスク作成画面に追加するフロントマター項目の定義（キー、表示ラベル、データ型）。

> 設定ファイルは `%APPDATA%/obsidian-task-launcher/config.json` に自動保存されます。

---

## 技術スタック (Tech Stack)

- **Backend / Desktop**: [Tauri v2](https://tauri.app/) (Rust 2024 Edition)
  - ネイティブグローバルホットキー (`tauri-plugin-global-shortcut`)
  - システムトレイメニュー (`tray-icon`)
  - 高速ファイルI/Oおよびアトミック書き込み
  - Moment.js互換のトークナイズ・ロケール解決エンジン
- **Frontend**: [React 19](https://react.dev/), [TypeScript](https://www.typescriptlang.org/)
- **Bundler & Build**: [Vite 8](https://vite.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Testing**:
  - バックエンド: Rust Unit Tests & E2E Integration Tests (25+ tests)
  - フロントエンド: [Vitest](https://vitest.dev/), [Testing Library](https://testing-library.com/)

---

## 開発環境のセットアップ (Development)

### 前提条件 (Prerequisites)

- **OS**: Windows 10 / 11 (64-bit)
- **Node.js**: v18.0 以上
- **パッケージマネージャー**: `pnpm` (推奨) または `npm`
- **Rust**: 最新の Stable Rust toolchain (`rustup`)
- **C++ Build Tools**: Microsoft C++ Build Tools (Visual Studio Build Tools)
- **WebView2**: Windows 10/11 標準搭載 (未導入の場合は Evergreen Runtime)

### インストール (Installation)

リポジトリをクローンし、依存パッケージをインストールします：

```bash
git clone https://github.com/kidinwell/obsidian-task-launcher.git
cd obsidian-task-launcher

# フロントエンド依存関係のインストール
pnpm install
```

### 開発モードの起動 (Development Run)

Tauri 開発環境（フロントエンド Vite サーバー + Tauri デスクトップアプリ）を起動します：

```bash
pnpm tauri dev
```

> フロントエンド単体（ブラウザ上）でUI開発を行う場合:
> ```bash
> pnpm dev
> ```

### テストの実行 (Running Tests)

#### バックエンド（Rust単体・統合テスト）

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

Vault生成、デイリーノート日付書式パース、アンカー挿入、タスクファイル重複連番処理などの E2E テストが実行されます。

#### フロントエンド（Vitest）

```bash
pnpm test
```

### ビルド・パッケージング (Production Build)

インストーラー（MSI / NSIS）および実行可能バイナリを作成します：

```bash
pnpm tauri build
```

ビルドされた成果物は `src-tauri/target/release/bundle/` 配下に出力されます。

---

## ディレクトリ構成 (Directory Structure)

```
obsidian-task-launcher/
├── src/                          # フロントエンド (React + TypeScript)
│   ├── components/               # UIコンポーネント (Header, WorklogForm, TaskForm, SettingsModal, Toast)
│   ├── services/                 # Tauriコマンド呼び出しレイヤー (config, task, worklog)
│   ├── types/                    # 型定義 (config, task, worklog)
│   ├── App.tsx                   # メインウィンドウ制御・ショートカット管理
│   └── main.tsx
├── src-tauri/                    # バックエンド (Rust / Tauri v2)
│   ├── src/
│   │   ├── commands.rs           # フロントエンド向け Tauri コマンドハンドラー
│   │   ├── config.rs             # 設定ファイルの読み書き・Vaultバリデーション
│   │   ├── hotkey.rs             # グローバルホットキー登録・管理
│   │   ├── task_capture.rs       # タスクノート生成・YAMLフロントマター出力
│   │   ├── worklog_capture.rs    # デイリーノート解決・Moment.js書式変換・追記ロジック
│   │   ├── tray.rs               # システムトレイアイコン・メニュー
│   │   ├── lib.rs                # Tauri アプリケーション初期化・ライフサイクル
│   │   └── main.rs
│   ├── tests/                    # 統合・E2Eテスト
│   ├── Cargo.toml                # Rust 依存関係
│   └── tauri.conf.json           # Tauri 設定 (ウィンドウ形状、権限等)
├── openspec/                     # 設計仕様・OpenSpec アーカイブ
├── package.json
└── vite.config.ts
```

---

## ライセンス (License)

本プロジェクトはプライベート開発プロジェクトです。無断転載・再配布を禁じます（詳細はリポジトリ管理者にお問い合わせください）。
