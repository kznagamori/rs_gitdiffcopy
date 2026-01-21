//! Excel出力

use crate::config::MergedConfig;
use crate::error::Result;
use crate::types::{
    DiffFile, FileStatus, Statistics, ThreeWayDiffFile, ThreeWayStatistics, ThreeWayStatus,
};
use chrono::Local;
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook, Worksheet};
use std::path::Path;

/// ファイルツリーのエントリ
struct FileTreeEntry<S> {
    component: String,
    depth: usize,
    is_directory: bool,
    status: Option<S>,
    children_count: usize,
}

/// Excel出力
pub struct ExcelWriter<'a> {
    config: &'a MergedConfig,
}

impl<'a> ExcelWriter<'a> {
    pub fn new(config: &'a MergedConfig) -> Self {
        Self { config }
    }

    /// 二者間比較のExcelレポートを出力
    pub fn write_two_way(
        &self,
        files: &[DiffFile],
        stats: &Statistics,
        source_ref: &str,
        source_commit: &str,
        target_ref: &str,
        target_commit: &str,
        repo_path: &Path,
        output_path: &Path,
    ) -> Result<()> {
        let mut workbook = Workbook::new();

        // Summary シート
        self.write_summary_sheet(
            &mut workbook,
            stats,
            source_ref,
            source_commit,
            target_ref,
            target_commit,
            repo_path,
        )?;

        // File Tree シート
        self.write_file_tree_sheet(&mut workbook, files)?;

        // Details シート
        self.write_details_sheet(&mut workbook, files)?;

        // ファイルを保存
        workbook
            .save(output_path)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// 三者間比較のExcelレポートを出力
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
        output_path: &Path,
    ) -> Result<()> {
        let mut workbook = Workbook::new();

        // Summary シート
        self.write_three_way_summary_sheet(
            &mut workbook,
            stats,
            base_ref,
            base_commit,
            ours_ref,
            ours_commit,
            theirs_ref,
            theirs_commit,
            repo_path,
        )?;

        // File Tree シート
        self.write_three_way_file_tree_sheet(&mut workbook, files)?;

        // Conflicts シート
        self.write_conflicts_sheet(&mut workbook, files)?;

        // Copied Files シート
        self.write_copied_files_sheet(&mut workbook, files)?;

        // ファイルを保存
        workbook
            .save(output_path)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// Summaryシートを作成
    fn write_summary_sheet(
        &self,
        workbook: &mut Workbook,
        stats: &Statistics,
        source_ref: &str,
        source_commit: &str,
        target_ref: &str,
        target_commit: &str,
        repo_path: &Path,
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("Summary")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // フォーマット定義
        let title_format = Format::new()
            .set_bold()
            .set_font_size(16)
            .set_font_color(Color::RGB(0x4472C4));

        let header_format = Format::new()
            .set_bold()
            .set_font_size(12)
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        let label_format = Format::new()
            .set_bold()
            .set_border(FormatBorder::Thin);

        let cell_format = Format::new()
            .set_border(FormatBorder::Thin);

        let mut row = 0u32;

        // タイトル
        worksheet
            .write_string_with_format(row, 0, "rs_gitdiffcopy Summary", &title_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // 基本情報（罫線付き）
        worksheet
            .write_string_with_format(row, 0, "Repository:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, repo_path.display().to_string(), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Source ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, format!("{} ({})", source_ref, &source_commit[..7.min(source_commit.len())]), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Target ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, format!("{} ({})", target_ref, &target_commit[..7.min(target_commit.len())]), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Output:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, self.config.output.display().to_string(), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Date:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // Options セクション
        worksheet
            .write_string_with_format(row, 0, "Options", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, "", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        // filter_status
        if !self.config.filter_status.is_empty() {
            worksheet
                .write_string_with_format(row, 0, "Filter status:", &label_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, self.config.filter_status.join(", "), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        // exclude patterns
        if !self.config.exclude.is_empty() {
            worksheet
                .write_string_with_format(row, 0, "Exclude:", &label_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, self.config.exclude.join(", "), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        // オプションがない場合は "(none)" を表示
        if self.config.filter_status.is_empty() && self.config.exclude.is_empty() {
            worksheet
                .write_string_with_format(row, 0, "(none)", &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, "", &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }
        row += 1;

        // Statistics セクション
        worksheet
            .write_string_with_format(row, 0, "Statistics", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, "", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        let added_format = Format::new()
            .set_font_color(Color::RGB(0x008000))
            .set_border(FormatBorder::Thin);
        let modified_format = Format::new()
            .set_font_color(Color::RGB(0x0066CC))
            .set_border(FormatBorder::Thin);
        let deleted_format = Format::new()
            .set_font_color(Color::RGB(0xCC0000))
            .set_border(FormatBorder::Thin);
        let renamed_format = Format::new()
            .set_font_color(Color::RGB(0xFF6600))
            .set_border(FormatBorder::Thin);
        let number_format = Format::new()
            .set_border(FormatBorder::Thin);

        worksheet
            .write_string_with_format(row, 0, "Added:", &added_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.added as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Modified:", &modified_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.modified as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Deleted:", &deleted_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.deleted as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Renamed:", &renamed_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.renamed as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Unchanged:", &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.unchanged as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Total:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.total() as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // 列幅を調整
        worksheet
            .set_column_width(0, 20)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(1, 50)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// File Treeシートを作成（ディレクトリ分割形式）
    fn write_file_tree_sheet(&self, workbook: &mut Workbook, files: &[DiffFile]) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("File Tree")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // ファイルをソート
        let mut sorted_files: Vec<_> = files.iter().collect();
        sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

        // パスコンポーネントを分解してエントリを作成
        let entries = Self::build_file_tree_entries(&sorted_files);

        // 最大深度を計算（列数の決定）
        let max_depth = entries.iter().map(|e| e.depth).max().unwrap_or(0);
        let status_col = max_depth as u16;

        // ヘッダー（罫線付き）
        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        // A1に "Path"、他は空
        worksheet
            .write_string_with_format(0, 0, "Path", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        for col in 1..status_col {
            worksheet
                .write_string_with_format(0, col, "", &header_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }
        worksheet
            .write_string_with_format(0, status_col, "Status", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // データセル用フォーマット（罫線付き）
        let cell_format = Format::new()
            .set_font_name("Consolas")
            .set_border(FormatBorder::Thin);

        let mut row = 1u32;
        let mut row_depths: Vec<usize> = Vec::new();

        for entry in &entries {
            // 深度情報を記録（グルーピング用）
            row_depths.push(entry.depth);

            // 各列に値を書き込み
            for col in 0..=status_col {
                if col == entry.depth as u16 {
                    // このコンポーネントの列
                    worksheet
                        .write_string_with_format(row, col, &entry.component, &cell_format)
                        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
                } else if col == status_col && entry.status.is_some() {
                    // ステータス列（ファイルのみ）
                    let status = entry.status.unwrap();
                    let status_str = match status {
                        FileStatus::Added => "added",
                        FileStatus::Modified => "modified",
                        FileStatus::Deleted => "deleted",
                        FileStatus::Renamed => "renamed",
                        FileStatus::Copied => "copied",
                        FileStatus::TypeChanged => "type-changed",
                        FileStatus::Unchanged => "unchanged",
                    };
                    let status_format = self.get_status_format_with_border(status);
                    worksheet
                        .write_string_with_format(row, col, status_str, &status_format)
                        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
                } else {
                    // 空セル
                    worksheet
                        .write_string_with_format(row, col, "", &cell_format)
                        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
                }
            }

            row += 1;
        }

        // グルーピングレベルを設定
        if let Some(fold_level) = self.config.excel_fold_level {
            Self::apply_row_grouping(worksheet, &row_depths, fold_level as usize);
        }

        // 列幅を調整
        for col in 0..status_col {
            worksheet
                .set_column_width(col, 15)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }
        worksheet
            .set_column_width(status_col, 15)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// ファイルツリーエントリを構築
    fn build_file_tree_entries(files: &[&DiffFile]) -> Vec<FileTreeEntry<FileStatus>> {
        let mut entries = Vec::new();
        let mut seen_dirs: std::collections::HashSet<String> = std::collections::HashSet::new();

        for file in files {
            let components: Vec<_> = file.path.components().collect();
            let mut current_path = String::new();

            // ディレクトリエントリを追加
            for (i, comp) in components.iter().enumerate() {
                let comp_str = comp.as_os_str().to_string_lossy().to_string();
                let is_file = i == components.len() - 1;

                if !current_path.is_empty() {
                    current_path.push('/');
                }
                current_path.push_str(&comp_str);

                if is_file {
                    // ファイルエントリ
                    entries.push(FileTreeEntry {
                        component: comp_str,
                        depth: i,
                        is_directory: false,
                        status: Some(file.status),
                        children_count: 0,
                    });
                } else if !seen_dirs.contains(&current_path) {
                    // ディレクトリエントリ（新規）
                    seen_dirs.insert(current_path.clone());
                    entries.push(FileTreeEntry {
                        component: format!("{}/", comp_str),
                        depth: i,
                        is_directory: true,
                        status: None,
                        children_count: 0,
                    });
                }
            }
        }

        // 子の数を計算
        Self::calculate_children_count(&mut entries);

        entries
    }

    /// 各ディレクトリの子の数を計算
    fn calculate_children_count<S: Copy>(entries: &mut [FileTreeEntry<S>]) {
        for i in 0..entries.len() {
            if entries[i].is_directory {
                let dir_depth = entries[i].depth;
                let mut count = 0;
                for j in (i + 1)..entries.len() {
                    if entries[j].depth <= dir_depth {
                        break;
                    }
                    count += 1;
                }
                entries[i].children_count = count;
            }
        }
    }

    /// Detailsシートを作成
    fn write_details_sheet(&self, workbook: &mut Workbook, files: &[DiffFile]) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("Details")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // ヘッダー（罫線付き）
        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        // データセル用フォーマット（罫線付き）
        let cell_format = Format::new().set_border(FormatBorder::Thin);

        let headers = ["Status", "Directory", "File", "Details"];
        for (col, header) in headers.iter().enumerate() {
            worksheet
                .write_string_with_format(0, col as u16, *header, &header_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }

        let mut row = 1u32;

        for file in files {
            let status_str = match file.status {
                FileStatus::Added => "Added",
                FileStatus::Modified => "Modified",
                FileStatus::Deleted => "Deleted",
                FileStatus::Renamed => "Renamed",
                FileStatus::Copied => "Copied",
                FileStatus::TypeChanged => "Type Changed",
                FileStatus::Unchanged => "Unchanged",
            };

            let (dir, filename) = if let Some(parent) = file.path.parent() {
                (
                    parent.display().to_string(),
                    file.path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default(),
                )
            } else {
                (String::new(), file.path.display().to_string())
            };

            let details = match file.status {
                FileStatus::Renamed | FileStatus::Copied => {
                    if let Some(ref orig) = file.original_path {
                        let sim = file.similarity.map(|s| format!(" ({}% similar)", s)).unwrap_or_default();
                        format!("from: {}{}", orig.display(), sim)
                    } else {
                        String::new()
                    }
                }
                _ => String::new(),
            };

            let status_format = self.get_status_format_with_border(file.status);
            worksheet
                .write_string_with_format(row, 0, status_str, &status_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, dir, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 2, filename, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 3, details, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

            row += 1;
        }

        // 列幅を調整
        worksheet
            .set_column_width(0, 12)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(1, 40)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(2, 30)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(3, 50)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// ステータスに応じたフォーマットを取得（罫線付き）
    fn get_status_format_with_border(&self, status: FileStatus) -> Format {
        match status {
            FileStatus::Added => Format::new().set_font_color(Color::RGB(0x008000)).set_border(FormatBorder::Thin),
            FileStatus::Modified => Format::new().set_font_color(Color::RGB(0x0066CC)).set_border(FormatBorder::Thin),
            FileStatus::Deleted => Format::new().set_font_color(Color::RGB(0xCC0000)).set_border(FormatBorder::Thin),
            FileStatus::Renamed => Format::new().set_font_color(Color::RGB(0xFF6600)).set_border(FormatBorder::Thin),
            FileStatus::Copied => Format::new().set_font_color(Color::RGB(0x00CCCC)).set_border(FormatBorder::Thin),
            FileStatus::TypeChanged => Format::new().set_font_color(Color::RGB(0x9933FF)).set_border(FormatBorder::Thin),
            FileStatus::Unchanged => Format::new().set_font_color(Color::RGB(0x808080)).set_border(FormatBorder::Thin),
        }
    }

    /// 三者間Summaryシートを作成
    fn write_three_way_summary_sheet(
        &self,
        workbook: &mut Workbook,
        stats: &ThreeWayStatistics,
        base_ref: &str,
        base_commit: &str,
        ours_ref: &str,
        ours_commit: &str,
        theirs_ref: &str,
        theirs_commit: &str,
        repo_path: &Path,
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("Summary")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        let title_format = Format::new()
            .set_bold()
            .set_font_size(16)
            .set_font_color(Color::RGB(0x4472C4));

        let header_format = Format::new()
            .set_bold()
            .set_font_size(12)
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        let label_format = Format::new()
            .set_bold()
            .set_border(FormatBorder::Thin);

        let cell_format = Format::new()
            .set_border(FormatBorder::Thin);

        let number_format = Format::new()
            .set_border(FormatBorder::Thin);

        let mut row = 0u32;

        // タイトル
        worksheet
            .write_string_with_format(row, 0, "rs_gitdiffcopy Summary (Three-way)", &title_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // 基本情報（罫線付き）
        worksheet
            .write_string_with_format(row, 0, "Repository:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, repo_path.display().to_string(), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Base ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, format!("{} ({})", base_ref, &base_commit[..7.min(base_commit.len())]), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Ours ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, format!("{} ({})", ours_ref, &ours_commit[..7.min(ours_commit.len())]), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Theirs ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, format!("{} ({})", theirs_ref, &theirs_commit[..7.min(theirs_commit.len())]), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Output:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, self.config.output.display().to_string(), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Date:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &cell_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // Options セクション
        worksheet
            .write_string_with_format(row, 0, "Options", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, "", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        // filter_status
        if !self.config.filter_status.is_empty() {
            worksheet
                .write_string_with_format(row, 0, "Filter status:", &label_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, self.config.filter_status.join(", "), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        // exclude patterns
        if !self.config.exclude.is_empty() {
            worksheet
                .write_string_with_format(row, 0, "Exclude:", &label_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, self.config.exclude.join(", "), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        // conflict_only
        if self.config.conflict_only {
            worksheet
                .write_string_with_format(row, 0, "Conflict only:", &label_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, "true", &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        // オプションがない場合は "(none)" を表示
        if self.config.filter_status.is_empty() && self.config.exclude.is_empty() && !self.config.conflict_only {
            worksheet
                .write_string_with_format(row, 0, "(none)", &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, "", &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }
        row += 1;

        // Statistics セクション
        worksheet
            .write_string_with_format(row, 0, "Statistics", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, "", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        let conflict_format = Format::new()
            .set_bold()
            .set_font_color(Color::RGB(0xCC0000))
            .set_border(FormatBorder::Thin);

        let stats_items: Vec<(&str, usize)> = vec![
            ("Unchanged:", stats.unchanged),
            ("Ours only:", stats.ours_only),
            ("Theirs only:", stats.theirs_only),
            ("Both same:", stats.both_same),
            ("Conflict:", stats.conflict),
            ("Added (ours):", stats.added_ours),
            ("Added (theirs):", stats.added_theirs),
            ("Deleted (ours):", stats.deleted_ours),
            ("Deleted (theirs):", stats.deleted_theirs),
        ];

        for (label, value) in stats_items {
            worksheet
                .write_string_with_format(row, 0, label, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_number_with_format(row, 1, value as f64, &number_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        worksheet
            .write_string_with_format(row, 0, "Total:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.total() as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Total Conflicts:", &conflict_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number_with_format(row, 1, stats.conflicts() as f64, &number_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // 列幅を調整
        worksheet
            .set_column_width(0, 20)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(1, 50)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// 三者間File Treeシートを作成（ディレクトリ分割形式）
    fn write_three_way_file_tree_sheet(
        &self,
        workbook: &mut Workbook,
        files: &[ThreeWayDiffFile],
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("File Tree")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // ファイルをソート
        let mut sorted_files: Vec<_> = files.iter().collect();
        sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

        // パスコンポーネントを分解してエントリを作成
        let entries = Self::build_three_way_file_tree_entries(&sorted_files);

        // 最大深度を計算（列数の決定）
        let max_depth = entries.iter().map(|e| e.depth).max().unwrap_or(0);
        let status_col = max_depth as u16;

        // ヘッダー（罫線付き）
        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        // A1に "Path"、他は空
        worksheet
            .write_string_with_format(0, 0, "Path", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        for col in 1..status_col {
            worksheet
                .write_string_with_format(0, col, "", &header_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }
        worksheet
            .write_string_with_format(0, status_col, "Status", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // データセル用フォーマット（罫線付き）
        let cell_format = Format::new()
            .set_font_name("Consolas")
            .set_border(FormatBorder::Thin);

        let mut row = 1u32;
        let mut row_depths: Vec<usize> = Vec::new();

        for entry in &entries {
            // 深度情報を記録（グルーピング用）
            row_depths.push(entry.depth);

            // 各列に値を書き込み
            for col in 0..=status_col {
                if col == entry.depth as u16 {
                    // このコンポーネントの列
                    worksheet
                        .write_string_with_format(row, col, &entry.component, &cell_format)
                        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
                } else if col == status_col && entry.status.is_some() {
                    // ステータス列（ファイルのみ）
                    let status = entry.status.unwrap();
                    let status_str = status.tag();
                    let status_format = self.get_three_way_status_format_with_border(status);
                    worksheet
                        .write_string_with_format(row, col, status_str, &status_format)
                        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
                } else {
                    // 空セル
                    worksheet
                        .write_string_with_format(row, col, "", &cell_format)
                        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
                }
            }

            row += 1;
        }

        // グルーピングレベルを設定
        if let Some(fold_level) = self.config.excel_fold_level {
            Self::apply_row_grouping(worksheet, &row_depths, fold_level as usize);
        }

        // 列幅を調整
        for col in 0..status_col {
            worksheet
                .set_column_width(col, 15)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }
        worksheet
            .set_column_width(status_col, 20)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// 三者間ファイルツリーエントリを構築
    fn build_three_way_file_tree_entries(files: &[&ThreeWayDiffFile]) -> Vec<FileTreeEntry<ThreeWayStatus>> {
        let mut entries = Vec::new();
        let mut seen_dirs: std::collections::HashSet<String> = std::collections::HashSet::new();

        for file in files {
            let components: Vec<_> = file.path.components().collect();
            let mut current_path = String::new();

            // ディレクトリエントリを追加
            for (i, comp) in components.iter().enumerate() {
                let comp_str = comp.as_os_str().to_string_lossy().to_string();
                let is_file = i == components.len() - 1;

                if !current_path.is_empty() {
                    current_path.push('/');
                }
                current_path.push_str(&comp_str);

                if is_file {
                    // ファイルエントリ
                    entries.push(FileTreeEntry {
                        component: comp_str,
                        depth: i,
                        is_directory: false,
                        status: Some(file.status),
                        children_count: 0,
                    });
                } else if !seen_dirs.contains(&current_path) {
                    // ディレクトリエントリ（新規）
                    seen_dirs.insert(current_path.clone());
                    entries.push(FileTreeEntry {
                        component: format!("{}/", comp_str),
                        depth: i,
                        is_directory: true,
                        status: None,
                        children_count: 0,
                    });
                }
            }
        }

        // 子の数を計算
        Self::calculate_children_count(&mut entries);

        entries
    }

    /// 三者間ステータスに応じたフォーマットを取得（罫線付き）
    fn get_three_way_status_format_with_border(&self, status: ThreeWayStatus) -> Format {
        match status {
            ThreeWayStatus::Unchanged => Format::new().set_font_color(Color::RGB(0x808080)).set_border(FormatBorder::Thin),
            ThreeWayStatus::OursOnly => Format::new().set_font_color(Color::RGB(0x0066CC)).set_border(FormatBorder::Thin),
            ThreeWayStatus::TheirsOnly => Format::new().set_font_color(Color::RGB(0xFF6600)).set_border(FormatBorder::Thin),
            ThreeWayStatus::BothSame => Format::new().set_font_color(Color::RGB(0x008000)).set_border(FormatBorder::Thin),
            ThreeWayStatus::Conflict => Format::new().set_font_color(Color::RGB(0xCC0000)).set_bold().set_border(FormatBorder::Thin),
            ThreeWayStatus::AddedOurs => Format::new().set_font_color(Color::RGB(0x0066CC)).set_border(FormatBorder::Thin),
            ThreeWayStatus::AddedTheirs => Format::new().set_font_color(Color::RGB(0xFF6600)).set_border(FormatBorder::Thin),
            ThreeWayStatus::AddedBothSame => Format::new().set_font_color(Color::RGB(0x008000)).set_border(FormatBorder::Thin),
            ThreeWayStatus::AddedBothDiff => Format::new().set_font_color(Color::RGB(0xCC0000)).set_bold().set_border(FormatBorder::Thin),
            ThreeWayStatus::DeletedOurs => Format::new().set_font_color(Color::RGB(0x0066CC)).set_border(FormatBorder::Thin),
            ThreeWayStatus::DeletedTheirs => Format::new().set_font_color(Color::RGB(0xFF6600)).set_border(FormatBorder::Thin),
            ThreeWayStatus::DeletedBoth => Format::new().set_font_color(Color::RGB(0x808080)).set_border(FormatBorder::Thin),
            ThreeWayStatus::ModifyDelete => Format::new().set_font_color(Color::RGB(0xCC0000)).set_bold().set_border(FormatBorder::Thin),
            ThreeWayStatus::DeleteModify => Format::new().set_font_color(Color::RGB(0xCC0000)).set_bold().set_border(FormatBorder::Thin),
        }
    }

    /// Conflictsシートを作成
    fn write_conflicts_sheet(
        &self,
        workbook: &mut Workbook,
        files: &[ThreeWayDiffFile],
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("Conflicts")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        let cell_format = Format::new().set_border(FormatBorder::Thin);

        let headers = ["Directory", "Filename", "Type", "Base Hash", "Ours Hash", "Theirs Hash"];

        for (col, header) in headers.iter().enumerate() {
            worksheet
                .write_string_with_format(0, col as u16, *header, &header_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }

        let conflicts: Vec<_> = files.iter().filter(|f| f.status.is_conflict()).collect();

        let mut row = 1u32;
        for file in conflicts {
            let (dir, filename) = if let Some(parent) = file.path.parent() {
                (
                    parent.display().to_string(),
                    file.path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default(),
                )
            } else {
                (String::new(), file.path.display().to_string())
            };

            worksheet
                .write_string_with_format(row, 0, dir, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, filename, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 2, file.status.tag(), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

            let base_hash = file.base_blob.as_ref().map(|b| &b[..8.min(b.len())]).unwrap_or("");
            let ours_hash = file.ours_blob.as_ref().map(|b| &b[..8.min(b.len())]).unwrap_or("");
            let theirs_hash = file.theirs_blob.as_ref().map(|b| &b[..8.min(b.len())]).unwrap_or("");

            worksheet
                .write_string_with_format(row, 3, base_hash, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 4, ours_hash, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 5, theirs_hash, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

            row += 1;
        }

        for col in 0..6 {
            worksheet
                .set_column_width(col, 20)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }

        Ok(())
    }

    /// Copied Filesシートを作成
    fn write_copied_files_sheet(
        &self,
        workbook: &mut Workbook,
        files: &[ThreeWayDiffFile],
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("Copied Files")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        let cell_format = Format::new().set_border(FormatBorder::Thin);

        let headers = ["Directory", "Filename", "Status", "Source"];

        for (col, header) in headers.iter().enumerate() {
            worksheet
                .write_string_with_format(0, col as u16, *header, &header_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        }

        // コピー対象ファイル（unchangedとdeleted以外）
        let copied: Vec<_> = files
            .iter()
            .filter(|f| !matches!(f.status, ThreeWayStatus::Unchanged | ThreeWayStatus::DeletedOurs | ThreeWayStatus::DeletedTheirs | ThreeWayStatus::DeletedBoth))
            .collect();

        let mut row = 1u32;
        for file in copied {
            let (dir, filename) = if let Some(parent) = file.path.parent() {
                (
                    parent.display().to_string(),
                    file.path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default(),
                )
            } else {
                (String::new(), file.path.display().to_string())
            };

            let source = match file.status {
                ThreeWayStatus::OursOnly | ThreeWayStatus::AddedOurs => "ours",
                ThreeWayStatus::TheirsOnly | ThreeWayStatus::AddedTheirs => "theirs",
                ThreeWayStatus::BothSame | ThreeWayStatus::AddedBothSame => "ours (same)",
                _ if file.status.is_conflict() => "conflict",
                _ => "",
            };

            worksheet
                .write_string_with_format(row, 0, dir, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, filename, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 2, file.status.tag(), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 3, source, &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

            row += 1;
        }

        worksheet
            .set_column_width(0, 40)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(1, 30)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(2, 15)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(3, 15)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
    }

    /// 行グルーピングを適用
    /// depth >= fold_level の連続する行をグループ化
    fn apply_row_grouping(worksheet: &mut Worksheet, row_depths: &[usize], fold_level: usize) {
        if row_depths.is_empty() {
            return;
        }

        // Group consecutive rows where depth >= fold_level
        let mut groups: Vec<(u32, u32)> = Vec::new();
        let mut current_group_start: Option<u32> = None;

        for (idx, depth) in row_depths.iter().enumerate() {
            let row = (idx + 1) as u32; // +1 for header row

            if *depth >= fold_level {
                if current_group_start.is_none() {
                    current_group_start = Some(row);
                }
            } else {
                if let Some(start) = current_group_start {
                    if row > start {
                        groups.push((start, row - 1));
                    }
                }
                current_group_start = None;
            }
        }

        // Handle last group
        if let Some(start) = current_group_start {
            let last_row = row_depths.len() as u32;
            if last_row >= start {
                groups.push((start, last_row));
            }
        }

        // Apply grouping
        for (start, end) in groups {
            worksheet.group_rows(start, end).ok();
        }
    }

    /// グルーピングの範囲を計算（テスト用）
    #[cfg(test)]
    fn calculate_row_groups(row_depths: &[usize], fold_level: usize) -> Vec<(u32, u32)> {
        if row_depths.is_empty() {
            return Vec::new();
        }

        let mut groups: Vec<(u32, u32)> = Vec::new();
        let mut current_group_start: Option<u32> = None;

        for (idx, depth) in row_depths.iter().enumerate() {
            let row = (idx + 1) as u32; // +1 for header row

            if *depth >= fold_level {
                if current_group_start.is_none() {
                    current_group_start = Some(row);
                }
            } else {
                if let Some(start) = current_group_start {
                    if row > start {
                        groups.push((start, row - 1));
                    }
                }
                current_group_start = None;
            }
        }

        // Handle last group
        if let Some(start) = current_group_start {
            let last_row = row_depths.len() as u32;
            if last_row >= start {
                groups.push((start, last_row));
            }
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_row_groups_empty() {
        let groups = ExcelWriter::calculate_row_groups(&[], 2);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_calculate_row_groups_no_deep_entries() {
        // 全てのエントリがfold_level未満の場合、グループなし
        let depths = vec![0, 1, 0, 1];
        let groups = ExcelWriter::calculate_row_groups(&depths, 2);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_calculate_row_groups_all_deep() {
        // 全てのエントリがfold_level以上の場合、1つの大きなグループ
        let depths = vec![2, 3, 2, 3];
        let groups = ExcelWriter::calculate_row_groups(&depths, 2);
        // row 1-4 (depth 2,3,2,3 all >= 2)
        assert_eq!(groups, vec![(1, 4)]);
    }

    #[test]
    fn test_calculate_row_groups_fold_level_2() {
        // fold_level=2の場合、depth >= 2の連続行がグループ化
        let depths = vec![
            1, // depth 1 - not grouped
            1, // depth 1 - not grouped
            2, // depth 2 - grouped (start)
            3, // depth 3 - grouped
            2, // depth 2 - grouped (end)
            1, // depth 1 - not grouped (breaks group)
            2, // depth 2 - grouped (new start)
            3, // depth 3 - grouped (end)
        ];
        let groups = ExcelWriter::calculate_row_groups(&depths, 2);
        // rows 3-5 (depths 2,3,2) and rows 7-8 (depths 2,3)
        assert_eq!(groups, vec![(3, 5), (7, 8)]);
    }

    #[test]
    fn test_calculate_row_groups_fold_level_3() {
        // fold_level=3の場合、depth >= 3の連続行のみがグループ化
        let depths = vec![
            1, // depth 1 - not grouped
            2, // depth 2 - not grouped (< 3)
            3, // depth 3 - grouped (start)
            4, // depth 4 - grouped
            3, // depth 3 - grouped (end)
            2, // depth 2 - not grouped (breaks group)
            3, // depth 3 - grouped (new single row)
        ];
        let groups = ExcelWriter::calculate_row_groups(&depths, 3);
        // rows 3-5 (depths 3,4,3) and row 7 (depth 3)
        assert_eq!(groups, vec![(3, 5), (7, 7)]);
    }

    #[test]
    fn test_calculate_row_groups_single_deep_entry() {
        // 単一の深いエントリ
        let depths = vec![1, 3, 1];
        let groups = ExcelWriter::calculate_row_groups(&depths, 2);
        // row 2 only (depth 3 >= 2)
        assert_eq!(groups, vec![(2, 2)]);
    }

    #[test]
    fn test_calculate_row_groups_ends_with_deep() {
        // 最後がグループで終わる場合
        let depths = vec![1, 2, 3, 4];
        let groups = ExcelWriter::calculate_row_groups(&depths, 2);
        // rows 2-4 (depths 2,3,4 all >= 2)
        assert_eq!(groups, vec![(2, 4)]);
    }

    #[test]
    fn test_calculate_row_groups_fold_level_1() {
        // fold_level=1の場合、depth >= 1の全てがグループ化
        let depths = vec![0, 1, 2, 0, 1];
        let groups = ExcelWriter::calculate_row_groups(&depths, 1);
        // rows 2-3 (depths 1,2) and row 5 (depth 1)
        assert_eq!(groups, vec![(2, 3), (5, 5)]);
    }
}
