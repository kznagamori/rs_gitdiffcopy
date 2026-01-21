//! 共通の型定義

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// ファイルステータス（二者間比較）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    TypeChanged,
    Unchanged,
}

impl FileStatus {
    /// git diff --name-status の出力からステータスを解析
    pub fn from_git_status(status: &str) -> Option<Self> {
        match status.chars().next()? {
            'A' => Some(Self::Added),
            'M' => Some(Self::Modified),
            'D' => Some(Self::Deleted),
            'R' => Some(Self::Renamed),
            'C' => Some(Self::Copied),
            'T' => Some(Self::TypeChanged),
            _ => None,
        }
    }

    /// ステータスのタグ文字列を取得
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Added => "[added]",
            Self::Modified => "[modified]",
            Self::Deleted => "[deleted]",
            Self::Renamed => "[renamed]",
            Self::Copied => "[copied]",
            Self::TypeChanged => "[type-changed]",
            Self::Unchanged => "[unchanged]",
        }
    }

    /// フィルター文字列からステータスを解析
    pub fn from_filter_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "added" | "add" | "a" => Some(Self::Added),
            "modified" | "modify" | "m" => Some(Self::Modified),
            "deleted" | "delete" | "d" => Some(Self::Deleted),
            "renamed" | "rename" | "r" => Some(Self::Renamed),
            "copied" | "copy" | "c" => Some(Self::Copied),
            "type-changed" | "typechanged" | "type_changed" | "t" => Some(Self::TypeChanged),
            "unchanged" | "same" | "u" => Some(Self::Unchanged),
            _ => None,
        }
    }
}

/// 三者間比較のファイルステータス
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThreeWayStatus {
    /// 3つとも同一（変更なし）
    Unchanged,
    /// oursのみ変更
    OursOnly,
    /// theirsのみ変更
    TheirsOnly,
    /// 両方が同じ変更
    BothSame,
    /// 両方が異なる変更（コンフリクト）
    Conflict,
    /// oursでのみ追加
    AddedOurs,
    /// theirsでのみ追加
    AddedTheirs,
    /// 両方で追加（同一内容）
    AddedBothSame,
    /// 両方で追加（異なる内容）- コンフリクト
    AddedBothDiff,
    /// oursで削除
    DeletedOurs,
    /// theirsで削除
    DeletedTheirs,
    /// 両方で削除
    DeletedBoth,
    /// oursで変更、theirsで削除 - コンフリクト
    ModifyDelete,
    /// oursで削除、theirsで変更 - コンフリクト
    DeleteModify,
}

impl ThreeWayStatus {
    /// コンフリクトかどうか
    pub fn is_conflict(&self) -> bool {
        matches!(
            self,
            Self::Conflict | Self::AddedBothDiff | Self::ModifyDelete | Self::DeleteModify
        )
    }

    /// ステータスのタグ文字列を取得
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::OursOnly => "ours-only",
            Self::TheirsOnly => "theirs-only",
            Self::BothSame => "both-same",
            Self::Conflict => "CONFLICT",
            Self::AddedOurs => "added-ours",
            Self::AddedTheirs => "added-theirs",
            Self::AddedBothSame => "added-both-same",
            Self::AddedBothDiff => "CONFLICT",
            Self::DeletedOurs => "deleted-ours",
            Self::DeletedTheirs => "deleted-theirs",
            Self::DeletedBoth => "deleted-both",
            Self::ModifyDelete => "CONFLICT",
            Self::DeleteModify => "CONFLICT",
        }
    }

    /// インジケータ文字列を取得（例: [○M=]）
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Unchanged => "[○==]",
            Self::OursOnly => "[○M=]",
            Self::TheirsOnly => "[○=M]",
            Self::BothSame => "[○MM]",
            Self::Conflict => "[○MM]",
            Self::AddedOurs => "[-A-]",
            Self::AddedTheirs => "[-−A]",
            Self::AddedBothSame => "[-AA]",
            Self::AddedBothDiff => "[-AA]",
            Self::DeletedOurs => "[○D=]",
            Self::DeletedTheirs => "[○=D]",
            Self::DeletedBoth => "[○DD]",
            Self::ModifyDelete => "[○MD]",
            Self::DeleteModify => "[○DM]",
        }
    }
}

/// 差分ファイル情報
#[derive(Debug, Clone)]
pub struct DiffFile {
    /// ファイルパス（target側のパス）
    pub path: PathBuf,
    /// 元のパス（リネーム/コピー時のソースパス）
    pub original_path: Option<PathBuf>,
    /// ファイルステータス
    pub status: FileStatus,
    /// 類似度（リネーム/コピー時のパーセンテージ）
    pub similarity: Option<u8>,
    /// ファイルモード（source側）
    pub source_mode: Option<String>,
    /// ファイルモード（target側）
    pub target_mode: Option<String>,
    /// シンボリックリンクかどうか
    pub is_symlink: bool,
    /// シンボリックリンクの場合のリンク先
    pub symlink_target: Option<String>,
    /// サブモジュールかどうか
    pub is_submodule: bool,
    /// source側のblob ID
    pub source_blob: Option<String>,
    /// target側のblob ID
    pub target_blob: Option<String>,
    /// source側のファイル内容
    pub source_content: Option<Vec<u8>>,
    /// target側のファイル内容
    pub target_content: Option<Vec<u8>>,
    /// エラーメッセージ（取得失敗時）
    pub error: Option<String>,
}

impl DiffFile {
    pub fn new(path: PathBuf, status: FileStatus) -> Self {
        Self {
            path,
            original_path: None,
            status,
            similarity: None,
            source_mode: None,
            target_mode: None,
            is_symlink: false,
            symlink_target: None,
            is_submodule: false,
            source_blob: None,
            target_blob: None,
            source_content: None,
            target_content: None,
            error: None,
        }
    }

    /// 権限のみ変更されたかどうか
    pub fn is_permission_only_change(&self) -> bool {
        if let (Some(src), Some(tgt)) = (&self.source_mode, &self.target_mode) {
            if src != tgt {
                // blob IDが同じなら権限のみの変更
                if let (Some(src_blob), Some(tgt_blob)) = (&self.source_blob, &self.target_blob) {
                    return src_blob == tgt_blob;
                }
            }
        }
        false
    }
}

/// 三者間差分ファイル情報
#[derive(Debug, Clone)]
pub struct ThreeWayDiffFile {
    /// ファイルパス
    pub path: PathBuf,
    /// 三者間ステータス
    pub status: ThreeWayStatus,
    /// base側のblob ID
    pub base_blob: Option<String>,
    /// ours側のblob ID
    pub ours_blob: Option<String>,
    /// theirs側のblob ID
    pub theirs_blob: Option<String>,
    /// base側のファイル内容
    pub base_content: Option<Vec<u8>>,
    /// ours側のファイル内容
    pub ours_content: Option<Vec<u8>>,
    /// theirs側のファイル内容
    pub theirs_content: Option<Vec<u8>>,
    /// base側のファイルサイズ
    pub base_size: Option<usize>,
    /// ours側のファイルサイズ
    pub ours_size: Option<usize>,
    /// theirs側のファイルサイズ
    pub theirs_size: Option<usize>,
    /// エラーメッセージ
    pub error: Option<String>,
}

impl ThreeWayDiffFile {
    pub fn new(path: PathBuf, status: ThreeWayStatus) -> Self {
        Self {
            path,
            status,
            base_blob: None,
            ours_blob: None,
            theirs_blob: None,
            base_content: None,
            ours_content: None,
            theirs_content: None,
            base_size: None,
            ours_size: None,
            theirs_size: None,
            error: None,
        }
    }
}

/// 権限チェックモード
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionCheckMode {
    #[default]
    None,
    Scripts,
    All,
}

impl std::str::FromStr for PermissionCheckMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "scripts" => Ok(Self::Scripts),
            "all" => Ok(Self::All),
            _ => Err(format!("Invalid permission check mode: {}", s)),
        }
    }
}

/// マージスタイル（三者間モード）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MergeStyle {
    #[default]
    All,
    Ours,
    Theirs,
}

impl std::str::FromStr for MergeStyle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(Self::All),
            "ours" => Ok(Self::Ours),
            "theirs" => Ok(Self::Theirs),
            _ => Err(format!("Invalid merge style: {}", s)),
        }
    }
}

/// カラー出力モード
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    #[default]
    Auto,
    Always,
    Never,
}

impl std::str::FromStr for ColorMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(format!("Invalid color mode: {}", s)),
        }
    }
}

/// ログレベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    #[default]
    Warn,
    Info,
    Debug,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "warn" => Ok(Self::Warn),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

impl From<LogLevel> for log::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
        }
    }
}

/// 統計情報
#[derive(Debug, Clone, Default)]
pub struct Statistics {
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub renamed: usize,
    pub copied: usize,
    pub type_changed: usize,
    pub symlinks: usize,
    pub submodules: usize,
    pub permission_changes: usize,
    pub unchanged: usize,
    pub errors: usize,
    pub copy_failed: usize,
}

impl Statistics {
    pub fn total(&self) -> usize {
        self.added
            + self.modified
            + self.deleted
            + self.renamed
            + self.copied
            + self.type_changed
            + self.unchanged
    }
}

/// 三者間比較の統計情報
#[derive(Debug, Clone, Default)]
pub struct ThreeWayStatistics {
    pub unchanged: usize,
    pub ours_only: usize,
    pub theirs_only: usize,
    pub both_same: usize,
    pub conflict: usize,
    pub added_ours: usize,
    pub added_theirs: usize,
    pub added_both_same: usize,
    pub added_both_diff: usize,
    pub deleted_ours: usize,
    pub deleted_theirs: usize,
    pub deleted_both: usize,
    pub modify_delete: usize,
    pub delete_modify: usize,
}

impl ThreeWayStatistics {
    pub fn total(&self) -> usize {
        self.unchanged
            + self.ours_only
            + self.theirs_only
            + self.both_same
            + self.conflict
            + self.added_ours
            + self.added_theirs
            + self.added_both_same
            + self.added_both_diff
            + self.deleted_ours
            + self.deleted_theirs
            + self.deleted_both
            + self.modify_delete
            + self.delete_modify
    }

    pub fn conflicts(&self) -> usize {
        self.conflict + self.added_both_diff + self.modify_delete + self.delete_modify
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // FileStatusテスト
    // ========================================

    #[test]
    fn test_file_status_from_git_status() {
        assert_eq!(FileStatus::from_git_status("A"), Some(FileStatus::Added));
        assert_eq!(FileStatus::from_git_status("M"), Some(FileStatus::Modified));
        assert_eq!(FileStatus::from_git_status("D"), Some(FileStatus::Deleted));
        assert_eq!(FileStatus::from_git_status("R100"), Some(FileStatus::Renamed));
        assert_eq!(FileStatus::from_git_status("C50"), Some(FileStatus::Copied));
        assert_eq!(FileStatus::from_git_status("T"), Some(FileStatus::TypeChanged));
        assert_eq!(FileStatus::from_git_status("X"), None);
        assert_eq!(FileStatus::from_git_status(""), None);
    }

    #[test]
    fn test_file_status_tag() {
        assert_eq!(FileStatus::Added.tag(), "[added]");
        assert_eq!(FileStatus::Modified.tag(), "[modified]");
        assert_eq!(FileStatus::Deleted.tag(), "[deleted]");
        assert_eq!(FileStatus::Renamed.tag(), "[renamed]");
        assert_eq!(FileStatus::Copied.tag(), "[copied]");
        assert_eq!(FileStatus::TypeChanged.tag(), "[type-changed]");
        assert_eq!(FileStatus::Unchanged.tag(), "[unchanged]");
    }

    #[test]
    fn test_file_status_from_filter_str() {
        // 完全一致
        assert_eq!(FileStatus::from_filter_str("added"), Some(FileStatus::Added));
        assert_eq!(FileStatus::from_filter_str("modified"), Some(FileStatus::Modified));
        assert_eq!(FileStatus::from_filter_str("deleted"), Some(FileStatus::Deleted));
        assert_eq!(FileStatus::from_filter_str("renamed"), Some(FileStatus::Renamed));
        assert_eq!(FileStatus::from_filter_str("copied"), Some(FileStatus::Copied));
        assert_eq!(FileStatus::from_filter_str("type-changed"), Some(FileStatus::TypeChanged));
        assert_eq!(FileStatus::from_filter_str("unchanged"), Some(FileStatus::Unchanged));

        // 大文字小文字
        assert_eq!(FileStatus::from_filter_str("ADDED"), Some(FileStatus::Added));
        assert_eq!(FileStatus::from_filter_str("Added"), Some(FileStatus::Added));

        // 別名
        assert_eq!(FileStatus::from_filter_str("add"), Some(FileStatus::Added));
        assert_eq!(FileStatus::from_filter_str("a"), Some(FileStatus::Added));
        assert_eq!(FileStatus::from_filter_str("modify"), Some(FileStatus::Modified));
        assert_eq!(FileStatus::from_filter_str("m"), Some(FileStatus::Modified));
        assert_eq!(FileStatus::from_filter_str("delete"), Some(FileStatus::Deleted));
        assert_eq!(FileStatus::from_filter_str("d"), Some(FileStatus::Deleted));

        // 無効な値
        assert_eq!(FileStatus::from_filter_str("invalid"), None);
        assert_eq!(FileStatus::from_filter_str(""), None);
    }

    // ========================================
    // ThreeWayStatusテスト
    // ========================================

    #[test]
    fn test_three_way_status_is_conflict() {
        assert!(ThreeWayStatus::Conflict.is_conflict());
        assert!(ThreeWayStatus::AddedBothDiff.is_conflict());
        assert!(ThreeWayStatus::ModifyDelete.is_conflict());
        assert!(ThreeWayStatus::DeleteModify.is_conflict());

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

    #[test]
    fn test_three_way_status_tag() {
        assert_eq!(ThreeWayStatus::Unchanged.tag(), "unchanged");
        assert_eq!(ThreeWayStatus::OursOnly.tag(), "ours-only");
        assert_eq!(ThreeWayStatus::TheirsOnly.tag(), "theirs-only");
        assert_eq!(ThreeWayStatus::BothSame.tag(), "both-same");
        assert_eq!(ThreeWayStatus::Conflict.tag(), "CONFLICT");
        assert_eq!(ThreeWayStatus::AddedBothDiff.tag(), "CONFLICT");
        assert_eq!(ThreeWayStatus::ModifyDelete.tag(), "CONFLICT");
        assert_eq!(ThreeWayStatus::DeleteModify.tag(), "CONFLICT");
    }

    // ========================================
    // DiffFileテスト
    // ========================================

    #[test]
    fn test_diff_file_new() {
        let file = DiffFile::new(PathBuf::from("test.txt"), FileStatus::Modified);
        assert_eq!(file.path, PathBuf::from("test.txt"));
        assert_eq!(file.status, FileStatus::Modified);
        assert!(file.original_path.is_none());
        assert!(file.similarity.is_none());
        assert!(!file.is_symlink);
        assert!(!file.is_submodule);
    }

    #[test]
    fn test_diff_file_is_permission_only_change() {
        let mut file = DiffFile::new(PathBuf::from("test.txt"), FileStatus::Modified);

        // 権限のみ変更（blobは同じ）
        file.source_mode = Some("100644".to_string());
        file.target_mode = Some("100755".to_string());
        file.source_blob = Some("abc123".to_string());
        file.target_blob = Some("abc123".to_string());
        assert!(file.is_permission_only_change());

        // 内容も変更
        file.target_blob = Some("def456".to_string());
        assert!(!file.is_permission_only_change());
    }

    // ========================================
    // PermissionCheckModeテスト
    // ========================================

    #[test]
    fn test_permission_check_mode_from_str() {
        use std::str::FromStr;

        assert_eq!(PermissionCheckMode::from_str("none").unwrap(), PermissionCheckMode::None);
        assert_eq!(PermissionCheckMode::from_str("scripts").unwrap(), PermissionCheckMode::Scripts);
        assert_eq!(PermissionCheckMode::from_str("all").unwrap(), PermissionCheckMode::All);
        assert_eq!(PermissionCheckMode::from_str("NONE").unwrap(), PermissionCheckMode::None);
        assert_eq!(PermissionCheckMode::from_str("SCRIPTS").unwrap(), PermissionCheckMode::Scripts);
        assert_eq!(PermissionCheckMode::from_str("ALL").unwrap(), PermissionCheckMode::All);

        assert!(PermissionCheckMode::from_str("invalid").is_err());
    }

    // ========================================
    // MergeStyleテスト
    // ========================================

    #[test]
    fn test_merge_style_from_str() {
        use std::str::FromStr;

        assert_eq!(MergeStyle::from_str("all").unwrap(), MergeStyle::All);
        assert_eq!(MergeStyle::from_str("ours").unwrap(), MergeStyle::Ours);
        assert_eq!(MergeStyle::from_str("theirs").unwrap(), MergeStyle::Theirs);
        assert_eq!(MergeStyle::from_str("ALL").unwrap(), MergeStyle::All);

        assert!(MergeStyle::from_str("invalid").is_err());
    }

    // ========================================
    // ColorModeテスト
    // ========================================

    #[test]
    fn test_color_mode_from_str() {
        use std::str::FromStr;

        assert_eq!(ColorMode::from_str("auto").unwrap(), ColorMode::Auto);
        assert_eq!(ColorMode::from_str("always").unwrap(), ColorMode::Always);
        assert_eq!(ColorMode::from_str("never").unwrap(), ColorMode::Never);
        assert_eq!(ColorMode::from_str("AUTO").unwrap(), ColorMode::Auto);

        assert!(ColorMode::from_str("invalid").is_err());
    }

    // ========================================
    // LogLevelテスト
    // ========================================

    #[test]
    fn test_log_level_from_str() {
        use std::str::FromStr;

        assert_eq!(LogLevel::from_str("error").unwrap(), LogLevel::Error);
        assert_eq!(LogLevel::from_str("warn").unwrap(), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("info").unwrap(), LogLevel::Info);
        assert_eq!(LogLevel::from_str("debug").unwrap(), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("ERROR").unwrap(), LogLevel::Error);

        assert!(LogLevel::from_str("invalid").is_err());
    }

    #[test]
    fn test_log_level_to_filter() {
        assert_eq!(log::LevelFilter::from(LogLevel::Error), log::LevelFilter::Error);
        assert_eq!(log::LevelFilter::from(LogLevel::Warn), log::LevelFilter::Warn);
        assert_eq!(log::LevelFilter::from(LogLevel::Info), log::LevelFilter::Info);
        assert_eq!(log::LevelFilter::from(LogLevel::Debug), log::LevelFilter::Debug);
    }

    // ========================================
    // Statisticsテスト
    // ========================================

    #[test]
    fn test_statistics_total() {
        let mut stats = Statistics::default();
        stats.added = 5;
        stats.modified = 3;
        stats.deleted = 2;
        stats.renamed = 1;
        stats.copied = 1;
        stats.type_changed = 1;
        stats.unchanged = 10;

        assert_eq!(stats.total(), 23);
    }

    // ========================================
    // ThreeWayStatisticsテスト
    // ========================================

    #[test]
    fn test_three_way_statistics_total() {
        let mut stats = ThreeWayStatistics::default();
        stats.unchanged = 10;
        stats.ours_only = 2;
        stats.theirs_only = 3;
        stats.both_same = 1;
        stats.conflict = 1;

        assert_eq!(stats.total(), 17);
    }

    #[test]
    fn test_three_way_statistics_conflicts() {
        let mut stats = ThreeWayStatistics::default();
        stats.conflict = 2;
        stats.added_both_diff = 1;
        stats.modify_delete = 1;
        stats.delete_modify = 1;

        assert_eq!(stats.conflicts(), 5);
    }
}
