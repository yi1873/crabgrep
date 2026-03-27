use colored::*;
use crate::search::SearchResult;

/// 打印搜索结果
pub fn print_results(results: &[SearchResult], _pattern: &str, _ignore_case: bool, max_results: usize) {
    if results.is_empty() {
        println!("{}", "没有找到匹配内容。".yellow());
        return;
    }

    // 按文件分组结果
    let mut file_results: std::collections::HashMap<String, Vec<&SearchResult>> = std::collections::HashMap::new();
    for result in results {
        file_results
            .entry(result.file_path.clone())
            .or_insert_with(Vec::new)
            .push(result);
    }

    let file_count = file_results.len();

    // 打印每个文件的结果
    for (file_path, file_hits) in file_results {
        println!("\n{}", file_path.cyan().bold().underline());

        let display_count = if max_results == 0 {
            // 0 表示显示全部
            file_hits.len()
        } else {
            file_hits.len().min(max_results)
        };

        // 只显示前 display_count 条结果
        for (i, result) in file_hits.iter().take(display_count).enumerate() {
            // 打印行号
            print!("  {}{}  ", result.line_number.to_string().green(), ":".green());

            // 打印行内容，高亮匹配部分
            print_highlighted_line(
                &result.line_content,
                result.match_start,
                result.match_end,
            );

            // 如果达到最大显示数量且还有更多结果
            if i == display_count - 1 && display_count < file_hits.len() {
                let remaining = file_hits.len() - display_count;
                println!("  ... 还有 {} 条结果未显示 (使用 --max-results {} 查看更多)",
                    remaining.to_string().dimmed(),
                    file_hits.len().to_string().dimmed()
                );
            }
        }
    }

    // 打印统计摘要
    println!("\n{}", "─".repeat(50).dimmed());
    println!(
        "共找到 {} 处匹配，涉及 {} 个文件",
        results.len().to_string().yellow().bold(),
        file_count.to_string().yellow().bold()
    );
}

/// 在一行中高亮显示匹配的部分
fn print_highlighted_line(line: &str, match_start: usize, match_end: usize) {
    // 注意：这里的 start/end 是字节索引，处理 Unicode 要小心
    // 检查字符边界，避免 panic
    if line.is_char_boundary(match_start) && line.is_char_boundary(match_end) {
        let before = &line[..match_start];
        let matched = &line[match_start..match_end];
        let after = &line[match_end..];

        print!("{}", before);
        print!("{}", matched.red().bold()); // 匹配部分红色加粗
        println!("{}", after);
    } else {
        // 如果边界不安全，就直接打印整行，不高亮
        println!("{}", line);
    }
}
