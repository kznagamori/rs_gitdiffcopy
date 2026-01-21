//! 設定ファイルの読み込み・パース

use crate::cli::Cli;
use crate::error::{AppError, Result};
use crate::types::{ColorMode, LogLevel, MergeStyle, PermissionCheckMode};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// アプリケーション設定ファイル（settings.toml）
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AppSettings {
    /// gitコマンドの実行パス
    pub git_path: Option<PathBuf>,
    /// リモートリポジトリのクローンキャッシュディレクトリ
    pub cache_dir: Option<PathBuf>,
    /// 一時ファイルの保存先
    pub temp_dir: Option<PathBuf>,
    /// 並列処理のワーカー数
    pub workers: Option<usize>,
    /// リモートリポジトリのデフォルトクローン方式（true: フルクローン）
    pub full_clone: Option<bool>,
    /// カラー出力設定
    pub color: Option<ColorMode>,
    /// ログ出力レベル
    pub log_level: Option<LogLevel>,
    /// デフォルトの除外パターン
    #[serde(default)]
    pub default_exclude: Vec<String>,
}

impl AppSettings {
    /// アプリケーション設定ファイルを読み込む
    pub fn load() -> Result<Self> {
        let exe_path = std::env::current_exe().map_err(|e| AppError::Io(e))?;
        let settings_path = exe_path.parent().unwrap_or(Path::new(".")).join("settings.toml");

        if settings_path.exists() {
            let content = std::fs::read_to_string(&settings_path)?;
            toml::from_str(&content).map_err(|e| AppError::ConfigReadFailed {
                path: settings_path.display().to_string(),
                detail: e.to_string(),
            })
        } else {
            Ok(Self::default())
        }
    }
}

/// 入力設定ファイル（-c で指定するTOMLファイル）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputConfig {
    /// Gitリポジトリのパス
    pub repository: Option<String>,
    /// gitコマンドのパス
    pub git_path: Option<PathBuf>,
    /// フルクローンを実行するか
    pub full_clone: Option<bool>,
    /// クローンのキャッシュディレクトリ
    pub cache_dir: Option<PathBuf>,
    /// 比較元ref
    pub source: Option<String>,
    /// 比較先ref
    pub target: Option<String>,
    /// 出力ディレクトリ
    pub output: Option<PathBuf>,
    /// 除外パターン
    #[serde(default)]
    pub exclude: Vec<String>,
    /// 強制上書き
    pub force: Option<bool>,
    /// 詳細出力
    pub verbose: Option<bool>,
    /// ドライラン
    pub dry_run: Option<bool>,
    /// 新旧両方をコピー
    pub both_versions: Option<bool>,
    /// サマリー出力先
    pub summary: Option<PathBuf>,
    /// 権限チェックモード
    pub check_permissions: Option<PermissionCheckMode>,
    /// 個別パッチファイル生成
    pub patch: Option<bool>,
    /// 統合パッチファイルパス
    pub patch_file: Option<PathBuf>,
    /// Excelレポート出力パス
    pub excel: Option<PathBuf>,
    /// Excelファイルツリーの折りたたみレベル
    pub excel_fold_level: Option<u32>,
    /// 変更がないファイルを表示
    pub show_unchanged: Option<bool>,
    /// フィルターするステータス
    #[serde(default)]
    pub filter_status: Vec<String>,
    /// 統計情報のみ表示
    pub stats_only: Option<bool>,
    /// File Treeセクション非表示
    pub no_tree: Option<bool>,
    /// 詳細セクション非表示
    pub no_details: Option<bool>,
    /// 削除ファイルもコピー
    pub copy_deleted: Option<bool>,
    /// タイムスタンプを保持
    pub preserve_timestamps: Option<bool>,
    /// ワーカー数
    pub workers: Option<usize>,
    /// 一時ファイルの保存先
    pub temp_dir: Option<PathBuf>,
    /// カラー出力
    pub color: Option<ColorMode>,
    /// ログ出力レベル
    pub log_level: Option<LogLevel>,
    /// 三者間比較モード
    pub three_way: Option<bool>,
    /// 共通祖先ref
    pub base: Option<String>,
    /// マージスタイル
    pub merge_style: Option<MergeStyle>,
    /// コンフリクトのみ出力
    pub conflict_only: Option<bool>,
}

impl InputConfig {
    /// 設定ファイルを読み込む
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| AppError::ConfigReadFailed {
            path: path.display().to_string(),
            detail: e.to_string(),
        })?;

        toml::from_str(&content).map_err(|e| AppError::ConfigReadFailed {
            path: path.display().to_string(),
            detail: e.to_string(),
        })
    }

    /// 設定ファイルに保存
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).map_err(|e| AppError::Other(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// マージされた設定
#[derive(Debug, Clone)]
pub struct MergedConfig {
    /// Gitリポジトリのパス
    pub repository: Option<String>,
    /// gitコマンドのパス
    pub git_path: Option<PathBuf>,
    /// フルクローンを実行するか
    pub full_clone: bool,
    /// クローンのキャッシュディレクトリ
    pub cache_dir: Option<PathBuf>,
    /// 比較元ref
    pub source: String,
    /// 比較先ref
    pub target: String,
    /// 出力ディレクトリ
    pub output: PathBuf,
    /// 除外パターン（マージ済み）
    pub exclude: Vec<String>,
    /// 強制上書き
    pub force: bool,
    /// 詳細出力
    pub verbose: bool,
    /// ドライラン
    pub dry_run: bool,
    /// 新旧両方をコピー
    pub both_versions: bool,
    /// サマリー出力先
    pub summary: Option<PathBuf>,
    /// 権限チェックモード
    pub check_permissions: PermissionCheckMode,
    /// 個別パッチファイル生成
    pub patch: bool,
    /// 統合パッチファイルパス
    pub patch_file: Option<PathBuf>,
    /// Excelレポート出力パス
    pub excel: Option<PathBuf>,
    /// Excelファイルツリーの折りたたみレベル
    pub excel_fold_level: Option<u32>,
    /// 変更がないファイルを表示
    pub show_unchanged: bool,
    /// フィルターするステータス
    pub filter_status: Vec<String>,
    /// 統計情報のみ表示
    pub stats_only: bool,
    /// File Treeセクション非表示
    pub no_tree: bool,
    /// 詳細セクション非表示
    pub no_details: bool,
    /// 削除ファイルもコピー
    pub copy_deleted: bool,
    /// タイムスタンプを保持
    pub preserve_timestamps: bool,
    /// ワーカー数
    pub workers: usize,
    /// 一時ファイルの保存先
    pub temp_dir: Option<PathBuf>,
    /// カラー出力
    pub color: ColorMode,
    /// ログ出力レベル
    pub log_level: LogLevel,
    /// 三者間比較モード
    pub three_way: bool,
    /// 共通祖先ref
    pub base: Option<String>,
    /// マージスタイル
    pub merge_style: MergeStyle,
    /// コンフリクトのみ出力
    pub conflict_only: bool,
}

impl MergedConfig {
    /// CLI引数、入力設定ファイル、アプリケーション設定をマージ
    pub fn merge(cli: &Cli, input_config: Option<&InputConfig>, app_settings: &AppSettings) -> Result<Self> {
        // source, target, output は必須
        let source = cli
            .source
            .clone()
            .or_else(|| input_config.and_then(|c| c.source.clone()))
            .ok_or_else(|| AppError::MissingRequiredField("source".to_string()))?;

        let target = cli
            .target
            .clone()
            .or_else(|| input_config.and_then(|c| c.target.clone()))
            .ok_or_else(|| AppError::MissingRequiredField("target".to_string()))?;

        let output = cli
            .output
            .clone()
            .or_else(|| input_config.and_then(|c| c.output.clone()))
            .ok_or_else(|| AppError::MissingRequiredField("output".to_string()))?;

        // 三者間モードの場合、baseが必須
        let three_way = cli.three_way || input_config.and_then(|c| c.three_way).unwrap_or(false);
        let base = cli
            .base
            .clone()
            .or_else(|| input_config.and_then(|c| c.base.clone()));

        if three_way && base.is_none() {
            return Err(AppError::ThreeWayRequiresBase);
        }

        // 除外パターンをマージ（default_exclude + exclude）
        let mut exclude = app_settings.default_exclude.clone();
        if let Some(config) = input_config {
            exclude.extend(config.exclude.clone());
        }
        exclude.extend(cli.exclude.clone());

        // ワーカー数のデフォルトはCPUコア数
        let default_workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Ok(Self {
            repository: cli
                .repository
                .clone()
                .or_else(|| input_config.and_then(|c| c.repository.clone())),
            git_path: cli
                .git_path
                .clone()
                .or_else(|| input_config.and_then(|c| c.git_path.clone()))
                .or_else(|| app_settings.git_path.clone()),
            full_clone: cli.full_clone
                || input_config.and_then(|c| c.full_clone).unwrap_or(false)
                || app_settings.full_clone.unwrap_or(false),
            cache_dir: cli
                .cache_dir
                .clone()
                .or_else(|| input_config.and_then(|c| c.cache_dir.clone()))
                .or_else(|| app_settings.cache_dir.clone()),
            source,
            target,
            output,
            exclude,
            force: cli.force || input_config.and_then(|c| c.force).unwrap_or(false),
            verbose: cli.verbose || input_config.and_then(|c| c.verbose).unwrap_or(false),
            dry_run: cli.dry_run || input_config.and_then(|c| c.dry_run).unwrap_or(false),
            both_versions: cli.both_versions
                || input_config.and_then(|c| c.both_versions).unwrap_or(false),
            summary: cli
                .summary
                .clone()
                .or_else(|| input_config.and_then(|c| c.summary.clone())),
            check_permissions: if cli.check_permissions != PermissionCheckMode::None {
                cli.check_permissions
            } else {
                input_config
                    .and_then(|c| c.check_permissions)
                    .unwrap_or(PermissionCheckMode::None)
            },
            patch: cli.patch || input_config.and_then(|c| c.patch).unwrap_or(false),
            patch_file: cli
                .patch_file
                .clone()
                .or_else(|| input_config.and_then(|c| c.patch_file.clone())),
            excel: cli
                .excel
                .clone()
                .or_else(|| input_config.and_then(|c| c.excel.clone())),
            excel_fold_level: cli
                .excel_fold_level
                .or_else(|| input_config.and_then(|c| c.excel_fold_level)),
            show_unchanged: cli.show_unchanged
                || input_config.and_then(|c| c.show_unchanged).unwrap_or(false),
            filter_status: if !cli.filter_status.is_empty() {
                cli.filter_status.clone()
            } else {
                input_config
                    .map(|c| c.filter_status.clone())
                    .unwrap_or_default()
            },
            stats_only: cli.stats_only || input_config.and_then(|c| c.stats_only).unwrap_or(false),
            no_tree: cli.no_tree || input_config.and_then(|c| c.no_tree).unwrap_or(false),
            no_details: cli.no_details || input_config.and_then(|c| c.no_details).unwrap_or(false),
            copy_deleted: cli.copy_deleted
                || input_config.and_then(|c| c.copy_deleted).unwrap_or(false),
            preserve_timestamps: cli.preserve_timestamps
                || input_config.and_then(|c| c.preserve_timestamps).unwrap_or(false),
            workers: cli
                .workers
                .or_else(|| input_config.and_then(|c| c.workers))
                .or(app_settings.workers)
                .unwrap_or(default_workers),
            temp_dir: cli
                .temp_dir
                .clone()
                .or_else(|| input_config.and_then(|c| c.temp_dir.clone()))
                .or_else(|| app_settings.temp_dir.clone()),
            color: if cli.color != ColorMode::Auto {
                cli.color
            } else {
                input_config
                    .and_then(|c| c.color)
                    .or(app_settings.color)
                    .unwrap_or(ColorMode::Auto)
            },
            log_level: if cli.log_level != LogLevel::Warn {
                cli.log_level
            } else {
                input_config
                    .and_then(|c| c.log_level)
                    .or(app_settings.log_level)
                    .unwrap_or(LogLevel::Warn)
            },
            three_way,
            base,
            merge_style: if cli.merge_style != MergeStyle::All {
                cli.merge_style
            } else {
                input_config
                    .and_then(|c| c.merge_style)
                    .unwrap_or(MergeStyle::All)
            },
            conflict_only: cli.conflict_only
                || input_config.and_then(|c| c.conflict_only).unwrap_or(false),
        })
    }

    /// 現在の設定をInputConfigに変換（保存用）
    pub fn to_input_config(&self) -> InputConfig {
        InputConfig {
            repository: self.repository.clone(),
            git_path: self.git_path.clone(),
            full_clone: Some(self.full_clone),
            cache_dir: self.cache_dir.clone(),
            source: Some(self.source.clone()),
            target: Some(self.target.clone()),
            output: Some(self.output.clone()),
            exclude: self.exclude.clone(),
            force: Some(self.force),
            verbose: Some(self.verbose),
            dry_run: Some(self.dry_run),
            both_versions: Some(self.both_versions),
            summary: self.summary.clone(),
            check_permissions: Some(self.check_permissions),
            patch: Some(self.patch),
            patch_file: self.patch_file.clone(),
            excel: self.excel.clone(),
            excel_fold_level: self.excel_fold_level,
            show_unchanged: Some(self.show_unchanged),
            filter_status: self.filter_status.clone(),
            stats_only: Some(self.stats_only),
            no_tree: Some(self.no_tree),
            no_details: Some(self.no_details),
            copy_deleted: Some(self.copy_deleted),
            preserve_timestamps: Some(self.preserve_timestamps),
            workers: Some(self.workers),
            temp_dir: self.temp_dir.clone(),
            color: Some(self.color),
            log_level: Some(self.log_level),
            three_way: Some(self.three_way),
            base: self.base.clone(),
            merge_style: Some(self.merge_style),
            conflict_only: Some(self.conflict_only),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_minimal_cli() -> Cli {
        Cli {
            source: Some("main".to_string()),
            target: Some("develop".to_string()),
            output: Some(PathBuf::from("/tmp/output")),
            repository: None,
            full_clone: false,
            cache_dir: None,
            config: None,
            git_path: None,
            exclude: vec![],
            force: false,
            summary: None,
            verbose: false,
            dry_run: false,
            both_versions: false,
            check_permissions: PermissionCheckMode::None,
            patch: false,
            patch_file: None,
            excel: None,
            excel_fold_level: None,
            show_unchanged: false,
            save_config: None,
            filter_status: vec![],
            stats_only: false,
            no_tree: false,
            no_details: false,
            copy_deleted: false,
            preserve_timestamps: false,
            workers: None,
            temp_dir: None,
            color: ColorMode::Auto,
            log_level: LogLevel::Warn,
            three_way: false,
            base: None,
            merge_style: MergeStyle::All,
            conflict_only: false,
        }
    }

    #[test]
    fn test_input_config_parse_basic_toml() {
        let toml_content = r#"
            source = "main"
            target = "develop"
            output = "/tmp/output"
        "#;
        let config: InputConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.source, Some("main".to_string()));
        assert_eq!(config.target, Some("develop".to_string()));
        assert_eq!(config.output, Some(PathBuf::from("/tmp/output")));
    }

    #[test]
    fn test_input_config_parse_with_exclude() {
        let toml_content = r#"
            source = "main"
            target = "develop"
            output = "/tmp/output"
            exclude = ["*.log", "node_modules/**"]
        "#;
        let config: InputConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.exclude.len(), 2);
        assert_eq!(config.exclude[0], "*.log");
        assert_eq!(config.exclude[1], "node_modules/**");
    }

    #[test]
    fn test_input_config_parse_with_filter_status() {
        let toml_content = r#"
            source = "main"
            target = "develop"
            output = "/tmp/output"
            filter_status = ["added", "modified"]
        "#;
        let config: InputConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.filter_status.len(), 2);
        assert_eq!(config.filter_status[0], "added");
        assert_eq!(config.filter_status[1], "modified");
    }

    #[test]
    fn test_input_config_parse_three_way_mode() {
        let toml_content = r#"
            source = "feature/ours"
            target = "feature/theirs"
            output = "/tmp/output"
            three_way = true
            base = "main"
            merge_style = "ours"
            conflict_only = true
        "#;
        let config: InputConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.three_way, Some(true));
        assert_eq!(config.base, Some("main".to_string()));
        assert_eq!(config.merge_style, Some(MergeStyle::Ours));
        assert_eq!(config.conflict_only, Some(true));
    }

    #[test]
    fn test_merged_config_missing_source() {
        let cli = Cli {
            source: None,
            target: Some("develop".to_string()),
            output: Some(PathBuf::from("/tmp/output")),
            ..create_minimal_cli()
        };
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, None, &app_settings);
        assert!(result.is_err());
    }

    #[test]
    fn test_merged_config_missing_target() {
        let cli = Cli {
            source: Some("main".to_string()),
            target: None,
            output: Some(PathBuf::from("/tmp/output")),
            ..create_minimal_cli()
        };
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, None, &app_settings);
        assert!(result.is_err());
    }

    #[test]
    fn test_merged_config_missing_output() {
        let cli = Cli {
            source: Some("main".to_string()),
            target: Some("develop".to_string()),
            output: None,
            ..create_minimal_cli()
        };
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, None, &app_settings);
        assert!(result.is_err());
    }

    #[test]
    fn test_merged_config_three_way_requires_base() {
        let cli = Cli {
            three_way: true,
            base: None,
            ..create_minimal_cli()
        };
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, None, &app_settings);
        assert!(result.is_err());
    }

    #[test]
    fn test_merged_config_three_way_with_base() {
        let cli = Cli {
            three_way: true,
            base: Some("main".to_string()),
            ..create_minimal_cli()
        };
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, None, &app_settings);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.three_way);
        assert_eq!(config.base, Some("main".to_string()));
    }

    #[test]
    fn test_merged_config_cli_priority_over_config() {
        let cli = Cli {
            source: Some("cli-source".to_string()),
            verbose: true,
            ..create_minimal_cli()
        };
        let input_config = InputConfig {
            source: Some("config-source".to_string()),
            verbose: Some(false),
            ..InputConfig::default()
        };
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, Some(&input_config), &app_settings);
        assert!(result.is_ok());
        let config = result.unwrap();
        // CLI引数が優先される
        assert_eq!(config.source, "cli-source");
        assert!(config.verbose);
    }

    #[test]
    fn test_merged_config_exclude_merge() {
        let cli = Cli {
            exclude: vec!["*.tmp".to_string()],
            ..create_minimal_cli()
        };
        let input_config = InputConfig {
            exclude: vec!["*.log".to_string()],
            ..InputConfig::default()
        };
        let app_settings = AppSettings {
            default_exclude: vec!["node_modules/**".to_string()],
            ..AppSettings::default()
        };
        let result = MergedConfig::merge(&cli, Some(&input_config), &app_settings);
        assert!(result.is_ok());
        let config = result.unwrap();
        // 全ての除外パターンがマージされる
        assert_eq!(config.exclude.len(), 3);
        assert!(config.exclude.contains(&"node_modules/**".to_string()));
        assert!(config.exclude.contains(&"*.log".to_string()));
        assert!(config.exclude.contains(&"*.tmp".to_string()));
    }

    #[test]
    fn test_merged_config_filter_status_cli_priority() {
        let cli = Cli {
            filter_status: vec!["added".to_string()],
            ..create_minimal_cli()
        };
        let input_config = InputConfig {
            filter_status: vec!["modified".to_string(), "deleted".to_string()],
            ..InputConfig::default()
        };
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, Some(&input_config), &app_settings);
        assert!(result.is_ok());
        let config = result.unwrap();
        // CLI引数が優先される（マージではない）
        assert_eq!(config.filter_status.len(), 1);
        assert_eq!(config.filter_status[0], "added");
    }

    #[test]
    fn test_merged_config_to_input_config_roundtrip() {
        let cli = Cli {
            force: true,
            verbose: true,
            dry_run: true,
            both_versions: true,
            ..create_minimal_cli()
        };
        let app_settings = AppSettings::default();
        let merged = MergedConfig::merge(&cli, None, &app_settings).unwrap();
        let input_config = merged.to_input_config();

        assert_eq!(input_config.source, Some("main".to_string()));
        assert_eq!(input_config.target, Some("develop".to_string()));
        assert_eq!(input_config.force, Some(true));
        assert_eq!(input_config.verbose, Some(true));
        assert_eq!(input_config.dry_run, Some(true));
        assert_eq!(input_config.both_versions, Some(true));
    }

    #[test]
    fn test_merged_config_default_workers() {
        let cli = create_minimal_cli();
        let app_settings = AppSettings::default();
        let result = MergedConfig::merge(&cli, None, &app_settings);
        assert!(result.is_ok());
        let config = result.unwrap();
        // デフォルトはCPUコア数（1以上であること）
        assert!(config.workers >= 1);
    }

    #[test]
    fn test_merged_config_workers_priority() {
        let cli = Cli {
            workers: Some(8),
            ..create_minimal_cli()
        };
        let input_config = InputConfig {
            workers: Some(4),
            ..InputConfig::default()
        };
        let app_settings = AppSettings {
            workers: Some(2),
            ..AppSettings::default()
        };
        let result = MergedConfig::merge(&cli, Some(&input_config), &app_settings);
        assert!(result.is_ok());
        let config = result.unwrap();
        // CLI > InputConfig > AppSettings の優先順位
        assert_eq!(config.workers, 8);
    }

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert!(settings.git_path.is_none());
        assert!(settings.cache_dir.is_none());
        assert!(settings.temp_dir.is_none());
        assert!(settings.workers.is_none());
        assert!(settings.full_clone.is_none());
        assert!(settings.color.is_none());
        assert!(settings.log_level.is_none());
        assert!(settings.default_exclude.is_empty());
    }

    #[test]
    fn test_input_config_default() {
        let config = InputConfig::default();
        assert!(config.source.is_none());
        assert!(config.target.is_none());
        assert!(config.output.is_none());
        assert!(config.exclude.is_empty());
        assert!(config.filter_status.is_empty());
    }
}
