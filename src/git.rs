//! Gitコマンドの実行

use crate::error::{AppError, Result};
use crate::types::{DiffFile, FileStatus};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Gitコマンド実行用の構造体
#[derive(Debug, Clone)]
pub struct Git {
    /// gitコマンドのパス
    git_path: PathBuf,
    /// リポジトリのパス
    repo_path: PathBuf,
}

impl Git {
    /// 新しいGitインスタンスを作成
    pub fn new(git_path: Option<&Path>, repo_path: &Path) -> Result<Self> {
        let git_path = Self::resolve_git_path(git_path)?;
        let repo_path = Self::find_repo_root(repo_path, &git_path)?;

        Ok(Self {
            git_path,
            repo_path,
        })
    }

    /// gitコマンドのパスを解決
    fn resolve_git_path(specified_path: Option<&Path>) -> Result<PathBuf> {
        if let Some(path) = specified_path {
            if path.exists() {
                return Ok(path.to_path_buf());
            }
            return Err(AppError::GitNotFound);
        }

        // PATHから検索
        if let Ok(output) = Command::new("which").arg("git").output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return Ok(PathBuf::from(path));
            }
        }

        // Windowsの場合
        if cfg!(windows) {
            if let Ok(output) = Command::new("where").arg("git").output() {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !path.is_empty() {
                        return Ok(PathBuf::from(path));
                    }
                }
            }
        }

        Err(AppError::GitNotFound)
    }

    /// リポジトリのルートディレクトリを検索
    fn find_repo_root(start_path: &Path, git_path: &Path) -> Result<PathBuf> {
        let output = Command::new(git_path)
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(start_path)
            .output()?;

        if output.status.success() {
            let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(PathBuf::from(root))
        } else {
            Err(AppError::NotGitRepository)
        }
    }

    /// Gitコマンドを実行
    fn run(&self, args: &[&str]) -> Result<Output> {
        let output = Command::new(&self.git_path)
            .args(args)
            .current_dir(&self.repo_path)
            .output()?;

        Ok(output)
    }

    /// Gitコマンドを実行して成功した場合に出力を返す
    fn run_success(&self, args: &[&str]) -> Result<String> {
        let output = self.run(args)?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(AppError::GitCommandFailed(stderr.to_string()))
        }
    }

    /// リポジトリパスを取得
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// refをコミットハッシュに解決
    pub fn resolve_ref(&self, ref_name: &str) -> Result<String> {
        let output = self.run_success(&["rev-parse", ref_name])?;
        let hash = output.trim().to_string();

        if hash.is_empty() {
            return Err(AppError::UnknownRevision(ref_name.to_string()));
        }

        Ok(hash)
    }

    /// 差分ファイルリストを取得
    pub fn diff_name_status(&self, source: &str, target: &str) -> Result<Vec<DiffFile>> {
        let output = self.run_success(&["diff", "--name-status", "-M", "-C", source, target])?;
        let mut files = Vec::new();

        for line in output.lines() {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.is_empty() {
                continue;
            }

            let status_str = parts[0];
            let status = match status_str.chars().next() {
                Some('A') => FileStatus::Added,
                Some('M') => FileStatus::Modified,
                Some('D') => FileStatus::Deleted,
                Some('R') => FileStatus::Renamed,
                Some('C') => FileStatus::Copied,
                Some('T') => FileStatus::TypeChanged,
                _ => continue,
            };

            let (path, original_path, similarity) = match status {
                FileStatus::Renamed | FileStatus::Copied => {
                    if parts.len() >= 3 {
                        let sim = status_str[1..].parse::<u8>().ok();
                        (
                            PathBuf::from(parts[2]),
                            Some(PathBuf::from(parts[1])),
                            sim,
                        )
                    } else {
                        continue;
                    }
                }
                _ => {
                    if parts.len() >= 2 {
                        (PathBuf::from(parts[1]), None, None)
                    } else {
                        continue;
                    }
                }
            };

            let mut file = DiffFile::new(path, status);
            file.original_path = original_path;
            file.similarity = similarity;
            files.push(file);
        }

        Ok(files)
    }

    /// ファイル内容を取得
    pub fn show(&self, commit: &str, path: &Path) -> Result<Vec<u8>> {
        let spec = format!("{}:{}", commit, path.display());
        let output = self.run(&["show", &spec])?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(AppError::FileGetFailed {
                path: path.display().to_string(),
                commit: commit.to_string(),
            })
        }
    }

    /// ファイルのモード（権限）を取得
    pub fn ls_tree(&self, commit: &str, path: &Path) -> Result<Option<String>> {
        let output = self.run_success(&["ls-tree", commit, &path.display().to_string()])?;

        if output.is_empty() {
            return Ok(None);
        }

        // 出力形式: 100644 blob abc123... path/to/file.txt
        let parts: Vec<&str> = output.split_whitespace().collect();
        if parts.len() >= 4 {
            Ok(Some(parts[0].to_string()))
        } else {
            Ok(None)
        }
    }

    /// コミット時のファイル一覧を取得（blob ID付き）
    pub fn ls_tree_recursive(&self, commit: &str) -> Result<HashMap<PathBuf, (String, String)>> {
        let output = self.run_success(&["ls-tree", "-r", commit])?;
        let mut files = HashMap::new();

        for line in output.lines() {
            // 出力形式: 100644 blob abc123...    path/to/file.txt
            let parts: Vec<&str> = line.splitn(4, |c| c == ' ' || c == '\t').collect();
            if parts.len() >= 4 {
                let mode = parts[0].to_string();
                let blob_id = parts[2].to_string();
                let path = PathBuf::from(parts[3].trim());
                files.insert(path, (mode, blob_id));
            }
        }

        Ok(files)
    }

    /// コミット日時を取得
    pub fn get_commit_date(&self, commit: &str, path: Option<&Path>) -> Result<String> {
        let mut args = vec!["log", "-1", "--format=%ci", commit];
        let path_str;
        if let Some(p) = path {
            args.push("--");
            path_str = p.display().to_string();
            args.push(&path_str);
        }

        let output = self.run_success(&args)?;
        Ok(output.trim().to_string())
    }

    /// 個別ファイルのパッチを生成
    pub fn diff_patch(&self, source: &str, target: &str, path: &Path) -> Result<String> {
        let path_str = path.display().to_string();
        let output = self.run_success(&["diff", "-M", "-C", source, target, "--", &path_str])?;
        Ok(output)
    }

    /// 全変更の統合パッチを生成
    pub fn diff_patch_all(&self, source: &str, target: &str) -> Result<String> {
        let output = self.run_success(&["diff", "-M", "-C", source, target])?;
        Ok(output)
    }

    /// リモートリポジトリかどうかを判定
    pub fn is_remote_url(url: &str) -> bool {
        url.starts_with("https://")
            || url.starts_with("http://")
            || url.starts_with("git://")
            || url.starts_with("git@")
            || url.starts_with("ssh://")
    }

    /// リモートリポジトリをクローン
    pub fn clone_remote(
        git_path: Option<&Path>,
        url: &str,
        dest: &Path,
        shallow: bool,
    ) -> Result<Self> {
        let git_path = Self::resolve_git_path(git_path)?;
        let dest_str = dest.display().to_string();

        let mut args = vec!["clone"];
        if shallow {
            args.extend(["--depth=1", "--no-checkout"]);
        }
        args.extend([url, &dest_str]);

        let output = Command::new(&git_path).args(&args).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("Authentication") || stderr.contains("Permission denied") {
                return Err(AppError::AuthenticationFailed(url.to_string()));
            }
            if stderr.contains("Could not resolve host") {
                let host = url
                    .split("://")
                    .nth(1)
                    .and_then(|s| s.split('/').next())
                    .unwrap_or(url);
                return Err(AppError::HostResolveFailed(host.to_string()));
            }
            return Err(AppError::CloneFailed(url.to_string()));
        }

        Ok(Self {
            git_path,
            repo_path: dest.to_path_buf(),
        })
    }

    /// リモートrefを取得
    pub fn fetch_ref(&self, ref_name: &str, shallow: bool) -> Result<()> {
        let mut args = vec!["fetch", "origin", ref_name];
        if shallow {
            args.insert(1, "--depth=1");
        }

        let output = self.run(&args)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("couldn't find remote ref") {
                return Err(AppError::RemoteRefFetchFailed(ref_name.to_string()));
            }
            return Err(AppError::GitCommandFailed(stderr.to_string()));
        }

        Ok(())
    }

    /// リポジトリが有効かどうかを確認
    pub fn is_valid_repository(&self) -> bool {
        self.run(&["rev-parse", "--git-dir"])
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}
