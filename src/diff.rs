//! 差分検出とファイル状態

use crate::config::MergedConfig;
use crate::error::Result;
use crate::git::Git;
use crate::types::{DiffFile, FileStatus, Statistics, ThreeWayDiffFile, ThreeWayStatistics, ThreeWayStatus};
use globset::{Glob, GlobSetBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::PathBuf;

/// 差分検出器
pub struct DiffDetector<'a> {
    git: &'a Git,
    config: &'a MergedConfig,
}

impl<'a> DiffDetector<'a> {
    pub fn new(git: &'a Git, config: &'a MergedConfig) -> Self {
        Self { git, config }
    }

    /// 二者間差分を検出
    pub fn detect_two_way(
        &self,
        source_commit: &str,
        target_commit: &str,
    ) -> Result<(Vec<DiffFile>, Statistics)> {
        // 差分ファイルリストを取得
        let mut files = self.git.diff_name_status(source_commit, target_commit)?;

        // 除外パターンでフィルタリング
        files = self.filter_excluded(files)?;

        // 変更がないファイルも必要な場合は追加
        if self.config.show_unchanged {
            let unchanged = self.detect_unchanged(source_commit, target_commit, &files)?;
            files.extend(unchanged);
        }

        // ファイル情報を取得（並列処理）
        files = self.fetch_file_info(files, source_commit, target_commit)?;

        // 統計情報を計算
        let stats = self.calculate_statistics(&files);

        Ok((files, stats))
    }

    /// 三者間差分を検出
    pub fn detect_three_way(
        &self,
        base_commit: &str,
        ours_commit: &str,
        theirs_commit: &str,
    ) -> Result<(Vec<ThreeWayDiffFile>, ThreeWayStatistics)> {
        // 各コミットのファイル一覧を取得
        let base_files = self.git.ls_tree_recursive(base_commit)?;
        let ours_files = self.git.ls_tree_recursive(ours_commit)?;
        let theirs_files = self.git.ls_tree_recursive(theirs_commit)?;

        // すべてのファイルパスを収集
        let all_paths: HashSet<PathBuf> = base_files
            .keys()
            .chain(ours_files.keys())
            .chain(theirs_files.keys())
            .cloned()
            .collect();

        // 各ファイルの状態を判定
        let mut files: Vec<ThreeWayDiffFile> = Vec::new();

        for path in all_paths {
            // 除外パターンをチェック
            if self.is_excluded(&path)? {
                continue;
            }

            let base_info = base_files.get(&path);
            let ours_info = ours_files.get(&path);
            let theirs_info = theirs_files.get(&path);

            let status = self.determine_three_way_status(base_info, ours_info, theirs_info);

            // 変更なしでshow_unchangedが無効なら除外
            if status == ThreeWayStatus::Unchanged && !self.config.show_unchanged {
                continue;
            }

            let mut file = ThreeWayDiffFile::new(path, status);
            file.base_blob = base_info.map(|(_, blob)| blob.clone());
            file.ours_blob = ours_info.map(|(_, blob)| blob.clone());
            file.theirs_blob = theirs_info.map(|(_, blob)| blob.clone());

            files.push(file);
        }

        // 統計情報を計算
        let stats = self.calculate_three_way_statistics(&files);

        Ok((files, stats))
    }

    /// 除外パターンでフィルタリング
    fn filter_excluded(&self, files: Vec<DiffFile>) -> Result<Vec<DiffFile>> {
        if self.config.exclude.is_empty() {
            return Ok(files);
        }

        let mut builder = GlobSetBuilder::new();
        for pattern in &self.config.exclude {
            let glob = Glob::new(pattern).map_err(|_| {
                crate::error::AppError::InvalidGlobPattern(pattern.clone())
            })?;
            builder.add(glob);
        }
        let globset = builder
            .build()
            .map_err(|e| crate::error::AppError::InvalidGlobPattern(e.to_string()))?;

        Ok(files
            .into_iter()
            .filter(|f| !globset.is_match(&f.path))
            .collect())
    }

    /// パスが除外対象かどうか
    fn is_excluded(&self, path: &PathBuf) -> Result<bool> {
        if self.config.exclude.is_empty() {
            return Ok(false);
        }

        let mut builder = GlobSetBuilder::new();
        for pattern in &self.config.exclude {
            let glob = Glob::new(pattern).map_err(|_| {
                crate::error::AppError::InvalidGlobPattern(pattern.clone())
            })?;
            builder.add(glob);
        }
        let globset = builder
            .build()
            .map_err(|e| crate::error::AppError::InvalidGlobPattern(e.to_string()))?;

        Ok(globset.is_match(path))
    }

    /// 変更がないファイルを検出
    fn detect_unchanged(
        &self,
        source_commit: &str,
        target_commit: &str,
        changed_files: &[DiffFile],
    ) -> Result<Vec<DiffFile>> {
        // source側とtarget側のファイル一覧を取得
        let source_files = self.git.ls_tree_recursive(source_commit)?;
        let target_files = self.git.ls_tree_recursive(target_commit)?;

        // 変更済みファイルのパスを収集
        let changed_paths: HashSet<PathBuf> = changed_files.iter().map(|f| f.path.clone()).collect();

        // 両方に存在し、blob IDが同じファイルを抽出
        let unchanged: Vec<DiffFile> = source_files
            .iter()
            .filter_map(|(path, (mode, blob))| {
                if changed_paths.contains(path) {
                    return None;
                }

                if let Some((target_mode, target_blob)) = target_files.get(path) {
                    if blob == target_blob {
                        // シンボリックリンクやサブモジュールは除外
                        if mode.starts_with("120") || mode.starts_with("160") {
                            return None;
                        }

                        let mut file = DiffFile::new(path.clone(), FileStatus::Unchanged);
                        file.source_mode = Some(mode.clone());
                        file.target_mode = Some(target_mode.clone());
                        file.source_blob = Some(blob.clone());
                        file.target_blob = Some(target_blob.clone());
                        return Some(file);
                    }
                }

                None
            })
            .collect();

        Ok(unchanged)
    }

    /// ファイル情報を取得（並列処理）
    fn fetch_file_info(
        &self,
        files: Vec<DiffFile>,
        source_commit: &str,
        target_commit: &str,
    ) -> Result<Vec<DiffFile>> {
        if files.is_empty() {
            return Ok(files);
        }

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] Fetching Files: [{bar:40}] {pos}/{len} ({percent}%)")
                .unwrap()
                .progress_chars("=>-"),
        );

        let result: Vec<DiffFile> = files
            .into_par_iter()
            .map(|mut file| {
                // ファイルモードを取得
                if let Ok(Some(mode)) = self.git.ls_tree(source_commit, &file.path) {
                    file.source_mode = Some(mode.clone());
                    file.is_symlink = mode.starts_with("120");
                    file.is_submodule = mode.starts_with("160");
                }
                if let Ok(Some(mode)) = self.git.ls_tree(target_commit, &file.path) {
                    file.target_mode = Some(mode.clone());
                    if mode.starts_with("120") {
                        file.is_symlink = true;
                    }
                    if mode.starts_with("160") {
                        file.is_submodule = true;
                    }
                }

                // リネーム/コピーの場合は元のパスからモードを取得
                if let Some(ref original_path) = file.original_path {
                    if let Ok(Some(mode)) = self.git.ls_tree(source_commit, original_path) {
                        file.source_mode = Some(mode);
                    }
                }

                pb.inc(1);
                file
            })
            .collect();

        pb.finish_with_message(format!("Fetched {} files.", result.len()));

        Ok(result)
    }

    /// 統計情報を計算
    fn calculate_statistics(&self, files: &[DiffFile]) -> Statistics {
        let mut stats = Statistics::default();

        for file in files {
            match file.status {
                FileStatus::Added => stats.added += 1,
                FileStatus::Modified => stats.modified += 1,
                FileStatus::Deleted => stats.deleted += 1,
                FileStatus::Renamed => stats.renamed += 1,
                FileStatus::Copied => stats.copied += 1,
                FileStatus::TypeChanged => stats.type_changed += 1,
                FileStatus::Unchanged => stats.unchanged += 1,
            }

            if file.is_symlink {
                stats.symlinks += 1;
            }
            if file.is_submodule {
                stats.submodules += 1;
            }
            if file.is_permission_only_change() {
                stats.permission_changes += 1;
            }
            if file.error.is_some() {
                stats.errors += 1;
            }
        }

        stats
    }

    /// 三者間比較の状態を判定
    fn determine_three_way_status(
        &self,
        base: Option<&(String, String)>,
        ours: Option<&(String, String)>,
        theirs: Option<&(String, String)>,
    ) -> ThreeWayStatus {
        match (base, ours, theirs) {
            // baseに存在する場合
            (Some((_, base_blob)), Some((_, ours_blob)), Some((_, theirs_blob))) => {
                if base_blob == ours_blob && base_blob == theirs_blob {
                    ThreeWayStatus::Unchanged
                } else if base_blob != ours_blob && base_blob == theirs_blob {
                    ThreeWayStatus::OursOnly
                } else if base_blob == ours_blob && base_blob != theirs_blob {
                    ThreeWayStatus::TheirsOnly
                } else if ours_blob == theirs_blob {
                    ThreeWayStatus::BothSame
                } else {
                    ThreeWayStatus::Conflict
                }
            }
            // baseに存在、oursで削除
            (Some(_), None, Some((_, theirs_blob))) => {
                if let Some((_, base_blob)) = base {
                    if base_blob == theirs_blob {
                        ThreeWayStatus::DeletedOurs
                    } else {
                        ThreeWayStatus::DeleteModify
                    }
                } else {
                    ThreeWayStatus::DeletedOurs
                }
            }
            // baseに存在、theirsで削除
            (Some(_), Some((_, ours_blob)), None) => {
                if let Some((_, base_blob)) = base {
                    if base_blob == ours_blob {
                        ThreeWayStatus::DeletedTheirs
                    } else {
                        ThreeWayStatus::ModifyDelete
                    }
                } else {
                    ThreeWayStatus::DeletedTheirs
                }
            }
            // baseに存在、両方で削除
            (Some(_), None, None) => ThreeWayStatus::DeletedBoth,
            // baseに存在しない、oursで追加
            (None, Some(_), None) => ThreeWayStatus::AddedOurs,
            // baseに存在しない、theirsで追加
            (None, None, Some(_)) => ThreeWayStatus::AddedTheirs,
            // baseに存在しない、両方で追加
            (None, Some((_, ours_blob)), Some((_, theirs_blob))) => {
                if ours_blob == theirs_blob {
                    ThreeWayStatus::AddedBothSame
                } else {
                    ThreeWayStatus::AddedBothDiff
                }
            }
            // どこにも存在しない（通常発生しない）
            (None, None, None) => ThreeWayStatus::Unchanged,
        }
    }

    /// 三者間比較の統計情報を計算
    fn calculate_three_way_statistics(&self, files: &[ThreeWayDiffFile]) -> ThreeWayStatistics {
        let mut stats = ThreeWayStatistics::default();

        for file in files {
            match file.status {
                ThreeWayStatus::Unchanged => stats.unchanged += 1,
                ThreeWayStatus::OursOnly => stats.ours_only += 1,
                ThreeWayStatus::TheirsOnly => stats.theirs_only += 1,
                ThreeWayStatus::BothSame => stats.both_same += 1,
                ThreeWayStatus::Conflict => stats.conflict += 1,
                ThreeWayStatus::AddedOurs => stats.added_ours += 1,
                ThreeWayStatus::AddedTheirs => stats.added_theirs += 1,
                ThreeWayStatus::AddedBothSame => stats.added_both_same += 1,
                ThreeWayStatus::AddedBothDiff => stats.added_both_diff += 1,
                ThreeWayStatus::DeletedOurs => stats.deleted_ours += 1,
                ThreeWayStatus::DeletedTheirs => stats.deleted_theirs += 1,
                ThreeWayStatus::DeletedBoth => stats.deleted_both += 1,
                ThreeWayStatus::ModifyDelete => stats.modify_delete += 1,
                ThreeWayStatus::DeleteModify => stats.delete_modify += 1,
            }
        }

        stats
    }

    /// 三者間比較の状態を判定（テスト用に公開）
    pub fn determine_three_way_status_static(
        base: Option<&(String, String)>,
        ours: Option<&(String, String)>,
        theirs: Option<&(String, String)>,
    ) -> ThreeWayStatus {
        match (base, ours, theirs) {
            // baseに存在する場合
            (Some((_, base_blob)), Some((_, ours_blob)), Some((_, theirs_blob))) => {
                if base_blob == ours_blob && base_blob == theirs_blob {
                    ThreeWayStatus::Unchanged
                } else if base_blob != ours_blob && base_blob == theirs_blob {
                    ThreeWayStatus::OursOnly
                } else if base_blob == ours_blob && base_blob != theirs_blob {
                    ThreeWayStatus::TheirsOnly
                } else if ours_blob == theirs_blob {
                    ThreeWayStatus::BothSame
                } else {
                    ThreeWayStatus::Conflict
                }
            }
            // baseに存在、oursで削除
            (Some(_), None, Some((_, theirs_blob))) => {
                if let Some((_, base_blob)) = base {
                    if base_blob == theirs_blob {
                        ThreeWayStatus::DeletedOurs
                    } else {
                        ThreeWayStatus::DeleteModify
                    }
                } else {
                    ThreeWayStatus::DeletedOurs
                }
            }
            // baseに存在、theirsで削除
            (Some(_), Some((_, ours_blob)), None) => {
                if let Some((_, base_blob)) = base {
                    if base_blob == ours_blob {
                        ThreeWayStatus::DeletedTheirs
                    } else {
                        ThreeWayStatus::ModifyDelete
                    }
                } else {
                    ThreeWayStatus::DeletedTheirs
                }
            }
            // baseに存在、両方で削除
            (Some(_), None, None) => ThreeWayStatus::DeletedBoth,
            // baseに存在しない、oursで追加
            (None, Some(_), None) => ThreeWayStatus::AddedOurs,
            // baseに存在しない、theirsで追加
            (None, None, Some(_)) => ThreeWayStatus::AddedTheirs,
            // baseに存在しない、両方で追加
            (None, Some((_, ours_blob)), Some((_, theirs_blob))) => {
                if ours_blob == theirs_blob {
                    ThreeWayStatus::AddedBothSame
                } else {
                    ThreeWayStatus::AddedBothDiff
                }
            }
            // どこにも存在しない（通常発生しない）
            (None, None, None) => ThreeWayStatus::Unchanged,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // determine_three_way_status_static のテスト

    #[test]
    fn test_three_way_unchanged() {
        let base = ("100644".to_string(), "abc123".to_string());
        let ours = ("100644".to_string(), "abc123".to_string());
        let theirs = ("100644".to_string(), "abc123".to_string());
        let status = DiffDetector::determine_three_way_status_static(Some(&base), Some(&ours), Some(&theirs));
        assert_eq!(status, ThreeWayStatus::Unchanged);
    }

    #[test]
    fn test_three_way_ours_only() {
        let base = ("100644".to_string(), "abc123".to_string());
        let ours = ("100644".to_string(), "def456".to_string()); // oursで変更
        let theirs = ("100644".to_string(), "abc123".to_string());
        let status = DiffDetector::determine_three_way_status_static(Some(&base), Some(&ours), Some(&theirs));
        assert_eq!(status, ThreeWayStatus::OursOnly);
    }

    #[test]
    fn test_three_way_theirs_only() {
        let base = ("100644".to_string(), "abc123".to_string());
        let ours = ("100644".to_string(), "abc123".to_string());
        let theirs = ("100644".to_string(), "def456".to_string()); // theirsで変更
        let status = DiffDetector::determine_three_way_status_static(Some(&base), Some(&ours), Some(&theirs));
        assert_eq!(status, ThreeWayStatus::TheirsOnly);
    }

    #[test]
    fn test_three_way_both_same() {
        let base = ("100644".to_string(), "abc123".to_string());
        let ours = ("100644".to_string(), "def456".to_string()); // 両方が同じ変更
        let theirs = ("100644".to_string(), "def456".to_string());
        let status = DiffDetector::determine_three_way_status_static(Some(&base), Some(&ours), Some(&theirs));
        assert_eq!(status, ThreeWayStatus::BothSame);
    }

    #[test]
    fn test_three_way_conflict() {
        let base = ("100644".to_string(), "abc123".to_string());
        let ours = ("100644".to_string(), "def456".to_string()); // 両方が異なる変更
        let theirs = ("100644".to_string(), "ghi789".to_string());
        let status = DiffDetector::determine_three_way_status_static(Some(&base), Some(&ours), Some(&theirs));
        assert_eq!(status, ThreeWayStatus::Conflict);
    }

    #[test]
    fn test_three_way_deleted_ours() {
        let base = ("100644".to_string(), "abc123".to_string());
        let theirs = ("100644".to_string(), "abc123".to_string()); // theirsは変更なし
        // oursは削除（None）
        let status = DiffDetector::determine_three_way_status_static(Some(&base), None, Some(&theirs));
        assert_eq!(status, ThreeWayStatus::DeletedOurs);
    }

    #[test]
    fn test_three_way_deleted_theirs() {
        let base = ("100644".to_string(), "abc123".to_string());
        let ours = ("100644".to_string(), "abc123".to_string()); // oursは変更なし
        // theirsは削除（None）
        let status = DiffDetector::determine_three_way_status_static(Some(&base), Some(&ours), None);
        assert_eq!(status, ThreeWayStatus::DeletedTheirs);
    }

    #[test]
    fn test_three_way_deleted_both() {
        let base = ("100644".to_string(), "abc123".to_string());
        // 両方で削除
        let status = DiffDetector::determine_three_way_status_static(Some(&base), None, None);
        assert_eq!(status, ThreeWayStatus::DeletedBoth);
    }

    #[test]
    fn test_three_way_delete_modify_conflict() {
        let base = ("100644".to_string(), "abc123".to_string());
        let theirs = ("100644".to_string(), "def456".to_string()); // theirsで変更
        // oursは削除（コンフリクト）
        let status = DiffDetector::determine_three_way_status_static(Some(&base), None, Some(&theirs));
        assert_eq!(status, ThreeWayStatus::DeleteModify);
    }

    #[test]
    fn test_three_way_modify_delete_conflict() {
        let base = ("100644".to_string(), "abc123".to_string());
        let ours = ("100644".to_string(), "def456".to_string()); // oursで変更
        // theirsは削除（コンフリクト）
        let status = DiffDetector::determine_three_way_status_static(Some(&base), Some(&ours), None);
        assert_eq!(status, ThreeWayStatus::ModifyDelete);
    }

    #[test]
    fn test_three_way_added_ours() {
        let ours = ("100644".to_string(), "abc123".to_string());
        // baseに存在せず、oursでのみ追加
        let status = DiffDetector::determine_three_way_status_static(None, Some(&ours), None);
        assert_eq!(status, ThreeWayStatus::AddedOurs);
    }

    #[test]
    fn test_three_way_added_theirs() {
        let theirs = ("100644".to_string(), "abc123".to_string());
        // baseに存在せず、theirsでのみ追加
        let status = DiffDetector::determine_three_way_status_static(None, None, Some(&theirs));
        assert_eq!(status, ThreeWayStatus::AddedTheirs);
    }

    #[test]
    fn test_three_way_added_both_same() {
        let ours = ("100644".to_string(), "abc123".to_string());
        let theirs = ("100644".to_string(), "abc123".to_string()); // 同じ内容で追加
        let status = DiffDetector::determine_three_way_status_static(None, Some(&ours), Some(&theirs));
        assert_eq!(status, ThreeWayStatus::AddedBothSame);
    }

    #[test]
    fn test_three_way_added_both_diff() {
        let ours = ("100644".to_string(), "abc123".to_string());
        let theirs = ("100644".to_string(), "def456".to_string()); // 異なる内容で追加（コンフリクト）
        let status = DiffDetector::determine_three_way_status_static(None, Some(&ours), Some(&theirs));
        assert_eq!(status, ThreeWayStatus::AddedBothDiff);
    }

    #[test]
    fn test_three_way_none_everywhere() {
        // どこにも存在しない（通常発生しない）
        let status = DiffDetector::determine_three_way_status_static(None, None, None);
        assert_eq!(status, ThreeWayStatus::Unchanged);
    }

    // Statistics計算のテスト
    #[test]
    fn test_calculate_statistics_empty() {
        let files: Vec<DiffFile> = vec![];
        let stats = Statistics::default();
        assert_eq!(stats.total(), 0);
    }

    #[test]
    fn test_diff_file_is_permission_only_change() {
        let mut file = DiffFile::new(PathBuf::from("test.sh"), FileStatus::Modified);
        file.source_mode = Some("100644".to_string());
        file.target_mode = Some("100755".to_string());
        file.source_blob = Some("abc123".to_string());
        file.target_blob = Some("abc123".to_string()); // 同じ内容
        assert!(file.is_permission_only_change());
    }

    #[test]
    fn test_diff_file_is_not_permission_only_change() {
        let mut file = DiffFile::new(PathBuf::from("test.rs"), FileStatus::Modified);
        file.source_mode = Some("100644".to_string());
        file.target_mode = Some("100755".to_string());
        file.source_blob = Some("abc123".to_string());
        file.target_blob = Some("def456".to_string()); // 異なる内容
        assert!(!file.is_permission_only_change());
    }
}
