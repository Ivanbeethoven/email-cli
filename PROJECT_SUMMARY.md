# Email CLI 项目 - 开发总结

**创建时间**: 2026-03-03  
**作者**: Xiaoyang Han (Ivanbeethoven)  
**仓库**: https://github.com/Ivanbeethoven/email-cli

---

## 📦 项目概述

用 Rust 开发的命令行邮箱管理工具，支持：
- IMAP 协议接收邮件
- SMTP 协议发送邮件
- 本地配置管理
- 跨平台支持（Windows/Linux/macOS）

---

## 🎯 功能特性

### 已完成
- ✅ 项目结构搭建
- ✅ CLI 参数解析（clap）
- ✅ 配置管理（JSON 存储）
- ✅ IMAP 收件箱查看
- ✅ 邮件详情读取
- ✅ 邮件搜索功能
- ✅ SMTP 邮件发送
- ✅ GitHub 仓库创建
- ✅ CI/CD 工作流配置

### 待完成
- ⏳ HTML 邮件解析
- ⏳ 附件处理
- ⏳ 多邮箱账户支持
- ⏳ 邮件标签/分类管理
- ⏳ 离线缓存

---

## 🛠️ 技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| **语言** | Rust 2021 | 内存安全、高性能 |
| **CLI 框架** | clap | 命令行参数解析 |
| **IMAP** | imap crate | 邮件接收 |
| **SMTP** | lettre | 邮件发送 |
| **配置** | serde + serde_json | JSON 序列化 |
| **CI/CD** | GitHub Actions | 自动构建发布 |

---

## 📁 项目结构

```
email-cli/
├── .github/
│   └── workflows/
│       └── ci.yml          # GitHub Actions 配置
├── src/
│   └── main.rs             # 主程序代码
├── .gitignore              # Git 忽略文件
├── Cargo.toml              # Rust 依赖配置
├── README.md               # 项目说明文档
└── PROJECT_SUMMARY.md      # 本项目总结（本文件）
```

---

## 🚀 快速开始

### 1. 克隆仓库
```bash
git clone https://github.com/Ivanbeethoven/email-cli.git
cd email-cli
```

### 2. 编译（需要 VS Build Tools）
```bash
# Windows
cargo build --release

# Linux/macOS
cargo build --release
```

### 3. 使用
```bash
# 配置邮箱
email config

# 查看收件箱
email inbox

# 发送邮件
email send -t xxx@example.com -s "主题" -b "正文"
```

---

## ⚠️ 当前限制

### Windows 编译要求
需要安装 **Visual Studio Build Tools 2022**：
```bash
winget install --id Microsoft.VisualStudio.2022.BuildTools
```

原因：Rust MSVC 目标需要 `link.exe` 链接器

### 替代方案
1. 使用 WSL2 (Windows Subsystem for Linux)
2. 使用 GNU 工具链（需要安装 MinGW）
3. 等待 GitHub Actions 自动构建 releases

---

## 📈 下一步计划

### 短期（1 周）
- [ ] 完成 VS Build Tools 安装
- [ ] 编译第一个可用版本
- [ ] 测试 QQ 邮箱、163 邮箱

### 中期（1 月）
- [ ] 添加 HTML 邮件支持
- [ ] 实现附件下载
- [ ] 多账户配置管理

### 长期（3 月）
- [ ] TUI 界面（使用 ratatui）
- [ ] 离线模式
- [ ] 插件系统

---

## 🔗 相关链接

- **GitHub 仓库**: https://github.com/Ivanbeethoven/email-cli
- **Rust 官方文档**: https://doc.rust-lang.org/
- **clap 文档**: https://docs.rs/clap/
- **lettre 文档**: https://docs.rs/lettre/

---

## 📝 开发笔记

### 依赖选择
- 使用 `native-tls` 而非 `rustls`，因为 Windows 兼容性更好
- `clap` 的 derive 特性让 CLI 定义更简洁
- `anyhow` 提供简单的错误处理

### 配置安全
- 密码以明文存储在本地（改进方向：使用系统密钥环）
- 配置文件权限应设置为仅用户可读

### 邮件协议
- IMAP 使用 SSL/TLS 加密连接（端口 993）
- SMTP 使用 STARTTLS 或 SSL（端口 465/587）

---

## 👨‍💻 作者

**Xiaoyang Han**
- GitHub: [@Ivanbeethoven](https://github.com/Ivanbeethoven)
- Email: lux1an@qq.com

---

*最后更新：2026-03-03*
