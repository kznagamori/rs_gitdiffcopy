//! ファイルコピー処理

use crate::config::MergedConfig;
use crate::error::Result;
use crate::git::Git;
use crate::types::{expand_filter_status_three_way, DiffFile, FileStatus, MergeStyle, ThreeWayDiffFile, ThreeWayStatus};
use chrono::DateTime;
use filetime::FileTime;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// ファイルコピー処理
pub struct FileCopier<'a> {
    git: &'a Git,
    config: &'a MergedConfig,
}

impl<'a> FileCopier<'a> {
    pub fn new(git: &'a Git, config: &'a MergedConfig) -> Self {
        Self { git, config }
    }

    /// 二者間比較のファイルをコピー
    pub fn copy_two_way(
        &self,
        files: &[DiffFile],
        source_commit: &str,
        target_commit: &str,
    ) -> Result<(usize, Vec<String>)> {
        if self.config.dry_run {
            return Ok((0, Vec::new()));
        }

        // 出力ディレクトリを作成
        fs::create_dir_all(&self.config.output)?;

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] Copying Files: [{bar:40}] {pos}/{len} ({percent}%)")
                .unwrap()
                .progress_chars("=>-"),
        );

        let copied_count = Mutex::new(0usize);
        let errors = Mutex::new(Vec::new());

        let files_to_copy: Vec<_> = files
            .iter()
            .filter(|f| self.should_copy(f))
            .collect();

        files_to_copy.par_iter().for_each(|file| {
            let result = self.copy_file(file, source_commit, target_commit);
            pb.inc(1);

            match result {
                Ok(_) => {
                    *copied_count.lock().unwrap() += 1;
                }
                Err(e) => {
                    errors.lock().unwrap().push(format!(
                        "{}: {}",
                        file.path.display(),
                        e
                    ));
                }
            }
        });

        pb.finish_with_message(format!("Copied {} files.", copied_count.lock().unwrap()));

        Ok((*copied_count.lock().unwrap(), errors.into_inner().unwrap()))
    }

    /// ファイルをコピーすべきかどうか
    fn should_copy(&self, file: &DiffFile) -> bool {
        // エラーがあるファイルはスキップ
        if file.error.is_some() {
            return false;
        }

        // シンボリックリンクとサブモジュールはスキップ
        if file.is_symlink || file.is_submodule {
            return false;
        }

        // filter_statusが指定されている場合、該当するステータスのみ許可
        if !self.config.filter_status.is_empty() {
            let status_matches = self.config.filter_status.iter().any(|s| {
                match FileStatus::from_filter_str(s) {
                    Some(filter_status) => file.status == filter_status,
                    None => false,
                }
            });
            if !status_matches {
                return false;
            }
        }

        match file.status {
            FileStatus::Deleted => self.config.copy_deleted,
            FileStatus::Unchanged => false,
            _ => true,
        }
    }

    /// 三者間比較でファイルをコピーすべきかどうか
    fn should_copy_three_way(&self, file: &ThreeWayDiffFile) -> bool {
        // conflict_onlyモードの場合、コンフリクトファイルのみ
        if self.config.conflict_only && !file.status.is_conflict() {
            return false;
        }

        // filter_statusが指定されている場合、展開してフィルタリング
        if !self.config.filter_status.is_empty() {
            let allowed_statuses = expand_filter_status_three_way(&self.config.filter_status);
            if !allowed_statuses.contains(&file.status) {
                return false;
            }
        }

        // Unchangedはコピーしない（show_unchangedオプションとは別）
        if file.status == ThreeWayStatus::Unchanged {
            return false;
        }

        true
    }

    /// 単一ファイルをコピー
    fn copy_file(
        &self,
        file: &DiffFile,
        source_commit: &str,
        target_commit: &str,
    ) -> Result<()> {
        let output_path = self.config.output.join(&file.path);

        // 親ディレクトリを作成
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        match file.status {
            FileStatus::Deleted => {
                // 削除ファイルはsource側からコピー
                if self.config.copy_deleted {
                    let content = self.git.show(source_commit, &file.path)?;
                    let dest = Self::add_extension(&output_path, "deleted");
                    fs::write(&dest, &content)?;

                    if self.config.preserve_timestamps {
                        self.set_timestamp(&dest, source_commit, &file.path)?;
                    }
                }
            }
            _ if self.config.both_versions => {
                // 新旧両方をコピー
                self.copy_both_versions(file, source_commit, target_commit, &output_path)?;
            }
            _ => {
                // 通常コピー（target側）
                let content = self.git.show(target_commit, &file.path)?;
                fs::write(&output_path, &content)?;

                if self.config.preserve_timestamps {
                    self.set_timestamp(&output_path, target_commit, &file.path)?;
                }
            }
        }

        Ok(())
    }

    /// 新旧両方をコピー
    fn copy_both_versions(
        &self,
        file: &DiffFile,
        source_commit: &str,
        target_commit: &str,
        output_path: &Path,
    ) -> Result<()> {
        match file.status {
            FileStatus::Added => {
                // 新規ファイルはそのままコピー
                let content = self.git.show(target_commit, &file.path)?;
                fs::write(output_path, &content)?;

                if self.config.preserve_timestamps {
                    self.set_timestamp(output_path, target_commit, &file.path)?;
                }
            }
            FileStatus::Modified | FileStatus::TypeChanged => {
                // 旧バージョン
                let old_content = self.git.show(source_commit, &file.path)?;
                let old_path = Self::add_extension(output_path, "old");
                fs::write(&old_path, &old_content)?;

                if self.config.preserve_timestamps {
                    self.set_timestamp(&old_path, source_commit, &file.path)?;
                }

                // 新バージョン
                let new_content = self.git.show(target_commit, &file.path)?;
                let new_path = Self::add_extension(output_path, "new");
                fs::write(&new_path, &new_content)?;

                if self.config.preserve_timestamps {
                    self.set_timestamp(&new_path, target_commit, &file.path)?;
                }
            }
            FileStatus::Renamed | FileStatus::Copied => {
                // リネーム/コピーの場合、元ファイルのパスを使用
                let source_path = file.original_path.as_ref().unwrap_or(&file.path);

                // 旧バージョン
                let old_content = self.git.show(source_commit, source_path)?;
                let old_path = Self::add_extension(output_path, "old");
                fs::write(&old_path, &old_content)?;

                if self.config.preserve_timestamps {
                    self.set_timestamp(&old_path, source_commit, source_path)?;
                }

                // 新バージョン
                let new_content = self.git.show(target_commit, &file.path)?;
                let new_path = Self::add_extension(output_path, "new");
                fs::write(&new_path, &new_content)?;

                if self.config.preserve_timestamps {
                    self.set_timestamp(&new_path, target_commit, &file.path)?;
                }
            }
            FileStatus::Deleted => {
                // 削除ファイルは.deleted を優先（both_versionsとcopy_deletedの併用時）
                if self.config.copy_deleted {
                    let content = self.git.show(source_commit, &file.path)?;
                    let dest = Self::add_extension(output_path, "deleted");
                    fs::write(&dest, &content)?;

                    if self.config.preserve_timestamps {
                        self.set_timestamp(&dest, source_commit, &file.path)?;
                    }
                }
            }
            FileStatus::Unchanged => {}
        }

        Ok(())
    }

    /// 三者間比較のファイルをコピー
    pub fn copy_three_way(
        &self,
        files: &[ThreeWayDiffFile],
        base_commit: &str,
        ours_commit: &str,
        theirs_commit: &str,
    ) -> Result<(usize, Vec<String>)> {
        if self.config.dry_run {
            return Ok((0, Vec::new()));
        }

        // 出力ディレクトリを作成
        let ours_only_dir = self.config.output.join("ours_only");
        let theirs_only_dir = self.config.output.join("theirs_only");
        let both_same_dir = self.config.output.join("both_same");
        let conflicts_dir = self.config.output.join("conflicts");

        fs::create_dir_all(&ours_only_dir)?;
        fs::create_dir_all(&theirs_only_dir)?;
        fs::create_dir_all(&both_same_dir)?;
        fs::create_dir_all(&conflicts_dir)?;

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] Copying Files: [{bar:40}] {pos}/{len} ({percent}%)")
                .unwrap()
                .progress_chars("=>-"),
        );

        let copied_count = Mutex::new(0usize);
        let errors = Mutex::new(Vec::new());

        // filter_statusに基づいてフィルタリング
        let files_to_copy: Vec<_> = files
            .iter()
            .filter(|f| self.should_copy_three_way(f))
            .collect();

        files_to_copy.par_iter().for_each(|file| {
            let result = self.copy_three_way_file(
                file,
                base_commit,
                ours_commit,
                theirs_commit,
                &ours_only_dir,
                &theirs_only_dir,
                &both_same_dir,
                &conflicts_dir,
            );
            pb.inc(1);

            match result {
                Ok(copied) if copied => {
                    *copied_count.lock().unwrap() += 1;
                }
                Err(e) => {
                    errors.lock().unwrap().push(format!(
                        "{}: {}",
                        file.path.display(),
                        e
                    ));
                }
                _ => {}
            }
        });

        pb.finish_with_message(format!("Copied {} files.", copied_count.lock().unwrap()));

        Ok((*copied_count.lock().unwrap(), errors.into_inner().unwrap()))
    }

    /// 三者間比較の単一ファイルをコピー
    fn copy_three_way_file(
        &self,
        file: &ThreeWayDiffFile,
        base_commit: &str,
        ours_commit: &str,
        theirs_commit: &str,
        ours_only_dir: &Path,
        theirs_only_dir: &Path,
        both_same_dir: &Path,
        conflicts_dir: &Path,
    ) -> Result<bool> {
        // conflict_onlyの場合、コンフリクト以外はスキップ
        if self.config.conflict_only && !file.status.is_conflict() {
            return Ok(false);
        }

        match file.status {
            ThreeWayStatus::Unchanged => Ok(false),
            ThreeWayStatus::OursOnly => {
                let output_path = ours_only_dir.join(&file.path);
                self.ensure_parent(&output_path)?;
                let content = self.git.show(ours_commit, &file.path)?;
                fs::write(&output_path, &content)?;
                Ok(true)
            }
            ThreeWayStatus::TheirsOnly => {
                let output_path = theirs_only_dir.join(&file.path);
                self.ensure_parent(&output_path)?;
                let content = self.git.show(theirs_commit, &file.path)?;
                fs::write(&output_path, &content)?;
                Ok(true)
            }
            ThreeWayStatus::BothSame => {
                let output_path = both_same_dir.join(&file.path);
                self.ensure_parent(&output_path)?;
                let content = self.git.show(ours_commit, &file.path)?;
                fs::write(&output_path, &content)?;
                Ok(true)
            }
            status if status.is_conflict() => {
                self.copy_conflict_file(file, base_commit, ours_commit, theirs_commit, conflicts_dir)
            }
            ThreeWayStatus::AddedOurs => {
                let output_path = ours_only_dir.join(&file.path);
                self.ensure_parent(&output_path)?;
                let content = self.git.show(ours_commit, &file.path)?;
                fs::write(&output_path, &content)?;
                Ok(true)
            }
            ThreeWayStatus::AddedTheirs => {
                let output_path = theirs_only_dir.join(&file.path);
                self.ensure_parent(&output_path)?;
                let content = self.git.show(theirs_commit, &file.path)?;
                fs::write(&output_path, &content)?;
                Ok(true)
            }
            ThreeWayStatus::AddedBothSame => {
                let output_path = both_same_dir.join(&file.path);
                self.ensure_parent(&output_path)?;
                let content = self.git.show(ours_commit, &file.path)?;
                fs::write(&output_path, &content)?;
                Ok(true)
            }
            ThreeWayStatus::DeletedOurs | ThreeWayStatus::DeletedTheirs | ThreeWayStatus::DeletedBoth => {
                // 削除されたファイルはコピーしない
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    /// コンフリクトファイルをコピー
    fn copy_conflict_file(
        &self,
        file: &ThreeWayDiffFile,
        base_commit: &str,
        ours_commit: &str,
        theirs_commit: &str,
        conflicts_dir: &Path,
    ) -> Result<bool> {
        let base_path = conflicts_dir.join(&file.path);
        self.ensure_parent(&base_path)?;

        match self.config.merge_style {
            MergeStyle::All => {
                // base, ours, theirs 全てを出力
                if file.base_blob.is_some() {
                    let content = self.git.show(base_commit, &file.path)?;
                    let dest = Self::add_extension(&base_path, "base");
                    fs::write(&dest, &content)?;
                }

                if file.ours_blob.is_some() {
                    let content = self.git.show(ours_commit, &file.path)?;
                    let dest = Self::add_extension(&base_path, "ours");
                    fs::write(&dest, &content)?;
                }

                if file.theirs_blob.is_some() {
                    let content = self.git.show(theirs_commit, &file.path)?;
                    let dest = Self::add_extension(&base_path, "theirs");
                    fs::write(&dest, &content)?;
                }
            }
            MergeStyle::Ours => {
                if file.ours_blob.is_some() {
                    let content = self.git.show(ours_commit, &file.path)?;
                    let dest = Self::add_extension(&base_path, "ours");
                    fs::write(&dest, &content)?;
                }
            }
            MergeStyle::Theirs => {
                if file.theirs_blob.is_some() {
                    let content = self.git.show(theirs_commit, &file.path)?;
                    let dest = Self::add_extension(&base_path, "theirs");
                    fs::write(&dest, &content)?;
                }
            }
        }

        Ok(true)
    }

    /// 親ディレクトリを作成
    fn ensure_parent(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    /// タイムスタンプを設定
    fn set_timestamp(&self, path: &Path, commit: &str, file_path: &Path) -> Result<()> {
        let date_str = self.git.get_commit_date(commit, Some(file_path))?;

        // 日時文字列をパース
        if let Ok(dt) = DateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S %z") {
            let timestamp = dt.timestamp();
            let ft = FileTime::from_unix_time(timestamp, 0);
            filetime::set_file_mtime(path, ft)?;
        }

        Ok(())
    }

    /// ファイル名に拡張子を追加
    pub fn add_extension(path: &Path, ext: &str) -> PathBuf {
        let mut new_path = path.as_os_str().to_os_string();
        new_path.push(".");
        new_path.push(ext);
        PathBuf::from(new_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // add_extension のテスト
    #[test]
    fn test_add_extension_simple() {
        let path = Path::new("file.txt");
        let result = FileCopier::add_extension(path, "old");
        assert_eq!(result, PathBuf::from("file.txt.old"));
    }

    #[test]
    fn test_add_extension_with_path() {
        let path = Path::new("path/to/file.txt");
        let result = FileCopier::add_extension(path, "new");
        assert_eq!(result, PathBuf::from("path/to/file.txt.new"));
    }

    #[test]
    fn test_add_extension_deleted() {
        let path = Path::new("config.toml");
        let result = FileCopier::add_extension(path, "deleted");
        assert_eq!(result, PathBuf::from("config.toml.deleted"));
    }

    #[test]
    fn test_add_extension_no_original_extension() {
        let path = Path::new("Makefile");
        let result = FileCopier::add_extension(path, "old");
        assert_eq!(result, PathBuf::from("Makefile.old"));
    }

    #[test]
    fn test_add_extension_dotfile() {
        let path = Path::new(".gitignore");
        let result = FileCopier::add_extension(path, "old");
        assert_eq!(result, PathBuf::from(".gitignore.old"));
    }

    #[test]
    fn test_add_extension_double_extension() {
        let path = Path::new("archive.tar.gz");
        let result = FileCopier::add_extension(path, "old");
        assert_eq!(result, PathBuf::from("archive.tar.gz.old"));
    }

    #[test]
    fn test_add_extension_japanese_filename() {
        let path = Path::new("日本語/ファイル.txt");
        let result = FileCopier::add_extension(path, "old");
        assert_eq!(result, PathBuf::from("日本語/ファイル.txt.old"));
    }

    #[test]
    fn test_add_extension_base() {
        let path = Path::new("conflict_file.rs");
        let result = FileCopier::add_extension(path, "base");
        assert_eq!(result, PathBuf::from("conflict_file.rs.base"));
    }

    #[test]
    fn test_add_extension_ours() {
        let path = Path::new("conflict_file.rs");
        let result = FileCopier::add_extension(path, "ours");
        assert_eq!(result, PathBuf::from("conflict_file.rs.ours"));
    }

    #[test]
    fn test_add_extension_theirs() {
        let path = Path::new("conflict_file.rs");
        let result = FileCopier::add_extension(path, "theirs");
        assert_eq!(result, PathBuf::from("conflict_file.rs.theirs"));
    }

    // ThreeWayStatus::is_conflict のテスト
    #[test]
    fn test_three_way_status_is_conflict_true() {
        assert!(ThreeWayStatus::Conflict.is_conflict());
        assert!(ThreeWayStatus::AddedBothDiff.is_conflict());
        assert!(ThreeWayStatus::ModifyDelete.is_conflict());
        assert!(ThreeWayStatus::DeleteModify.is_conflict());
    }

    #[test]
    fn test_three_way_status_is_conflict_false() {
        assert!(!ThreeWayStatus::Unchanged.is_conflict());
        assert!(!ThreeWayStatus::OursOnly.is_conflict());
        assert!(!ThreeWayStatus::TheirsOnly.is_conflict());
        assert!(!ThreeWayStatus::BothSame.is_conflict());
        assert!(!ThreeWayStatus::AddedOurs.is_conflict());
        assert!(!ThreeWayStatus::AddedTheirs.is_conflict());
        assert!(!ThreeWayStatus::AddedBothSame.is_conflict());
        assert!(!ThreeWayStatus::DeletedOurs.is_conflict());
        assert!(!ThreeWayStatus::DeletedTheirs.is_conflict());
        assert!(!ThreeWayStatus::DeletedBoth.is_conflict());
    }
}
