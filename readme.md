# 🦀 crabgrep

一个用 Rust 编写的高性能文件内容搜索工具（致敬🦀）。

## 注
- 本项目仅用于学习 Rust 语言，不用于生产环境;
- 源码出自 [微信公众号-土豆学代码-crabgrep](https://mp.weixin.qq.com/s/WaMggLI-67wyivcdnJUFWA?scene=1);
- 仅显示部分优化修改，故软件名称未改，特此标注源码出处；

## 特性
- ✅ **高性能**：使用 Rust 编写，内存安全，速度快
- ✅ **递归搜索**：支持递归搜索子目录
- ✅ **彩色高亮**：匹配内容高亮显示，清晰易读
- ✅ **智能显示**：默认每个文件最多显示 5 条结果，避免刷屏
- ✅ **忽略大小写**：可选的大小写不敏感搜索
- ✅ **优雅错误处理**：友好的错误提示，不会因为个别文件失败而中断
- ✅ **跨平台**：支持 Linux、macOS 和 Windows
- ✅ **隐藏文件过滤**：自动跳过隐藏文件和二进制文件

## 安装

### 使用 Cargo 安装

```bash
cargo install crabgrep
```

### 从源码编译

```bash
git clone https://github.com/yi1873/crabgrep.git
cd crabgrep
cargo build --release
```

编译后的二进制文件位于 `target/release/crabgrep`。

## 使用方法

### 基本用法

```bash
# 在当前目录搜索 "hello"
crabgrep "hello"

# 在指定目录搜索
crabgrep "hello" ./src

# 忽略大小写搜索
crabgrep "hello" ./src -i

# 递归搜索子目录（默认开启）
crabgrep "hello" ./src -r

# 不递归搜索
crabgrep "hello" ./src --no-recursive
```

### 控制结果显示数量

```bash
# 默认每个文件只显示 5 条结果
crabgrep "hello" ./src

# 每个文件显示 10 条结果
crabgrep "hello" ./src -m 10

# 显示全部结果（不限制）
crabgrep "hello" ./src --max-results 0
```


## 命令行参数

| 参数 | 简写 | 说明 | 默认值 |
|------|--------|------|--------|
| `pattern` | - | 要搜索的关键词（必选） | - |
| `path` | - | 搜索的目标路径（文件或目录） | `.` |
| `--ignore-case` | `-i` | 忽略大小写 | `false` |
| `--recursive` | `-r` | 递归搜索子目录 | `true` |
| `--max-results` | `-m` | 每个文件最多显示的匹配结果数（0 表示全部） | `5` |
| `--help` | `-h` | 显示帮助信息 | - |
| `--version` | `-V` | 显示版本信息 | - |

## 输出示例

### 示例 1：少量结果

```bash
$ crabgrep "hello" ./src

🔍 在 ./src 中搜索 "hello"...

./src/search.rs
  139:          let file = create_temp_file("hello world\nrust is awesome\nhello again");
  141:              pattern: "hello".to_string(),
  154:          let file = create_temp_file("Hello World\nhello world\nHELLO WORLD");
  156:              pattern: "hello".to_string(),

──────────────────────────────────────────────────
共找到 4 处匹配，涉及 1 个文件
```

### 示例 2：结果被截断

```bash
$ crabgrep "FCV" ./FCV

🔍 在 ./FCV 中搜索 "FCV"...

./FCV/primer_evaluation/primer.fa
  1:  >sgFCV-1-1.F
  3:  >sgFCV-1-1.R
  5:  >sgFCV-1-2.F
  7:  >sgFCV-1-2.R
  9:  >sgFCV-1-3.F
  ... 还有 3 条结果未显示 (使用 --max-results 8 查看更多)

──────────────────────────────────────────────────
共找到 8 处匹配，涉及 1 个文件
```

## 开发

### 运行测试

```bash
cargo test
```


## 与其他工具的对比

| 特性 | crabgrep | grep | ripgrep |
|------|----------|------|---------|
| 语言 | Rust | C | Rust |
| 彩色输出 | ✅ | ✅ (需 --color) | ✅ |
| 递归搜索 | ✅ | ❌ (需 -r) | ✅ |
| 忽略大小写 | ✅ | ✅ (-i) | ✅ (-i) |
| 结果限制 | ✅ | ❌ | ❌ |
| 性能 | 快 | 中 | 最快 |




