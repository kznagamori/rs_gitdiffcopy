//! サマリー出力

use crate::config::MergedConfig;
use crate::types::{
    DiffFile, FileStatus, Statistics, ThreeWayDiffFile, ThreeWayStatistics,
};
use chrono::Local;
use colored::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// サマリー出力
pub struct SummaryWriter<'a> {
    config: &'a MergedConfig,
    use_color: bool,
}

impl<'a> SummaryWriter<'a> {
    pub fn new(config: &'a MergedConfig) -> Self {
        let use_color = match config.color {
            crate::types::ColorMode::Always => true,
            crate::types::ColorMode::Never => false,
            crate::types::ColorMode::Auto => atty::is(atty::Stream::Stdout),
        };

        Self { config, use_color }
    }

    /// 二者間比較のサマリーを出力
    pub fn write_two_way(
        &self,
        files: &[DiffFile],
        stats: &Statistics,
        source_ref: &str,
        source_commit: &str,
        target_ref: &str,
        target_commit: &str,
        repo_path: &Path,
        copy_errors: &[String],
    ) -> io::Result<()> {
        let mut output = String::new();

        // ヘッダー
        output.push_str("rs_gitdiffcopy Summary\n");
        output.push_str("======================\n");
        output.push_str(&format!("Repository: {}\n", repo_path.display()));
        output.push_str(&format!(
            "Source ref: {} ({})\n",
            source_ref,
            &source_commit[..7.min(source_commit.len())]
        ));
        output.push_str(&format!(
            "Target ref: {} ({})\n",
            target_ref,
            &target_commit[..7.min(target_commit.len())]
        ));
        output.push_str(&format!("Output: {}\n", self.config.output.display()));
        output.push_str(&format!("Date: {}\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        output.push('\n');

        // オプション
        self.write_options(&mut output);

        // 統計情報
        self.write_statistics(&mut output, stats);

        // ファイルツリー
        if !self.config.no_tree && !self.config.stats_only {
            self.write_file_tree(&mut output, files);
        }

        // 詳細セクション
        if !self.config.no_details && !self.config.stats_only {
            self.write_details(&mut output, files);
        }

        // エラーセクション
        if !copy_errors.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Copy Failed\n");
            output.push_str("================\n");
            for err in copy_errors {
                output.push_str(&format!("  {}\n", err));
            }
        }

        // 出力
        self.output_summary(&output)
    }

    /// 三者間比較のサマリーを出力
    pub fn write_three_way(
        &self,
        files: &[ThreeWayDiffFile],
        stats: &ThreeWayStatistics,
        base_ref: &str,
        base_commit: &str,
        ours_ref: &str,
        ours_commit: &str,
        theirs_ref: &str,
        theirs_commit: &str,
        repo_path: &Path,
        copy_errors: &[String],
    ) -> io::Result<()> {
        let mut output = String::new();

        // ヘッダー
        output.push_str("rs_gitdiffcopy Summary (Three-way)\n");
        output.push_str("==================================\n");
        output.push_str(&format!("Repository: {}\n", repo_path.display()));
        output.push_str(&format!(
            "Base ref: {} ({})\n",
            base_ref,
            &base_commit[..7.min(base_commit.len())]
        ));
        output.push_str(&format!(
            "Ours ref: {} ({})\n",
            ours_ref,
            &ours_commit[..7.min(ours_commit.len())]
        ));
        output.push_str(&format!(
            "Theirs ref: {} ({})\n",
            theirs_ref,
            &theirs_commit[..7.min(theirs_commit.len())]
        ));
        output.push_str(&format!("Output: {}\n", self.config.output.display()));
        output.push_str(&format!("Date: {}\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        output.push('\n');

        // オプション
        self.write_options(&mut output);

        // 統計情報（三者間）
        self.write_three_way_statistics(&mut output, stats);

        // ファイルツリー（三者間）
        if !self.config.no_tree && !self.config.stats_only {
            self.write_three_way_file_tree(&mut output, files);
        }

        // コンフリクト詳細
        if !self.config.no_details && !self.config.stats_only {
            self.write_conflict_details(&mut output, files);
        }

        // エラーセクション
        if !copy_errors.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Copy Failed\n");
            output.push_str("================\n");
            for err in copy_errors {
                output.push_str(&format!("  {}\n", err));
            }
        }

        // 出力
        self.output_summary(&output)
    }

    /// オプションセクションを出力
    fn write_options(&self, output: &mut String) {
        output.push_str("Options:\n");

        if self.config.dry_run {
            output.push_str("  Mode: Dry run (no files copied)\n");
        }
        if self.config.both_versions {
            output.push_str("  Copy mode: Both versions (.old/.new)\n");
        }
        if self.config.copy_deleted {
            output.push_str("  Copy deleted: Yes (.deleted)\n");
        }
        if self.config.preserve_timestamps {
            output.push_str("  Preserve timestamps: Yes\n");
        }
        if self.config.check_permissions != crate::types::PermissionCheckMode::None {
            output.push_str(&format!("  Permission check: {:?}\n", self.config.check_permissions));
        }
        if self.config.patch {
            output.push_str("  Patch mode: Individual files (.patch)\n");
        }
        if let Some(ref patch_file) = self.config.patch_file {
            output.push_str(&format!("  Combined patch file: {}\n", patch_file.display()));
        }
        if !self.config.filter_status.is_empty() {
            output.push_str(&format!("  Filter status: {}\n", self.config.filter_status.join(", ")));
        }
        if !self.config.exclude.is_empty() {
            output.push_str("  Exclude patterns:\n");
            for pattern in &self.config.exclude {
                output.push_str(&format!("    - {}\n", pattern));
            }
        }
        output.push('\n');
    }

    /// 統計情報を出力
    fn write_statistics(&self, output: &mut String, stats: &Statistics) {
        output.push_str(&format!("Added:         {} files\n", stats.added));
        output.push_str(&format!("Modified:      {} files\n", stats.modified));
        output.push_str(&format!("Deleted:       {} files\n", stats.deleted));
        output.push_str(&format!("Renamed:       {} files\n", stats.renamed));
        if stats.copied > 0 {
            output.push_str(&format!("Copied:        {} files\n", stats.copied));
        }
        if stats.type_changed > 0 {
            output.push_str(&format!("Type changed:  {} files\n", stats.type_changed));
        }
        if stats.symlinks > 0 {
            output.push_str(&format!("Symlinks:      {} files\n", stats.symlinks));
        }
        if stats.submodules > 0 {
            output.push_str(&format!("Submodules:    {}\n", stats.submodules));
        }
        if stats.permission_changes > 0 {
            output.push_str(&format!("Permission changes: {} files\n", stats.permission_changes));
        }
        output.push_str(&format!("Unchanged:     {} files\n", stats.unchanged));
        output.push_str("--------------------------\n");
        output.push_str(&format!("Total:        {} items\n", stats.total()));
        output.push('\n');
    }

    /// 三者間統計情報を出力
    fn write_three_way_statistics(&self, output: &mut String, stats: &ThreeWayStatistics) {
        output.push_str("================\n");
        output.push_str("Change Matrix\n");
        output.push_str("================\n");
        output.push_str("Status          | Count\n");
        output.push_str("----------------|------\n");
        output.push_str(&format!("Unchanged       | {:>5}\n", stats.unchanged));
        output.push_str(&format!("Ours only       | {:>5}\n", stats.ours_only));
        output.push_str(&format!("Theirs only     | {:>5}\n", stats.theirs_only));
        output.push_str(&format!("Both same       | {:>5}\n", stats.both_same));
        output.push_str(&format!("Conflict        | {:>5}\n", stats.conflict));
        output.push_str(&format!("Added (ours)    | {:>5}\n", stats.added_ours));
        output.push_str(&format!("Added (theirs)  | {:>5}\n", stats.added_theirs));
        output.push_str(&format!("Added (both)    | {:>5}\n", stats.added_both_same + stats.added_both_diff));
        output.push_str(&format!("Deleted (ours)  | {:>5}\n", stats.deleted_ours));
        output.push_str(&format!("Deleted (theirs)| {:>5}\n", stats.deleted_theirs));
        output.push_str(&format!("Deleted (both)  | {:>5}\n", stats.deleted_both));
        output.push_str("--------------------------\n");
        output.push_str(&format!("Total           | {:>5}\n", stats.total()));
        output.push_str(&format!("Conflicts       | {:>5}\n", stats.conflicts()));
        output.push('\n');
    }

    /// ファイルツリーを出力
    fn write_file_tree(&self, output: &mut String, files: &[DiffFile]) {
        output.push_str("\n================\n");
        output.push_str("File Tree\n");
        output.push_str("================\n");
        output.push_str(".\n");

        // ファイルをディレクトリ構造に整理
        let tree = self.build_tree(files);
        self.write_tree_node(output, &tree, "", true);
    }

    /// 三者間ファイルツリーを出力
    fn write_three_way_file_tree(&self, output: &mut String, files: &[ThreeWayDiffFile]) {
        output.push_str("\n================\n");
        output.push_str("File Tree\n");
        output.push_str("================\n");
        output.push_str("Legend: [Base|Ours|Theirs] ○=exists -=missing ==same M=modified A=added D=deleted\n");
        output.push_str(".\n");

        // 簡略化した出力
        for file in files {
            let indicator = file.status.indicator();
            let tag = file.status.tag();
            output.push_str(&format!("├── {} {} {}\n", file.path.display(), indicator, tag));
        }
    }

    /// ディレクトリツリーを構築
    fn build_tree(&self, files: &[DiffFile]) -> BTreeMap<String, TreeNode> {
        let mut root = BTreeMap::new();

        for file in files {
            let components: Vec<_> = file.path.components().collect();
            Self::insert_into_tree(&mut root, &components, 0, file);
        }

        root
    }

    /// ツリーにファイルを挿入（再帰ヘルパー）
    fn insert_into_tree(
        tree: &mut BTreeMap<String, TreeNode>,
        components: &[std::path::Component],
        index: usize,
        file: &DiffFile,
    ) {
        if index >= components.len() {
            return;
        }

        let name = components[index].as_os_str().to_string_lossy().to_string();
        let is_last = index == components.len() - 1;

        if is_last {
            tree.entry(name).or_insert_with(|| TreeNode::File(file.clone()));
        } else {
            let entry = tree
                .entry(name)
                .or_insert_with(|| TreeNode::Dir(BTreeMap::new()));
            if let TreeNode::Dir(children) = entry {
                Self::insert_into_tree(children, components, index + 1, file);
            }
        }
    }

    /// ツリーノードを出力
    fn write_tree_node(
        &self,
        output: &mut String,
        tree: &BTreeMap<String, TreeNode>,
        prefix: &str,
        _is_last: bool,
    ) {
        let entries: Vec<_> = tree.iter().collect();
        let len = entries.len();

        for (i, (name, node)) in entries.iter().enumerate() {
            let is_last_entry = i == len - 1;
            let connector = if is_last_entry { "└── " } else { "├── " };
            let child_prefix = if is_last_entry { "    " } else { "│   " };

            match node {
                TreeNode::File(file) => {
                    let status_tag = self.format_status_tag(file);
                    output.push_str(&format!("{}{}{} {}\n", prefix, connector, name, status_tag));
                }
                TreeNode::Dir(children) => {
                    output.push_str(&format!("{}{}{}/\n", prefix, connector, name));
                    self.write_tree_node(output, children, &format!("{}{}", prefix, child_prefix), is_last_entry);
                }
            }
        }
    }

    /// ステータスタグをフォーマット
    fn format_status_tag(&self, file: &DiffFile) -> String {
        let tag = file.status.tag();

        if self.use_color {
            match file.status {
                FileStatus::Added => tag.green().to_string(),
                FileStatus::Modified => tag.blue().to_string(),
                FileStatus::Deleted => tag.red().to_string(),
                FileStatus::Renamed => tag.yellow().to_string(),
                FileStatus::Copied => tag.cyan().to_string(),
                FileStatus::TypeChanged => tag.magenta().to_string(),
                FileStatus::Unchanged => tag.dimmed().to_string(),
            }
        } else {
            tag.to_string()
        }
    }

    /// 詳細セクションを出力
    fn write_details(&self, output: &mut String, files: &[DiffFile]) {
        // Added Files
        let added: Vec<_> = files.iter().filter(|f| f.status == FileStatus::Added).collect();
        if !added.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Added Files\n");
            output.push_str("================\n");
            for file in added {
                output.push_str(&format!("  {}\n", file.path.display()));
            }
        }

        // Modified Files
        let modified: Vec<_> = files.iter().filter(|f| f.status == FileStatus::Modified).collect();
        if !modified.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Modified Files\n");
            output.push_str("================\n");
            for file in modified {
                output.push_str(&format!("  {}\n", file.path.display()));
            }
        }

        // Deleted Files
        let deleted: Vec<_> = files.iter().filter(|f| f.status == FileStatus::Deleted).collect();
        if !deleted.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Deleted Files\n");
            output.push_str("================\n");
            for file in deleted {
                output.push_str(&format!("  {}\n", file.path.display()));
            }
        }

        // Renamed Files
        let renamed: Vec<_> = files.iter().filter(|f| f.status == FileStatus::Renamed).collect();
        if !renamed.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Renamed Files\n");
            output.push_str("================\n");
            for file in renamed {
                if let Some(ref orig) = file.original_path {
                    let sim = file.similarity.map(|s| format!(" ({}% similar)", s)).unwrap_or_default();
                    output.push_str(&format!("  {} -> {}{}\n", orig.display(), file.path.display(), sim));
                }
            }
        }

        // Copied Files
        let copied: Vec<_> = files.iter().filter(|f| f.status == FileStatus::Copied).collect();
        if !copied.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Copied Files\n");
            output.push_str("================\n");
            for file in copied {
                if let Some(ref orig) = file.original_path {
                    let sim = file.similarity.map(|s| format!(" ({}% similar)", s)).unwrap_or_default();
                    output.push_str(&format!("  {} -> {}{}\n", orig.display(), file.path.display(), sim));
                }
            }
        }

        // Symlinks
        let symlinks: Vec<_> = files.iter().filter(|f| f.is_symlink).collect();
        if !symlinks.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Symlink Details\n");
            output.push_str("================\n");
            for file in symlinks {
                if let Some(ref target) = file.symlink_target {
                    output.push_str(&format!("  {} -> {}\n", file.path.display(), target));
                }
            }
        }

        // Submodules
        let submodules: Vec<_> = files.iter().filter(|f| f.is_submodule).collect();
        if !submodules.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Submodule Details\n");
            output.push_str("================\n");
            for file in submodules {
                output.push_str(&format!("  {}/\n", file.path.display()));
            }
        }

        // Permission Changes
        let perm_changes: Vec<_> = files.iter().filter(|f| f.is_permission_only_change()).collect();
        if !perm_changes.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Permission Changes\n");
            output.push_str("================\n");
            for file in perm_changes {
                if let (Some(src), Some(tgt)) = (&file.source_mode, &file.target_mode) {
                    output.push_str(&format!("{}: {} -> {}\n", file.path.display(), src, tgt));
                }
            }
        }

        // Errors
        let errors: Vec<_> = files.iter().filter(|f| f.error.is_some()).collect();
        if !errors.is_empty() {
            output.push_str("\n================\n");
            output.push_str("Errors\n");
            output.push_str("================\n");
            for file in errors {
                if let Some(ref err) = file.error {
                    output.push_str(&format!("  {} ({})\n", file.path.display(), err));
                }
            }
        }
    }

    /// コンフリクト詳細を出力
    fn write_conflict_details(&self, output: &mut String, files: &[ThreeWayDiffFile]) {
        let conflicts: Vec<_> = files.iter().filter(|f| f.status.is_conflict()).collect();

        if conflicts.is_empty() {
            return;
        }

        output.push_str("\n================\n");
        output.push_str("Conflict Details\n");
        output.push_str("================\n");

        for (i, file) in conflicts.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, file.path.display()));
            output.push_str(&format!("   Type: {:?}\n", file.status));

            if let Some(ref blob) = file.base_blob {
                let size = file.base_size.map(|s| format!(" ({} bytes)", s)).unwrap_or_default();
                output.push_str(&format!("   Base:   {}...{}\n", &blob[..8.min(blob.len())], size));
            }
            if let Some(ref blob) = file.ours_blob {
                let size = file.ours_size.map(|s| format!(" ({} bytes)", s)).unwrap_or_default();
                output.push_str(&format!("   Ours:   {}...{}\n", &blob[..8.min(blob.len())], size));
            }
            if let Some(ref blob) = file.theirs_blob {
                let size = file.theirs_size.map(|s| format!(" ({} bytes)", s)).unwrap_or_default();
                output.push_str(&format!("   Theirs: {}...{}\n", &blob[..8.min(blob.len())], size));
            }
            output.push('\n');
        }
    }

    /// サマリーを出力
    fn output_summary(&self, content: &str) -> io::Result<()> {
        // コンソールに出力
        print!("{}", content);

        // ファイルに出力（指定されている場合）
        if let Some(ref path) = self.config.summary {
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
        }

        Ok(())
    }
}

/// ツリーノード
enum TreeNode {
    File(DiffFile),
    Dir(BTreeMap<String, TreeNode>),
}
