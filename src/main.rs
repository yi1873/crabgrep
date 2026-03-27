use clap::Parser;
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

mod search;
mod output;

/// 🦀 crabgrep - 一个用 Rust 写的文件搜索工具
#[derive(Parser, Debug)]
#[command(
    name = "crabgrep",
    version = "0.1.0",
    author = "Your Name <you@example.com>",
    about = "在文件中搜索关键词，比你手快，比你准"
)]
struct Cli {
    /// 要搜索的关键词
    pattern: String,

    /// 搜索的目标路径（文件或目录）
    #[arg(default_value = ".")]
    path: PathBuf,

    /// 忽略大小写
    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,

    /// 递归搜索子目录
    #[arg(short = 'r', long = "recursive", default_value_t = true)]
    recursive: bool,

    /// 每个文件最多显示的匹配结果数 (0 表示显示全部)
    #[arg(short = 'm', long = "max-results", default_value_t = 5)]
    max_results: usize,
}

fn main() {
    // 把真正的逻辑放到 run() 里，main 只负责处理顶层错误
    if let Err(e) = run() {
        eprintln!("{} {}", "错误:".red().bold(), e);

        // 打印错误链（anyhow 的 context 会形成错误链）
        for cause in e.chain().skip(1) {
            eprintln!("  原因: {}", cause);
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // 验证路径存在
    if !cli.path.exists() {
        anyhow::bail!("路径不存在: {}", cli.path.display());
    }

    let config = search::SearchConfig {
        pattern: cli.pattern.clone(),
        ignore_case: cli.ignore_case,
        recursive: cli.recursive,
    };

    println!(
        "🔍 在 {} 中搜索 \"{}\"{}...",
        cli.path.display(),
        cli.pattern,
        if cli.ignore_case { "（忽略大小写）" } else { "" }
    );

    let results = search::search(&cli.path, &config)?;
    output::print_results(&results, &cli.pattern, cli.ignore_case, cli.max_results);

    Ok(())
}
