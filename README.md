# rs_gitdiffcopy

[English](README_en.md)

`git diff` の結果を、初心者・非技術者でも視覚的に把握しやすい形で確認できるCLIツールです。差分のあるファイルを元のディレクトリ構造のまま別フォルダへコピーし、サマリーやExcelレポートも出力できます。

## 特徴

- **二方向比較**: 2つのref（ブランチ/タグ/コミット）を比較し、差分ファイルを抽出
- **三方向比較**: base/ours/theirs を比較してコンフリクト候補を検出
- **構造保持**: 出力ディレクトリに元のディレクトリ構造を保持
- **パッチ生成**: 変更ファイルに対してunified diff形式のパッチを生成
- **Excelレポート**: 比較結果をExcelファイル（.xlsx）として出力
- **並列処理**: 大量のファイルを高速に処理
- **設定ファイル**: TOML形式の設定ファイルで再利用可能

## インストール

### ソースからビルド

```bash
cargo build --release
```

ビルドされたバイナリは `target/release/rs_gitdiffcopy` に生成されます。

### Cargoでインストール

```bash
cargo install --path .
```

> 実行には `git` コマンドが必要です。

## 基本的な使い方

### 二方向比較

```bash
# 基本的な使い方
rs_gitdiffcopy -S <source> -T <target> -O <output>

# 例: main と develop を比較して output に出力
rs_gitdiffcopy -S main -T develop -O output
```

### 三方向比較

```bash
# 三方向比較（マージ）
rs_gitdiffcopy --three-way -B <base> -S <ours> -T <theirs> -O <output>

# 例: 共通祖先からの変更を比較
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output
```

## 全オプション

| オプション | 短縮形 | 説明 |
|-----------|--------|------|
| `--source` | `-S` | 比較元ref（ブランチ/タグ/コミット） |
| `--target` | `-T` | 比較先ref（ブランチ/タグ/コミット） |
| `--output` | `-O` | 出力ディレクトリ |
| `--repository` | `-R` | GitリポジトリのパスまたはURL（現在の版はローカルパスのみ対応） |
| `--full-clone` | | リモートURL時のフルクローン（未実装） |
| `--cache-dir <PATH>` | | リモートクローンのキャッシュ先（未実装） |
| `--config <PATH>` | `-c` | 設定ファイルのパス（TOML） |
| `--git-path <PATH>` | | gitコマンドの実行パス |
| `--exclude <PATTERN>` | `-e` | 除外パターン（glob形式、複数指定可） |
| `--force` | `-f` | 出力ディレクトリを削除して再実行（確認あり） |
| `--summary <PATH>` | `-s` | サマリーをファイルに保存 |
| `--verbose` | `-v` | 詳細出力モード（処理中ファイル名を表示） |
| `--dry-run` | `-n` | ドライラン（実際にコピーしない） |
| `--both-versions` | `-b` | 新旧両バージョンをコピー（.old/.new） |
| `--check-permissions <MODE>` | `-P` | 権限変更チェック（none/scripts/all） |
| `--patch` | `-p` | 個別パッチファイルを生成 |
| `--patch-file <PATH>` | `-F` | 統合パッチファイルを生成 |
| `--excel <PATH>` | `-E` | Excelレポートを生成 |
| `--excel-fold-level <LEVEL>` | `-L` | Excelファイルツリーの折りたたみレベル |
| `--show-unchanged` | `-u` | 未変更ファイルもサマリー詳細に表示 |
| `--save-config <PATH>` | `-C` | 現在のオプションを設定ファイルに保存して終了 |
| `--filter-status <STATUS>` | | ステータスフィルタ（カンマ区切り） |
| `--stats-only` | | 統計情報のみ表示 |
| `--no-tree` | | File Treeセクション非表示 |
| `--no-details` | | 詳細セクション非表示 |
| `--copy-deleted` | | 削除ファイルもコピー（.deleted） |
| `--preserve-timestamps` | | タイムスタンプを保持 |
| `--workers <NUM>` | `-j` | 並列ワーカー数 |
| `--temp-dir <PATH>` | | 一時ファイルの保存先 |
| `--color <MODE>` | | カラー出力（auto/always/never） |
| `--log-level <LEVEL>` | | ログレベル（error/warn/info/debug） |
| `--three-way` | `-3` | 三方向比較モード |
| `--base` | `-B` | 三方向比較の共通祖先ref |
| `--merge-style` | `-M` | マージスタイル（all/ours/theirs） |
| `--conflict-only` | | コンフリクトのみ出力 |
| `--help` | `-h` | ヘルプ表示 |
| `--version` | `-V` | バージョン表示 |

> 注意: `--stats-only` は**コンソール出力のみ**に影響します。`--summary` や `--excel` には完全なサマリーが出力されます。

## 使用例（詳細）

### 除外パターン

```bash
# ログファイルとnode_modulesを除外
rs_gitdiffcopy -S main -T develop -O output -e "*.log" -e "node_modules/**"
```

### 出力とレポート

```bash
# サマリーをファイルに出力
rs_gitdiffcopy -S main -T develop -O output --summary summary.txt

# Excelレポートの生成
rs_gitdiffcopy -S main -T develop -O output --excel report.xlsx
```

### パッチ生成

```bash
# 個別のパッチファイルを生成
rs_gitdiffcopy -S main -T develop -O output --patch

# 統合パッチファイルを生成
rs_gitdiffcopy -S main -T develop -O output --patch-file changes.patch

# 両方を生成
rs_gitdiffcopy -S main -T develop -O output --patch --patch-file changes.patch
```

### コピーオプション

```bash
# 変更ファイルの新旧両方をコピー
rs_gitdiffcopy -S main -T develop -O output --both-versions

# 削除ファイルもコピー
rs_gitdiffcopy -S main -T develop -O output --copy-deleted

# タイムスタンプ保持でコピー
rs_gitdiffcopy -S main -T develop -O output --preserve-timestamps
```

### フィルタ・表示制御

```bash
# 追加と変更のみ表示（コピー対象もフィルタされる）
rs_gitdiffcopy -S main -T develop -O output --filter-status added,modified

# 変更なし以外すべて表示（暗黙の all + 除外）
rs_gitdiffcopy -S main -T develop -O output --filter-status ^unchanged

# File Treeを非表示
rs_gitdiffcopy -S main -T develop -O output --no-tree

# 詳細セクションを非表示
rs_gitdiffcopy -S main -T develop -O output --no-details
```

### 出力色とログ

```bash
# 常にカラー出力
rs_gitdiffcopy -S main -T develop -O output --color always

# ログレベルをデバッグに
rs_gitdiffcopy -S main -T develop -O output --log-level debug
```

### パフォーマンス調整

```bash
# ワーカー数を指定
rs_gitdiffcopy -S main -T develop -O output --workers 4

# 一時ディレクトリを指定
rs_gitdiffcopy -S main -T develop -O output --temp-dir /tmp/rs_gitdiffcopy
```

### 権限チェック

```bash
# スクリプトファイルのみ権限変更チェック
rs_gitdiffcopy -S main -T develop -O output --check-permissions scripts

# すべてのファイルの権限変更チェック
rs_gitdiffcopy -S main -T develop -O output --check-permissions all
```

### 設定ファイルの保存

```bash
# 現在のオプションを設定ファイルに保存して終了
rs_gitdiffcopy -S main -T develop -O output -e "*.log" --save-config gitdiffcopy.toml
```

### 三方向比較

```bash
# コンフリクトのみ出力
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output --conflict-only

# Oursを優先
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output --merge-style ours

# Theirsを優先
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output --merge-style theirs
```

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 差分あり（正常終了） |
| 1 | エラー発生 |
| 2 | 差分なし |
| 3 | コンフリクトあり（三方向比較時） |

## 設定ファイル

TOML形式の設定ファイルを使用できます。
アプリケーション共通の `settings.toml` については `rs_gitdiffcopy.md` を参照してください。

```toml
# config.toml
source = "main"
target = "develop"
output = "./output"
exclude = ["*.log", "node_modules/**", "__pycache__/**"]
both_versions = true
patch = true
verbose = false
```

設定ファイルの使用:

```bash
rs_gitdiffcopy --config config.toml
```

現在のオプションを設定ファイルに保存して終了:

```bash
rs_gitdiffcopy -S main -T develop -O output -e "*.log" --save-config gitdiffcopy.toml
```

## 重要な注意

- 現在の版は **リモートURLの比較に未対応** です。`--repository` はローカルパスのみ使用してください。

## 動作要件

- Rust 1.70以上
- サポートOS: Linux, macOS, Windows

## ライセンス

MIT License
