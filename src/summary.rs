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
use unicode_width::UnicodeWidthChar;

/// Box Drawing文字を幅2として扱う表示幅計算
///
/// unicode_widthクレートはBox Drawing文字(U+2500-U+257F)を幅1として返すが、
/// CJK/日本語端末ではこれらの文字は幅2で表示されるため、補正が必要
fn display_width(s: &str) -> usize {
    s.chars().map(|c| {
        // Box Drawing文字(U+2500-U+257F)は幅2として扱う
        if ('\u{2500}'..='\u{257F}').contains(&c) {
            2
        } else {
            UnicodeWidthChar::width(c).unwrap_or(0)
        }
    }).sum()
}

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
        // コンソール出力用（カラーあり、コンパクト形式）
        let console_output = self.build_two_way_output(
            files,
            stats,
            source_ref,
            source_commit,
            target_ref,
            target_commit,
            repo_path,
            copy_errors,
            self.use_color,
            false, // to_file: false = compact format
        );

        // ファイル出力用（カラーなし、整列形式）
        let file_output = self.build_two_way_output(
            files,
            stats,
            source_ref,
            source_commit,
            target_ref,
            target_commit,
            repo_path,
            copy_errors,
            false,
            true, // to_file: true = aligned format
        );

        // 出力
        self.output_summary(&console_output, &file_output)
    }

    /// 二者間比較のサマリー文字列を構築
    fn build_two_way_output(
        &self,
        files: &[DiffFile],
        stats: &Statistics,
        source_ref: &str,
        source_commit: &str,
        target_ref: &str,
        target_commit: &str,
        repo_path: &Path,
        copy_errors: &[String],
        use_color: bool,
        to_file: bool, // true: aligned format, false: compact format
    ) -> String {
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
            self.write_file_tree(&mut output, files, use_color, to_file);
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

        output
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
        // コンソール出力用（カラーあり、コンパクト形式）
        let console_output = self.build_three_way_output(
            files,
            stats,
            base_ref,
            base_commit,
            ours_ref,
            ours_commit,
            theirs_ref,
            theirs_commit,
            repo_path,
            copy_errors,
            self.use_color,
            false, // to_file: false = compact format
        );

        // ファイル出力用（カラーなし、整列形式）
        let file_output = self.build_three_way_output(
            files,
            stats,
            base_ref,
            base_commit,
            ours_ref,
            ours_commit,
            theirs_ref,
            theirs_commit,
            repo_path,
            copy_errors,
            false,
            true, // to_file: true = aligned format
        );

        // 出力
        self.output_summary(&console_output, &file_output)
    }

    /// 三者間比較のサマリー文字列を構築
    fn build_three_way_output(
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
        _use_color: bool,
        to_file: bool, // true: aligned format, false: compact format
    ) -> String {
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
            self.write_three_way_file_tree(&mut output, files, to_file);
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

        output
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
    /// to_file: true=整列形式（ファイル出力用）、false=コンパクト形式（コンソール出力用）
    fn write_file_tree(&self, output: &mut String, files: &[DiffFile], use_color: bool, to_file: bool) {
        output.push_str("\n================\n");
        output.push_str("File Tree\n");
        output.push_str("================\n");
        output.push_str(".\n");

        // ファイルをディレクトリ構造に整理
        let tree = self.build_tree(files);

        // ファイル出力時のみ整列のため最長幅を計算
        let max_width = if to_file {
            self.calculate_max_tree_width(&tree, "")
        } else {
            0 // コンソール出力はコンパクト形式（整列なし）
        };

        self.write_tree_node(output, &tree, "", true, max_width, use_color);
    }

    /// ツリー内の最長行幅を計算
    fn calculate_max_tree_width(&self, tree: &BTreeMap<String, TreeNode>, prefix: &str) -> usize {
        let mut max_width = 0usize;
        let entries: Vec<_> = tree.iter().collect();
        let len = entries.len();

        for (i, (name, node)) in entries.iter().enumerate() {
            let is_last_entry = i == len - 1;
            let connector = if is_last_entry { "└── " } else { "├── " };
            let child_prefix = if is_last_entry { "    " } else { "│   " };

            match node {
                TreeNode::File(_) => {
                    // prefix + connector + name の表示幅を計算
                    let line = format!("{}{}{}", prefix, connector, name);
                    let width = display_width(&line);
                    if width > max_width {
                        max_width = width;
                    }
                }
                TreeNode::Dir(children) => {
                    // ディレクトリ名も幅計算に含める（"/"を含む）
                    let line = format!("{}{}{}/", prefix, connector, name);
                    let width = display_width(&line);
                    if width > max_width {
                        max_width = width;
                    }
                    // 再帰的に子ノードの幅を計算
                    let child_max = self.calculate_max_tree_width(children, &format!("{}{}", prefix, child_prefix));
                    if child_max > max_width {
                        max_width = child_max;
                    }
                }
            }
        }

        max_width
    }

    /// 三者間ファイルツリーを出力
    /// to_file: true=整列形式（ファイル出力用）、false=コンパクト形式（コンソール出力用）
    fn write_three_way_file_tree(&self, output: &mut String, files: &[ThreeWayDiffFile], to_file: bool) {
        output.push_str("\n================\n");
        output.push_str("File Tree\n");
        output.push_str("================\n");
        output.push_str("Legend: [Base|Ours|Theirs] ○=exists -=missing ==same M=modified A=added D=deleted\n");
        output.push_str(".\n");

        // ファイルをディレクトリ構造に整理
        let tree = self.build_three_way_tree(files);

        // ファイル出力時のみ整列のため最長幅を計算
        let max_width = if to_file {
            self.calculate_max_three_way_tree_width(&tree, "")
        } else {
            0 // コンソール出力はコンパクト形式（整列なし）
        };

        self.write_three_way_tree_node(output, &tree, "", true, max_width);
    }

    /// 三者間比較用ディレクトリツリーを構築
    fn build_three_way_tree(&self, files: &[ThreeWayDiffFile]) -> BTreeMap<String, ThreeWayTreeNode> {
        let mut root = BTreeMap::new();

        for file in files {
            let components: Vec<_> = file.path.components().collect();
            Self::insert_into_three_way_tree(&mut root, &components, 0, file);
        }

        root
    }

    /// 三者間比較用ツリーにファイルを挿入（再帰ヘルパー）
    fn insert_into_three_way_tree(
        tree: &mut BTreeMap<String, ThreeWayTreeNode>,
        components: &[std::path::Component],
        index: usize,
        file: &ThreeWayDiffFile,
    ) {
        if index >= components.len() {
            return;
        }

        let name = components[index].as_os_str().to_string_lossy().to_string();
        let is_last = index == components.len() - 1;

        if is_last {
            tree.entry(name).or_insert_with(|| ThreeWayTreeNode::File(file.clone()));
        } else {
            let entry = tree
                .entry(name)
                .or_insert_with(|| ThreeWayTreeNode::Dir(BTreeMap::new()));
            if let ThreeWayTreeNode::Dir(children) = entry {
                Self::insert_into_three_way_tree(children, components, index + 1, file);
            }
        }
    }

    /// 三者間比較用ツリー内の最長行幅を計算
    fn calculate_max_three_way_tree_width(&self, tree: &BTreeMap<String, ThreeWayTreeNode>, prefix: &str) -> usize {
        let mut max_width = 0usize;
        let entries: Vec<_> = tree.iter().collect();
        let len = entries.len();

        for (i, (name, node)) in entries.iter().enumerate() {
            let is_last_entry = i == len - 1;
            let connector = if is_last_entry { "└── " } else { "├── " };
            let child_prefix = if is_last_entry { "    " } else { "│   " };

            match node {
                ThreeWayTreeNode::File(_) => {
                    let line = format!("{}{}{}", prefix, connector, name);
                    let width = display_width(&line);
                    if width > max_width {
                        max_width = width;
                    }
                }
                ThreeWayTreeNode::Dir(children) => {
                    let line = format!("{}{}{}/", prefix, connector, name);
                    let width = display_width(&line);
                    if width > max_width {
                        max_width = width;
                    }
                    let child_max = self.calculate_max_three_way_tree_width(children, &format!("{}{}", prefix, child_prefix));
                    if child_max > max_width {
                        max_width = child_max;
                    }
                }
            }
        }

        max_width
    }

    /// 三者間比較用ツリーノードを出力
    fn write_three_way_tree_node(
        &self,
        output: &mut String,
        tree: &BTreeMap<String, ThreeWayTreeNode>,
        prefix: &str,
        _is_last: bool,
        max_width: usize,
    ) {
        let entries: Vec<_> = tree.iter().collect();
        let len = entries.len();

        for (i, (name, node)) in entries.iter().enumerate() {
            let is_last_entry = i == len - 1;
            let connector = if is_last_entry { "└── " } else { "├── " };
            let child_prefix = if is_last_entry { "    " } else { "│   " };

            match node {
                ThreeWayTreeNode::File(file) => {
                    let line = format!("{}{}{}", prefix, connector, name);
                    let indicator = file.status.indicator();
                    let tag = file.status.tag();

                    if max_width > 0 {
                        let current_width = display_width(&line);
                        let padding = if max_width > current_width {
                            " ".repeat(max_width - current_width)
                        } else {
                            String::new()
                        };
                        output.push_str(&format!("{}{} {} {}\n", line, padding, indicator, tag));
                    } else {
                        output.push_str(&format!("{} {} {}\n", line, indicator, tag));
                    }
                }
                ThreeWayTreeNode::Dir(children) => {
                    output.push_str(&format!("{}{}{}/\n", prefix, connector, name));
                    self.write_three_way_tree_node(output, children, &format!("{}{}", prefix, child_prefix), is_last_entry, max_width);
                }
            }
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
        max_width: usize,
        use_color: bool,
    ) {
        let entries: Vec<_> = tree.iter().collect();
        let len = entries.len();

        for (i, (name, node)) in entries.iter().enumerate() {
            let is_last_entry = i == len - 1;
            let connector = if is_last_entry { "└── " } else { "├── " };
            let child_prefix = if is_last_entry { "    " } else { "│   " };

            match node {
                TreeNode::File(file) => {
                    let line = format!("{}{}{}", prefix, connector, name);
                    let current_width = display_width(&line);
                    let padding = if max_width > current_width {
                        " ".repeat(max_width - current_width)
                    } else {
                        String::new()
                    };
                    let status_tag = self.format_status_tag(file, use_color);
                    output.push_str(&format!("{}{} {}\n", line, padding, status_tag));
                }
                TreeNode::Dir(children) => {
                    output.push_str(&format!("{}{}{}/\n", prefix, connector, name));
                    self.write_tree_node(output, children, &format!("{}{}", prefix, child_prefix), is_last_entry, max_width, use_color);
                }
            }
        }
    }

    /// ステータスタグをフォーマット
    fn format_status_tag(&self, file: &DiffFile, use_color: bool) -> String {
        let tag = file.status.tag();

        if use_color {
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
    fn output_summary(&self, console_content: &str, file_content: &str) -> io::Result<()> {
        // コンソールに出力（カラー付き）
        print!("{}", console_content);

        // ファイルに出力（カラーなし、指定されている場合）
        if let Some(ref path) = self.config.summary {
            let mut file = File::create(path)?;
            file.write_all(file_content.as_bytes())?;
        }

        Ok(())
    }
}

/// ツリーノード
enum TreeNode {
    File(DiffFile),
    Dir(BTreeMap<String, TreeNode>),
}

/// 三者間比較用ツリーノード
enum ThreeWayTreeNode {
    File(ThreeWayDiffFile),
    Dir(BTreeMap<String, ThreeWayTreeNode>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_config() -> MergedConfig {
        MergedConfig {
            repository: None,
            git_path: None,
            full_clone: false,
            cache_dir: None,
            source: "main".to_string(),
            target: "develop".to_string(),
            output: PathBuf::from("/tmp/output"),
            exclude: vec![],
            force: false,
            verbose: false,
            dry_run: false,
            both_versions: false,
            summary: None,
            check_permissions: crate::types::PermissionCheckMode::None,
            patch: false,
            patch_file: None,
            excel: None,
            excel_fold_level: None,
            show_unchanged: false,
            filter_status: vec![],
            stats_only: false,
            no_tree: false,
            no_details: false,
            copy_deleted: false,
            preserve_timestamps: false,
            workers: 4,
            temp_dir: None,
            color: crate::types::ColorMode::Never,
            log_level: crate::types::LogLevel::Warn,
            three_way: false,
            base: None,
            merge_style: crate::types::MergeStyle::All,
            conflict_only: false,
        }
    }

    fn create_test_file(path: &str, status: FileStatus) -> DiffFile {
        DiffFile::new(PathBuf::from(path), status)
    }

    // format_status_tag のテスト（カラーなし）
    #[test]
    fn test_format_status_tag_added_no_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::Added);
        let tag = writer.format_status_tag(&file, false);
        assert_eq!(tag, "[added]");
        assert!(!tag.contains("\x1b[")); // ANSIエスケープコードが含まれないことを確認
    }

    #[test]
    fn test_format_status_tag_modified_no_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::Modified);
        let tag = writer.format_status_tag(&file, false);
        assert_eq!(tag, "[modified]");
        assert!(!tag.contains("\x1b["));
    }

    #[test]
    fn test_format_status_tag_deleted_no_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::Deleted);
        let tag = writer.format_status_tag(&file, false);
        assert_eq!(tag, "[deleted]");
        assert!(!tag.contains("\x1b["));
    }

    #[test]
    fn test_format_status_tag_renamed_no_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::Renamed);
        let tag = writer.format_status_tag(&file, false);
        assert_eq!(tag, "[renamed]");
        assert!(!tag.contains("\x1b["));
    }

    #[test]
    fn test_format_status_tag_copied_no_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::Copied);
        let tag = writer.format_status_tag(&file, false);
        assert_eq!(tag, "[copied]");
        assert!(!tag.contains("\x1b["));
    }

    #[test]
    fn test_format_status_tag_type_changed_no_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::TypeChanged);
        let tag = writer.format_status_tag(&file, false);
        assert_eq!(tag, "[type-changed]");
        assert!(!tag.contains("\x1b["));
    }

    #[test]
    fn test_format_status_tag_unchanged_no_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::Unchanged);
        let tag = writer.format_status_tag(&file, false);
        assert_eq!(tag, "[unchanged]");
        assert!(!tag.contains("\x1b["));
    }

    // format_status_tag のテスト（カラーあり）
    #[test]
    fn test_format_status_tag_added_with_color() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let file = create_test_file("file.txt", FileStatus::Added);
        let tag = writer.format_status_tag(&file, true);
        // カラー付きの場合、ANSIエスケープコードが含まれる
        assert!(tag.contains("[added]"));
    }

    // build_tree のテスト
    #[test]
    fn test_build_tree_single_file() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![create_test_file("file.txt", FileStatus::Added)];
        let tree = writer.build_tree(&files);
        assert!(tree.contains_key("file.txt"));
    }

    #[test]
    fn test_build_tree_nested_file() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![create_test_file("src/main.rs", FileStatus::Added)];
        let tree = writer.build_tree(&files);
        assert!(tree.contains_key("src"));
        if let Some(TreeNode::Dir(children)) = tree.get("src") {
            assert!(children.contains_key("main.rs"));
        } else {
            panic!("Expected directory node");
        }
    }

    #[test]
    fn test_build_tree_multiple_files_same_dir() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![
            create_test_file("src/main.rs", FileStatus::Added),
            create_test_file("src/lib.rs", FileStatus::Modified),
        ];
        let tree = writer.build_tree(&files);
        if let Some(TreeNode::Dir(children)) = tree.get("src") {
            assert!(children.contains_key("main.rs"));
            assert!(children.contains_key("lib.rs"));
        } else {
            panic!("Expected directory node");
        }
    }

    #[test]
    fn test_build_tree_deep_nesting() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![create_test_file("a/b/c/d/file.txt", FileStatus::Added)];
        let tree = writer.build_tree(&files);
        assert!(tree.contains_key("a"));
    }

    #[test]
    fn test_build_tree_japanese_path() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![create_test_file("日本語/ファイル.txt", FileStatus::Added)];
        let tree = writer.build_tree(&files);
        assert!(tree.contains_key("日本語"));
    }

    // calculate_max_tree_width のテスト
    #[test]
    fn test_calculate_max_tree_width_single_file() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![create_test_file("file.txt", FileStatus::Added)];
        let tree = writer.build_tree(&files);
        let width = writer.calculate_max_tree_width(&tree, "");
        // "├── file.txt" の幅
        assert!(width > 0);
    }

    #[test]
    fn test_calculate_max_tree_width_long_filename() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![
            create_test_file("short.txt", FileStatus::Added),
            create_test_file("very_long_filename_that_should_be_longer.txt", FileStatus::Added),
        ];
        let tree = writer.build_tree(&files);
        let width = writer.calculate_max_tree_width(&tree, "");
        // 長いファイル名の幅が使用されることを確認
        assert!(width > 20);
    }

    #[test]
    fn test_calculate_max_tree_width_nested_long() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![
            create_test_file("a.txt", FileStatus::Added),
            create_test_file("dir/very_long_nested_filename.txt", FileStatus::Added),
        ];
        let tree = writer.build_tree(&files);
        let width = writer.calculate_max_tree_width(&tree, "");
        // ネストされたファイルの幅が使用されることを確認
        assert!(width > 20);
    }

    #[test]
    fn test_calculate_max_tree_width_japanese() {
        let config = create_test_config();
        let writer = SummaryWriter::new(&config);
        let files = vec![
            create_test_file("a.txt", FileStatus::Added),
            create_test_file("日本語ディレクトリ/日本語ファイル名.txt", FileStatus::Added),
        ];
        let tree = writer.build_tree(&files);
        let width = writer.calculate_max_tree_width(&tree, "");
        // 日本語は表示幅が2倍であることを考慮
        assert!(width > 20);
    }

    // Statistics のテスト
    #[test]
    fn test_statistics_total() {
        let stats = Statistics {
            added: 5,
            modified: 3,
            deleted: 2,
            renamed: 1,
            copied: 0,
            type_changed: 0,
            unchanged: 10,
            symlinks: 1,
            submodules: 0,
            permission_changes: 1,
            errors: 0,
            copy_failed: 0,
        };
        assert_eq!(stats.total(), 21);
    }

    // ThreeWayStatistics のテスト
    #[test]
    fn test_three_way_statistics_total() {
        let stats = ThreeWayStatistics {
            unchanged: 10,
            ours_only: 5,
            theirs_only: 3,
            both_same: 2,
            conflict: 1,
            added_ours: 2,
            added_theirs: 1,
            added_both_same: 1,
            added_both_diff: 1,
            deleted_ours: 1,
            deleted_theirs: 1,
            deleted_both: 1,
            modify_delete: 0,
            delete_modify: 0,
        };
        assert_eq!(stats.total(), 29);
    }

    #[test]
    fn test_three_way_statistics_conflicts() {
        let stats = ThreeWayStatistics {
            unchanged: 10,
            ours_only: 5,
            theirs_only: 3,
            both_same: 2,
            conflict: 3,
            added_ours: 2,
            added_theirs: 1,
            added_both_same: 1,
            added_both_diff: 2,
            deleted_ours: 1,
            deleted_theirs: 1,
            deleted_both: 1,
            modify_delete: 1,
            delete_modify: 1,
        };
        // conflict + added_both_diff + modify_delete + delete_modify = 3 + 2 + 1 + 1 = 7
        assert_eq!(stats.conflicts(), 7);
    }

    // display_width のテスト
    #[test]
    fn test_display_width_ascii() {
        // ASCII文字は幅1
        assert_eq!(display_width("abc"), 3);
        assert_eq!(display_width("hello world"), 11);
    }

    #[test]
    fn test_display_width_japanese() {
        // 日本語文字は幅2
        assert_eq!(display_width("日本語"), 6);
        assert_eq!(display_width("ファイル"), 8);
    }

    #[test]
    fn test_display_width_box_drawing() {
        // Box Drawing文字(U+2500-U+257F)は幅2として計算される
        // ├ = U+251C, └ = U+2514, │ = U+2502, ─ = U+2500
        assert_eq!(display_width("├"), 2);
        assert_eq!(display_width("└"), 2);
        assert_eq!(display_width("│"), 2);
        assert_eq!(display_width("─"), 2);
        assert_eq!(display_width("├──"), 6); // 3文字 x 2 = 6
        // │(2) + space(1) + space(1) + space(1) + ├(2) + ─(2) + ─(2) = 11
        assert_eq!(display_width("│   ├──"), 11);
    }

    #[test]
    fn test_display_width_mixed() {
        // ASCII + Box Drawing
        assert_eq!(display_width("├── file.txt"), 2 + 2 + 2 + 1 + 8); // ├(2) + ─(2) + ─(2) + space(1) + "file.txt"(8) = 15
        // 日本語 + Box Drawing
        assert_eq!(display_width("├── 日本語.txt"), 2 + 2 + 2 + 1 + 6 + 4); // ├── + space + 日本語(6) + .txt(4) = 17
    }

    #[test]
    fn test_display_width_empty() {
        assert_eq!(display_width(""), 0);
    }

    #[test]
    fn test_display_width_tree_prefix() {
        // 実際のツリー表示で使われるプレフィックス
        // "│   " = │(2) + space(1) + space(1) + space(1) = 5
        assert_eq!(display_width("│   "), 5);
        // "    " = 4 spaces = 4
        assert_eq!(display_width("    "), 4);
        // "├── " = ├(2) + ─(2) + ─(2) + space(1) = 7
        assert_eq!(display_width("├── "), 7);
        // "└── " = └(2) + ─(2) + ─(2) + space(1) = 7
        assert_eq!(display_width("└── "), 7);
    }
}
