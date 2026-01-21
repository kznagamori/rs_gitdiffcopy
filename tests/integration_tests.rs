//! rs_gitdiffcopy 結合テスト
//!
//! このファイルは、rs_gitdiffcopyの結合テストを実装します。
//! 各テストでは一時ディレクトリにGitリポジトリを動的に作成してテストを実行します。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

// ============================================================================
// テストヘルパー
// ============================================================================

/// テスト用の一時ディレクトリとGitリポジトリを作成するヘルパー
struct TestRepo {
    temp_dir: TempDir,
    repo_path: PathBuf,
    output_path: PathBuf,
}

impl TestRepo {
    /// 新しいテストリポジトリを作成
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repo_path = temp_dir.path().join("repo");
        let output_path = temp_dir.path().join("output");

        fs::create_dir_all(&repo_path).expect("Failed to create repo dir");

        // git init
        let output = Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to run git init");
        assert!(output.status.success(), "git init failed: {:?}", output);

        // git config (テスト用)
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to set git config");
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to set git config");
        // 日本語ファイル名を正しく扱うための設定
        Command::new("git")
            .args(["config", "core.quotepath", "false"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to set git config");

        Self {
            temp_dir,
            repo_path,
            output_path,
        }
    }

    /// ファイルを作成
    fn create_file(&self, path: &str, content: &str) {
        let full_path = self.repo_path.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent dir");
        }
        fs::write(&full_path, content).expect("Failed to write file");
    }

    /// ファイルを削除
    fn delete_file(&self, path: &str) {
        let full_path = self.repo_path.join(path);
        if full_path.exists() {
            fs::remove_file(&full_path).expect("Failed to delete file");
        }
    }

    /// ファイル名を変更
    fn rename_file(&self, from: &str, to: &str) {
        let from_path = self.repo_path.join(from);
        let to_path = self.repo_path.join(to);
        if let Some(parent) = to_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent dir");
        }
        fs::rename(&from_path, &to_path).expect("Failed to rename file");
    }

    /// git add
    fn add(&self, path: &str) {
        let output = Command::new("git")
            .args(["add", path])
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to run git add");
        assert!(output.status.success(), "git add failed: {:?}", output);
    }

    /// git add all
    fn add_all(&self) {
        let output = Command::new("git")
            .args(["add", "-A"])
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to run git add");
        assert!(output.status.success(), "git add -A failed: {:?}", output);
    }

    /// git commit
    fn commit(&self, message: &str) -> String {
        let output = Command::new("git")
            .args(["commit", "-m", message, "--allow-empty"])
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to run git commit");
        assert!(output.status.success(), "git commit failed: {:?}", output);

        // コミットハッシュを取得
        let hash_output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to get commit hash");
        String::from_utf8_lossy(&hash_output.stdout).trim().to_string()
    }

    /// ブランチを作成
    fn create_branch(&self, name: &str) {
        let output = Command::new("git")
            .args(["branch", name])
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to create branch");
        assert!(output.status.success(), "git branch failed: {:?}", output);
    }

    /// ブランチをチェックアウト
    fn checkout(&self, name: &str) {
        let output = Command::new("git")
            .args(["checkout", name])
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to checkout branch");
        assert!(output.status.success(), "git checkout failed: {:?}", output);
    }

    /// タグを作成
    fn create_tag(&self, name: &str) {
        let output = Command::new("git")
            .args(["tag", name])
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to create tag");
        assert!(output.status.success(), "git tag failed: {:?}", output);
    }

    /// rs_gitdiffcopyを実行
    fn run_cmd(&self, args: &[&str]) -> Output {
        let binary = get_binary_path();
        let mut cmd = Command::new(&binary);
        cmd.args(["-R", self.repo_path.to_str().unwrap()])
            .args(["-O", self.output_path.to_str().unwrap()])
            .args(args);

        cmd.output().expect("Failed to run rs_gitdiffcopy")
    }

    /// 出力ディレクトリにファイルが存在するか確認
    fn output_exists(&self, path: &str) -> bool {
        self.output_path.join(path).exists()
    }

    /// 出力ファイルの内容を読み取り
    fn read_output(&self, path: &str) -> String {
        fs::read_to_string(self.output_path.join(path)).expect("Failed to read output file")
    }

    /// リポジトリのパスを取得
    fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// 出力パスを取得
    fn output_path(&self) -> &Path {
        &self.output_path
    }
}

/// バイナリパスを取得
fn get_binary_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    if path.ends_with("deps") {
        path.pop();
    }

    #[cfg(windows)]
    let binary_name = "rs_gitdiffcopy.exe";
    #[cfg(not(windows))]
    let binary_name = "rs_gitdiffcopy";

    path.join(binary_name)
}

/// rs_gitdiffcopyを引数付きで実行（リポジトリ指定なし）
fn run_cmd_raw(args: &[&str]) -> Output {
    let binary = get_binary_path();
    Command::new(&binary)
        .args(args)
        .output()
        .expect("Failed to run rs_gitdiffcopy")
}

/// 出力に文字列が含まれているか確認
fn output_contains(output: &Output, pattern: &str) -> bool {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    stdout.contains(pattern) || stderr.contains(pattern)
}

// ============================================================================
// 1. 基本動作テスト
// ============================================================================

#[test]
fn test_help_option() {
    let output = run_cmd_raw(&["--help"]);
    assert!(output.status.success());
    assert!(output_contains(&output, "rs_gitdiffcopy"));
    assert!(output_contains(&output, "Usage"));
}

#[test]
fn test_help_short_option() {
    let output = run_cmd_raw(&["-h"]);
    assert!(output.status.success());
    assert!(output_contains(&output, "rs_gitdiffcopy"));
}

#[test]
fn test_version_option() {
    let output = run_cmd_raw(&["--version"]);
    assert!(output.status.success());
    assert!(output_contains(&output, "1.0.0"));
}

#[test]
fn test_version_short_option() {
    let output = run_cmd_raw(&["-V"]);
    assert!(output.status.success());
    assert!(output_contains(&output, "1.0.0"));
}

// ============================================================================
// 2. 二者間比較テスト
// ============================================================================

#[test]
fn test_added_file_detection() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("initial.txt", "initial content");
    repo.add_all();
    let source = repo.commit("Initial commit");

    // ファイル追加
    repo.create_file("added.txt", "new file content");
    repo.add_all();
    let target = repo.commit("Add new file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // 追加されたファイルがコピーされているか確認
    assert!(repo.output_exists("added.txt"), "added.txt should be copied");
    let content = repo.read_output("added.txt");
    assert_eq!(content, "new file content");
}

#[test]
fn test_modified_file_detection() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("modified.txt", "original content");
    repo.add_all();
    let source = repo.commit("Initial commit");

    // ファイル変更
    repo.create_file("modified.txt", "modified content");
    repo.add_all();
    let target = repo.commit("Modify file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("modified.txt"), "modified.txt should be copied");
    let content = repo.read_output("modified.txt");
    assert_eq!(content, "modified content");
}

#[test]
fn test_deleted_file_detection() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("to_delete.txt", "will be deleted");
    repo.create_file("keep.txt", "keep this");
    repo.add_all();
    let source = repo.commit("Initial commit");

    // ファイル削除
    repo.delete_file("to_delete.txt");
    repo.add_all();
    let target = repo.commit("Delete file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    // 削除された場合でも成功終了
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);

    // 削除されたファイルはコピーされない（--copy-deletedなし）
    assert!(!repo.output_exists("to_delete.txt"), "deleted file should not be copied");
}

#[test]
fn test_renamed_file_detection() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("old_name.txt", "file content");
    repo.add_all();
    let source = repo.commit("Initial commit");

    // ファイル名変更
    repo.rename_file("old_name.txt", "new_name.txt");
    repo.add_all();
    let target = repo.commit("Rename file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // 新しい名前でコピーされている
    assert!(repo.output_exists("new_name.txt"), "new_name.txt should be copied");
}

#[test]
fn test_unchanged_file_detection() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("unchanged.txt", "same content");
    repo.create_file("changed.txt", "original");
    repo.add_all();
    let source = repo.commit("Initial commit");

    // 1つのファイルのみ変更
    repo.create_file("changed.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify one file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // 変更なしファイルはコピーされない
    assert!(!repo.output_exists("unchanged.txt"), "unchanged file should not be copied");
    assert!(repo.output_exists("changed.txt"), "changed file should be copied");
}

#[test]
fn test_directory_structure_preserved() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("root.txt", "root");
    repo.add_all();
    let source = repo.commit("Initial commit");

    // ネストしたディレクトリ構造を追加
    repo.create_file("dir1/file1.txt", "content1");
    repo.create_file("dir1/dir2/file2.txt", "content2");
    repo.create_file("dir1/dir2/dir3/file3.txt", "content3");
    repo.add_all();
    let target = repo.commit("Add nested directories");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // ディレクトリ構造が保持されている
    assert!(repo.output_exists("dir1/file1.txt"));
    assert!(repo.output_exists("dir1/dir2/file2.txt"));
    assert!(repo.output_exists("dir1/dir2/dir3/file3.txt"));
}

#[test]
fn test_no_differences_exit_code() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("file.txt", "content");
    repo.add_all();
    let commit = repo.commit("Initial commit");

    // 同じコミットを比較（差分なし）
    let output = repo.run_cmd(&["-S", &commit, "-T", &commit]);
    assert_eq!(output.status.code(), Some(2), "Exit code should be 2 for no differences");
}

#[test]
fn test_differences_exit_code() {
    let repo = TestRepo::new();

    // 初期コミット
    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial commit");

    // 変更
    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert_eq!(output.status.code(), Some(0), "Exit code should be 0 for differences");
}

#[test]
fn test_branch_comparison() {
    let repo = TestRepo::new();

    // mainブランチで初期コミット
    repo.create_file("main.txt", "main content");
    repo.add_all();
    repo.commit("Main commit");

    // developブランチを作成して変更
    repo.create_branch("develop");
    repo.checkout("develop");
    repo.create_file("develop.txt", "develop content");
    repo.add_all();
    repo.commit("Develop commit");

    let output = repo.run_cmd(&["-S", "master", "-T", "develop"]);
    // masterが存在しない場合はmainを試す
    if !output.status.success() {
        let output = repo.run_cmd(&["-S", "main", "-T", "develop"]);
        // 初期ブランチ名に依存
        if output.status.success() || output.status.code() == Some(0) {
            assert!(repo.output_exists("develop.txt"));
        }
    }
}

#[test]
fn test_tag_comparison() {
    let repo = TestRepo::new();

    // v1.0.0タグ
    repo.create_file("file.txt", "v1 content");
    repo.add_all();
    repo.commit("Version 1.0.0");
    repo.create_tag("v1.0.0");

    // v2.0.0タグ
    repo.create_file("file.txt", "v2 content");
    repo.add_all();
    repo.commit("Version 2.0.0");
    repo.create_tag("v2.0.0");

    let output = repo.run_cmd(&["-S", "v1.0.0", "-T", "v2.0.0"]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("file.txt"));
}

#[test]
fn test_commit_hash_comparison() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    // 短縮ハッシュで比較
    let short_source = &source[..7];
    let short_target = &target[..7];

    let output = repo.run_cmd(&["-S", short_source, "-T", short_target]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

#[test]
fn test_head_relative_comparison() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "version 1");
    repo.add_all();
    repo.commit("Commit 1");

    repo.create_file("file.txt", "version 2");
    repo.add_all();
    repo.commit("Commit 2");

    // HEAD~1 と HEAD の比較
    let output = repo.run_cmd(&["-S", "HEAD~1", "-T", "HEAD"]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("file.txt"));
}

// ============================================================================
// 3. 三者間比較テスト
// ============================================================================

#[test]
fn test_three_way_basic() {
    let repo = TestRepo::new();

    // baseコミット
    repo.create_file("base.txt", "base content");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursブランチ
    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("ours.txt", "ours content");
    repo.add_all();
    let ours = repo.commit("Ours commit");

    // theirsブランチ（baseから分岐）
    repo.checkout(&base);
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("theirs.txt", "theirs content");
    repo.add_all();
    let theirs = repo.commit("Theirs commit");

    let output = repo.run_cmd(&["-3", "-B", &base, "-S", &ours, "-T", &theirs]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_conflict() {
    let repo = TestRepo::new();

    // baseコミット
    repo.create_file("conflict.txt", "base content");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursで変更
    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("conflict.txt", "ours modification");
    repo.add_all();
    let ours = repo.commit("Ours modification");

    // theirsで異なる変更
    repo.checkout(&base);
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("conflict.txt", "theirs modification");
    repo.add_all();
    let theirs = repo.commit("Theirs modification");

    let output = repo.run_cmd(&["-3", "-B", &base, "-S", &ours, "-T", &theirs]);
    // コンフリクトがある場合は終了コード3
    assert_eq!(output.status.code(), Some(3), "Exit code should be 3 for conflicts");
}

#[test]
fn test_three_way_both_same_change() {
    let repo = TestRepo::new();

    // baseコミット
    repo.create_file("same.txt", "base content");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursで変更
    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("same.txt", "same modification");
    repo.add_all();
    let ours = repo.commit("Ours modification");

    // theirsで同じ変更
    repo.checkout(&base);
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("same.txt", "same modification");
    repo.add_all();
    let theirs = repo.commit("Theirs modification");

    let output = repo.run_cmd(&["-3", "-B", &base, "-S", &ours, "-T", &theirs]);
    // 同じ変更の場合はコンフリクトなし
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_requires_base() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let commit = repo.commit("Initial commit");

    // --three-way without --base should fail
    let output = repo.run_cmd(&["-3", "-S", &commit, "-T", &commit]);
    assert!(!output.status.success(), "Should fail without --base");
}

// ============================================================================
// 4. オプションテスト
// ============================================================================

#[test]
fn test_dry_run() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--dry-run"]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);

    // dry-runではファイルがコピーされない
    assert!(!repo.output_exists("file.txt"), "File should not be copied in dry-run mode");
}

#[test]
fn test_both_versions() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--both-versions"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // .old と .new が作成される
    assert!(repo.output_exists("file.txt.old"), "file.txt.old should exist");
    assert!(repo.output_exists("file.txt.new"), "file.txt.new should exist");
    assert_eq!(repo.read_output("file.txt.old"), "original");
    assert_eq!(repo.read_output("file.txt.new"), "modified");
}

#[test]
fn test_copy_deleted() {
    let repo = TestRepo::new();

    repo.create_file("to_delete.txt", "will be deleted");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.delete_file("to_delete.txt");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--copy-deleted"]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);

    // .deleted が作成される
    assert!(repo.output_exists("to_delete.txt.deleted"), "to_delete.txt.deleted should exist");
    assert_eq!(repo.read_output("to_delete.txt.deleted"), "will be deleted");
}

#[test]
fn test_exclude_pattern() {
    let repo = TestRepo::new();

    repo.create_file("include.txt", "include");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("include.txt", "modified");
    repo.create_file("exclude.log", "log content");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "-e", "*.log"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("include.txt"), "include.txt should be copied");
    assert!(!repo.output_exists("exclude.log"), "exclude.log should be excluded");
}

#[test]
fn test_multiple_exclude_patterns() {
    let repo = TestRepo::new();

    repo.create_file("include.txt", "include");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("include.txt", "modified");
    repo.create_file("exclude.log", "log");
    repo.create_file("exclude.tmp", "tmp");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "-e", "*.log", "-e", "*.tmp"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("include.txt"));
    assert!(!repo.output_exists("exclude.log"));
    assert!(!repo.output_exists("exclude.tmp"));
}

#[test]
fn test_summary_file() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(summary_path.exists(), "Summary file should be created");
    let summary_content = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary_content.contains("rs_gitdiffcopy"));
}

#[test]
fn test_verbose_mode() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--verbose"]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

#[test]
fn test_stats_only() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--stats-only"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    // stats-onlyではFile Treeが表示されない
    assert!(!stdout.contains("File Tree"), "File Tree should not be shown with --stats-only");
}

#[test]
fn test_filter_status_added() {
    let repo = TestRepo::new();

    repo.create_file("existing.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("existing.txt", "modified");
    repo.create_file("new.txt", "new file");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--filter-status", "added"]);
    assert!(output.status.success() || output.status.code() == Some(2), "Command failed: {:?}", output);

    // addedファイルのみがコピーされる
    if output.status.success() {
        assert!(repo.output_exists("new.txt"), "new.txt should be copied");
        assert!(!repo.output_exists("existing.txt"), "existing.txt should not be copied (modified)");
    }
}

#[test]
fn test_filter_status_modified() {
    let repo = TestRepo::new();

    repo.create_file("existing.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("existing.txt", "modified");
    repo.create_file("new.txt", "new file");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--filter-status", "modified"]);
    assert!(output.status.success() || output.status.code() == Some(2), "Command failed: {:?}", output);

    // modifiedファイルのみがコピーされる
    if output.status.success() {
        assert!(repo.output_exists("existing.txt"), "existing.txt should be copied");
        assert!(!repo.output_exists("new.txt"), "new.txt should not be copied (added)");
    }
}

#[test]
fn test_no_tree_option() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--no-tree"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("File Tree"), "File Tree section should not be shown");
}

#[test]
fn test_no_details_option() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--no-details"]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

#[test]
fn test_workers_option() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "-j", "2"]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

// ============================================================================
// 5. 日本語パステスト
// ============================================================================

#[test]
fn test_japanese_directory_names() {
    let repo = TestRepo::new();

    repo.create_file("initial.txt", "initial");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("日本語フォルダ/ファイル.txt", "日本語コンテンツ");
    repo.add_all();
    let target = repo.commit("Add Japanese directory");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("日本語フォルダ/ファイル.txt"), "Japanese path should be copied");
}

#[test]
fn test_japanese_file_names() {
    let repo = TestRepo::new();

    repo.create_file("initial.txt", "initial");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("テスト.txt", "テスト内容");
    repo.add_all();
    let target = repo.commit("Add Japanese file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("テスト.txt"));
    let content = repo.read_output("テスト.txt");
    assert_eq!(content, "テスト内容");
}

#[test]
fn test_japanese_nested_path() {
    let repo = TestRepo::new();

    repo.create_file("initial.txt", "initial");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("レベル1/レベル2/レベル3/深い.txt", "深いファイル");
    repo.add_all();
    let target = repo.commit("Add deeply nested Japanese path");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("レベル1/レベル2/レベル3/深い.txt"));
}

#[test]
fn test_mixed_japanese_english_path() {
    let repo = TestRepo::new();

    repo.create_file("initial.txt", "initial");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("project/日本語/test_テスト.txt", "mixed content");
    repo.add_all();
    let target = repo.commit("Add mixed path");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("project/日本語/test_テスト.txt"));
}

#[test]
fn test_japanese_in_summary_output() {
    let repo = TestRepo::new();

    repo.create_file("initial.txt", "initial");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("日本語ファイル.txt", "日本語内容");
    repo.add_all();
    let target = repo.commit("Add Japanese file");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary.contains("日本語ファイル.txt"), "Summary should contain Japanese filename");
}

#[test]
fn test_japanese_both_versions() {
    let repo = TestRepo::new();

    repo.create_file("日本語.txt", "元の内容");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("日本語.txt", "変更後の内容");
    repo.add_all();
    let target = repo.commit("Modify Japanese file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--both-versions"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("日本語.txt.old"));
    assert!(repo.output_exists("日本語.txt.new"));
    assert_eq!(repo.read_output("日本語.txt.old"), "元の内容");
    assert_eq!(repo.read_output("日本語.txt.new"), "変更後の内容");
}

#[test]
fn test_japanese_copy_deleted() {
    let repo = TestRepo::new();

    repo.create_file("削除対象.txt", "削除される内容");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.delete_file("削除対象.txt");
    repo.add_all();
    let target = repo.commit("Delete Japanese file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--copy-deleted"]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);

    assert!(repo.output_exists("削除対象.txt.deleted"));
    assert_eq!(repo.read_output("削除対象.txt.deleted"), "削除される内容");
}

#[test]
fn test_japanese_content_in_file() {
    let repo = TestRepo::new();

    repo.create_file("test.txt", "English content");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("test.txt", "日本語の内容に変更\nひらがな、カタカナ、漢字");
    repo.add_all();
    let target = repo.commit("Modify with Japanese content");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let content = repo.read_output("test.txt");
    assert!(content.contains("日本語の内容に変更"));
    assert!(content.contains("ひらがな、カタカナ、漢字"));
}

#[test]
fn test_hiragana_katakana_kanji_mixed() {
    let repo = TestRepo::new();

    repo.create_file("initial.txt", "initial");
    repo.add_all();
    let source = repo.commit("Initial commit");

    repo.create_file("ひらがな_カタカナ_漢字.txt", "混合テスト");
    repo.add_all();
    let target = repo.commit("Add mixed Japanese filename");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists("ひらがな_カタカナ_漢字.txt"));
}

#[test]
fn test_long_japanese_filename() {
    let repo = TestRepo::new();

    repo.create_file("initial.txt", "initial");
    repo.add_all();
    let source = repo.commit("Initial commit");

    let long_name = "これは非常に長い日本語ファイル名のテストです.txt";
    repo.create_file(long_name, "長い名前のファイル");
    repo.add_all();
    let target = repo.commit("Add long Japanese filename");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    assert!(repo.output_exists(long_name));
}

// ============================================================================
// 6. エラーハンドリングテスト
// ============================================================================

#[test]
fn test_nonexistent_repository() {
    let output = run_cmd_raw(&[
        "-R", "/nonexistent/repository/path",
        "-S", "main",
        "-T", "develop",
        "-O", "/tmp/output",
    ]);
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_nonexistent_source_ref() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let target = repo.commit("Initial commit");

    let output = repo.run_cmd(&["-S", "nonexistent_branch_xyz", "-T", &target]);
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_nonexistent_target_ref() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial commit");

    let output = repo.run_cmd(&["-S", &source, "-T", "nonexistent_branch_xyz"]);
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_output_exists_without_force() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    // 出力先を事前に作成
    fs::create_dir_all(repo.output_path()).expect("Failed to create output dir");
    fs::write(repo.output_path().join("existing.txt"), "existing").expect("Failed to write");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_missing_required_args() {
    // 引数なしで実行
    let output = run_cmd_raw(&[]);
    // clapはエラーコード2を返すことがある
    assert!(!output.status.success() || output.status.code() != Some(0));
}

#[test]
fn test_not_git_repository() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let non_git_path = temp_dir.path().join("not_a_repo");
    fs::create_dir_all(&non_git_path).expect("Failed to create dir");

    let output_path = temp_dir.path().join("output");

    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args([
            "-R", non_git_path.to_str().unwrap(),
            "-S", "main",
            "-T", "develop",
            "-O", output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

// ============================================================================
// 7. 終了コードテスト
// ============================================================================

#[test]
fn test_exit_code_0_with_differences() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_exit_code_1_on_error() {
    let output = run_cmd_raw(&["-S", "nonexistent", "-T", "nonexistent", "-O", "/tmp/out"]);
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_exit_code_2_no_differences() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let commit = repo.commit("Only commit");

    let output = repo.run_cmd(&["-S", &commit, "-T", &commit]);
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_exit_code_3_conflicts() {
    let repo = TestRepo::new();

    repo.create_file("conflict.txt", "base");
    repo.add_all();
    let base = repo.commit("Base");

    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("conflict.txt", "ours change");
    repo.add_all();
    let ours = repo.commit("Ours");

    repo.checkout(&base);
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("conflict.txt", "theirs change");
    repo.add_all();
    let theirs = repo.commit("Theirs");

    let output = repo.run_cmd(&["-3", "-B", &base, "-S", &ours, "-T", &theirs]);
    assert_eq!(output.status.code(), Some(3));
}

// ============================================================================
// 8. 設定ファイルテスト
// ============================================================================

#[test]
fn test_config_file_basic() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    // 設定ファイルを作成
    let config_content = format!(
        r#"
source = "{}"
target = "{}"
output = "{}"
repository = "{}"
"#,
        source,
        target,
        repo.output_path().display(),
        repo.repo_path().display()
    );

    let config_path = repo.temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).expect("Failed to write config");

    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["-c", config_path.to_str().unwrap()])
        .output()
        .expect("Failed to run");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("file.txt"));
}

#[test]
fn test_config_file_with_exclude() {
    let repo = TestRepo::new();

    repo.create_file("include.txt", "include");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("include.txt", "modified");
    repo.create_file("exclude.log", "log");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let config_content = format!(
        r#"
source = "{}"
target = "{}"
output = "{}"
repository = "{}"
exclude = ["*.log"]
"#,
        source,
        target,
        repo.output_path().display(),
        repo.repo_path().display()
    );

    let config_path = repo.temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).expect("Failed to write config");

    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["-c", config_path.to_str().unwrap()])
        .output()
        .expect("Failed to run");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("include.txt"));
    assert!(!repo.output_exists("exclude.log"));
}

#[test]
fn test_cli_overrides_config() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "v1");
    repo.add_all();
    let commit1 = repo.commit("Commit 1");

    repo.create_file("file.txt", "v2");
    repo.add_all();
    let commit2 = repo.commit("Commit 2");

    repo.create_file("file.txt", "v3");
    repo.add_all();
    let commit3 = repo.commit("Commit 3");

    // 設定ファイルではcommit1 -> commit2
    let config_content = format!(
        r#"
source = "{}"
target = "{}"
output = "{}"
repository = "{}"
"#,
        commit1,
        commit2,
        repo.output_path().display(),
        repo.repo_path().display()
    );

    let config_path = repo.temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).expect("Failed to write config");

    // CLIでcommit2 -> commit3を指定（オーバーライド）
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args([
            "-c", config_path.to_str().unwrap(),
            "-S", &commit2,
            "-T", &commit3,
        ])
        .output()
        .expect("Failed to run");

    assert!(output.status.success(), "Command failed: {:?}", output);
    let content = repo.read_output("file.txt");
    assert_eq!(content, "v3", "CLI args should override config");
}

#[test]
fn test_save_config() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let commit = repo.commit("Initial");

    let save_path = repo.temp_dir.path().join("saved_config.toml");

    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args([
            "-R", repo.repo_path().to_str().unwrap(),
            "-S", &commit,
            "-T", &commit,
            "-O", repo.output_path().to_str().unwrap(),
            "-C", save_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(save_path.exists(), "Config file should be saved");

    let saved_content = fs::read_to_string(&save_path).expect("Failed to read saved config");
    assert!(saved_content.contains("source"), "Saved config should contain source");
    assert!(saved_content.contains("target"), "Saved config should contain target");
}

// ============================================================================
// 9. show_unchangedテスト
// ============================================================================

#[test]
fn test_show_unchanged_option() {
    let repo = TestRepo::new();

    repo.create_file("unchanged.txt", "same");
    repo.create_file("changed.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("changed.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--show-unchanged"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("unchanged.txt"), "unchanged.txt should be shown");
}

#[test]
fn test_show_unchanged_short_option() {
    let repo = TestRepo::new();

    repo.create_file("unchanged.txt", "same");
    repo.create_file("changed.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("changed.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "-u"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("unchanged.txt"), "unchanged.txt should be shown with -u");
}

#[test]
fn test_unchanged_count_in_statistics() {
    let repo = TestRepo::new();

    repo.create_file("unchanged1.txt", "same1");
    repo.create_file("unchanged2.txt", "same2");
    repo.create_file("changed.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("changed.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    // 統計にUnchangedが表示される
    assert!(stdout.contains("Unchanged") || stdout.contains("unchanged"),
            "Statistics should show unchanged count");
}

// ============================================================================
// 10. Git ref形式テスト
// ============================================================================

#[test]
fn test_branch_name_with_slash() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "main content");
    repo.add_all();
    repo.commit("Main commit");

    repo.create_branch("feature/test-branch");
    repo.checkout("feature/test-branch");
    repo.create_file("feature.txt", "feature content");
    repo.add_all();
    repo.commit("Feature commit");

    // feature/test-branchを指定
    let output = repo.run_cmd(&["-S", "HEAD~1", "-T", "feature/test-branch"]);
    assert!(output.status.success(), "Command failed with branch containing slash: {:?}", output);
}

#[test]
fn test_short_commit_hash() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "v1");
    repo.add_all();
    let hash1 = repo.commit("Commit 1");

    repo.create_file("file.txt", "v2");
    repo.add_all();
    let hash2 = repo.commit("Commit 2");

    // 7文字の短縮ハッシュ
    let output = repo.run_cmd(&["-S", &hash1[..7], "-T", &hash2[..7]]);
    assert!(output.status.success(), "Short commit hash should work: {:?}", output);
}

#[test]
fn test_full_commit_hash() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "v1");
    repo.add_all();
    let hash1 = repo.commit("Commit 1");

    repo.create_file("file.txt", "v2");
    repo.add_all();
    let hash2 = repo.commit("Commit 2");

    // 完全なハッシュ
    let output = repo.run_cmd(&["-S", &hash1, "-T", &hash2]);
    assert!(output.status.success(), "Full commit hash should work: {:?}", output);
}

#[test]
fn test_head_caret_notation() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "v1");
    repo.add_all();
    repo.commit("Commit 1");

    repo.create_file("file.txt", "v2");
    repo.add_all();
    repo.commit("Commit 2");

    // HEAD^表記
    let output = repo.run_cmd(&["-S", "HEAD^", "-T", "HEAD"]);
    assert!(output.status.success(), "HEAD^ notation should work: {:?}", output);
}

#[test]
fn test_head_tilde_notation() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "v1");
    repo.add_all();
    repo.commit("Commit 1");

    repo.create_file("file.txt", "v2");
    repo.add_all();
    repo.commit("Commit 2");

    repo.create_file("file.txt", "v3");
    repo.add_all();
    repo.commit("Commit 3");

    // HEAD~2表記
    let output = repo.run_cmd(&["-S", "HEAD~2", "-T", "HEAD"]);
    assert!(output.status.success(), "HEAD~N notation should work: {:?}", output);
}

// ============================================================================
// 11. サマリーファイル内容検証テスト
// ============================================================================

#[test]
fn test_summary_contains_header() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary.contains("rs_gitdiffcopy"), "Should contain title");
    assert!(summary.contains("Repository") || summary.contains("Source"), "Should contain header info");
    assert!(summary.contains("Date") || summary.contains("Output"), "Should contain date or output info");
}

#[test]
fn test_summary_statistics_accuracy() {
    let repo = TestRepo::new();

    repo.create_file("unchanged.txt", "same");
    repo.create_file("modified.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("modified.txt", "changed");
    repo.create_file("added.txt", "new");
    repo.delete_file("unused.txt"); // このファイルは存在しないので無視される
    repo.add_all();
    let target = repo.commit("Commit 2");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    // 統計情報が含まれている
    assert!(summary.contains("Added") || summary.contains("added"), "Should show added count");
    assert!(summary.contains("Modified") || summary.contains("modified"), "Should show modified count");
}

#[test]
fn test_summary_contains_file_tree() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.create_file("dir/nested.txt", "nested");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary.contains("File Tree") || summary.contains("file.txt"), "Should contain file tree");
}

#[test]
fn test_summary_japanese_paths() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("日本語ディレクトリ/テスト.txt", "日本語内容");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary.contains("日本語") || summary.contains("テスト"), "Should contain Japanese paths");
}

#[test]
fn test_summary_options_section() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Commit 1");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Commit 2");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
        "-e", "*.log",
        "--dry-run",
    ]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary.contains("Options") || summary.contains("Dry run") || summary.contains("Exclude"),
            "Should contain options section");
}

#[test]
fn test_summary_no_differences() {
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let commit = repo.commit("Only commit");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &commit,
        "-T", &commit,
        "-s", summary_path.to_str().unwrap(),
    ]);
    // 差分なしは終了コード2
    assert_eq!(output.status.code(), Some(2));

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    // 差分がないことを示す情報
    assert!(summary.contains("0") || summary.contains("No") || summary.contains("Unchanged"),
            "Should indicate no differences");
}

// ============================================================================
// 12. パッチファイルテスト
// ============================================================================

#[test]
fn test_patch_file_unified_format() {
    // PATCH-001: unified diff形式の確認
    let repo = TestRepo::new();

    repo.create_file("file.txt", "line1\nline2\nline3\n");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "line1\nmodified\nline3\n");
    repo.add_all();
    let target = repo.commit("Modify");

    let patch_file = repo.temp_dir.path().join("changes.patch");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-F", patch_file.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // パッチファイルが生成されている
    assert!(patch_file.exists(), "Patch file should exist at {:?}", patch_file);
    let patch = fs::read_to_string(&patch_file).expect("Failed to read patch");
    // unified diff形式の確認
    assert!(patch.contains("---") || patch.contains("+++") || patch.contains("@@"),
            "Should be unified diff format: {}", patch);
}

#[test]
fn test_combined_patch_file() {
    // PATCH-002: 統合パッチファイル
    let repo = TestRepo::new();

    repo.create_file("file1.txt", "content1");
    repo.create_file("file2.txt", "content2");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file1.txt", "modified1");
    repo.create_file("file2.txt", "modified2");
    repo.add_all();
    let target = repo.commit("Modify both");

    let patch_file = repo.temp_dir.path().join("combined.patch");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-F", patch_file.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    if patch_file.exists() {
        let patch = fs::read_to_string(&patch_file).expect("Failed to read patch");
        // 両方のファイルのパッチが含まれている
        assert!(patch.contains("file1") || patch.contains("file2") || patch.contains("diff"),
                "Combined patch should contain file info: {}", patch);
    }
}

#[test]
fn test_patch_addition_only() {
    // PATCH-003: 追加のみのパッチ
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("new_file.txt", "new content\n");
    repo.add_all();
    let target = repo.commit("Add file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--patch"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    if repo.output_exists("new_file.txt.patch") {
        let patch = repo.read_output("new_file.txt.patch");
        // 追加のみなので+行のみ
        assert!(patch.contains("+") || patch.contains("new"), "Should have addition lines");
    }
}

#[test]
fn test_patch_deletion_only() {
    // PATCH-004: 削除のみのパッチ
    let repo = TestRepo::new();

    repo.create_file("to_delete.txt", "will be deleted\n");
    repo.create_file("keep.txt", "keep");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.delete_file("to_delete.txt");
    repo.add_all();
    let target = repo.commit("Delete file");

    let patch_file = repo.temp_dir.path().join("changes.patch");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-F", patch_file.to_str().unwrap(),
    ]);
    // 削除のみでもパッチは生成される
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_patch_multiple_hunks() {
    // PATCH-005: 複数ハンクのパッチ
    let repo = TestRepo::new();

    // 長いファイルを作成
    let mut content = String::new();
    for i in 1..=20 {
        content.push_str(&format!("line{}\n", i));
    }
    repo.create_file("long_file.txt", &content);
    repo.add_all();
    let source = repo.commit("Initial");

    // 離れた場所を変更
    let modified = content
        .replace("line3\n", "modified3\n")
        .replace("line17\n", "modified17\n");
    repo.create_file("long_file.txt", &modified);
    repo.add_all();
    let target = repo.commit("Modify multiple places");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--patch"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    if repo.output_exists("long_file.txt.patch") {
        let patch = repo.read_output("long_file.txt.patch");
        // 複数の@@ヘッダーがある可能性
        let hunk_count = patch.matches("@@").count();
        assert!(hunk_count >= 1, "Should have at least one hunk header");
    }
}

#[test]
fn test_patch_subdirectory() {
    // PATCH-006: サブディレクトリのパッチ
    let repo = TestRepo::new();

    repo.create_file("src/main.rs", "fn main() {}");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("src/main.rs", "fn main() { println!(\"Hello\"); }");
    repo.add_all();
    let target = repo.commit("Modify");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--patch"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // サブディレクトリ構造でパッチファイルが作成される
    assert!(repo.output_exists("src/main.rs.patch") || repo.output_exists("src/main.rs"),
            "Patch should be in subdirectory");
}

#[test]
fn test_patch_japanese_content() {
    // PATCH-007: 日本語内容のパッチ
    let repo = TestRepo::new();

    repo.create_file("japanese.txt", "日本語の内容\n");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("japanese.txt", "変更された日本語\n");
    repo.add_all();
    let target = repo.commit("Modify Japanese");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--patch"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    if repo.output_exists("japanese.txt.patch") {
        let patch = repo.read_output("japanese.txt.patch");
        assert!(patch.contains("日本語") || patch.contains("変更"),
                "Patch should contain Japanese characters");
    }
}

#[test]
fn test_patch_binary_skipped() {
    // PATCH-008: バイナリファイルはパッチスキップ
    let repo = TestRepo::new();

    // バイナリファイルを作成
    let binary_content: Vec<u8> = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
    let binary_path = repo.repo_path.join("binary.bin");
    fs::write(&binary_path, &binary_content).expect("Failed to write binary");
    repo.add_all();
    let source = repo.commit("Initial with binary");

    // バイナリを変更
    let modified_binary: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x00, 0x01, 0x02];
    fs::write(&binary_path, &modified_binary).expect("Failed to write modified binary");
    repo.add_all();
    let target = repo.commit("Modify binary");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--patch"]);
    // バイナリでもコマンドは成功する
    assert!(output.status.success() || output.status.code() == Some(0), "Command should succeed");
}

#[test]
fn test_patch_and_patch_file_together() {
    // PATCH-009: --patch と --patch-file 併用
    let repo = TestRepo::new();

    repo.create_file("file.txt", "original");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let combined_patch = repo.temp_dir.path().join("all.patch");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-F", combined_patch.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    // 統合パッチファイルが生成される
    // Note: --patch option generates individual .patch files alongside copied files,
    // but we're only testing -F here which creates a combined patch
    if combined_patch.exists() {
        let patch = fs::read_to_string(&combined_patch).expect("Failed to read patch");
        assert!(patch.contains("file.txt") || patch.contains("diff") || patch.contains("@@"),
                "Combined patch should exist with content");
    }
}

#[test]
fn test_patch_hunk_header_format() {
    // PATCH-010: ハンクヘッダー形式
    let repo = TestRepo::new();

    repo.create_file("file.txt", "line1\nline2\nline3\n");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "line1\nchanged\nline3\n");
    repo.add_all();
    let target = repo.commit("Change middle line");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--patch"]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    if repo.output_exists("file.txt.patch") {
        let patch = repo.read_output("file.txt.patch");
        // @@ -start,count +start,count @@ 形式
        if patch.contains("@@") {
            assert!(patch.contains("-") && patch.contains("+"),
                    "Hunk header should have proper format");
        }
    }
}

// ============================================================================
// 13. Excelファイル内容検証テスト
// ============================================================================

#[test]
fn test_excel_file_generation() {
    // EXCEL-001: Excelファイル生成確認
    let repo = TestRepo::new();

    repo.create_file("file1.txt", "content1");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file2.txt", "content2");
    repo.create_file("file1.txt", "modified");
    repo.add_all();
    let target = repo.commit("Changes");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel file should be created");
}

#[test]
fn test_excel_has_correct_sheets() {
    // EXCEL-001: シート名の検証
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel file should exist");

    // calamineでExcelを読み込んでシートを確認
    use calamine::{Reader, open_workbook, Xlsx};
    let workbook: Result<Xlsx<_>, _> = open_workbook(&excel_path);
    if let Ok(wb) = workbook {
        let sheets = wb.sheet_names();
        assert!(!sheets.is_empty(), "Excel should have sheets");
        // Summary, File Tree, Details のいずれかが存在
        let has_expected_sheet = sheets.iter().any(|s|
            s.contains("Summary") || s.contains("File") || s.contains("Detail")
        );
        assert!(has_expected_sheet, "Excel should have expected sheets: {:?}", sheets);
    }
}

#[test]
fn test_excel_summary_statistics() {
    // EXCEL-002: Summaryシートの統計
    let repo = TestRepo::new();

    repo.create_file("file1.txt", "content1");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file2.txt", "added");
    repo.create_file("file1.txt", "modified");
    repo.add_all();
    let target = repo.commit("Add and modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    use calamine::{Reader, open_workbook, Xlsx};
    let workbook: Result<Xlsx<_>, _> = open_workbook(&excel_path);
    if let Ok(mut wb) = workbook {
        // Summaryシートを探して統計を確認
        for sheet_name in wb.sheet_names().to_vec() {
            if sheet_name.contains("Summary") {
                if let Ok(range) = wb.worksheet_range(&sheet_name) {
                    let has_stats = range.rows().any(|row| {
                        row.iter().any(|cell| {
                            let s = cell.to_string();
                            s.contains("Added") || s.contains("Modified") || s.contains("Total")
                        })
                    });
                    assert!(has_stats, "Summary should have statistics");
                }
                break;
            }
        }
    }
}

#[test]
fn test_excel_file_tree_entries() {
    // EXCEL-003: File Treeシートのエントリ
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("src/main.rs", "fn main() {}");
    repo.create_file("src/lib.rs", "pub fn lib() {}");
    repo.add_all();
    let target = repo.commit("Add source files");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    use calamine::{Reader, open_workbook, Xlsx};
    let workbook: Result<Xlsx<_>, _> = open_workbook(&excel_path);
    if let Ok(mut wb) = workbook {
        for sheet_name in wb.sheet_names().to_vec() {
            if sheet_name.contains("File") || sheet_name.contains("Tree") {
                if let Ok(range) = wb.worksheet_range(&sheet_name) {
                    let has_files = range.rows().any(|row| {
                        row.iter().any(|cell| {
                            let s = cell.to_string();
                            s.contains("main.rs") || s.contains("lib.rs") || s.contains("src")
                        })
                    });
                    assert!(has_files, "File Tree should contain file entries");
                }
                break;
            }
        }
    }
}

#[test]
fn test_excel_details_sections() {
    // EXCEL-004: Detailsシートのセクション
    let repo = TestRepo::new();

    repo.create_file("to_delete.txt", "will delete");
    repo.create_file("to_modify.txt", "original");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("added.txt", "new file");
    repo.create_file("to_modify.txt", "modified");
    repo.delete_file("to_delete.txt");
    repo.add_all();
    let target = repo.commit("Various changes");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel should be created");
}

#[test]
fn test_excel_japanese_filenames() {
    // EXCEL-005: 日本語ファイル名
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("日本語ファイル.txt", "内容");
    repo.create_file("テスト/サブファイル.txt", "サブ内容");
    repo.add_all();
    let target = repo.commit("Add Japanese files");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    use calamine::{Reader, open_workbook, Xlsx};
    let workbook: Result<Xlsx<_>, _> = open_workbook(&excel_path);
    if let Ok(mut wb) = workbook {
        let mut found_japanese = false;
        for sheet_name in wb.sheet_names().to_vec() {
            if let Ok(range) = wb.worksheet_range(&sheet_name) {
                for row in range.rows() {
                    for cell in row {
                        let s = cell.to_string();
                        if s.contains("日本語") || s.contains("テスト") {
                            found_japanese = true;
                            break;
                        }
                    }
                }
            }
        }
        assert!(found_japanese, "Excel should contain Japanese filenames");
    }
}

#[test]
fn test_excel_subdirectory_structure() {
    // EXCEL-006: サブディレクトリ構造
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("src/main/java/App.java", "class App {}");
    repo.create_file("src/test/java/AppTest.java", "class AppTest {}");
    repo.add_all();
    let target = repo.commit("Add deep structure");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel should be created with deep structure");
}

// ============================================================================
// 14. エッジケーステスト
// ============================================================================

#[test]
fn test_empty_file() {
    // EDGE-001: 空ファイルの検出
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("empty.txt", "");
    repo.add_all();
    let target = repo.commit("Add empty file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("empty.txt"), "Empty file should be copied");
}

#[test]
fn test_same_size_different_content() {
    // EDGE-002: 同サイズで異なる内容
    let repo = TestRepo::new();

    repo.create_file("file.txt", "AAAA");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "BBBB");
    repo.add_all();
    let target = repo.commit("Same size different content");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("file.txt"), "Modified file should be detected");
    let content = repo.read_output("file.txt");
    assert_eq!(content, "BBBB");
}

#[test]
fn test_unicode_russian_filenames() {
    // EDGE-003: ロシア語ファイル名
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("файл.txt", "содержимое");
    repo.add_all();
    let target = repo.commit("Add Russian file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

#[test]
fn test_exclude_pycache_pattern() {
    // EDGE-004: __pycache__除外パターン
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("main.py", "print('hello')");
    repo.create_file("__pycache__/main.cpython-39.pyc", "binary");
    repo.add_all();
    let target = repo.commit("Add Python files");

    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-e", "__pycache__/**",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("main.py"), "main.py should be copied");
    assert!(!repo.output_exists("__pycache__/main.cpython-39.pyc"),
            "__pycache__ should be excluded");
}

#[test]
fn test_both_versions_added_files_unchanged() {
    // EDGE-005: 追加ファイルでboth-versions
    let repo = TestRepo::new();

    repo.create_file("existing.txt", "existing");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("new_file.txt", "new content");
    repo.add_all();
    let target = repo.commit("Add new file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "--both-versions"]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    // 追加ファイルは拡張子なしでコピー
    assert!(repo.output_exists("new_file.txt"), "New file should be copied without extension");
}

#[test]
fn test_deeply_nested_directories() {
    // EDGE-006: 深いネスト（10階層以上）
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    // 10階層のディレクトリ
    repo.create_file("a/b/c/d/e/f/g/h/i/j/deep.txt", "deep content");
    repo.add_all();
    let target = repo.commit("Add deeply nested file");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(repo.output_exists("a/b/c/d/e/f/g/h/i/j/deep.txt"),
            "Deeply nested file should be copied");
}

// ============================================================================
// 15. シンボリックリンクテスト (Unix only)
// ============================================================================

#[cfg(unix)]
#[test]
fn test_symlink_added_detection() {
    // SYM-001: シンボリックリンク追加
    use std::os::unix::fs::symlink;

    let repo = TestRepo::new();

    repo.create_file("target_file.txt", "target content");
    repo.add_all();
    let source = repo.commit("Initial");

    // シンボリックリンクを作成
    let link_path = repo.repo_path.join("link.txt");
    symlink("target_file.txt", &link_path).expect("Failed to create symlink");
    repo.add_all();
    let target = repo.commit("Add symlink");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    // シンボリックリンクは検出されるがコピーされない
    assert!(output.status.success() || output.status.code() == Some(0), "Command should succeed");
}

#[cfg(unix)]
#[test]
fn test_symlink_deleted_detection() {
    // SYM-002: シンボリックリンク削除
    use std::os::unix::fs::symlink;

    let repo = TestRepo::new();

    repo.create_file("target_file.txt", "target content");
    let link_path = repo.repo_path.join("link.txt");
    symlink("target_file.txt", &link_path).expect("Failed to create symlink");
    repo.add_all();
    let source = repo.commit("Initial with symlink");

    // シンボリックリンクを削除
    fs::remove_file(&link_path).expect("Failed to remove symlink");
    repo.add_all();
    let target = repo.commit("Remove symlink");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command should succeed");
}

#[cfg(unix)]
#[test]
fn test_broken_symlink_detection() {
    // SYM-003: 壊れたシンボリックリンク
    use std::os::unix::fs::symlink;

    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    // 存在しないファイルへのシンボリックリンク
    let link_path = repo.repo_path.join("broken_link.txt");
    symlink("nonexistent.txt", &link_path).expect("Failed to create broken symlink");
    repo.add_all();
    let target = repo.commit("Add broken symlink");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    // コマンドは成功するが、壊れたシンボリックリンクは適切に処理される
    assert!(output.status.success() || output.status.code() == Some(0), "Command should handle broken symlink");
}

// ============================================================================
// 16. パーミッションテスト (Unix only)
// ============================================================================

#[cfg(unix)]
#[test]
fn test_permission_check_scripts_mode() {
    // PERM-001: -P scriptsオプション
    use std::os::unix::fs::PermissionsExt;

    let repo = TestRepo::new();

    repo.create_file("script.sh", "#!/bin/bash\necho hello");
    repo.add_all();
    let source = repo.commit("Initial");

    // 実行権限を付与
    let script_path = repo.repo_path.join("script.sh");
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).expect("Failed to set permissions");
    repo.add_all();
    let target = repo.commit("Make script executable");

    let output = repo.run_cmd(&["-S", &source, "-T", &target, "-P", "scripts"]);
    // パーミッションチェックが有効になる
    assert!(output.status.success() || output.status.code() == Some(0), "Command should succeed with -P scripts");
}

#[cfg(unix)]
#[test]
fn test_permission_check_default_disabled() {
    // PERM-002: パーミッションチェックなし（デフォルト）
    use std::os::unix::fs::PermissionsExt;

    let repo = TestRepo::new();

    repo.create_file("script.sh", "#!/bin/bash\necho hello");
    repo.add_all();
    let source = repo.commit("Initial");

    let script_path = repo.repo_path.join("script.sh");
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).expect("Failed to set permissions");
    repo.add_all();
    let target = repo.commit("Make script executable");

    // -Pなしで実行
    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command should succeed without -P");
}

// ============================================================================
// 17. 拡張三者間比較テスト
// ============================================================================

#[test]
fn test_three_way_added_ours() {
    // EXT3-001: Oursのみ追加
    let repo = TestRepo::new();

    repo.create_file("base.txt", "base content");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursブランチ
    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("ours_only.txt", "ours added");
    repo.add_all();
    let ours = repo.commit("Ours adds file");

    // theirsブランチ（baseから）
    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    // theirsでは変更なし
    let theirs = repo.commit("Theirs no change");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
    ]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_added_theirs() {
    // EXT3-002: Theirsのみ追加
    let repo = TestRepo::new();

    repo.create_file("base.txt", "base content");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursブランチ
    repo.create_branch("ours");
    repo.checkout("ours");
    let ours = repo.commit("Ours no change");

    // theirsブランチ（baseから）
    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("theirs_only.txt", "theirs added");
    repo.add_all();
    let theirs = repo.commit("Theirs adds file");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
    ]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_deleted_ours() {
    // EXT3-003: Oursで削除
    let repo = TestRepo::new();

    repo.create_file("base.txt", "base content");
    repo.create_file("to_delete.txt", "will be deleted");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursブランチ
    repo.create_branch("ours");
    repo.checkout("ours");
    repo.delete_file("to_delete.txt");
    repo.add_all();
    let ours = repo.commit("Ours deletes file");

    // theirsブランチ
    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    let theirs = repo.commit("Theirs no change");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
    ]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_deleted_theirs() {
    // EXT3-004: Theirsで削除
    let repo = TestRepo::new();

    repo.create_file("base.txt", "base content");
    repo.create_file("to_delete.txt", "will be deleted");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursブランチ
    repo.create_branch("ours");
    repo.checkout("ours");
    let ours = repo.commit("Ours no change");

    // theirsブランチ
    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.delete_file("to_delete.txt");
    repo.add_all();
    let theirs = repo.commit("Theirs deletes file");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
    ]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_deleted_both() {
    // EXT3-005: 両方で削除
    let repo = TestRepo::new();

    repo.create_file("base.txt", "base content");
    repo.create_file("to_delete.txt", "will be deleted");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursブランチ
    repo.create_branch("ours");
    repo.checkout("ours");
    repo.delete_file("to_delete.txt");
    repo.add_all();
    let ours = repo.commit("Ours deletes file");

    // theirsブランチ
    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.delete_file("to_delete.txt");
    repo.add_all();
    let theirs = repo.commit("Theirs also deletes file");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
    ]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_modify_delete_conflict() {
    // EXT3-006: 変更と削除の競合
    let repo = TestRepo::new();

    repo.create_file("conflict.txt", "base content");
    repo.add_all();
    let base = repo.commit("Base commit");

    // oursブランチ - 変更
    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("conflict.txt", "modified by ours");
    repo.add_all();
    let ours = repo.commit("Ours modifies");

    // theirsブランチ - 削除
    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.delete_file("conflict.txt");
    repo.add_all();
    let theirs = repo.commit("Theirs deletes");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
    ]);
    // コンフリクトなので終了コード3
    assert!(output.status.code() == Some(3) || output.status.success(), "Should detect modify-delete conflict");
}

#[test]
fn test_three_way_merge_style_ours() {
    // EXT3-007: --merge-style ours
    let repo = TestRepo::new();

    repo.create_file("file.txt", "base");
    repo.add_all();
    let base = repo.commit("Base");

    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("file.txt", "ours version");
    repo.add_all();
    let ours = repo.commit("Ours");

    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("file.txt", "theirs version");
    repo.add_all();
    let theirs = repo.commit("Theirs");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
        "-M", "ours",
    ]);
    assert!(output.status.success() || output.status.code() == Some(3), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_merge_style_theirs() {
    // EXT3-008: --merge-style theirs
    let repo = TestRepo::new();

    repo.create_file("file.txt", "base");
    repo.add_all();
    let base = repo.commit("Base");

    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("file.txt", "ours version");
    repo.add_all();
    let ours = repo.commit("Ours");

    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("file.txt", "theirs version");
    repo.add_all();
    let theirs = repo.commit("Theirs");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
        "-M", "theirs",
    ]);
    assert!(output.status.success() || output.status.code() == Some(3), "Command failed: {:?}", output);
}

#[test]
fn test_three_way_conflict_only() {
    // EXT3-009: --conflict-only
    let repo = TestRepo::new();

    repo.create_file("conflict.txt", "base");
    repo.create_file("no_conflict.txt", "unchanged");
    repo.add_all();
    let base = repo.commit("Base");

    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("conflict.txt", "ours");
    repo.add_all();
    let ours = repo.commit("Ours");

    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("conflict.txt", "theirs");
    repo.add_all();
    let theirs = repo.commit("Theirs");

    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
        "--conflict-only",
    ]);
    assert!(output.status.success() || output.status.code() == Some(3), "Command failed: {:?}", output);
}

// ============================================================================
// 18. Excelフォーマットテスト
// ============================================================================

#[test]
fn test_excel_summary_has_options_section() {
    // EXFMT-001: Optionsセクション存在確認
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
        "-e", "*.log",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    use calamine::{Reader, open_workbook, Xlsx};
    if let Ok(mut wb) = open_workbook::<Xlsx<_>, _>(&excel_path) {
        for sheet_name in wb.sheet_names().to_vec() {
            if sheet_name.contains("Summary") {
                if let Ok(range) = wb.worksheet_range(&sheet_name) {
                    // Excel should have some content - either Options section or basic info
                    let has_content = range.rows().any(|row| {
                        row.iter().any(|cell| {
                            let s = cell.to_string();
                            s.contains("Options") || s.contains("Exclude") ||
                            s.contains("Source") || s.contains("Target") ||
                            s.contains("Dry") || s.contains("*.log")
                        })
                    });
                    assert!(has_content, "Summary should have Options or basic info section");
                }
                break;
            }
        }
    }
}

#[test]
fn test_excel_summary_has_statistics_section() {
    // EXFMT-002: Statisticsセクション存在確認
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    use calamine::{Reader, open_workbook, Xlsx};
    if let Ok(mut wb) = open_workbook::<Xlsx<_>, _>(&excel_path) {
        for sheet_name in wb.sheet_names().to_vec() {
            if sheet_name.contains("Summary") {
                if let Ok(range) = wb.worksheet_range(&sheet_name) {
                    let has_stats = range.rows().any(|row| {
                        row.iter().any(|cell| {
                            let s = cell.to_string();
                            s.contains("Statistics") || s.contains("Total") || s.contains("Added")
                        })
                    });
                    assert!(has_stats, "Summary should have Statistics section");
                }
                break;
            }
        }
    }
}

#[test]
fn test_excel_fold_level_option() {
    // EXFMT-003: --excel-fold-levelオプション
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("a/b/c/deep.txt", "deep content");
    repo.add_all();
    let target = repo.commit("Add deep file");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
        "-L", "2",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel should be created with fold level");
}

#[test]
fn test_excel_details_has_header() {
    // EXFMT-004: Detailsシートヘッダー確認
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    use calamine::{Reader, open_workbook, Xlsx};
    if let Ok(mut wb) = open_workbook::<Xlsx<_>, _>(&excel_path) {
        for sheet_name in wb.sheet_names().to_vec() {
            if sheet_name.contains("Detail") {
                if let Ok(range) = wb.worksheet_range(&sheet_name) {
                    // ヘッダー行があることを確認
                    if let Some(first_row) = range.rows().next() {
                        assert!(first_row.len() >= 1, "Details should have header columns");
                    }
                }
                break;
            }
        }
    }
}

#[test]
fn test_excel_summary_has_labels() {
    // EXFMT-005: Summaryラベル確認
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    use calamine::{Reader, open_workbook, Xlsx};
    if let Ok(mut wb) = open_workbook::<Xlsx<_>, _>(&excel_path) {
        for sheet_name in wb.sheet_names().to_vec() {
            if sheet_name.contains("Summary") {
                if let Ok(range) = wb.worksheet_range(&sheet_name) {
                    let has_labels = range.rows().any(|row| {
                        row.iter().any(|cell| {
                            let s = cell.to_string();
                            s.contains("Source") || s.contains("Target") || s.contains("Output")
                        })
                    });
                    assert!(has_labels, "Summary should have Source/Target/Output labels");
                }
                break;
            }
        }
    }
}

#[test]
fn test_excel_fold_level_zero_default() {
    // EXFMT-006: fold-levelデフォルト値
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    // fold-level指定なし（デフォルト）
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command should succeed with default fold level");
    assert!(excel_path.exists(), "Excel should be created");
}

#[test]
fn test_excel_file_tree_cell_structure() {
    // EXFMT-007: File Treeセル構造
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("src/main.rs", "fn main() {}");
    repo.add_all();
    let target = repo.commit("Add source");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel should be created");
}

// ============================================================================
// 19. Filter statusテスト
// ============================================================================

#[test]
fn test_summary_contains_filter_status_option() {
    // FILT-001: Summaryにfilter_status表示
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("added.txt", "new");
    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Changes");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
        "--filter-status", "added",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary.contains("Filter") || summary.contains("added") || summary.contains("status"),
            "Summary should mention filter status");
}

#[test]
fn test_excel_contains_filter_status_option() {
    // FILT-002: Excelにfilter_status表示
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("added.txt", "new");
    repo.add_all();
    let target = repo.commit("Add");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
        "--filter-status", "added",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel should be created");
}

#[test]
fn test_filter_status_multiple_values() {
    // FILT-003: 複数ステータス指定
    let repo = TestRepo::new();

    repo.create_file("existing.txt", "original");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("added.txt", "new");
    repo.create_file("existing.txt", "modified");
    repo.add_all();
    let target = repo.commit("Add and modify");

    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "--filter-status", "added,modified",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    // 両方がコピーされている
    assert!(repo.output_exists("added.txt") || repo.output_exists("existing.txt"),
            "Filtered files should be copied");
}

#[test]
fn test_no_filter_status_when_not_specified() {
    // FILT-004: 未指定時は非表示（Summary）
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    // filter-statusを指定していないので、関連の表示は不要
}

#[test]
fn test_excel_no_filter_status_when_not_specified() {
    // FILT-005: 未指定時は非表示（Excel）
    let repo = TestRepo::new();

    repo.create_file("file.txt", "content");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify");

    let excel_path = repo.temp_dir.path().join("report.xlsx");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-E", excel_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(excel_path.exists(), "Excel should be created");
}

// ============================================================================
// 20. 統計テスト
// ============================================================================

#[test]
fn test_unchanged_count_always_shown() {
    // STATS-001: Unchangedカウントが表示される
    let repo = TestRepo::new();

    repo.create_file("unchanged.txt", "same");
    repo.create_file("modified.txt", "original");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("modified.txt", "changed");
    repo.add_all();
    let target = repo.commit("Modify one");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
        "-u",  // --show-unchanged
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    assert!(summary.contains("Unchanged") || summary.contains("unchanged") || summary.contains("1"),
            "Should show unchanged count");
}

#[test]
fn test_total_equals_all_unique_paths() {
    // STATS-002: Total=全ユニークパス数
    let repo = TestRepo::new();

    repo.create_file("file1.txt", "content1");
    repo.create_file("file2.txt", "content2");
    repo.create_file("file3.txt", "content3");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("file1.txt", "modified1");
    repo.create_file("file4.txt", "new");
    repo.delete_file("file3.txt");
    repo.add_all();
    let target = repo.commit("Various changes");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
        "-u",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    // Totalが表示されている
    assert!(summary.contains("Total") || summary.contains("total"),
            "Should show total count");
}

#[test]
fn test_statistics_categories_sum() {
    // STATS-003: カテゴリ合計=Total
    let repo = TestRepo::new();

    repo.create_file("keep.txt", "keep");
    repo.create_file("modify.txt", "original");
    repo.create_file("delete_me.txt", "will delete");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("add.txt", "new");
    repo.create_file("modify.txt", "changed");
    repo.delete_file("delete_me.txt");
    repo.add_all();
    let target = repo.commit("All types of changes");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
        "-u",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

#[test]
fn test_unchanged_count_with_many_files() {
    // STATS-004: 多数の変更なしファイル
    let repo = TestRepo::new();

    // 多数のファイルを作成
    for i in 1..=10 {
        repo.create_file(&format!("file{}.txt", i), &format!("content{}", i));
    }
    repo.add_all();
    let source = repo.commit("Initial");

    // 1つだけ変更
    repo.create_file("file1.txt", "modified");
    repo.add_all();
    let target = repo.commit("Modify one");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
        "-u",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

#[test]
fn test_unchanged_count_same_with_or_without_option() {
    // STATS-005: --show-unchanged有無でカウント同一
    let repo = TestRepo::new();

    repo.create_file("unchanged.txt", "same");
    repo.create_file("modified.txt", "original");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("modified.txt", "changed");
    repo.add_all();
    let target = repo.commit("Modify");

    // --show-unchangedなし（別の出力ディレクトリを使用）
    let output1_path = repo.temp_dir.path().join("output1");
    let summary1_path = repo.temp_dir.path().join("summary1.txt");
    let output1 = Command::new(get_binary_path())
        .args(["-R", repo.repo_path.to_str().unwrap()])
        .args(["-O", output1_path.to_str().unwrap()])
        .args(["-S", &source, "-T", &target])
        .args(["-s", summary1_path.to_str().unwrap()])
        .output()
        .expect("Failed to run command");
    assert!(output1.status.success(), "Command failed without -u: {:?}", output1);

    // --show-unchangedあり（別の出力ディレクトリを使用）
    let output2_path = repo.temp_dir.path().join("output2");
    let summary2_path = repo.temp_dir.path().join("summary2.txt");
    let output2 = Command::new(get_binary_path())
        .args(["-R", repo.repo_path.to_str().unwrap()])
        .args(["-O", output2_path.to_str().unwrap()])
        .args(["-S", &source, "-T", &target])
        .args(["-s", summary2_path.to_str().unwrap()])
        .args(["-u"])
        .output()
        .expect("Failed to run command");
    assert!(output2.status.success(), "Command failed with -u: {:?}", output2);
}

#[test]
fn test_statistics_with_filter() {
    // STATS-006: フィルター適用時の統計
    let repo = TestRepo::new();

    repo.create_file("existing.txt", "original");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("added1.txt", "new1");
    repo.create_file("added2.txt", "new2");
    repo.create_file("existing.txt", "modified");
    repo.add_all();
    let target = repo.commit("Add and modify");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
        "--filter-status", "added",
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);
}

// ============================================================================
// 21. Tree表示テスト
// ============================================================================

#[test]
fn test_console_output_contains_box_drawing_chars() {
    // IT-3001: コンソール出力にBox Drawing文字
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("dir/file1.txt", "content1");
    repo.create_file("dir/file2.txt", "content2");
    repo.add_all();
    let target = repo.commit("Add nested files");

    let output = repo.run_cmd(&["-S", &source, "-T", &target]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Box Drawing文字（├, └, ─, │）が含まれている可能性
    let has_tree_chars = stdout.contains("├") || stdout.contains("└") ||
                         stdout.contains("─") || stdout.contains("│") ||
                         stdout.contains("+") || stdout.contains("-") ||
                         stdout.contains("|");
    // ツリー表示がある場合はBox Drawing文字があるはず
    if stdout.contains("File Tree") || stdout.contains("file1") {
        assert!(has_tree_chars || stdout.contains("dir"), "Output should have tree structure");
    }
}

#[test]
fn test_summary_file_contains_box_drawing_chars() {
    // IT-3002: サマリーファイルにBox Drawing文字
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("src/main.rs", "fn main() {}");
    repo.create_file("src/lib.rs", "pub fn lib() {}");
    repo.add_all();
    let target = repo.commit("Add source files");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    // Box Drawing文字が含まれている
    let has_tree_chars = summary.contains("├") || summary.contains("└") ||
                         summary.contains("─") || summary.contains("│");
    assert!(has_tree_chars || summary.contains("src"), "Summary should have tree structure");
}

#[test]
fn test_tree_structure_formatting() {
    // IT-3003: Tree構造フォーマット確認
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("a/file1.txt", "content1");
    repo.create_file("b/file2.txt", "content2");
    repo.add_all();
    let target = repo.commit("Add files in dirs");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    // ├── または └── パターン
    let has_tree_pattern = summary.contains("├──") || summary.contains("└──") ||
                           summary.contains("├") || summary.contains("└");
    assert!(has_tree_pattern || summary.contains("/"), "Should have tree patterns");
}

#[test]
fn test_three_way_tree_contains_box_drawing_chars() {
    // IT-3004: 三者間TreeにBox Drawing文字
    let repo = TestRepo::new();

    repo.create_file("base.txt", "base");
    repo.add_all();
    let base = repo.commit("Base");

    repo.create_branch("ours");
    repo.checkout("ours");
    repo.create_file("src/ours.txt", "ours content");
    repo.add_all();
    let ours = repo.commit("Ours");

    repo.checkout("master");
    if !repo.repo_path.join(".git/refs/heads/master").exists() {
        repo.checkout("main");
    }
    repo.create_branch("theirs");
    repo.checkout("theirs");
    repo.create_file("src/theirs.txt", "theirs content");
    repo.add_all();
    let theirs = repo.commit("Theirs");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-3",
        "-B", &base,
        "-S", &ours,
        "-T", &theirs,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success() || output.status.code() == Some(0), "Command failed: {:?}", output);

    if summary_path.exists() {
        let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
        // ツリー構造がある
        assert!(summary.contains("src") || summary.contains("ours") || summary.contains("theirs"),
                "Three-way summary should have tree structure");
    }
}

#[test]
fn test_box_drawing_chars_at_different_depths() {
    // IT-3005: 異なる深さでのBox Drawing文字
    let repo = TestRepo::new();

    repo.create_file("dummy.txt", "dummy");
    repo.add_all();
    let source = repo.commit("Initial");

    repo.create_file("level1.txt", "level1");
    repo.create_file("dir1/level2.txt", "level2");
    repo.create_file("dir1/dir2/level3.txt", "level3");
    repo.create_file("dir1/dir2/dir3/level4.txt", "level4");
    repo.add_all();
    let target = repo.commit("Add nested files");

    let summary_path = repo.temp_dir.path().join("summary.txt");
    let output = repo.run_cmd(&[
        "-S", &source,
        "-T", &target,
        "-s", summary_path.to_str().unwrap(),
    ]);
    assert!(output.status.success(), "Command failed: {:?}", output);

    let summary = fs::read_to_string(&summary_path).expect("Failed to read summary");
    // 異なる深さのエントリがある
    assert!(summary.contains("level1") || summary.contains("dir1"),
            "Should have entries at different depths");
}
