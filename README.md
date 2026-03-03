# Email CLI - 邮箱管理工具

一个用 Rust 编写的简单邮箱管理命令行工具。

## 功能

- ✅ 配置邮箱账户（IMAP/SMTP）
- ✅ 查看收件箱邮件列表
- ✅ 读取邮件详情
- ✅ 搜索邮件
- ✅ 发送邮件

## 安装

### 前置要求

1. **Rust 工具链** - 安装 [rustup](https://rustup.rs/)
2. **Visual Studio Build Tools** - Windows 需要 MSVC 链接器

```bash
# 安装 VS Build Tools (Windows)
winget install --id Microsoft.VisualStudio.2022.BuildTools --custom "--add Microsoft.VisualStudio.Workload.VCTools --passive"
```

### 编译安装

```bash
# 克隆仓库
git clone https://github.com/Ivanbeethoven/email-cli.git
cd email-cli

# 编译发布版本
cargo build --release

# 二进制文件位置
# Windows: target/release/email_cli.exe
# Linux/macOS: target/release/email_cli
```

## 使用方法

### 1. 配置邮箱

```bash
# 交互式配置
email config

# 或命令行参数
email config --imap-server imap.qq.com --smtp-server smtp.qq.com --email your@qq.com --password your-auth-code
```

**注意**: QQ 邮箱需要使用 [授权码](https://service.mail.qq.com/detail/0/75) 而非登录密码

### 2. 查看收件箱

```bash
# 查看最新 10 封邮件
email inbox

# 查看最新 20 封
email inbox -c 20
```

### 3. 读取邮件

```bash
# 读取指定 UID 的邮件
email read -u 123
```

### 4. 搜索邮件

```bash
# 搜索关键词
email search -q "项目报告"

# 限制结果数量
email search -q "发票" -l 5
```

### 5. 发送邮件

```bash
email send -t recipient@example.com -s "会议通知" -b "明天下午 3 点开会"
```

### 6. 配置管理

```bash
# 查看当前配置
email show-config

# 清除配置
email clear-config
```

## 配置示例

配置文件位置：
- Windows: `%APPDATA%\email_cli\config.json`
- Linux/macOS: `~/.config/email_cli/config.json`

```json
{
  "imap_server": "imap.qq.com",
  "imap_port": 993,
  "smtp_server": "smtp.qq.com",
  "smtp_port": 465,
  "email": "your@qq.com"
}
```

## 常用邮箱服务器配置

| 服务商 | IMAP 服务器 | SMTP 服务器 | IMAP 端口 | SMTP 端口 |
|--------|-----------|-----------|---------|---------|
| QQ 邮箱 | imap.qq.com | smtp.qq.com | 993 | 465 |
| 163 邮箱 | imap.163.com | smtp.163.com | 993 | 465 |
| Gmail | imap.gmail.com | smtp.gmail.com | 993 | 465 |
| Outlook | outlook.office365.com | smtp.office365.com | 993 | 587 |
| 企业邮箱 | 咨询公司 IT | 咨询公司 IT | 993 | 465 |

## 开发

```bash
# 调试模式编译
cargo build

# 运行
cargo run -- inbox

# 运行测试
cargo test
```

## 许可证

MIT License

## 作者

Xiaoyang Han <lux1an@qq.com>

## 贡献

欢迎提交 Issue 和 Pull Request！
