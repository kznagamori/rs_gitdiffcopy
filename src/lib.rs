//! rs_gitdiffcopy - Git diff コマンドの出力を視覚的に分かりやすくするツール

pub mod cli;
pub mod config;
pub mod copy;
pub mod diff;
pub mod error;
pub mod excel;
pub mod git;
pub mod safety;
pub mod summary;
pub mod types;

pub use error::{AppError, ExitCode, Result};
