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

    /// フィルター文字列からステータスを解析（三者間用）
    pub fn from_filter_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "unchanged" | "same" => Some(Self::Unchanged),
            "ours-only" | "ours_only" | "oursonly" => Some(Self::OursOnly),
            "theirs-only" | "theirs_only" | "theirsonly" => Some(Self::TheirsOnly),
            "both-same" | "both_same" | "bothsame" => Some(Self::BothSame),
            "conflict" => Some(Self::Conflict),
            "added-ours" | "added_ours" | "addedours" => Some(Self::AddedOurs),
            "added-theirs" | "added_theirs" | "addedtheirs" => Some(Self::AddedTheirs),
            "added-both-same" | "added_both_same" | "addedbothsame" => Some(Self::AddedBothSame),
            "added-both-diff" | "added_both_diff" | "addedbothdiff" => Some(Self::AddedBothDiff),
            "deleted-ours" | "deleted_ours" | "deletedours" => Some(Self::DeletedOurs),
            "deleted-theirs" | "deleted_theirs" | "deletedtheirs" => Some(Self::DeletedTheirs),
            "deleted-both" | "deleted_both" | "deletedboth" => Some(Self::DeletedBoth),
            "modify-delete" | "modify_delete" | "modifydelete" => Some(Self::ModifyDelete),
            "delete-modify" | "delete_modify" | "deletemodify" => Some(Self::DeleteModify),
            _ => None,
        }
    }

    /// 全ステータスのリストを取得
    pub fn all() -> Vec<Self> {
        vec![
            Self::Unchanged,
            Self::OursOnly,
            Self::TheirsOnly,
            Self::BothSame,
            Self::Conflict,
            Self::AddedOurs,
            Self::AddedTheirs,
            Self::AddedBothSame,
            Self::AddedBothDiff,
            Self::DeletedOurs,
            Self::DeletedTheirs,
            Self::DeletedBoth,
            Self::ModifyDelete,
            Self::DeleteModify,
        ]
    }
}

/// グループキーワードの定義（三者間モード用）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterGroup {
    /// added グループ: added-ours, added-theirs, added-both-same, added-both-diff
    Added,
    /// modified グループ: ours-only, theirs-only, both-same, conflict
    Modified,
    /// deleted グループ: deleted-ours, deleted-theirs, deleted-both
    Deleted,
    /// conflicts グループ: conflict, added-both-diff, modify-delete, delete-modify
    Conflicts,
    /// all: 全ステータス
    All,
}

impl FilterGroup {
    /// グループ名からFilterGroupを解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "added" => Some(Self::Added),
            "modified" => Some(Self::Modified),
            "deleted" => Some(Self::Deleted),
            "conflicts" => Some(Self::Conflicts),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    /// グループに含まれるThreeWayStatusを取得
    pub fn expand(&self) -> Vec<ThreeWayStatus> {
        match self {
            Self::Added => vec![
                ThreeWayStatus::AddedOurs,
                ThreeWayStatus::AddedTheirs,
                ThreeWayStatus::AddedBothSame,
                ThreeWayStatus::AddedBothDiff,
            ],
            Self::Modified => vec![
                ThreeWayStatus::OursOnly,
                ThreeWayStatus::TheirsOnly,
                ThreeWayStatus::BothSame,
                ThreeWayStatus::Conflict,
            ],
            Self::Deleted => vec![
                ThreeWayStatus::DeletedOurs,
                ThreeWayStatus::DeletedTheirs,
                ThreeWayStatus::DeletedBoth,
            ],
            Self::Conflicts => vec![
                ThreeWayStatus::Conflict,
                ThreeWayStatus::AddedBothDiff,
                ThreeWayStatus::ModifyDelete,
                ThreeWayStatus::DeleteModify,
            ],
            Self::All => ThreeWayStatus::all(),
        }
    }
}

/// filter_status文字列を展開してThreeWayStatusのセットに変換
///
/// # 引数
/// - `filter_status`: filter_status文字列のリスト（例: ["added", "^deleted"]）
///
/// # 戻り値
/// - 含めるべきThreeWayStatusのセット
///
/// # 動作
/// - グループキーワード（added, modified, deleted, conflicts, all）は展開される
/// - `^`プレフィックスは除外を意味する
/// - 除外のみの場合、暗黙的にallが適用される
pub fn expand_filter_status_three_way(filter_status: &[String]) -> std::collections::HashSet<ThreeWayStatus> {
    use std::collections::HashSet;

    if filter_status.is_empty() {
        // フィルタ未指定の場合は全て含める
        return ThreeWayStatus::all().into_iter().collect();
    }

    let mut include: HashSet<ThreeWayStatus> = HashSet::new();
    let mut exclude: HashSet<ThreeWayStatus> = HashSet::new();
    let mut has_include = false;

    for s in filter_status {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (is_exclude, keyword) = if let Some(stripped) = trimmed.strip_prefix('^') {
            (true, stripped)
        } else {
            (false, trimmed)
        };

        // グループキーワードか個別ステータスかを判定
        let statuses: Vec<ThreeWayStatus> = if let Some(group) = FilterGroup::from_str(keyword) {
            group.expand()
        } else if let Some(status) = ThreeWayStatus::from_filter_str(keyword) {
            vec![status]
        } else {
            // 二者間用のステータス（added, modified, deleted等）は三者間では無視
            // または不明なキーワードは無視
            continue;
        };

        if is_exclude {
            exclude.extend(statuses);
        } else {
            has_include = true;
            include.extend(statuses);
        }
    }

    // 除外のみの場合は暗黙的にallを適用
    if !has_include && !exclude.is_empty() {
        include = ThreeWayStatus::all().into_iter().collect();
    }

    // 除外を適用
    include.difference(&exclude).cloned().collect()
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

    // ========================================
    // ThreeWayStatus::from_filter_strテスト
    // ========================================

    #[test]
    fn test_three_way_status_from_filter_str() {
        // 完全一致（kebab-case）
        assert_eq!(ThreeWayStatus::from_filter_str("unchanged"), Some(ThreeWayStatus::Unchanged));
        assert_eq!(ThreeWayStatus::from_filter_str("ours-only"), Some(ThreeWayStatus::OursOnly));
        assert_eq!(ThreeWayStatus::from_filter_str("theirs-only"), Some(ThreeWayStatus::TheirsOnly));
        assert_eq!(ThreeWayStatus::from_filter_str("both-same"), Some(ThreeWayStatus::BothSame));
        assert_eq!(ThreeWayStatus::from_filter_str("conflict"), Some(ThreeWayStatus::Conflict));
        assert_eq!(ThreeWayStatus::from_filter_str("added-ours"), Some(ThreeWayStatus::AddedOurs));
        assert_eq!(ThreeWayStatus::from_filter_str("added-theirs"), Some(ThreeWayStatus::AddedTheirs));
        assert_eq!(ThreeWayStatus::from_filter_str("added-both-same"), Some(ThreeWayStatus::AddedBothSame));
        assert_eq!(ThreeWayStatus::from_filter_str("added-both-diff"), Some(ThreeWayStatus::AddedBothDiff));
        assert_eq!(ThreeWayStatus::from_filter_str("deleted-ours"), Some(ThreeWayStatus::DeletedOurs));
        assert_eq!(ThreeWayStatus::from_filter_str("deleted-theirs"), Some(ThreeWayStatus::DeletedTheirs));
        assert_eq!(ThreeWayStatus::from_filter_str("deleted-both"), Some(ThreeWayStatus::DeletedBoth));
        assert_eq!(ThreeWayStatus::from_filter_str("modify-delete"), Some(ThreeWayStatus::ModifyDelete));
        assert_eq!(ThreeWayStatus::from_filter_str("delete-modify"), Some(ThreeWayStatus::DeleteModify));
    }

    #[test]
    fn test_three_way_status_from_filter_str_case_insensitive() {
        assert_eq!(ThreeWayStatus::from_filter_str("OURS-ONLY"), Some(ThreeWayStatus::OursOnly));
        assert_eq!(ThreeWayStatus::from_filter_str("Ours-Only"), Some(ThreeWayStatus::OursOnly));
        assert_eq!(ThreeWayStatus::from_filter_str("CONFLICT"), Some(ThreeWayStatus::Conflict));
    }

    #[test]
    fn test_three_way_status_from_filter_str_underscore() {
        // アンダースコア形式
        assert_eq!(ThreeWayStatus::from_filter_str("ours_only"), Some(ThreeWayStatus::OursOnly));
        assert_eq!(ThreeWayStatus::from_filter_str("added_ours"), Some(ThreeWayStatus::AddedOurs));
        assert_eq!(ThreeWayStatus::from_filter_str("deleted_both"), Some(ThreeWayStatus::DeletedBoth));
    }

    #[test]
    fn test_three_way_status_from_filter_str_invalid() {
        assert_eq!(ThreeWayStatus::from_filter_str("invalid"), None);
        assert_eq!(ThreeWayStatus::from_filter_str(""), None);
        assert_eq!(ThreeWayStatus::from_filter_str("added"), None); // グループキーワードはfrom_filter_strでは無効
    }

    #[test]
    fn test_three_way_status_all() {
        let all = ThreeWayStatus::all();
        assert_eq!(all.len(), 14);
        assert!(all.contains(&ThreeWayStatus::Unchanged));
        assert!(all.contains(&ThreeWayStatus::Conflict));
        assert!(all.contains(&ThreeWayStatus::AddedOurs));
        assert!(all.contains(&ThreeWayStatus::DeleteModify));
    }

    // ========================================
    // FilterGroupテスト
    // ========================================

    #[test]
    fn test_filter_group_from_str() {
        assert_eq!(FilterGroup::from_str("added"), Some(FilterGroup::Added));
        assert_eq!(FilterGroup::from_str("modified"), Some(FilterGroup::Modified));
        assert_eq!(FilterGroup::from_str("deleted"), Some(FilterGroup::Deleted));
        assert_eq!(FilterGroup::from_str("conflicts"), Some(FilterGroup::Conflicts));
        assert_eq!(FilterGroup::from_str("all"), Some(FilterGroup::All));
        assert_eq!(FilterGroup::from_str("ADDED"), Some(FilterGroup::Added));
        assert_eq!(FilterGroup::from_str("invalid"), None);
    }

    #[test]
    fn test_filter_group_expand_added() {
        let statuses = FilterGroup::Added.expand();
        assert_eq!(statuses.len(), 4);
        assert!(statuses.contains(&ThreeWayStatus::AddedOurs));
        assert!(statuses.contains(&ThreeWayStatus::AddedTheirs));
        assert!(statuses.contains(&ThreeWayStatus::AddedBothSame));
        assert!(statuses.contains(&ThreeWayStatus::AddedBothDiff));
    }

    #[test]
    fn test_filter_group_expand_modified() {
        let statuses = FilterGroup::Modified.expand();
        assert_eq!(statuses.len(), 4);
        assert!(statuses.contains(&ThreeWayStatus::OursOnly));
        assert!(statuses.contains(&ThreeWayStatus::TheirsOnly));
        assert!(statuses.contains(&ThreeWayStatus::BothSame));
        assert!(statuses.contains(&ThreeWayStatus::Conflict));
    }

    #[test]
    fn test_filter_group_expand_deleted() {
        let statuses = FilterGroup::Deleted.expand();
        assert_eq!(statuses.len(), 3);
        assert!(statuses.contains(&ThreeWayStatus::DeletedOurs));
        assert!(statuses.contains(&ThreeWayStatus::DeletedTheirs));
        assert!(statuses.contains(&ThreeWayStatus::DeletedBoth));
    }

    #[test]
    fn test_filter_group_expand_conflicts() {
        let statuses = FilterGroup::Conflicts.expand();
        assert_eq!(statuses.len(), 4);
        assert!(statuses.contains(&ThreeWayStatus::Conflict));
        assert!(statuses.contains(&ThreeWayStatus::AddedBothDiff));
        assert!(statuses.contains(&ThreeWayStatus::ModifyDelete));
        assert!(statuses.contains(&ThreeWayStatus::DeleteModify));
    }

    #[test]
    fn test_filter_group_expand_all() {
        let statuses = FilterGroup::All.expand();
        assert_eq!(statuses.len(), 14);
    }

    // ========================================
    // expand_filter_status_three_wayテスト
    // ========================================

    #[test]
    fn test_expand_filter_status_three_way_empty() {
        let result = expand_filter_status_three_way(&[]);
        assert_eq!(result.len(), 14); // 全ステータス
    }

    #[test]
    fn test_expand_filter_status_three_way_single_status() {
        let result = expand_filter_status_three_way(&["conflict".to_string()]);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&ThreeWayStatus::Conflict));
    }

    #[test]
    fn test_expand_filter_status_three_way_group_added() {
        let result = expand_filter_status_three_way(&["added".to_string()]);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&ThreeWayStatus::AddedOurs));
        assert!(result.contains(&ThreeWayStatus::AddedTheirs));
        assert!(result.contains(&ThreeWayStatus::AddedBothSame));
        assert!(result.contains(&ThreeWayStatus::AddedBothDiff));
    }

    #[test]
    fn test_expand_filter_status_three_way_group_conflicts() {
        let result = expand_filter_status_three_way(&["conflicts".to_string()]);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&ThreeWayStatus::Conflict));
        assert!(result.contains(&ThreeWayStatus::AddedBothDiff));
        assert!(result.contains(&ThreeWayStatus::ModifyDelete));
        assert!(result.contains(&ThreeWayStatus::DeleteModify));
    }

    #[test]
    fn test_expand_filter_status_three_way_exclude() {
        // ^added: 全ステータスからaddedグループを除外
        let result = expand_filter_status_three_way(&["^added".to_string()]);
        assert_eq!(result.len(), 10); // 14 - 4 = 10
        assert!(!result.contains(&ThreeWayStatus::AddedOurs));
        assert!(!result.contains(&ThreeWayStatus::AddedTheirs));
        assert!(!result.contains(&ThreeWayStatus::AddedBothSame));
        assert!(!result.contains(&ThreeWayStatus::AddedBothDiff));
        assert!(result.contains(&ThreeWayStatus::Unchanged));
        assert!(result.contains(&ThreeWayStatus::Conflict));
    }

    #[test]
    fn test_expand_filter_status_three_way_all_exclude() {
        // all,^deleted: 全ステータスからdeletedグループを除外
        let result = expand_filter_status_three_way(&["all".to_string(), "^deleted".to_string()]);
        assert_eq!(result.len(), 11); // 14 - 3 = 11
        assert!(!result.contains(&ThreeWayStatus::DeletedOurs));
        assert!(!result.contains(&ThreeWayStatus::DeletedTheirs));
        assert!(!result.contains(&ThreeWayStatus::DeletedBoth));
    }

    #[test]
    fn test_expand_filter_status_three_way_multiple_groups() {
        // added,modified: addedとmodifiedグループ
        let result = expand_filter_status_three_way(&["added".to_string(), "modified".to_string()]);
        assert_eq!(result.len(), 8); // 4 + 4 = 8
        assert!(result.contains(&ThreeWayStatus::AddedOurs));
        assert!(result.contains(&ThreeWayStatus::OursOnly));
    }

    #[test]
    fn test_expand_filter_status_three_way_exclude_conflicts() {
        // all,^conflicts: コンフリクトを除外
        let result = expand_filter_status_three_way(&["all".to_string(), "^conflicts".to_string()]);
        assert_eq!(result.len(), 10); // 14 - 4 = 10
        assert!(!result.contains(&ThreeWayStatus::Conflict));
        assert!(!result.contains(&ThreeWayStatus::AddedBothDiff));
        assert!(!result.contains(&ThreeWayStatus::ModifyDelete));
        assert!(!result.contains(&ThreeWayStatus::DeleteModify));
    }

    #[test]
    fn test_expand_filter_status_three_way_mixed_include_exclude() {
        // added,^added-both-diff: addedグループからadded-both-diffを除外
        let result = expand_filter_status_three_way(&["added".to_string(), "^added-both-diff".to_string()]);
        assert_eq!(result.len(), 3); // 4 - 1 = 3
        assert!(result.contains(&ThreeWayStatus::AddedOurs));
        assert!(result.contains(&ThreeWayStatus::AddedTheirs));
        assert!(result.contains(&ThreeWayStatus::AddedBothSame));
        assert!(!result.contains(&ThreeWayStatus::AddedBothDiff));
    }
}
