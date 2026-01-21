//! Excel出力

use crate::config::MergedConfig;
use crate::error::Result;
use crate::types::{
    DiffFile, FileStatus, Statistics, ThreeWayDiffFile, ThreeWayStatistics, ThreeWayStatus,
};
use chrono::Local;
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook};
use std::path::Path;

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
            .set_font_color(Color::White);

        let label_format = Format::new().set_bold();

        let mut row = 0u32;

        // タイトル
        worksheet
            .write_string_with_format(row, 0, "rs_gitdiffcopy Summary", &title_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // 基本情報
        worksheet
            .write_string_with_format(row, 0, "Repository:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, repo_path.display().to_string())
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Source ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, format!("{} ({})", source_ref, &source_commit[..7.min(source_commit.len())]))
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Target ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, format!("{} ({})", target_ref, &target_commit[..7.min(target_commit.len())]))
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Output:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, self.config.output.display().to_string())
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Date:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // Statistics セクション
        worksheet
            .write_string_with_format(row, 0, "Statistics", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, "", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        let added_format = Format::new().set_font_color(Color::RGB(0x008000));
        let modified_format = Format::new().set_font_color(Color::RGB(0x0066CC));
        let deleted_format = Format::new().set_font_color(Color::RGB(0xCC0000));
        let renamed_format = Format::new().set_font_color(Color::RGB(0xFF6600));

        worksheet
            .write_string_with_format(row, 0, "Added:", &added_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.added as f64)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Modified:", &modified_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.modified as f64)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Deleted:", &deleted_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.deleted as f64)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Renamed:", &renamed_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.renamed as f64)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string(row, 0, "Unchanged:")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.unchanged as f64)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Total:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.total() as f64)
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

    /// File Treeシートを作成
    fn write_file_tree_sheet(&self, workbook: &mut Workbook, files: &[DiffFile]) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("File Tree")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // ヘッダー（罫線付き）
        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        // データセル用フォーマット（罫線付き）
        let mono_format = Format::new()
            .set_font_name("Consolas")
            .set_border(FormatBorder::Thin);

        worksheet
            .write_string_with_format(0, 0, "Path", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(0, 1, "Status", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // ファイルをソート
        let mut sorted_files: Vec<_> = files.iter().collect();
        sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

        let mut row = 1u32;

        for file in sorted_files {
            worksheet
                .write_string_with_format(row, 0, file.path.display().to_string(), &mono_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

            let status_str = match file.status {
                FileStatus::Added => "added",
                FileStatus::Modified => "modified",
                FileStatus::Deleted => "deleted",
                FileStatus::Renamed => "renamed",
                FileStatus::Copied => "copied",
                FileStatus::TypeChanged => "type-changed",
                FileStatus::Unchanged => "unchanged",
            };

            let status_format = self.get_status_format_with_border(file.status);
            worksheet
                .write_string_with_format(row, 1, status_str, &status_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

            row += 1;
        }

        // 列幅を調整
        worksheet
            .set_column_width(0, 60)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(1, 15)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
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

    /// ステータスに応じたフォーマットを取得
    fn get_status_format(&self, status: FileStatus) -> Format {
        match status {
            FileStatus::Added => Format::new().set_font_color(Color::RGB(0x008000)),
            FileStatus::Modified => Format::new().set_font_color(Color::RGB(0x0066CC)),
            FileStatus::Deleted => Format::new().set_font_color(Color::RGB(0xCC0000)),
            FileStatus::Renamed => Format::new().set_font_color(Color::RGB(0xFF6600)),
            FileStatus::Copied => Format::new().set_font_color(Color::RGB(0x00CCCC)),
            FileStatus::TypeChanged => Format::new().set_font_color(Color::RGB(0x9933FF)),
            FileStatus::Unchanged => Format::new().set_font_color(Color::RGB(0x808080)),
        }
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
            .set_font_color(Color::White);

        let label_format = Format::new().set_bold();

        let mut row = 0u32;

        // タイトル
        worksheet
            .write_string_with_format(row, 0, "rs_gitdiffcopy Summary (Three-way)", &title_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // 基本情報
        worksheet
            .write_string_with_format(row, 0, "Repository:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, repo_path.display().to_string())
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Base ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, format!("{} ({})", base_ref, &base_commit[..7.min(base_commit.len())]))
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Ours ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, format!("{} ({})", ours_ref, &ours_commit[..7.min(ours_commit.len())]))
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Theirs ref:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string(row, 1, format!("{} ({})", theirs_ref, &theirs_commit[..7.min(theirs_commit.len())]))
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 2;

        // Statistics
        worksheet
            .write_string_with_format(row, 0, "Statistics", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(row, 1, "", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        let conflict_format = Format::new()
            .set_bold()
            .set_font_color(Color::RGB(0xCC0000));

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
                .write_string(row, 0, label)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_number(row, 1, value as f64)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        worksheet
            .write_string_with_format(row, 0, "Total:", &label_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.total() as f64)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        row += 1;

        worksheet
            .write_string_with_format(row, 0, "Total Conflicts:", &conflict_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_number(row, 1, stats.conflicts() as f64)
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

    /// 三者間File Treeシートを作成
    fn write_three_way_file_tree_sheet(
        &self,
        workbook: &mut Workbook,
        files: &[ThreeWayDiffFile],
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name("File Tree")
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin);

        let cell_format = Format::new().set_border(FormatBorder::Thin);

        worksheet
            .write_string_with_format(0, 0, "Path", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .write_string_with_format(0, 1, "Status", &header_format)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        let mut row = 1u32;
        for file in files {
            worksheet
                .write_string_with_format(row, 0, file.path.display().to_string(), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            worksheet
                .write_string_with_format(row, 1, file.status.tag(), &cell_format)
                .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
            row += 1;
        }

        worksheet
            .set_column_width(0, 60)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
        worksheet
            .set_column_width(1, 20)
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        Ok(())
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
}
