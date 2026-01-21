# rs_gitdiffcopy 要件定義

## 1. プロジェクト概要

- **アプリケーション名**: `rs_gitdiffcopy`
- **バージョン**: `v1.0.1`
- **開発目的**: git diffコマンドの出力は初心者には変更点が分かりにくいため、初心者・非技術者でもgitの変更点が視覚的に分かるツールを作成
- **ゴール**: Gitリポジトリの2つのコミット/ブランチ/タグを比較し、差異があるファイルを階層構造を維持したまま別フォルダへ抽出する
- **想定ユーザー**: 初心者、非技術者

---

## 2. 対象プラットフォーム・開発環境

| 項目 | 内容 |
|------|------|
| 対象OS | Windows 10/11 (x64), Ubuntu 22+ (x64), macOS |
| 開発言語 | Rust |
| インターフェース | CLI |
| 配布形態 | GitHub経由で `cargo install`、またはzipで配布 |
| 依存ツール | Git (外部コマンドとして実行) |

---

## 3. 機能一覧（v1.0.1）

### 3.1 比較コア機能

| 機能 | 説明 |
|------|------|
| 二者間比較 | 同一リポジトリ内の2つのコミット/ブランチ/タグ（ref）を比較し、差分ファイルを抽出する。refはブランチ名、タグ名、コミットハッシュ等で指定可能。 |
| 三者間比較 | `base`(共通祖先)、`ours`(自分)、`theirs`(相手)の3つのrefを比較し、コンフリクトを検出する。マージ前の事前確認などに利用。 |
| 差分検出 | 内部で `git diff --name-status -M -C` を実行し、追加(A)、変更(M)、削除(D)、リネーム(R)、コピー(C)といったファイルの状態を正確に検出する。 |
| 権限チェック | `git ls-tree` を利用してファイルのパーミッション変更を検出し、サマリーに表示する。実行権限が付与されたスクリプトファイルのみを対象とすることも可能 (`-P, --check-permissions <MODE>`: none/scripts/all)。 |
| ファイル内容取得 | `git show <commit>:<path>` を利用して、各コミット時点のファイル内容を正確に取得する。 |

### 3.2 出力・レポート機能

| 機能 | 説明 |
|------|------|
| 新旧両方コピー | 変更があったファイルについて、変更前（source側）と変更後（target側）の両方を `.old`/`.new` という拡張子を付けてコピーする (`--both-versions`)。`--copy-deleted` と併用時は削除ファイルのみ `.deleted` を優先する。 |
| 削除ファイルのコピー | 比較先（target）で削除されたファイルを、`.deleted` という拡張子を付けてコピーする (`--copy-deleted`)。 |
| タイムスタンプ保持 | ファイルをコピーする際に、元となったGitコミットのタイムスタンプをファイルの更新日時として設定する (`--preserve-timestamps`)。 |
| パッチ生成 | 変更点を含むunified diff形式のパッチファイルを生成する (`--patch`, `-F, --patch-file <PATH>`)。`git apply`での適用が可能。 |
| Excelレポート | 差分サマリーをExcelファイル(.xlsx)として出力する。ファイルリストや統計情報がシート別に整理され、非技術者でも変更内容を確認しやすい (`-E, --excel <PATH>`)。 |
| サマリー表示 | 差分の統計、ファイルツリー、ファイルリストなどを整形して表示する。 |
| 出力フィルター | ファイルのステータス（追加、変更など）でコピー対象や表示対象をフィルタリングしたり、サマリーの特定セクションを非表示にしたりできる。 |
| 変更なしファイル表示 | 差分がないファイルもサマリーに含めて表示する (`--show-unchanged`)。 |

### 3.3 設定・制御機能

| 機能 | 説明 |
|------|------|
| リポジトリ指定 | ローカルパスまたはリモートURL（HTTPS/SSH）でGitリポジトリを直接指定できる。指定がない場合はカレントディレクトリから自動で探索する。 |
| Gitコマンドパス設定 | 使用するgitコマンドの実行パスを明示的に指定できる（`--git-path` または設定ファイル）。 |
| 設定ファイル | TOML形式の設定ファイルで、比較対象のrefや各種オプションを保存・再利用できる (`--config`)。 |
| 除外パターン | glob形式のパターンを用いて、特定のファイルやディレクトリを比較対象から除外する (`--exclude`)。 |
| ドライラン | ファイルのコピーを実際には行わず、実行結果のプレビューのみを行う (`--dry-run`)。 |

### 3.4 パフォーマンス・安全性

| 機能 | 説明 |
|------|------|
| 並列処理 | 差分ファイルのデータ取得やコピー処理を並列実行することで、全体の処理時間を短縮する。 |
| リモートクローン | リモートリポジトリを指定した場合、必要なrefのみを効率的に取得するshallow cloneをデフォルトで使用する。 |
| クローンキャッシュ | 一度クローンしたリモートリポジトリをローカルにキャッシュし、二回目以降の実行を高速化する (`--cache-dir`)。 |
| 進捗表示 | 処理のフェーズごとにプログレスバーを表示し、実行状況をリアルタイムで確認できる。 |
| 安全機能 | `/`, `C:\Windows` といったシステム上重要なディレクトリへの上書きを防ぐ。また、出力先にファイルが存在する場合に強制上書き(`--force`)する際は、必ず確認プロンプトを表示する。 |

---

## 4. Git設定

### 4.1 Gitコマンドの解決

gitコマンドは以下の優先順位で解決されます：

1. コマンドライン引数 `--git-path`
2. 入力設定ファイル（`-c, --config`で指定）の `git_path`
3. アプリケーション設定ファイル（`settings.toml`）の `git_path`
4. 環境変数 `PATH` で見つかるgitコマンド

### 4.2 アプリケーション設定ファイル（settings.toml）

CLIアプリケーションと同じディレクトリに `settings.toml` を配置することで、アプリケーション全体のデフォルト設定を行えます。

#### 設定ファイルの配置場所

| OS | 配置場所 |
|----|---------|
| Windows | `rs_gitdiffcopy.exe` と同じディレクトリ |
| Linux/macOS | `rs_gitdiffcopy` バイナリと同じディレクトリ |

#### 設定ファイルの例

```toml
# settings.toml（アプリケーションと同じディレクトリに配置）

# ===================
# Git設定
# ===================
# gitコマンドの実行パス（省略時はPATHから検索）
# git_path = "/usr/bin/git"

# ===================
# キャッシュ設定
# ===================
# リモートリポジトリのクローンをキャッシュするデフォルトディレクトリ
# 省略時はキャッシュ無効（毎回一時ディレクトリにクローン）
# cache_dir = "~/.cache/rs_gitdiffcopy"

# 一時ファイルの保存先ディレクトリ（省略時はシステムのtempディレクトリ）
# temp_dir = "/tmp/rs_gitdiffcopy"

# ===================
# パフォーマンス設定
# ===================
# 並列処理のワーカー数（省略時は論理CPUコア数）
# 低スペックPCでは小さい値を設定して負荷を軽減
# workers = 4

# リモートリポジトリのデフォルトクローン方式
# true: フルクローン（全履歴取得）、false: shallow clone（高速）
# full_clone = false

# ===================
# 出力設定
# ===================
# カラー出力のデフォルト設定
# "auto": 端末判定で自動切替（デフォルト）
# "always": 常にカラー出力
# "never": 常にカラーなし
# color = "auto"

# ログ出力レベル
# "error": エラーのみ
# "warn": 警告以上（デフォルト）
# "info": 情報以上
# "debug": デバッグ情報を含む全て
# log_level = "warn"

# ===================
# デフォルト除外パターン
# ===================
# 全ての比較で適用されるデフォルトの除外パターン
# 入力設定ファイル（-c）のexcludeと併用される
# default_exclude = [
#     ".git/**",
#     "node_modules/**",
#     "__pycache__/**",
#     "*.pyc",
#     ".DS_Store",
#     "Thumbs.db"
# ]
```

#### 設定項目一覧

| 項目 | 型 | デフォルト | 説明 |
|------|------|------------|------|
| `git_path` | string | PATH検索 | gitコマンドの実行パス |
| `cache_dir` | string | - | リモートリポジトリのクローンキャッシュディレクトリ |
| `temp_dir` | string | システムtempディレクトリ | 一時ファイルの保存先 |
| `workers` | integer | 論理CPUコア数 | 並列処理のワーカー数（1以上） |
| `full_clone` | bool | `false` | リモートリポジトリのデフォルトクローン方式 |
| `color` | string | `"auto"` | カラー出力設定（`auto`/`always`/`never`） |
| `log_level` | string | `"warn"` | ログ出力レベル（`error`/`warn`/`info`/`debug`） |
| `default_exclude` | array | `[]` | デフォルトの除外パターン（全比較に適用） |

#### 設定の優先順位

各設定項目は以下の優先順位で解決されます（上が最優先）：

1. コマンドライン引数（`-e`, `--cache-dir` 等）
2. 入力設定ファイル（`-c, --config` で指定）
3. アプリケーション設定ファイル（`settings.toml`）
4. デフォルト値

#### 除外パターンの合成

`default_exclude`（settings.toml）と`exclude`（入力設定ファイル/-eオプション）は**合成**されます：

```toml
# settings.toml
default_exclude = ["node_modules/**", ".git/**"]

# rs_gitdiffcopy.toml（または -e オプション）
exclude = ["*.log", "dist/**"]

# 実際に適用される除外パターン:
# - node_modules/**
# - .git/**
# - *.log
# - dist/**
```

※ `settings.toml` が存在しない場合、または各項目が記載されていない場合は、デフォルト値が使用されます。

### 4.3 入力設定ファイルでのGit設定

`-c, --config` で指定する設定ファイルでも `git_path` を指定できます。この設定は `settings.toml` より優先されます。

```toml
# rs_gitdiffcopy.toml（入力設定ファイル）
git_path = "/usr/bin/git"  # 省略時はsettings.tomlまたはPATHから検索
```

### 4.4 リポジトリの指定

リポジトリは以下の方法で指定できます：

- `-R, --repository <PATH|URL>` オプションで明示的に指定
  - ローカルパス: `/path/to/repo` または `./relative/path`
  - リモートURL: HTTPS、SSH、Gitプロトコル対応
- 省略時はカレントディレクトリから親方向に`.git`ディレクトリを検索

#### 対応するリモートURL形式

| 形式 | 例 |
|------|-----|
| HTTPS | `https://github.com/user/repo.git` |
| SSH | `git@github.com:user/repo.git` |
| Git プロトコル | `git://github.com/user/repo.git` |

### 4.5 リモートリポジトリの設定

リモートURLが指定された場合、一時ディレクトリにクローンして処理を行います。

#### クローン方式

| オプション | 説明 |
|-----------|------|
| デフォルト（shallow clone） | 必要なコミットのみ取得。高速だが履歴は限定的 |
| `--full-clone` | 全履歴をクローン。大規模リポジトリでは時間がかかる |

#### クローン処理の詳細

1. 一時ディレクトリに `git clone --depth=1 --no-checkout` でクローン
2. 必要なrefを `git fetch --depth=1 origin <ref>` で個別に取得（source/target、三者間ならbase含む）
3. 処理完了後、一時ディレクトリを削除（キャッシュ無効時）

#### キャッシュ設定

| オプション | 説明 |
|-----------|------|
| デフォルト | 処理完了後にクローンを削除 |
| `--cache-dir <PATH>` | 指定ディレクトリにクローンをキャッシュ。次回以降は`git fetch`のみで高速化 |

キャッシュディレクトリの構造:
```
cache_dir/
├── github.com_user_repo1/
│   └── .git/
├── github.com_user_repo2/
│   └── .git/
└── ...
```

#### 認証

リモートリポジトリへのアクセスには、gitコマンドの標準的な認証方式を使用します：

- **HTTPS**: credential helper、環境変数（`GIT_ASKPASS`）、`.netrc`
- **SSH**: SSHキー（`~/.ssh/id_rsa`等）、ssh-agent

プライベートリポジトリの場合は、事前に認証設定が必要です。

---

## 5. CLI仕様

### 5.1 コマンドライン引数

```
rs_gitdiffcopy [OPTIONS]

必須オプション（設定ファイル未使用時）:
  -S, --source <REF>               比較元ref（before）: ブランチ名/タグ名/コミットハッシュ
  -T, --target <REF>               比較先ref（after）: ブランチ名/タグ名/コミットハッシュ
  -O, --output <PATH>              差分ファイルの出力先

リポジトリオプション:
  -R, --repository <PATH|URL>      GitリポジトリのパスまたはリモートURL
      --full-clone                 リモートURL時にフルクローンを実行（デフォルト: shallow clone）
      --cache-dir <PATH>           リモートリポジトリのクローンをキャッシュするディレクトリ

オプション:
  -c, --config <PATH>              設定ファイル（TOML形式）
      --git-path <PATH>            使用するgitコマンドの実行パス
  -e, --exclude <PATTERN>          除外パターン（複数指定可、glob形式）
  -f, --force                      出力先を全削除して再実行（削除前に確認プロンプト表示）
  -s, --summary <PATH>             サマリーをファイルに出力（デフォルト: 標準出力、`--dry-run` でも出力）
  -v, --verbose                    詳細出力モード（処理中ファイル名を表示）
  -n, --dry-run                    実際にコピーせず、対象ファイルを表示
  -b, --both-versions              変更ファイルの新旧両方をコピー（.old/.new拡張子付与）
  -P, --check-permissions <MODE>   権限変更をチェック（none/scripts/all、デフォルト: none）
  -p, --patch                      変更ファイルごとに個別パッチファイル(.patch)を生成（`--dry-run` 時は生成しない）
  -F, --patch-file <PATH>          全変更を統合したパッチファイルを生成（`--dry-run` 時は生成しない）
  -E, --excel <PATH>               サマリーをExcelファイル(.xlsx)に出力（`--dry-run` でも生成）
  -L, --excel-fold-level <LEVEL>   Excelファイルツリーの折りたたみレベル（指定深さ以上を折りたたみ）
  -u, --show-unchanged             変更がないファイルをサマリー詳細に表示
  -C, --save-config <PATH>         現在のオプションを設定ファイル(TOML形式)に保存
  --filter-status <STATUS>         指定ステータスのファイルのみコピー/表示（複数指定可: カンマ区切り）
  --stats-only                     統計情報のみ表示（--no-tree と --no-details の両方を指定した場合と同じ）
  --no-tree                        File Treeセクションを非表示
  --no-details                     詳細セクション（Added/Modified/Deleted Files等）を非表示
  --copy-deleted                   削除ファイルもコピー（.deleted拡張子付与）
  --preserve-timestamps            コピー時にコミットのタイムスタンプを保持
  -j, --workers <NUM>              並列処理のワーカー数（デフォルト: CPUコア数）
      --temp-dir <PATH>            一時ファイルの保存先ディレクトリ
      --color <MODE>               カラー出力（auto/always/never、デフォルト: auto）
      --log-level <LEVEL>          ログ出力レベル（error/warn/info/debug、デフォルト: warn）
  -h, --help                       ヘルプ表示
  -V, --version                    バージョン表示

三者間モード専用オプション:
  -3, --three-way                  三者間比較モードを有効化
  -B, --base <REF>                 共通祖先ref
  -M, --merge-style <STYLE>        コンフリクト時のコピー方式（all/ours/theirs）
      --conflict-only              コンフリクト候補のみ出力
```

#### オプションの補足説明

**`--verbose` と `--log-level` の違い**

| オプション | 用途 | 出力先 |
|-----------|------|--------|
| `--verbose` | 処理中のファイル名をリアルタイム表示 | 標準エラー出力（プログレスと同時表示） |
| `--log-level` | アプリケーション内部のログ出力レベル | 標準エラー出力 |

- `--verbose`は進捗表示の詳細化（処理中ファイル名の表示）に使用
- `--log-level`はデバッグや問題調査用のログ出力制御に使用
- 両方を同時に指定可能

**`--dry-run` の動作**

ドライランモードでは以下の処理が**実行されます**：
- ref解決、差分ファイルリスト取得
- `git show`によるファイル内容取得（内容確認のため）
- サマリー生成・表示

以下の処理は**スキップされます**：
- ファイルのコピー（出力ディレクトリへの書き込み）
- パッチファイルの生成

ファイルコピー処理がスキップされるため、実際の変更を適用するよりも**迅速に差分結果を確認できます**。

コピー系オプション（`--copy-deleted` / `--both-versions` / `--preserve-timestamps`）は
ドライラン時は反映されません。

サマリーには `Mode: Dry run (no files copied)` と表示されます。

**`--filter-status` の許容値**

指定可能なステータスは以下です（大文字小文字は区別しない）:

受理値（推奨）:
- `added`
- `modified`
- `deleted`
- `renamed`
- `copied`
- `type-changed`
- `unchanged`（`--show-unchanged` が必要）

別名:
- `add` / `a`
- `modify` / `m`
- `delete` / `d`
- `rename` / `r`
- `copy` / `c`
- `typechanged` / `type_changed` / `t`
- `same` / `u`（`--show-unchanged` が必要）

※ CLIのヘルプには受理値（推奨）のみを掲載し、別名は仕様書内の互換情報として扱う

**三者間モード用ステータス値：**

| ステータス値 | 対象 |
|-------------|------|
| `all` | 全ステータス（除外指定と組み合わせて使用） |
| `unchanged` | 3つとも同一（変更なし） |
| `ours-only` | oursのみ変更 |
| `theirs-only` | theirsのみ変更 |
| `both-same` | 両方が同じ変更 |
| `conflict` | コンフリクト（両方が異なる変更） |
| `added-ours` | oursでのみ追加 |
| `added-theirs` | theirsでのみ追加 |
| `added-both-same` | 両方で追加（同一内容） |
| `added-both-diff` | 両方で追加（異なる内容）- コンフリクト |
| `deleted-ours` | oursで削除 |
| `deleted-theirs` | theirsで削除 |
| `deleted-both` | 両方で削除 |
| `modify-delete` | oursで変更、theirsで削除 - コンフリクト |
| `delete-modify` | oursで削除、theirsで変更 - コンフリクト |

**三者間モード用グループキーワード：**

| グループ | 含まれるステータス |
|----------|-------------------|
| `added` | added-ours, added-theirs, added-both-same, added-both-diff |
| `modified` | ours-only, theirs-only, both-same, conflict |
| `deleted` | deleted-ours, deleted-theirs, deleted-both |
| `conflicts` | conflict, added-both-diff, modify-delete, delete-modify |

**グループキーワードの展開：**

グループキーワードは指定時（追加・除外どちらも）に自動的に含まれるステータスに展開されます：

```bash
# 追加系のみ表示
--filter-status added
# → added-ours, added-theirs, added-both-same, added-both-diff に展開

# 追加系を除外（全ステータスから追加系を除く）
--filter-status all,^added
# → all, ^added-ours, ^added-theirs, ^added-both-same, ^added-both-diff と同義

# 削除系を除外（暗黙のall + 削除系除外）
--filter-status ^deleted
# → ^deleted-ours, ^deleted-theirs, ^deleted-both に展開

# 追加と変更のみ表示（削除と変更なしを除外）
--filter-status added,modified
```

※ `--filter-status` 指定時は、コピー対象・サマリーの詳細・統計情報の全てに同じフィルターが適用される

**`--summary` / `--excel` の上書き動作**

出力先ファイルが既に存在する場合は、確認なしで上書きされる。

**`--force` と `--dry-run` の確認プロンプト**

`--dry-run` 時は出力先ディレクトリの削除は行われないため、`--force` の確認プロンプトは表示されない。

**`--no-tree` / `--no-details` の影響**

これらはサマリー表示の制御のみであり、ファイルコピーは通常通り実行される。

**`--stats-only` の影響**

`--stats-only` はコンソール表示にのみ影響するため、Excel出力には影響しません。`--stats-only` 指定時でも、`--excel` が指定されていれば**完全なレポートがExcelファイルに出力されます**。

**`--summary` の出力先**

`--summary` で指定したファイルにサマリーを出力します。ファイル出力と同時に、コンソールにも完全なサマリーが表示されます。

**`--summary` と `--excel` の併用**

両方指定した場合は、テキストサマリーとExcelレポートをそれぞれ出力する。

### 5.2 Git ref の指定形式

| 形式 | 例 | 説明 |
|------|-----|------|
| ブランチ名 | `main`, `develop`, `feature/login` | ブランチのHEADコミットを参照 |
| リモートブランチ | `origin/main`, `origin/develop` | リモート追跡ブランチを参照 |
| タグ名 | `v1.0.0`, `release-2024` | タグが指すコミットを参照 |
| コミットハッシュ（完全） | `abc123def456...` | 40文字のコミットハッシュ（SHA-1） |
| コミットハッシュ（短縮） | `abc123d` | 7文字以上の短縮ハッシュ |
| HEAD相対 | `HEAD`, `HEAD~1`, `HEAD^2` | HEADからの相対参照 |
| ブランチ相対 | `main~3`, `develop^` | ブランチからの相対参照 |

### 5.3 使用例

```bash
# 基本使用（2つのブランチを比較）
rs_gitdiffcopy -S main -T develop -O output

# コミットハッシュを指定
rs_gitdiffcopy -S abc123 -T def456 -O output

# タグ間の比較
rs_gitdiffcopy -S v1.0.0 -T v2.0.0 -O output

# リモートブランチとの比較
rs_gitdiffcopy -S origin/main -T HEAD -O output

# HEAD相対参照（直前のコミットと現在を比較）
rs_gitdiffcopy -S HEAD~1 -T HEAD -O output

# リポジトリパスを明示的に指定
rs_gitdiffcopy -R /path/to/repo -S main -T develop -O output

# リモートリポジトリを直接指定（HTTPS）
rs_gitdiffcopy -R https://github.com/user/repo.git -S main -T develop -O output

# リモートリポジトリを直接指定（SSH）
rs_gitdiffcopy -R git@github.com:user/repo.git -S v1.0.0 -T v2.0.0 -O output

# フルクローンでリモートリポジトリを比較
rs_gitdiffcopy -R https://github.com/user/repo.git --full-clone -S main -T develop -O output

# クローンをキャッシュして次回以降高速化
rs_gitdiffcopy -R https://github.com/user/repo.git --cache-dir ~/.cache/rs_gitdiffcopy -S main -T develop -O output

# サマリーをファイルに保存
rs_gitdiffcopy -S main -T develop -O output -s ./summary.txt

# 除外パターン指定（複数可）
rs_gitdiffcopy -S main -T develop -O output -e "*.log" -e "node_modules/**"

# 強制再実行（出力先を削除して実行）
rs_gitdiffcopy -S main -T develop -O output --force

# ドライラン（確認のみ）
rs_gitdiffcopy -S main -T develop -O output --dry-run

# 変更ファイルの新旧両方をコピー
rs_gitdiffcopy -S main -T develop -O output --both-versions

# 権限チェック（スクリプトファイルのみ）
rs_gitdiffcopy -S main -T develop -O output -P scripts

# 個別パッチファイルを生成（変更ファイルごとに.patchファイル作成）
rs_gitdiffcopy -S main -T develop -O output --patch

# 統合パッチファイルを生成（全変更を1ファイルに）
rs_gitdiffcopy -S main -T develop -O output -F changes.patch

# Excelレポートを出力
rs_gitdiffcopy -S main -T develop -O output -E report.xlsx

# 特定のステータスのファイルのみ表示（追加と変更のみ）
rs_gitdiffcopy -S main -T develop -O output --filter-status added,modified

# 設定ファイルを使用
rs_gitdiffcopy --config ./rs_gitdiffcopy.toml

# 三者間比較（3つのブランチ/コミットを比較）
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output

# 三者間比較でコンフリクトのみ抽出
rs_gitdiffcopy -3 -B main -S feature/a -T feature/b -O output --conflict-only
```

### 5.4 設定ファイル（TOML形式）

※ CLIオプションはkebab-case、設定ファイルはsnake_caseで表記し、名称は1:1に対応する

#### 設定ファイルの例（rs_gitdiffcopy.toml）

```toml
# Git設定
repository = "/path/to/repo"  # ローカルパスまたはリモートURL（省略時: カレントディレクトリから検索）
# git_path = "/usr/bin/git"   # gitコマンドのパス（省略または空文字列: PATHから検索）

# リモートリポジトリ設定（repositoryがURLの場合に使用）
full_clone = false            # true: フルクローン、false: shallow clone
# cache_dir = "~/.cache/rs_gitdiffcopy"  # クローンのキャッシュディレクトリ（省略: キャッシュ無効）

# 比較対象（必須）
source = "main"               # 比較元ref（ブランチ/タグ/コミット）
target = "develop"            # 比較先ref（ブランチ/タグ/コミット）
output = "./diff_output"      # 出力ディレクトリ

# オプション設定
force = false
verbose = false
dry_run = false
both_versions = false
summary = "./summary.txt"
check_permissions = "none"    # none / scripts / all
patch = false                 # 個別パッチファイル生成
patch_file = ""               # 統合パッチファイルパス（空で無効）
excel = ""                    # Excelレポート出力パス（空で無効）
# excel_fold_level = 2        # Excelファイルツリーの折りたたみレベル（省略時は折りたたみなし）
show_unchanged = false        # 変更がないファイルをサマリー詳細に表示

# 出力フィルター設定
# filter_status = ["added", "modified"]  # 表示するステータス（省略時は全て表示）
stats_only = false            # 統計情報のみ表示
no_tree = false               # File Treeセクション非表示
no_details = false            # 詳細セクション非表示

# コピーオプション
copy_deleted = false          # 削除ファイルもコピー（.deleted拡張子）
preserve_timestamps = false   # タイムスタンプを保持

# パフォーマンス・出力設定
# workers = 4                 # 並列処理のワーカー数（省略時: CPUコア数）
# temp_dir = "/tmp"           # 一時ファイルの保存先（省略時: システムtempディレクトリ）
# color = "auto"              # カラー出力（auto/always/never）
# log_level = "warn"          # ログ出力レベル（error/warn/info/debug）

# 除外パターン（複数指定可）
exclude = [
    "*.log",
    "*.tmp",
    "node_modules/**",
    "__pycache__/**"
]

# 三者間比較オプション
three_way = false             # 三者間比較モード
# base = "main"               # 共通祖先ref（three_way = true の場合は必須）
merge_style = "all"           # all / ours / theirs
conflict_only = false         # コンフリクトのみ出力
# filter_status = ["conflict", "ours-only"]  # 三者間モードでのフィルタリング
```

#### 設定ファイルの項目

| 項目 | 型 | 必須 | デフォルト | 説明 |
|------|------|:----:|------------|------|
| `repository` | string | - | カレントディレクトリ | GitリポジトリのパスまたはリモートURL |
| `git_path` | string | - | PATH検索 | gitコマンドのパス |
| `full_clone` | bool | - | `false` | リモートURL時にフルクローンを実行 |
| `cache_dir` | string | - | - | クローンのキャッシュディレクトリ |
| `source` | string | ✅ | - | 比較元ref（ブランチ/タグ/コミット） |
| `target` | string | ✅ | - | 比較先ref（ブランチ/タグ/コミット） |
| `output` | string | ✅ | - | 出力先ディレクトリ |
| `exclude` | array | - | `[]` | 除外パターンのリスト |
| `force` | bool | - | `false` | 出力先を削除して再実行（削除前に確認プロンプト表示） |
| `verbose` | bool | - | `false` | 詳細出力モード |
| `dry_run` | bool | - | `false` | ドライラン |
| `both_versions` | bool | - | `false` | 新旧両方をコピー |
| `summary` | string | - | - | サマリー出力先ファイル |
| `check_permissions` | string | - | `"none"` | 権限チェックモード |
| `patch` | bool | - | `false` | 個別パッチファイル生成 |
| `patch_file` | string | - | - | 統合パッチファイルパス |
| `excel` | string | - | - | Excelレポート出力パス |
| `excel_fold_level` | integer | - | - | Excelファイルツリーの折りたたみレベル |
| `show_unchanged` | bool | - | `false` | 変更がないファイルをサマリー詳細に表示 |
| `filter_status` | array | - | `[]` | 表示するステータスのリスト |
| `stats_only` | bool | - | `false` | 統計情報のみ表示 |
| `no_tree` | bool | - | `false` | File Treeセクションを非表示 |
| `no_details` | bool | - | `false` | 詳細セクションを非表示 |
| `copy_deleted` | bool | - | `false` | 削除ファイルもコピー |
| `preserve_timestamps` | bool | - | `false` | コミット時のタイムスタンプを保持 |
| `workers` | integer | - | CPUコア数 | 並列処理のワーカー数 |
| `temp_dir` | string | - | システムtempディレクトリ | 一時ファイルの保存先 |
| `color` | string | - | `"auto"` | カラー出力設定 |
| `log_level` | string | - | `"warn"` | ログ出力レベル |
| `three_way` | bool | - | `false` | 三者間比較モード |
| `base` | string | ※ | - | 共通祖先ref（三者間モード時必須） |
| `merge_style` | string | - | `"all"` | コンフリクト時のコピー方式 |
| `conflict_only` | bool | - | `false` | コンフリクトのみ出力 |

#### 優先順位

コマンドライン引数と設定ファイルの両方が指定された場合、**コマンドライン引数が優先**されます。

---

## 6. Git コマンドの使用

### 6.1 使用するGitコマンド一覧

| 処理 | コマンド | 用途 |
|------|---------|------|
| リポジトリ確認 | `git rev-parse --git-dir` | .gitディレクトリの存在確認 |
| ref解決 | `git rev-parse <ref>` | ブランチ/タグ/相対参照をコミットハッシュに解決 |
| 差分ファイルリスト | `git diff --name-status -M -C <source> <target>` | 差分ファイルとステータスを取得（リネーム/コピー検出を含む） |
| ファイル内容取得 | `git show <commit>:<path>` | 特定コミットのファイル内容を取得 |
| ファイル権限取得 | `git ls-tree <commit> <path>` | ファイルのモード（権限）を取得 |
| コミット日時取得 | `git log -1 --format=%ci <commit> -- <path>` | ファイルのコミット日時を取得 |
| パッチ生成 | `git diff -M -C <source> <target> -- <path>` | 個別ファイルのパッチを生成 |
| 統合パッチ | `git diff -M -C <source> <target>` | 全変更の統合パッチを生成 |

### 6.2 差分検出の処理フロー

1. `git rev-parse` でsource/targetのrefをコミットハッシュに解決
2. `git diff --name-status -M -C` で差分ファイルリストを取得
3. ステータス（A/M/D/R/C等）に基づいてファイル状態を分類
4. `git show` で必要なファイル内容を取得
5. 出力ディレクトリにファイルをコピー

### 6.3 git diff --name-status の出力形式

```
A       path/to/new_file.txt       # 追加
M       path/to/modified_file.txt  # 変更
D       path/to/deleted_file.txt   # 削除
R100    old/path.txt   new/path.txt # リネーム（100%一致）
R095    old/path.txt   new/path.txt # リネーム（95%一致）
C100    src/path.txt   dst/path.txt # コピー（100%一致）
T       path/to/typechange.txt     # タイプ変更（ファイル⇔シンボリックリンク等）
```

※ R/C の行を得るために `-M -C` を付与する

### 6.4 ファイル状態のマッピング

| gitステータス | rs_gitdiffcopyステータス | 説明 |
|--------------|------------------------|------|
| A | added | 新規追加 |
| M | modified | 内容変更 |
| D | deleted | 削除 |
| R | renamed | リネーム（old→new） |
| C | copied | コピー（src→dst） |
| T | type-changed | タイプ変更 |

---

## 7. 差異判定

### 7.1 ファイル内容比較

gitの差分検出機能を利用するため、独自のハッシュ比較は不要です。
通常は `git diff --name-status -M -C` の結果をそのまま使用します。

ただし `--show-unchanged` 指定時は変更がないファイルも必要になるため、
以下の追加処理を行います：

1. `git ls-tree -r <commit>` で source/target の全ファイルを取得
2. パスごとに blob ID を比較し、同一なら unchanged と判定
3. モードが `120000`（シンボリックリンク）または `160000`（サブモジュール）の場合は比較対象から除外し、該当セクションへ記載
4. 差分結果（added/modified/deleted/renamed/copied/type-changed）と統合して表示

### 7.2 権限・属性チェック

`-P/--check-permissions` オプションで、ファイルの権限変更を検出できます。

検出された権限変更は `Permission Changes` セクションに詳細が表示されます。また、権限のみが変更されたファイルは、File Treeに `[permission]` タグ付きで表示されます。

#### チェック方法

```bash
# 各コミットのファイル権限を取得
git ls-tree <commit> <path>
# 出力例: 100644 blob abc123... path/to/file.txt
#         ^^^^^^
#         モード（6桁: タイプ+権限）
```

#### モードの解釈

| モード | 意味 |
|--------|------|
| 100644 | 通常ファイル（読み書き） |
| 100755 | 実行可能ファイル |
| 120000 | シンボリックリンク |
| 160000 | Gitサブモジュール |

※ 120000/160000 を検出した場合は通常コピー対象から除外し、サマリーにのみ記載する

#### scriptsモードの対象拡張子

以下の拡張子を持つファイルが権限チェックの対象となります：

| 拡張子 | 種別 |
|--------|------|
| `.sh` | シェルスクリプト |
| `.bash` | Bashスクリプト |
| `.ps1` | PowerShellスクリプト |
| `.cmd` | Windowsバッチファイル |
| `.bat` | Windowsバッチファイル |
| `.zsh` | Zshスクリプト |
| `.py` | Pythonスクリプト |
| `.rb` | Rubyスクリプト |
| `.pl` | Perlスクリプト |
| `.js` | JavaScriptスクリプト |
| `.ts` | TypeScriptスクリプト |
| `.php` | PHPスクリプト |
| 拡張子なし | シェバン(`#!`)で判定 |

---

## 8. ファイル状態別の扱い

### 8.1 通常モード（デフォルト）

※ 本セクションは二者間比較の挙動であり、三者間モードには適用されない

| 状態 | 扱い |
|------|------|
| 新規ファイル (A) | 出力先にコピー |
| 変更ファイル (M) | 出力先にコピー（target側のファイル） |
| 削除ファイル (D) | サマリーに記載のみ（コピーしない） |
| リネーム (R) | 新しいパスにコピー（サマリーに旧パス→新パスを表示） |
| コピー (C) | 新しいパスとしてコピー |
| タイプ変更 (T) | サマリーに記載、シンボリックリンクならスキップ |
| シンボリックリンク | サマリーに記載のみ（コピーしない） |
| サブモジュール | サマリーに記載のみ（コピーしない） |
| バイナリファイル | コピーは実行、パッチ生成はスキップ |
| 権限変更のみ | 出力先にコピー（target側のファイル）、サマリーのFile Treeに `[permission]` タグで表示 |

### 8.2 --copy-deleted モード

| 状態 | 扱い |
|------|------|
| 削除ファイル (D) | 出力先にコピー（`.deleted`拡張子付与） |
| その他 | 通常モードと同じ |

### 8.3 --preserve-timestamps モード

| 状態 | 扱い |
|------|------|
| コピー対象ファイル（単一出力） | target側のコミット日時を設定 |
| `--both-versions` の `.old` | source側のコミット日時を設定 |
| `--both-versions` の `.new` | target側のコミット日時を設定 |
| `.deleted` | source側のコミット日時を設定 |
| その他 | 通常モードと同じ |

### 8.4 --both-versions モード

※ 本セクションは二者間比較の挙動であり、三者間モードには適用されない

`--both-versions` モードでは、変更があったファイルの変更前と変更後の両方のバージョンが出力されます。

| 状態 | 扱い |
|------|------|
| 新規ファイル (A) | 出力先にコピー（拡張子なし） |
| 変更ファイル (M) | 新旧両方をコピー（`filename.ext.old` / `filename.ext.new`） |
| リネーム (R) | 新パスに新旧両方をコピー。例えば `old.txt` -> `new.txt` の場合、`new.txt.old` (内容は `old.txt`) と `new.txt.new` (内容は `new.txt`) が生成される。 |
| コピー (C) | コピー先パスに新旧両方をコピー（`copied_path.old` / `copied_path.new`） |
| その他 | 通常モードと同じ |

※ `--both-versions` と `--copy-deleted` を同時指定した場合、削除ファイルは `.deleted` を優先して1つだけコピーする

### 8.5 通常モードのリネーム/コピーの扱い

※ 本セクションは二者間比較の挙動であり、三者間モードには適用されない

| ケース | 表示 | コピー |
|--------|------|--------|
| リネーム | `old.txt -> new.txt [renamed]` | 新パスのみコピー |
| コピー | `src.txt -> dst.txt [copied]` | コピー先のみコピー |

※ `--both-versions` 指定時は 8.4 のルールを優先する

---

## 9. 出力構造

### 9.1 通常モード

```
output_dir/
├── src/
│   ├── main.rs          # 変更ファイル
│   └── new_feature.rs   # 新規ファイル
├── docs/                # 新規ディレクトリ
└── config.toml          # 変更ファイル
```

### 9.2 --both-versions モード

```
output_dir/
├── src/
│   ├── main.rs.old      # 変更前のファイル（source ref）
│   ├── main.rs.new      # 変更後のファイル（target ref）
│   └── new_feature.rs   # 新規ファイルはそのまま
├── docs/
├── config.toml.old
└── config.toml.new
```

### 9.3 --patch モード

```
output_dir/
├── src/
│   ├── main.rs          # 変更ファイル
│   ├── main.rs.patch    # 個別パッチファイル（git diffから生成）
│   └── new_feature.rs   # 新規ファイル（パッチなし）
├── docs/
├── config.toml
└── config.toml.patch
```

`--patch`と`-F, --patch-file`は同時に指定可能です。その場合、個別パッチファイルと統合パッチファイルの両方が生成されます。

---

## 10. 進捗表示

処理はフェーズ別に進捗表示されます：

#### ローカルリポジトリの場合

```
[1/5] Resolving Refs...
Source: main -> abc123def456
Target: develop -> 789xyz012345
[2/5] Getting Diff File List...
Found 50 changed files.
[3/5] Fetching Files: [=============>              ] 45% (22/50)
Fetched 50 files.
[4/5] Copying Files...
Copied 45 files.
[5/5] Writing Summary...
Completed.
```

#### リモートリポジトリの場合

```
[1/7] Cloning Repository...
Repository: https://github.com/user/repo.git (shallow)
Clone completed.
[2/7] Fetching Refs...
Refs: main, develop
[3/7] Resolving Refs...
Source: main -> abc123def456
Target: develop -> 789xyz012345
[4/7] Getting Diff File List...
Found 50 changed files.
[5/7] Fetching Files: [=============>              ] 45% (22/50)
Fetched 50 files.
[6/7] Copying Files...
Copied 45 files.
[7/7] Writing Summary...
Completed.
```

### 10.1 処理フェーズ

#### ローカルリポジトリ

| フェーズ | 処理内容 | 並列化 |
|---------|---------|:------:|
| Phase 1 | Resolving Refs（ref解決） | - |
| Phase 2 | Getting Diff File List（差分ファイルリスト取得） | - |
| Phase 3 | Fetching Files（git showでファイル取得） | 並列 |
| Phase 4 | Copying Files（ファイルコピー） | 並列 |
| Phase 5 | Writing Summary | - |

#### リモートリポジトリ

| フェーズ | 処理内容 | 並列化 |
|---------|---------|:------:|
| Phase 1 | Cloning Repository（リポジトリのクローン） | - |
| Phase 2 | Fetching Refs（必要なrefの取得） | - |
| Phase 3 | Resolving Refs（ref解決） | - |
| Phase 4 | Getting Diff File List（差分ファイルリスト取得） | - |
| Phase 5 | Fetching Files（git showでファイル取得） | 並列 |
| Phase 6 | Copying Files（ファイルコピー） | 並列 |
| Phase 7 | Writing Summary | - |

※ パッチ生成時は別途フェーズが追加
※ キャッシュ使用時はPhase 1がスキップされ、Phase 2でgit fetchのみ実行

---

## 11. サマリー出力形式

### 11.1 差分ありの場合（ローカルリポジトリ）

```
rs_gitdiffcopy Summary
======================
Repository: /path/to/repo
Source ref: main (abc123d)
Target ref: develop (def456e)
Output: /path/to/output
Date: 2025-12-18 10:30:00

Options:
  Mode: Dry run (no files copied)
  Copy mode: Both versions (.old/.new)
  Copy deleted: Yes (.deleted)
  Preserve timestamps: Yes
  Permission check: scripts
  Patch mode: Individual files (.patch)
  Combined patch file: changes.patch
  Config file: rs_gitdiffcopy.toml
  Filter status: added, modified
  Exclude patterns:
    - *.log
    - __pycache__

Added:         5 files
Modified:      8 files
Deleted:       2 files
Renamed:       1 file
Symlinks:      3 files
Submodules:    1
Permission changes: 2 files
Unchanged:     50 files
--------------------------
Total:        72 items

================
File Tree
================
.
├── src/
│   ├── main.rs [modified]
│   ├── new_feature.rs [added]
│   ├── old_module.rs [deleted]
│   └── utils.rs -> lib/utils.rs [renamed]
├── docs/ [added]
├── scripts/
│   └── build.sh [permission]
├── config.toml [modified]
├── cache -> /tmp/cache [symlink: added]
├── vendor/ [submodule]
└── legacy/ [deleted]

※ ファイルの内容も同時に変更された場合の権限変更の詳細は `Permission Changes` セクションに一覧表示されます。

### 11.2 File Tree出力形式

コンソール出力とファイル出力で形式が異なります。

| 出力先 | 形式 | 説明 |
|--------|------|------|
| コンソール | コンパクト形式 | パス直後にステータス表示 |
| ファイル（`-s`指定時） | 整列形式 | 最長パス幅に合わせてステータス位置を揃える |

**ファイル出力例**（整列形式）:

```
================
File Tree
================
.
├── src/
│   ├── main.rs                      [modified]
│   ├── new_feature.rs               [added]
│   └── 日本語ファイル.rs            [modified]
├── docs/                            [added]
└── config.toml                      [modified]
```

- ファイル出力では、すべてのファイルのステータス表示位置を最長パス幅に合わせて揃える
- 日本語文字は表示幅2、半角文字は表示幅1として計算（`unicode_width`クレートを使用）
- 半角カナ等のUnicode文字も正確に表示幅を計算

================
Added Files
================
  src/new_feature.rs
  docs/

================
Modified Files
================
  src/main.rs
  config.toml

================
Deleted Files
================
  src/old_module.rs
  legacy/

================
Renamed Files
================
  src/utils.rs -> lib/utils.rs (95% similar)

================
Symlink Details
================
Added:
  cache -> /tmp/cache
    Type: directory

================
Submodule Details
================
  vendor/
    Source: abc123d
    Target: def456e

================
Permission Changes
================
scripts/build.sh: 100644 -> 100755

================
Patch Details
================
Generated: 8 patches, Skipped: 2 (binary)

Generated:
  src/main.rs.patch
  config.toml.patch

Skipped (binary):
  images/logo.png [skip]

================
Errors
================
  src/missing.txt (git show failed)

================
Copy Failed
================
  docs/guide.pdf
```

### 11.2 リモートリポジトリの場合

リモートURLを指定した場合、Repository行にURLが表示されます。

```
rs_gitdiffcopy Summary
======================
Repository: https://github.com/user/repo.git (remote)
  Clone mode: shallow
  Cached: ~/.cache/rs_gitdiffcopy/github.com_user_repo
Source ref: main (abc123d)
Target ref: develop (def456e)
Output: /path/to/output
Date: 2025-12-18 10:30:00
...
```

### 11.3 差分なしの場合

```
rs_gitdiffcopy Summary
======================
Repository: /path/to/repo
Source ref: main (abc123d)
Target ref: main (abc123d)
Output: /path/to/output
Date: 2025-12-18 10:30:00

No differences found.
```

### 11.4 統計情報の計算

| 項目 | 説明 |
|------|------|
| Added | targetにのみ存在するファイル数 |
| Modified | 両方に存在し、内容が変更されたファイル数 |
| Deleted | sourceにのみ存在するファイル数 |
| Renamed | リネームされたファイル数 |
| Copied | コピーされたファイル数 |
| Symlinks | 追加/削除/変更されたシンボリックリンク数（※変更なしのシンボリックリンクは含まない） |
| Submodules | サブモジュール数 |
| Permission changes | 権限のみ変更されたファイル数 |
| Unchanged | 両方に存在し、変更がないファイル数（**常に統計に表示**、`--show-unchanged`はFile Treeと詳細セクションの表示のみ制御） |
| **Total** | 検査した全ユニークパス数（source ∪ target）

**注意**:
- Totalは `source側のファイル数 + target側のファイル数 - 両方に存在するファイル数` で計算されます
- Unchangedは `--show-unchanged` オプションの有無に関わらず、常に統計情報に正確な数が表示されます
- `--show-unchanged` オプションはFile Treeや詳細セクションへの表示のみを制御し、統計情報には影響しません
- Gitの `git diff --name-status` で取得できるのは変更があったファイルのみです。Unchangedを表示するには追加で `git ls-tree` を使用して全ファイルを比較します

### 11.5 ステータスタグ一覧

| タグ | 意味 |
|------|------|
| `[added]` | 新規追加 |
| `[modified]` | 内容変更あり |
| `[deleted]` | 削除 |
| `[renamed]` | リネーム |
| `[copied]` | コピー |
| `[type-changed]` | タイプ変更 |
| `[permission]` | 権限のみ変更 |
| `[unchanged]` | 変更なし |
| `[symlink: added]` | 新規シンボリックリンク |
| `[symlink: deleted]` | 削除されたシンボリックリンク |
| `[submodule]` | Gitサブモジュール |
| `[skip]` | パッチ生成スキップ（バイナリ） |

※ シンボリックリンクはコピーされず、Symlink Detailsセクションに詳細が表示される（変更なしのシンボリックリンクは統計にもFile Treeにも含まれない）
※ バイナリファイルはパッチ生成時にスキップされ、Patch Detailsセクションに記載
※ サブモジュールはコピーされず、Submodule Detailsセクションに記載

### 11.6 Excelレポート形式

`-E, --excel <PATH>` オプションを使用すると、サマリーをExcelファイル(.xlsx)として出力できます。

#### シート構成

| シート名 | 内容 |
|----------|------|
| Summary | 基本情報、オプション、統計情報 |
| File Tree | ツリー形式のファイル一覧（セルでインデント表示、折りたたみ対応） |
| Details | 追加/変更/削除/リネーム/コピーファイル、シンボリックリンク、サブモジュール、権限変更、パッチの詳細一覧 |

#### 共通書式設定

- **罫線**: 全てのデータ領域には外枠罫線（細線）を設定し、視認性を向上
- **タイトル**: 青色太字、16pt、左揃え
- **セクションヘッダー**: 青背景（#4472C4）、白文字、太字、12pt
- **ラベル列**: 太字で表示（`Repository:`、`Source ref:`等）
- **ステータス色分け**:
  - 追加（Added）: 緑色 (#008000)
  - 変更（Modified）: 青色 (#0066CC)
  - 削除（Deleted）: 赤色 (#CC0000)
  - リネーム（Renamed）: オレンジ色 (#FF6600)
  - コピー（Copied）: シアン色 (#00CCCC)
  - シンボリックリンク: 紫色 (#9933FF)
  - サブモジュール: 茶色 (#996633)
  - 変更なし（Unchanged）: グレー (#808080)

#### Summaryシート詳細

| セクション | 内容 | 書式 |
|-----------|------|------|
| タイトル | "rs_gitdiffcopy Summary" | 16pt、青色太字 |
| 基本情報 | Repository, Source ref, Target ref, Output, Date | ラベル列は太字 |
| Options | 使用したオプション一覧（dry_run, both_versions, copy_deleted, preserve_timestamps, check_permissions, patch, patch_file, show_unchanged, filter_status, exclude, stats_only, no_tree, no_details, config_file等） | ラベル列は太字、セクションヘッダーは青背景 |
| Statistics | 統計情報（Added, Modified, Deleted, Renamed, Copied, Symlinks, Submodules, Unchanged, Total等） | セクションヘッダーは青背景、ヘッダーの幅は2列に適用 |

#### File Treeシート詳細

- **ツリー構造**: 各パスコンポーネント（ディレクトリ階層）をセル単位で分離して表示
  - 各ディレクトリ階層を別々の列に配置し、ファイル名は最後の列に配置
  - **重複省略**: 上のセルと同じ値の場合は記載しない（ツリー構造を視覚的に表現）
  - **パス展開**: 新しいディレクトリに初めて入る場合、中間ディレクトリを各行に展開する
    - これにより`--excel-fold-level`が深いパスでも正しく動作する
    - 例: `a/b/c/d.txt`が最初のパスの場合:
      ```
         | A | B | C | D    | Status |
       1 |a/ |   |   |      |        |
       2 |   |b/ |   |      |        |
       3 |   |   |c/ |      |        |
       4 |   |   |   | d.txt| added  |
      ```
    - 中間ディレクトリ行にはStatusが空（ファイルではないため）
  - ディレクトリは末尾に"/"を付与
  - 例: 以下のファイル構造の場合
    ```
    ├── a.txt
    ├── b/
    │   ├── d.txt
    │   └── e/
    │       └── g.txt
    └── c/
        ├── e.txt
        └── f/
            └── h.txt
    ```
    Excelセル表示:
    ```
       |   A   |  B   |   C   | Status  |
     1 | a.txt |      |       | added   |
     2 | b/    |      |       |         |
     3 |       | d.txt|       | modified|
     4 |       | e/   |       |         |
     5 |       |      | g.txt | modified|
     6 | c/    |      |       |         |
     7 |       | e.txt|       | deleted |
     8 |       | f/   |       |         |
     9 |       |      | h.txt | modified|
    ```
  - A列の値は行2で"b/"、行6で"c/"と変わるタイミングのみ記載
  - B列の値は親ディレクトリが変わるか、値自体が変わる場合に記載
  - 中間ディレクトリ行（Statusが空の行）はfold-levelの対象になる
- **Status列の位置**: Status列は常に「最大深さ + 1」列目に配置
  - 最大深さ3のファイル構造の場合、A〜C列がパスコンポーネント、D列がStatus
  - パスコンポーネントとStatus列が重ならないよう、動的に列位置を計算
- **フォント**: 等幅フォント（Consolas）を使用
- **ディレクトリ区切り罫線**: 第1階層（A列）のディレクトリが変わるタイミングで下罫線を追加
  - 上記例では行1、行5、行9の下に罫線を追加し、ディレクトリ単位を視覚的に区切る
- **行グループ化（折りたたみ）**: `-L, --excel-fold-level <LEVEL>` オプションで指定した深さ以上の項目をグループ化
  - 深さNは、ルートからのパス階層数（ルート直下=深さ1）
  - グループ化は各ディレクトリ単位で行う（連続した行ではなく、同一ディレクトリ配下をまとめる）
  - 例: `-L 2` の場合
    - 行3-5（b/配下）と行7-9（c/配下）がそれぞれ別グループとして折りたたみ可能
  - 例: `-L 3` の場合
    - 行5のみ（e/配下）と行9のみ（f/配下）がそれぞれ別グループとして折りたたみ可能
  - Excelの行グループ化機能（`group_rows`）を使用
- **ヘッダー行**: 動的列数（最大深さに応じて拡張）、最終列がStatus、青背景

#### Options表示仕様

SummaryファイルおよびExcelのOptionsセクションにおける`filter_status`の表示:

- `--filter-status added,modified` → "Filter status: added, modified"
- `--filter-status all` → "Filter status: all"
- `--filter-status all,^deleted` → "Filter status: all, ^deleted"
- `--filter-status ^deleted,^unchanged` → "Filter status: all (implied), ^deleted, ^unchanged"

※ 除外指定（^prefix）のみの場合は、暗黙的に"all"が適用される

#### Detailsシート詳細

- **ヘッダー行**: Status, Directory, File, Details の4列
  - ヘッダーの背景色幅は全4列に適用（merge_rangeではなく個別セル設定）
- **セクション分離**: ステータス別にセクションを分け、セクションヘッダー行を挿入
- **パス分離**: フルパスを「Directory」列と「File」列に分離して表示
- **Git固有情報**: Renamed/Copiedの場合は元パスを、Submoduleの場合はコミット情報をDetailsに表示

---

## 12. 三者間差分機能

### 12.1 概要

三者間差分（Three-way diff）機能は、共通の祖先（base）と2つの派生ref（ours/theirs）を比較し、マージ作業を支援します。

**ユースケース**:
- ブランチマージ前のコンフリクト事前確認
- 複数人で並行開発した変更の統合確認
- フォークしたプロジェクトの差分把握

### 12.2 CLI仕様（三者間モード）

```
rs_gitdiffcopy --three-way [OPTIONS]

必須オプション:
  -B, --base <REF>                 共通祖先ref
  -S, --source <REF>               自分の変更（ours）
  -T, --target <REF>               相手の変更（theirs）
  -O, --output <PATH>              差分ファイルの出力先

三者間モード専用オプション:
  -3, --three-way                  三者間比較モードを有効化
  -M, --merge-style <STYLE>        コンフリクト時のコピー方式（all/ours/theirs、デフォルト: all）
      --conflict-only              コンフリクト候補のみ出力

既存オプション（二者間と共通）:
  -e, --exclude <PATTERN>        除外パターン
  -f, --force                    出力先を全削除して再実行
  -s, --summary <PATH>           サマリーをファイルに出力
  --filter-status <STATUS>       指定ステータスのファイルのみコピー/表示
  --stats-only                   統計情報のみ表示
  --no-tree                      File Treeセクションを非表示
  --no-details                   詳細セクションを非表示
  -v, --verbose                  詳細出力モード
  -n, --dry-run                  ドライラン
  -E, --excel <PATH>             Excelレポート出力
  -L, --excel-fold-level <LEVEL> Excelファイルツリーの折りたたみレベル
  -C, --save-config <PATH>       設定ファイル保存
```

※ 三者間モードでは `--both-versions` / `--copy-deleted` / `--preserve-timestamps` は適用されない

### 12.3 使用例

```bash
# 基本的な三者間比較
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output

# コンフリクト候補のみ抽出
rs_gitdiffcopy -3 -B v1.0.0 -S hotfix -T develop -O output --conflict-only

# タグとブランチの混在
rs_gitdiffcopy -3 -B v1.0.0 -S main -T v1.1.0 -O output

# Excelレポート付き
rs_gitdiffcopy -3 -B main -S feature/a -T feature/b -O output -E report.xlsx

# Excelレポート付き（深さ2以上を折りたたみ）
rs_gitdiffcopy -3 -B main -S feature/a -T feature/b -O output -E report.xlsx -L 2

# コンフリクトのみフィルタリング
rs_gitdiffcopy -3 -B main -S feature/a -T feature/b -O output --filter-status conflict

# oursの変更のみ表示
rs_gitdiffcopy -3 -B main -S feature/a -T feature/b -O output --filter-status ours-only,added-ours
```

### 12.4 ファイル状態の判定

三者間比較では、各ファイルについてbase/ours/theirsの3つの状態を比較し、以下のように分類します：

| Base | Ours | Theirs | 状態 | 説明 |
|:----:|:----:|:------:|------|------|
| ○ | = | = | unchanged | 3つとも同一（変更なし） |
| ○ | M | = | ours-only | oursのみ変更 |
| ○ | = | M | theirs-only | theirsのみ変更 |
| ○ | M | M(same) | both-same | 両方が同じ変更 |
| ○ | M | M(diff) | **CONFLICT** | 両方が異なる変更（コンフリクト） |
| ○ | D | = | deleted-ours | oursで削除 |
| ○ | = | D | deleted-theirs | theirsで削除 |
| ○ | D | D | deleted-both | 両方で削除 |
| ○ | D | M | **CONFLICT** | oursで削除、theirsで変更（コンフリクト） |
| ○ | M | D | **CONFLICT** | oursで変更、theirsで削除（コンフリクト） |
| - | A | - | added-ours | oursで追加 |
| - | - | A | added-theirs | theirsで追加 |
| - | A | A(same) | added-both-same | 両方で同じファイルを追加 |
| - | A | A(diff) | **CONFLICT** | 両方で異なるファイルを追加（コンフリクト） |

凡例: ○=存在, -=不在, ==同一, M=変更, D=削除, A=追加

### 12.5 出力構造（三者間モード）

三者間モードでは、元のディレクトリ構造を維持したまま、ファイル名にステータスサフィックスを付与してコピーします。

```
output_dir/
├── src/
│   ├── feature.rs.ours-only           # oursのみ変更
│   ├── bugfix.rs.theirs-only          # theirsのみ変更
│   ├── handler.rs.conflict.base       # コンフリクト（base版）
│   ├── handler.rs.conflict.ours       # コンフリクト（ours版）
│   └── handler.rs.conflict.theirs     # コンフリクト（theirs版）
├── config.toml.both-same              # 両方が同じ変更
├── new_ours.txt.added-ours            # oursで追加
├── new_theirs.txt.added-theirs        # theirsで追加
└── summary.txt
```

#### ステータスサフィックス一覧

| ステータス | サフィックス | 説明 |
|-----------|-------------|------|
| ours-only | `.ours-only` | oursのみ変更されたファイル |
| theirs-only | `.theirs-only` | theirsのみ変更されたファイル |
| both-same | `.both-same` | 両方が同じ変更をしたファイル |
| added-ours | `.added-ours` | oursで追加されたファイル |
| added-theirs | `.added-theirs` | theirsで追加されたファイル |
| added-both-same | `.added-both-same` | 両方で同じ内容を追加 |
| deleted-ours | `.deleted-ours` | oursで削除されたファイル（source版をコピー） |
| deleted-theirs | `.deleted-theirs` | theirsで削除されたファイル（source版をコピー） |
| conflict | `.conflict.base`, `.conflict.ours`, `.conflict.theirs` | コンフリクト（3バージョン出力） |
| added-both-diff | `.added-both-diff.ours`, `.added-both-diff.theirs` | 両方で異なる内容を追加（コンフリクト） |
| modify-delete | `.modify-delete.base`, `.modify-delete.ours` | oursで変更、theirsで削除（コンフリクト） |
| delete-modify | `.delete-modify.base`, `.delete-modify.theirs` | oursで削除、theirsで変更（コンフリクト） |

#### `--merge-style` オプションによる出力の違い

| スタイル | 出力内容 |
|---------|---------|
| `all` | コンフリクト時にbase/ours/theirs全てのバージョンを出力（デフォルト） |
| `ours` | コンフリクト時にoursバージョンのみ出力 |
| `theirs` | コンフリクト時にtheirsバージョンのみ出力 |

例: `--merge-style ours` の場合、コンフリクトファイルは `.conflict.ours` のみが出力されます。

例: `--merge-style theirs` の場合、コンフリクトファイルは `.conflict.theirs` のみが出力されます。

### 12.6 サマリー出力形式（三者間モード）

```
rs_gitdiffcopy Summary (Three-way)
==================================
Repository: /path/to/repo
Base ref: main (abc123d)
Ours ref: feature/ours (def456e)
Theirs ref: feature/theirs (789xyz0)
Output: /path/to/output
Date: 2025-12-18 10:30:00

================
Change Matrix
================
Status          | Count
----------------|------
Unchanged       |   50
Ours only       |    8
...
--------------------------
Total           |   78
Conflicts       |    4

================
File Tree
================
Legend: [Base|Ours|Theirs] ○=exists -=missing ==same M=modified A=added D=deleted
.
├── file1.txt [○M=] ours-only
├── new_ours.txt [-A-] added-ours
├── handler.rs [○MM] CONFLICT
└── subdir/
    └── nested.txt [○=M] theirs-only
```

#### File Tree出力形式（三者間モード）

コンソール出力とファイル出力で形式が異なります。

| 出力先 | 形式 | 説明 |
|--------|------|------|
| コンソール | コンパクト形式 | `[○M=]`のように詰めて表示 |
| ファイル（`-s`指定時） | 整列形式 | 最長パス幅に合わせてインジケータ位置を揃える |

**ファイル出力例**（整列形式）:

```
================
File Tree
================
Legend: [Base|Ours|Theirs] ○=exists -=missing ==same M=modified A=added D=deleted
                                     B  O  T
.
├── file1.txt                      [○  M  =] ours-only
├── new_ours.txt                   [-  A  -] added-ours
├── 日本語ファイル.txt             [○  M  =] ours-only
└── subdir/
    └── nested.txt                 [○  =  M] theirs-only
```

- ファイル出力では、すべてのファイルのインジケータ位置を最長パス幅に合わせて揃える
- 日本語文字は表示幅2、半角文字は表示幅1として計算（`unicode_width`クレートを使用）
- 半角カナ等のUnicode文字も正確に表示幅を計算

```
================
Conflict Details
================
1. src/handler.rs
   Type: Content conflict
   Base:   abc123... (1024 bytes)
   Ours:   def456... (1100 bytes)
   Theirs: 789abc... (1050 bytes)
```

### 12.7 Excelレポート（三者間モード）

| シート名 | 内容 |
|----------|------|
| Summary | 基本情報、オプション、統計情報、コンフリクト数 |
| File Tree | ツリー形式のファイル一覧（二者間比較と同じ形式、三者間ステータス表示） |
| Conflicts | コンフリクト候補の詳細一覧 |
| Copied Files | コピーされたファイルの一覧 |

#### Summaryシートのオプション表示

三者間モードでも以下のオプションがOptionsセクションに表示される：
- `--filter-status`: フィルタリングするステータス
- `--merge-style`: マージスタイル
- `--conflict-only`: コンフリクトのみ出力
- `--exclude`: 除外パターン

#### File Treeシート

二者間比較と同じツリー形式を使用：
- 各パスコンポーネントをセル単位で分離表示
- Status列に三者間ステータス（unchanged, ours-only, theirs-only, conflict等）を表示
- `--excel-fold-level`オプションで折りたたみレベルを指定可能
- ディレクトリ区切り罫線、パス展開機能も二者間と同様に動作

#### Conflictsシート詳細

| 列 | 内容 |
|----|------|
| Directory | ディレクトリパス |
| Filename | ファイル名 |
| Type | コンフリクトタイプ |
| Base Hash | Baseのハッシュ（先頭8文字） |
| Ours Hash | Oursのハッシュ（先頭8文字） |
| Theirs Hash | Theirsのハッシュ（先頭8文字） |
| Base Size | Baseのサイズ |
| Ours Size | Oursのサイズ |
| Theirs Size | Theirsのサイズ |

- ヘッダー行の背景色は全列（9列）に適用
- パスは「Directory」と「Filename」に分離して表示

#### Copied Filesシート詳細

| 列 | 内容 |
|----|------|
| Directory | ディレクトリパス |
| Filename | ファイル名 |
| Status | ステータス |
| Source | コピー元 |

- ヘッダー行の背景色は全列（4列）に適用
- パスは「Directory」と「Filename」に分離して表示

#### ステータスの色分け

| 状態 | 色 |
|------|-----|
| unchanged | グレー (#808080) |
| ours-only | 緑 (#008000) |
| theirs-only | 青 (#0066CC) |
| both-same | 水色 (#00B0F0) |
| conflict | 赤・太字 (#CC0000) |
| added-* | 薄緑 (#92D050) |
| deleted-* | 薄赤 (#FFC7CE) |

---

## 13. 終了コード

| コード | 意味 | 説明 |
|:------:|------|------|
| 0 | 正常終了（差分あり） | 差分が検出され、正常に処理完了 |
| 1 | エラー終了 | Gitエラー、refが存在しない、権限不足など |
| 2 | 正常終了（差分なし） | 比較対象に差分がなかった |
| 3 | 正常終了（コンフリクトあり） | 三者間モードでコンフリクト検出 |

---

## 14. エラーハンドリング

### 14.1 処理停止するエラー

| ケース | 挙動 |
|--------|------|
| Gitリポジトリが見つからない | エラー終了（コード1） |
| gitコマンドが見つからない | エラー終了（コード1） |
| 無効なref（ブランチ/タグ/コミットが存在しない） | エラー終了（コード1） |
| 出力先が既に存在 | エラー終了（`--force`で全削除して実行可能）。ただし`--dry-run`指定時は、実際に書き込みを行わないためこのチェックはスキップされる。 |
| 設定ファイルの必須項目不足 | エラー終了（コード1） |
| 無効なglob パターン | エラー終了（コード1） |
| 危険なパスへの`--force` | エラー終了（コード1） |
| リモートURLへの接続失敗 | エラー終了（コード1） |
| クローン失敗 | エラー終了（コード1） |
| 認証失敗（プライベートリポジトリ） | エラー終了（コード1） |

### 14.2 処理継続する事象（警告/情報）

| ケース | 挙動 | サマリーセクション |
|--------|------|-----------------|
| git showでファイル取得失敗 | スキップ（警告） | Errors |
| ファイルコピー失敗 | スキップ（警告） | Copy Failed |
| パッチ生成失敗 | スキップ（警告） | Patch Details |
| シンボリックリンク | 情報として記載（コピーしない） | Symlink Details |
| サブモジュール | 情報として記載（コピーしない） | Submodule Details |

### 14.3 エラーメッセージ一覧

| エラー | メッセージ例 |
|--------|-------------|
| リポジトリ未検出 | `Error: Not a git repository (or any parent up to /)` |
| gitコマンド未検出 | `Error: Git command not found. Please install Git or specify path with --git-path or settings.toml/--config.` |
| ref解決失敗 | `Error: Unknown revision 'invalid-branch'` |
| ファイル取得失敗 | `Error: Failed to get file 'path/to/file' from commit abc123` |
| 出力先が既に存在 | `Error: Output directory '/path/to/output' already exists. Use --force to overwrite.` |
| 設定ファイル読み込み失敗 | `Error: Failed to read config file 'path/to/config.toml': <詳細>` |
| 設定ファイル必須項目不足 | `Error: Missing required field 'source' in config file` |
| 無効な除外パターン | `Error: Invalid glob pattern '**[invalid'` |
| 危険なパスへの--force | `Error: Cannot use --force on protected path '/home'` |
| 接続失敗 | `Error: Could not resolve host 'github.com'` |
| クローン失敗 | `Error: Failed to clone repository 'https://github.com/user/repo.git'` |
| 認証失敗 | `Error: Authentication failed for 'https://github.com/user/private-repo.git'` |
| リモートref取得失敗 | `Error: Could not fetch ref 'feature/branch' from remote` |

---

## 15. --force オプションの安全機能

### 15.1 危険なパスの検出

`--force` オプション使用時、以下のパスへの削除を禁止します：

| OS | 禁止パス |
|----|---------|
| 共通 | `/`, `~`, `$HOME` |
| Windows | `C:\`, `C:\Windows`, `C:\Program Files`, `C:\Users`, `C:\Users\<username>`, `C:\Users\<username>\<foldername>` |
| Linux | `/bin`, `/sbin`, `/usr`, `/etc`, `/var`, `/home`, `/root` |
| macOS | `/System`, `/Library`, `/Users`, `/Applications` |

**保護理由（Windowsユーザーディレクトリ）**:
`C:\Users\<username>\<foldername>` には AppData などの重要なフォルダーが含まれ、通常はアプリケーションの出力先になりえません。
このため安全のために保護対象としています。

### 15.2 安全チェックの動作

1. 出力先パスを正規化（シンボリックリンク解決、相対パス展開）
2. 禁止パスリストと照合
3. 禁止パスまたはその直下の場合、エラー終了（コード1）

### 15.3 削除前の確認

`--force` 使用時、出力先ディレクトリが存在する場合は必ず確認プロンプトを表示します：

```
Output directory '/path/to/output' already exists.
  Contains: 45 files, 12 directories
  Total size: 1.2 MB

Delete and continue? [y/N]:
```

※ 安全のため、確認プロンプトをスキップするオプションはありません。

---

## 16. 文字エンコーディング

### 16.1 対応エンコーディング

| エンコーディング | 対応状況 |
|-----------------|---------|
| UTF-8 | 完全対応（推奨） |
| UTF-8 with BOM | 対応（BOM保持） |
| Shift_JIS | 対応（変換なし、バイナリとして処理） |
| EUC-JP | 対応（変換なし、バイナリとして処理） |
| その他 | バイナリとして処理 |

### 16.2 ファイル名のエンコーディング

- **Windows**: UTF-16をUTF-8に変換して処理
- **Linux/macOS**: UTF-8として処理
- **非UTF-8ファイル名**: エラーをサマリーに記載し、スキップ

### 16.3 gitコマンドの出力

gitコマンドの出力はUTF-8として解釈します。日本語ファイル名がエスケープされる場合は、`core.quotepath=false` の設定を推奨します。

```bash
git config --global core.quotepath false
```

---

## 17. 技術仕様まとめ

| 項目 | 仕様 |
|------|------|
| 入力 | Git ref（ブランチ/タグ/コミット） |
| 差分検出 | `git diff --name-status -M -C` |
| ファイル取得 | `git show <commit>:<path>` |
| タイムスタンプ | `git log` のコミット日時 |
| 権限取得 | `git ls-tree` |
| パッチ生成 | `git diff` コマンド |
| シンボリックリンク | git diff結果から判定、コピーはスキップ |
| サブモジュール | 検出してスキップ、サマリーに記載 |
| リネーム検出 | git diffの `-M -C` オプションで検出 |
| 外部依存 | Git（必須） |

---

## 18. 将来の機能検討

### 18.1 v1.1 で検討する機能

| 機能 | オプション案 | 説明 |
|------|-------------|------|
| merge-base自動検出 | `--auto-base` | 三者間比較でgit merge-baseを自動使用 |

### 18.2 v1.2 以降で検討する機能

| 機能 | オプション案 | 説明 |
|------|-------------|------|
| ステージング領域対応 | `--staged` | ステージング領域との比較 |
| ワーキングツリー対応 | `--working` | ワーキングツリーとの比較 |
| 比較前の自動fetch | `--fetch` | 比較前に`git fetch`を自動実行する |
| サブモジュール展開 | `--recurse-submodules` | サブモジュール内も比較 |
| JSON出力 | `--json` | 機械可読なJSON形式でサマリー出力 |

---

## 19. 付録：gitコマンドリファレンス

### 19.1 よく使うgitコマンドオプション

```bash
# ref解決
git rev-parse main              # ブランチ -> コミットハッシュ
git rev-parse v1.0.0            # タグ -> コミットハッシュ
git rev-parse HEAD~3            # 相対参照 -> コミットハッシュ

# 差分ファイルリスト
git diff --name-status -M -C abc123 def456  # ステータス付きファイルリスト
git diff --name-only abc123 def456          # ファイルリストのみ
git diff -M -C --name-status abc123 def456  # リネーム/コピー検出付き

# ファイル内容取得
git show abc123:path/to/file.txt            # 特定コミットのファイル内容

# ファイル権限取得
git ls-tree abc123 path/to/file.txt         # モード・タイプ・ハッシュ・パス

# コミット日時取得
git log -1 --format=%ci abc123 -- path/to/file.txt

# パッチ生成
git diff -M -C abc123 def456 -- path/to/file.txt  # 個別ファイルのパッチ
git diff -M -C abc123 def456                       # 全変更のパッチ
```

---

## 20. 要件定義 変更履歴

| 日付 | バージョン | 変更内容 |
|------|-----------|---------|
| 2026-01-22 | 1.0.1 | Excel File TreeのStatus列位置仕様を明記（最大深さ + 1列目に配置） |
| 2026-01-22 | 1.0.1 | 三者間モードのファイル出力形式を修正（ステータス別サブディレクトリからステータスサフィックス形式に変更） |
| 2026-01-22 | 1.0.1 | 三者間モードのFileTree表示がTree構造で出力されることを確認（コンソール・サマリーファイル両方） |
