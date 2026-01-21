//! エラー型定義

use thiserror::Error;

/// アプリケーションのエラー型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not a git repository (or any parent up to /)")]
    NotGitRepository,

    #[error("Git command not found. Please install Git or specify path with --git-path or settings.toml/--config.")]
    GitNotFound,

    #[error("Unknown revision '{0}'")]
    UnknownRevision(String),

    #[error("Failed to get file '{path}' from commit {commit}")]
    FileGetFailed { path: String, commit: String },

    #[error("Output directory '{0}' already exists. Use --force to overwrite.")]
    OutputExists(String),

    #[error("Failed to read config file '{path}': {detail}")]
    ConfigReadFailed { path: String, detail: String },

    #[error("Missing required field '{0}' in config file")]
    MissingRequiredField(String),

    #[error("Invalid glob pattern '{0}'")]
    InvalidGlobPattern(String),

    #[error("Cannot use --force on protected path '{0}'")]
    ProtectedPath(String),

    #[error("Could not resolve host '{0}'")]
    HostResolveFailed(String),

    #[error("Failed to clone repository '{0}'")]
    CloneFailed(String),

    #[error("Authentication failed for '{0}'")]
    AuthenticationFailed(String),

    #[error("Could not fetch ref '{0}' from remote")]
    RemoteRefFetchFailed(String),

    #[error("Git command failed: {0}")]
    GitCommandFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid filter status: {0}")]
    InvalidFilterStatus(String),

    #[error("Three-way mode requires --base option")]
    ThreeWayRequiresBase,

    #[error("{0}")]
    Other(String),
}

/// 終了コード
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// 正常終了（差分あり）
    Success = 0,
    /// エラー終了
    Error = 1,
    /// 正常終了（差分なし）
    NoDifference = 2,
    /// 正常終了（コンフリクトあり）
    Conflict = 3,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
