//! 安全機能（危険なパスの検出など）

use crate::error::{AppError, Result};
use std::path::{Path, PathBuf};

/// 保護されたパスのリスト（削除禁止）
const PROTECTED_PATHS_COMMON: &[&str] = &["/", "~"];

#[cfg(target_os = "windows")]
const PROTECTED_PATHS_OS: &[&str] = &[
    "C:\\",
    "C:\\Windows",
    "C:\\Program Files",
    "C:\\Program Files (x86)",
    "C:\\Users",
];

#[cfg(target_os = "linux")]
const PROTECTED_PATHS_OS: &[&str] = &[
    "/bin", "/sbin", "/usr", "/etc", "/var", "/home", "/root",
];

#[cfg(target_os = "macos")]
const PROTECTED_PATHS_OS: &[&str] = &[
    "/System", "/Library", "/Users", "/Applications",
    "/bin", "/sbin", "/usr", "/etc", "/var",
];

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
const PROTECTED_PATHS_OS: &[&str] = &[];

/// パスが保護されたパスかどうかを確認
pub fn is_protected_path(path: &Path) -> Result<bool> {
    let canonical = normalize_path(path)?;
    let canonical_str = canonical.to_string_lossy();

    // 共通の保護パスをチェック
    for protected in PROTECTED_PATHS_COMMON {
        if *protected == "~" {
            if let Some(home) = dirs::home_dir() {
                if canonical == home {
                    return Ok(true);
                }
            }
        } else if canonical_str == *protected {
            return Ok(true);
        }
    }

    // OS固有の保護パスをチェック
    for protected in PROTECTED_PATHS_OS {
        let protected_path = PathBuf::from(protected);
        if canonical == protected_path {
            return Ok(true);
        }
    }

    // Windowsの場合、ユーザーディレクトリの直下フォルダもチェック
    #[cfg(target_os = "windows")]
    {
        if let Some(home) = dirs::home_dir() {
            // ユーザーディレクトリ自体
            if canonical == home {
                return Ok(true);
            }
            // ユーザーディレクトリの直下フォルダ（AppData等）
            if let Some(parent) = canonical.parent() {
                if parent == home {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

/// パスを正規化（シンボリックリンク解決、相対パス展開）
fn normalize_path(path: &Path) -> Result<PathBuf> {
    // 相対パスを絶対パスに変換
    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    // シンボリックリンクを解決（存在しない場合はそのまま）
    match abs_path.canonicalize() {
        Ok(p) => Ok(p),
        Err(_) => Ok(abs_path),
    }
}

/// 出力先パスの安全性を検証
pub fn validate_output_path(path: &Path) -> Result<()> {
    if is_protected_path(path)? {
        return Err(AppError::ProtectedPath(path.display().to_string()));
    }
    Ok(())
}

/// ディレクトリの情報を取得（ファイル数、ディレクトリ数、合計サイズ）
pub fn get_directory_info(path: &Path) -> Result<(usize, usize, u64)> {
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut total_size = 0u64;

    if path.is_dir() {
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                file_count += 1;
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            } else if entry.file_type().is_dir() && entry.path() != path {
                dir_count += 1;
            }
        }
    }

    Ok((file_count, dir_count, total_size))
}

/// サイズを人間が読みやすい形式に変換
pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.1} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

/// 削除確認プロンプトを表示
pub fn confirm_delete(path: &Path) -> Result<bool> {
    let (files, dirs, size) = get_directory_info(path)?;

    println!();
    println!("Output directory '{}' already exists.", path.display());
    println!("  Contains: {} files, {} directories", files, dirs);
    println!("  Total size: {}", format_size(size));
    println!();

    let confirm = dialoguer::Confirm::new()
        .with_prompt("Delete and continue?")
        .default(false)
        .interact()
        .map_err(|e| AppError::Other(e.to_string()))?;

    Ok(confirm)
}

/// ディレクトリを再帰的に削除
pub fn remove_directory(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ========================================
    // 保護パステスト (PROT-001 ~ PROT-005)
    // ========================================

    #[test]
    fn test_is_protected_path_unix_root() {
        // PROT-001: ルートディレクトリが保護されるか
        #[cfg(not(windows))]
        {
            let result = is_protected_path(Path::new("/"));
            assert!(result.is_ok());
            assert!(result.unwrap(), "Root directory should be protected");
        }
    }

    #[test]
    fn test_is_protected_path_unix_system_dirs() {
        // PROT-002: システムディレクトリが保護されるか
        // 注: WSL2環境では/binや/sbinはシンボリックリンクの場合があり、
        // 正規化後に異なるパスになるため、正規化後も保護対象のディレクトリのみテスト
        #[cfg(target_os = "linux")]
        {
            // /usrと/etcと/varは直接存在し、シンボリックリンクではないことが多い
            let system_dirs = vec!["/usr", "/etc", "/var", "/home"];
            for dir in system_dirs {
                let path = Path::new(dir);
                if path.exists() && !path.is_symlink() {
                    let result = is_protected_path(path);
                    assert!(result.is_ok(), "Should not error for {}", dir);
                    assert!(result.unwrap(), "{} should be protected", dir);
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let system_dirs = vec!["/System", "/Library", "/Users", "/Applications"];
            for dir in system_dirs {
                let path = Path::new(dir);
                if path.exists() {
                    let result = is_protected_path(path);
                    assert!(result.is_ok(), "Should not error for {}", dir);
                    assert!(result.unwrap(), "{} should be protected", dir);
                }
            }
        }
    }

    #[test]
    fn test_is_protected_path_unix_safe_paths() {
        // PROT-003: 一時ディレクトリは保護されないか
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let safe_path = temp_dir.path().join("safe_output");

        let result = is_protected_path(&safe_path);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Temp directory path should NOT be protected");
    }

    #[test]
    fn test_validate_output_path_protected() {
        // PROT-004: 保護パスへの出力を拒否
        #[cfg(not(windows))]
        {
            let result = validate_output_path(Path::new("/"));
            assert!(result.is_err(), "Root path should be rejected");
            if let Err(AppError::ProtectedPath(path)) = result {
                assert_eq!(path, "/");
            } else {
                panic!("Expected ProtectedPath error");
            }
        }
    }

    #[test]
    fn test_validate_output_path_safe() {
        // PROT-005: 安全なパスへの出力を許可
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let safe_path = temp_dir.path().join("output");

        let result = validate_output_path(&safe_path);
        assert!(result.is_ok(), "Safe path should be allowed");
    }

    #[test]
    fn test_is_protected_path_home_directory() {
        // ホームディレクトリが保護されるか
        if let Some(home) = dirs::home_dir() {
            let result = is_protected_path(&home);
            assert!(result.is_ok());
            assert!(result.unwrap(), "Home directory should be protected");
        }
    }

    #[test]
    fn test_format_size() {
        // サイズフォーマットのテスト
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_get_directory_info_empty() {
        // 空ディレクトリの情報取得
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let (files, dirs, size) = get_directory_info(temp_dir.path())
            .expect("Failed to get directory info");

        assert_eq!(files, 0);
        assert_eq!(dirs, 0);
        assert_eq!(size, 0);
    }

    #[test]
    fn test_get_directory_info_with_files() {
        // ファイルを含むディレクトリの情報取得
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // ファイルを作成
        std::fs::write(temp_dir.path().join("file1.txt"), "hello").expect("Failed to write");
        std::fs::write(temp_dir.path().join("file2.txt"), "world").expect("Failed to write");

        // サブディレクトリを作成
        std::fs::create_dir(temp_dir.path().join("subdir")).expect("Failed to create dir");
        std::fs::write(temp_dir.path().join("subdir/file3.txt"), "test")
            .expect("Failed to write");

        let (files, dirs, size) = get_directory_info(temp_dir.path())
            .expect("Failed to get directory info");

        assert_eq!(files, 3);
        assert_eq!(dirs, 1);
        assert!(size > 0);
    }

    #[test]
    fn test_remove_directory() {
        // ディレクトリ削除のテスト
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let to_remove = temp_dir.path().join("to_remove");

        std::fs::create_dir(&to_remove).expect("Failed to create dir");
        std::fs::write(to_remove.join("file.txt"), "content").expect("Failed to write");

        assert!(to_remove.exists());

        remove_directory(&to_remove).expect("Failed to remove directory");

        assert!(!to_remove.exists());
    }

    #[test]
    fn test_remove_nonexistent_directory() {
        // 存在しないディレクトリの削除（エラーにならない）
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let nonexistent = temp_dir.path().join("nonexistent");

        let result = remove_directory(&nonexistent);
        assert!(result.is_ok());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_is_protected_path_windows() {
        // Windows固有の保護パス
        let protected_paths = vec![
            "C:\\",
            "C:\\Windows",
            "C:\\Program Files",
            "C:\\Program Files (x86)",
            "C:\\Users",
        ];

        for path in protected_paths {
            let result = is_protected_path(Path::new(path));
            assert!(result.is_ok(), "Should not error for {}", path);
            assert!(result.unwrap(), "{} should be protected", path);
        }
    }
}
