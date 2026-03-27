use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;

/// 单条搜索结果
#[derive(Debug)]
pub struct SearchResult {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize, // 匹配开始位置（用于高亮）
    pub match_end: usize,   // 匹配结束位置
}

/// 搜索配置
pub struct SearchConfig {
    pub pattern: String,
    pub ignore_case: bool,
    pub recursive: bool,
}

/// 在单个文件中搜索
pub fn search_in_file(
    path: &Path,
    config: &SearchConfig,
) -> Result<Vec<SearchResult>> {
    // 读取文件内容，遇到错误时附带上下文信息
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("无法读取文件: {}", path.display()))?;

    let pattern = if config.ignore_case {
        config.pattern.to_lowercase()
    } else {
        config.pattern.clone()
    };

    let mut results = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        let search_line = if config.ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        // 找到所有匹配位置
        if let Some(match_start) = search_line.find(&pattern) {
            results.push(SearchResult {
                file_path: path.display().to_string(),
                line_number: line_idx + 1, // 行号从 1 开始
                line_content: line.to_string(),
                match_start,
                match_end: match_start + pattern.len(),
            });
        }
    }

    Ok(results)
}

/// 在路径（文件或目录）中搜索
pub fn search(
    path: &Path,
    config: &SearchConfig,
) -> Result<Vec<SearchResult>> {
    let mut all_results = Vec::new();

    if path.is_file() {
        // 直接搜索单个文件
        let results = search_in_file(path, config)?;
        all_results.extend(results);
    } else if path.is_dir() {
        // 遍历目录
        let walker = if config.recursive {
            WalkDir::new(path)
        } else {
            WalkDir::new(path).max_depth(1)
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();

            // 只处理文件，跳过目录和隐藏文件
            if entry_path.is_file() && !is_hidden(entry_path) {
                // 跳过二进制文件
                if is_likely_binary(entry_path) {
                    continue;
                }

                // 尝试读取文件，失败则直接跳过（不打印警告）
                if let Ok(results) = search_in_file(entry_path, config) {
                    all_results.extend(results);
                }
            }
        }
    }

    Ok(all_results)
}

/// 判断是否是隐藏文件（以 . 开头）
fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

/// 简单判断是否可能是二进制文件
fn is_likely_binary(path: &Path) -> bool {
    let binary_extensions = [
        "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg",
        "pdf", "zip", "tar", "gz", "rar", "7z",
        "exe", "dll", "so", "dylib", "bin",
        "mp3", "mp4", "avi", "mov", "flv",
        "woff", "woff2", "ttf", "eot",
    ];

    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| binary_extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("创建临时文件失败");
        file.write_all(content.as_bytes()).expect("写入失败");
        file
    }

    #[test]
    fn test_search_basic() {
        let file = create_temp_file("hello world\nrust is awesome\nhello again");
        let config = SearchConfig {
            pattern: "hello".to_string(),
            ignore_case: false,
            recursive: false,
        };

        let results = search_in_file(file.path(), &config).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 3);
    }

    #[test]
    fn test_search_ignore_case() {
        let file = create_temp_file("Hello World\nhello world\nHELLO WORLD");
        let config = SearchConfig {
            pattern: "hello".to_string(),
            ignore_case: true,
            recursive: false,
        };

        let results = search_in_file(file.path(), &config).unwrap();
        assert_eq!(results.len(), 3); // 三行都匹配
    }

    #[test]
    fn test_search_no_match() {
        let file = create_temp_file("rust is great\nno match here");
        let config = SearchConfig {
            pattern: "python".to_string(),
            ignore_case: false,
            recursive: false,
        };

        let results = search_in_file(file.path(), &config).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_file_not_found() {
        let config = SearchConfig {
            pattern: "test".to_string(),
            ignore_case: false,
            recursive: false,
        };

        let result = search_in_file(std::path::Path::new("/不存在的路径/file.txt"), &config);
        assert!(result.is_err());
    }
}
