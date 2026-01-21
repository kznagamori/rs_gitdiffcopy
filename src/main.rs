//! rs_gitdiffcopy - Git diff コマンドの出力を視覚的に分かりやすくするツール

use rs_gitdiffcopy::cli::Cli;
use rs_gitdiffcopy::config::{AppSettings, InputConfig, MergedConfig};
use rs_gitdiffcopy::copy::FileCopier;
use rs_gitdiffcopy::diff::DiffDetector;
use rs_gitdiffcopy::error::{AppError, ExitCode, Result};
use rs_gitdiffcopy::excel::ExcelWriter;
use rs_gitdiffcopy::git::Git;
use rs_gitdiffcopy::safety;
use rs_gitdiffcopy::summary::SummaryWriter;
use rs_gitdiffcopy::types::expand_filter_status_three_way;

use std::process;

fn main() {
    let exit_code = run();
    process::exit(exit_code.into());
}

fn run() -> ExitCode {
    // CLI引数をパース
    let cli = Cli::parse_args();

    // アプリケーション設定を読み込み
    let app_settings = match AppSettings::load() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Warning: Failed to load settings.toml: {}", e);
            AppSettings::default()
        }
    };

    // 入力設定ファイルを読み込み（指定されている場合）
    let input_config = if let Some(ref config_path) = cli.config {
        match InputConfig::load(config_path) {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitCode::Error;
            }
        }
    } else {
        None
    };

    // 設定をマージ
    let config = match MergedConfig::merge(&cli, input_config.as_ref(), &app_settings) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::Error;
        }
    };

    // ログレベルを設定
    env_logger::Builder::new()
        .filter_level(config.log_level.into())
        .init();

    // 設定ファイルを保存（指定されている場合）
    if let Some(ref save_path) = cli.save_config {
        let save_config = config.to_input_config();
        if let Err(e) = save_config.save(save_path) {
            eprintln!("Error: Failed to save config: {}", e);
            return ExitCode::Error;
        }
        println!("Configuration saved to: {}", save_path.display());
        return ExitCode::Success;
    }

    // 実行
    match execute(&config) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::Error
        }
    }
}

fn execute(config: &MergedConfig) -> Result<ExitCode> {
    // 出力先の安全性チェック
    if config.force {
        safety::validate_output_path(&config.output)?;
    }

    // 出力先が既に存在する場合の処理
    if config.output.exists() && !config.dry_run {
        if config.force {
            // 確認プロンプトを表示
            if !safety::confirm_delete(&config.output)? {
                println!("Aborted.");
                return Ok(ExitCode::Error);
            }
            safety::remove_directory(&config.output)?;
        } else {
            return Err(AppError::OutputExists(config.output.display().to_string()));
        }
    }

    // リポジトリパスを決定
    let repo_path = if let Some(ref repo) = config.repository {
        if Git::is_remote_url(repo) {
            // リモートリポジトリの処理（後で実装）
            return Err(AppError::Other(
                "Remote repository support is not yet implemented.".to_string(),
            ));
        } else {
            std::path::PathBuf::from(repo)
        }
    } else {
        std::env::current_dir()?
    };

    // Gitインスタンスを作成
    let git = Git::new(config.git_path.as_deref(), &repo_path)?;

    // 進捗表示
    println!("[1/5] Resolving Refs...");

    // ref を解決
    let source_commit = git.resolve_ref(&config.source)?;
    let target_commit = git.resolve_ref(&config.target)?;

    println!(
        "Source: {} -> {}",
        config.source,
        &source_commit[..7.min(source_commit.len())]
    );
    println!(
        "Target: {} -> {}",
        config.target,
        &target_commit[..7.min(target_commit.len())]
    );

    if config.three_way {
        // 三者間比較
        execute_three_way(config, &git, &source_commit, &target_commit)
    } else {
        // 二者間比較
        execute_two_way(config, &git, &source_commit, &target_commit)
    }
}

fn execute_two_way(
    config: &MergedConfig,
    git: &Git,
    source_commit: &str,
    target_commit: &str,
) -> Result<ExitCode> {
    println!("[2/5] Getting Diff File List...");

    // 差分を検出
    let detector = DiffDetector::new(git, config);
    let (files, stats) = detector.detect_two_way(source_commit, target_commit)?;

    println!("Found {} changed files.", files.len());

    // 差分がない場合
    if stats.total() == stats.unchanged {
        let writer = SummaryWriter::new(config);
        writer.write_two_way(
            &files,
            &stats,
            &config.source,
            source_commit,
            &config.target,
            target_commit,
            git.repo_path(),
            &[],
        )?;
        return Ok(ExitCode::NoDifference);
    }

    // ファイルをコピー
    println!("[3/5] Fetching Files...");
    println!("[4/5] Copying Files...");

    let copier = FileCopier::new(git, config);
    let (copied_count, copy_errors) = copier.copy_two_way(&files, source_commit, target_commit)?;

    if !config.dry_run {
        println!("Copied {} files.", copied_count);
    }

    // サマリーを出力
    println!("[5/5] Writing Summary...");

    let writer = SummaryWriter::new(config);
    writer.write_two_way(
        &files,
        &stats,
        &config.source,
        source_commit,
        &config.target,
        target_commit,
        git.repo_path(),
        &copy_errors,
    )?;

    // Excelレポートを出力
    if let Some(ref excel_path) = config.excel {
        let excel_writer = ExcelWriter::new(config);
        excel_writer.write_two_way(
            &files,
            &stats,
            &config.source,
            source_commit,
            &config.target,
            target_commit,
            git.repo_path(),
            excel_path,
        )?;
        println!("Excel report saved to: {}", excel_path.display());
    }

    // 統合パッチファイルを出力
    if let Some(ref patch_path) = config.patch_file {
        let patch_content = git.diff_patch_all(source_commit, target_commit)?;
        std::fs::write(patch_path, &patch_content)?;
        println!("Patch file saved to: {}", patch_path.display());
    }

    println!("Completed.");

    Ok(ExitCode::Success)
}

fn execute_three_way(
    config: &MergedConfig,
    git: &Git,
    ours_commit: &str,
    theirs_commit: &str,
) -> Result<ExitCode> {
    let base_ref = config.base.as_ref().unwrap();
    let base_commit = git.resolve_ref(base_ref)?;

    println!(
        "Base: {} -> {}",
        base_ref,
        &base_commit[..7.min(base_commit.len())]
    );

    println!("[2/7] Getting Diff File List...");

    // 差分を検出
    let detector = DiffDetector::new(git, config);
    let (files, stats) = detector.detect_three_way(&base_commit, ours_commit, theirs_commit)?;

    println!("Found {} files to compare.", files.len());

    // filter_statusに基づいてファイルをフィルタリング（表示用）
    let display_files: Vec<_> = if !config.filter_status.is_empty() {
        let allowed_statuses = expand_filter_status_three_way(&config.filter_status);
        files
            .iter()
            .filter(|f| allowed_statuses.contains(&f.status))
            .cloned()
            .collect()
    } else {
        files.clone()
    };

    // 差分がない場合
    if stats.total() == stats.unchanged {
        let writer = SummaryWriter::new(config);
        writer.write_three_way(
            &display_files,
            &stats,
            base_ref,
            &base_commit,
            &config.source,
            ours_commit,
            &config.target,
            theirs_commit,
            git.repo_path(),
            &[],
        )?;
        return Ok(ExitCode::NoDifference);
    }

    // ファイルをコピー
    println!("[3/7] Fetching Files...");
    println!("[4/7] Copying Files...");

    let copier = FileCopier::new(git, config);
    let (copied_count, copy_errors) =
        copier.copy_three_way(&files, &base_commit, ours_commit, theirs_commit)?;

    if !config.dry_run {
        println!("Copied {} files.", copied_count);
    }

    // サマリーを出力
    println!("[5/7] Writing Summary...");

    let writer = SummaryWriter::new(config);
    writer.write_three_way(
        &display_files,
        &stats,
        base_ref,
        &base_commit,
        &config.source,
        ours_commit,
        &config.target,
        theirs_commit,
        git.repo_path(),
        &copy_errors,
    )?;

    // Excelレポートを出力
    if let Some(ref excel_path) = config.excel {
        let excel_writer = ExcelWriter::new(config);
        excel_writer.write_three_way(
            &display_files,
            &stats,
            base_ref,
            &base_commit,
            &config.source,
            ours_commit,
            &config.target,
            theirs_commit,
            git.repo_path(),
            excel_path,
        )?;
        println!("Excel report saved to: {}", excel_path.display());
    }

    // 統合パッチファイルを出力（ours vs base）
    if let Some(ref patch_path) = config.patch_file {
        let patch_content = git.diff_patch_all(&base_commit, ours_commit)?;
        std::fs::write(patch_path, &patch_content)?;
        println!("Patch file saved to: {}", patch_path.display());
    }

    println!("Completed.");

    // コンフリクトがある場合
    if stats.conflicts() > 0 {
        Ok(ExitCode::Conflict)
    } else {
        Ok(ExitCode::Success)
    }
}
