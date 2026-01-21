//! CLI引数解析

use crate::types::{ColorMode, LogLevel, MergeStyle, PermissionCheckMode};
use clap::{ArgAction, Parser};
use std::path::PathBuf;

/// Git diff コマンドの出力を初心者・非技術者でも視覚的に分かるツール
#[derive(Parser, Debug, Clone)]
#[command(name = "rs_gitdiffcopy")]
#[command(version = "1.0.0")]
#[command(about = "Git diff コマンドの出力を初心者・非技術者でも視覚的に分かるツール")]
#[command(author = "Your Name")]
pub struct Cli {
    // === 必須オプション（設定ファイル未使用時） ===
    /// 比較元ref（before）: ブランチ名/タグ名/コミットハッシュ
    #[arg(short = 'S', long)]
    pub source: Option<String>,

    /// 比較先ref（after）: ブランチ名/タグ名/コミットハッシュ
    #[arg(short = 'T', long)]
    pub target: Option<String>,

    /// 差分ファイルの出力先
    #[arg(short = 'O', long)]
    pub output: Option<PathBuf>,

    // === リポジトリオプション ===
    /// GitリポジトリのパスまたはリモートURL
    #[arg(short = 'R', long)]
    pub repository: Option<String>,

    /// リモートURL時にフルクローンを実行（デフォルト: shallow clone）
    #[arg(long)]
    pub full_clone: bool,

    /// リモートリポジトリのクローンをキャッシュするディレクトリ
    #[arg(long)]
    pub cache_dir: Option<PathBuf>,

    // === オプション ===
    /// 設定ファイル（TOML形式）
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,

    /// 使用するgitコマンドの実行パス
    #[arg(long)]
    pub git_path: Option<PathBuf>,

    /// 除外パターン（複数指定可、glob形式）
    #[arg(short = 'e', long, action = ArgAction::Append)]
    pub exclude: Vec<String>,

    /// 出力先を全削除して再実行（削除前に確認プロンプト表示）
    #[arg(short = 'f', long)]
    pub force: bool,

    /// サマリーをファイルに出力（デフォルト: 標準出力）
    #[arg(short = 's', long)]
    pub summary: Option<PathBuf>,

    /// 詳細出力モード（処理中ファイル名を表示）
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// 実際にコピーせず、対象ファイルを表示
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// 変更ファイルの新旧両方をコピー（.old/.new拡張子付与）
    #[arg(short = 'b', long)]
    pub both_versions: bool,

    /// 権限変更をチェック（none/scripts/all）
    #[arg(short = 'P', long, default_value = "none")]
    pub check_permissions: PermissionCheckMode,

    /// 変更ファイルごとに個別パッチファイル(.patch)を生成
    #[arg(short = 'p', long)]
    pub patch: bool,

    /// 全変更を統合したパッチファイルを生成
    #[arg(short = 'F', long)]
    pub patch_file: Option<PathBuf>,

    /// サマリーをExcelファイル(.xlsx)に出力
    #[arg(short = 'E', long)]
    pub excel: Option<PathBuf>,

    /// Excelファイルツリーの折りたたみレベル（指定深さ以上を折りたたみ）
    #[arg(short = 'L', long)]
    pub excel_fold_level: Option<u32>,

    /// 変更がないファイルをサマリー詳細に表示
    #[arg(short = 'u', long)]
    pub show_unchanged: bool,

    /// 現在のオプションを設定ファイル(TOML形式)に保存
    #[arg(short = 'C', long)]
    pub save_config: Option<PathBuf>,

    /// 指定ステータスのファイルのみコピー/表示（複数指定可: カンマ区切り）
    #[arg(long, value_delimiter = ',')]
    pub filter_status: Vec<String>,

    /// 統計情報のみ表示
    #[arg(long)]
    pub stats_only: bool,

    /// File Treeセクションを非表示
    #[arg(long)]
    pub no_tree: bool,

    /// 詳細セクション（Added/Modified/Deleted Files等）を非表示
    #[arg(long)]
    pub no_details: bool,

    /// 削除ファイルもコピー（.deleted拡張子付与）
    #[arg(long)]
    pub copy_deleted: bool,

    /// コピー時にコミットのタイムスタンプを保持
    #[arg(long)]
    pub preserve_timestamps: bool,

    /// 並列処理のワーカー数（デフォルト: CPUコア数）
    #[arg(short = 'j', long)]
    pub workers: Option<usize>,

    /// 一時ファイルの保存先ディレクトリ
    #[arg(long)]
    pub temp_dir: Option<PathBuf>,

    /// カラー出力（auto/always/never）
    #[arg(long, default_value = "auto")]
    pub color: ColorMode,

    /// ログ出力レベル（error/warn/info/debug）
    #[arg(long, default_value = "warn")]
    pub log_level: LogLevel,

    // === 三者間モード専用オプション ===
    /// 三者間比較モードを有効化
    #[arg(short = '3', long)]
    pub three_way: bool,

    /// 共通祖先ref
    #[arg(short = 'B', long)]
    pub base: Option<String>,

    /// コンフリクト時のコピー方式（all/ours/theirs）
    #[arg(short = 'M', long, default_value = "all")]
    pub merge_style: MergeStyle,

    /// コンフリクト候補のみ出力
    #[arg(long)]
    pub conflict_only: bool,
}

impl Cli {
    /// 引数をパースして取得
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
