//! Email CLI - 邮箱管理命令行工具
//! 
//! 功能：
//! - 查看收件箱邮件
//! - 读取邮件详情
//! - 发送邮件
//! - 管理邮件配置

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use imap::Session;
use native_tls::TlsConnector;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// 邮箱管理 CLI 工具
#[derive(Parser)]
#[command(name = "email")]
#[command(author = "Xiaoyang Han <lux1an@qq.com>")]
#[command(version = "0.1.0")]
#[command(about = "一个简单的邮箱管理命令行工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 配置邮箱账户
    Config {
        /// IMAP 服务器地址 (如：imap.qq.com)
        #[arg(long)]
        imap_server: Option<String>,
        
        /// SMTP 服务器地址 (如：smtp.qq.com)
        #[arg(long)]
        smtp_server: Option<String>,
        
        /// 邮箱地址
        #[arg(long)]
        email: Option<String>,
        
        /// 邮箱密码/授权码
        #[arg(long)]
        password: Option<String>,
        
        /// IMAP 端口 (默认：993)
        #[arg(long, default_value = "993")]
        imap_port: u16,
        
        /// SMTP 端口 (默认：465)
        #[arg(long, default_value = "465")]
        smtp_port: u16,
    },
    
    /// 查看收件箱邮件列表
    Inbox {
        /// 显示邮件数量 (默认：10)
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    
    /// 读取邮件详情
    Read {
        /// 邮件 UID
        #[arg(short, long)]
        uid: u32,
    },
    
    /// 发送邮件
    Send {
        /// 收件人邮箱
        #[arg(short, long)]
        to: String,
        
        /// 邮件主题
        #[arg(short, long)]
        subject: String,
        
        /// 邮件正文
        #[arg(short, long)]
        body: String,
    },
    
    /// 搜索邮件
    Search {
        /// 搜索关键词
        #[arg(short, long)]
        query: String,
        
        /// 最大结果数 (默认：10)
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// 显示当前配置
    ShowConfig,
    
    /// 清除配置
    ClearConfig,
}

/// 邮箱配置结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct EmailConfig {
    imap_server: String,
    imap_port: u16,
    smtp_server: String,
    smtp_port: u16,
    email: String,
    #[serde(skip_serializing)]
    password: String,
}

impl EmailConfig {
    /// 获取配置文件路径
    fn config_path() -> Result<PathBuf> {
        let config_dir = if cfg!(windows) {
            std::env::var("APPDATA")
                .context("Failed to get APPDATA")?
                .into()
        } else {
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"))
        };
        
        let config_dir = config_dir.join("email_cli");
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("config.json"))
    }
    
    /// 加载配置
    fn load() -> Result<Self> {
        let path = Self::config_path()?;
        let content = fs::read_to_string(&path)
            .context("配置文件不存在，请先运行 'email config' 命令")?;
        let config: EmailConfig = serde_json::from_str(&content)
            .context("配置文件格式错误")?;
        Ok(config)
    }
    
    /// 保存配置
    fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        println!("✓ 配置已保存到：{:?}", path);
        Ok(())
    }
    
    /// 删除配置
    fn delete() -> Result<()> {
        let path = Self::config_path()?;
        if path.exists() {
            fs::remove_file(&path)?;
            println!("✓ 配置已清除");
        } else {
            println!("ℹ 配置文件不存在");
        }
        Ok(())
    }
}

/// 连接到 IMAP 服务器
fn connect_imap(config: &EmailConfig) -> Result<Session<impl std::io::Read + std::io::Write>> {
    let connector = TlsConnector::from(native_tls::TlsConnector::new()?);
    let client = imap::Client::new(
        format!("{}:{}", config.imap_server, config.imap_port),
        connector,
    )?;
    
    let mut session = client.login(&config.email, &config.password)?;
    session.select("INBOX")?;
    Ok(session)
}

/// 查看收件箱
fn view_inbox(config: &EmailConfig, count: usize) -> Result<()> {
    let mut session = connect_imap(config)?;
    
    // 获取邮件数量
    let messages = session.search("ALL")?;
    let total = messages.len();
    
    if total == 0 {
        println!("📭 收件箱为空");
        return Ok(());
    }
    
    println!("📬 收件箱 (共 {} 封邮件)", total);
    println!("{}", "─".repeat(80));
    
    // 获取最新的 count 封邮件
    let fetch_from = if total > count { total - count } else { 0 };
    let fetch_ids: Vec<u32> = messages.iter().skip(fetch_from).cloned().collect();
    
    if fetch_ids.is_empty() {
        println!("📭 没有邮件");
        return Ok(());
    }
    
    let fetch_range = format!("{}:*", fetch_ids.first().unwrap());
    let fetches = session.fetch(&fetch_range, "(ENVELOPE FLAGS INTERNALDATE)")?;
    
    for fetch in fetches.iter().rev() {
        let envelope = fetch.envelope().context("无法解析邮件信封")?;
        
        let subject = envelope.subject
            .and_then(|s| std::str::from_utf8(s).ok())
            .unwrap_or("(无主题)")
            .to_string();
        
        let from = envelope.from
            .first()
            .map(|m| {
                let name = m.name().unwrap_or("");
                let mailbox = m.mailbox().unwrap_or("");
                format!("{} <{}>", name, mailbox)
            })
            .unwrap_or_else(|| "未知".to_string());
        
        let date = envelope.date
            .and_then(|d| std::str::from_utf8(d).ok())
            .unwrap_or("未知日期");
        
        let uid = fetch.uid.context("无法获取邮件 UID")?;
        let flags = fetch.flags();
        let is_read = flags.iter().any(|f| f == "\\Seen");
        
        let icon = if is_read { "📄" } else { "📕" };
        
        println!("{} UID: {} | {}", icon, uid, date);
        println!("   发件人：{}", from);
        println!("   主题：{}", subject);
        println!("{}", "─".repeat(80));
    }
    
    Ok(())
}

/// 读取邮件详情
fn read_email(config: &EmailConfig, uid: u32) -> Result<()> {
    let mut session = connect_imap(config)?;
    
    let fetches = session.fetch(&format!("{}", uid), "(ENVELOPE BODY[TEXT])")?;
    
    if fetches.is_empty() {
        println!("❌ 未找到邮件 UID: {}", uid);
        return Ok(());
    }
    
    let fetch = fetches.first().context("无法获取邮件")?;
    let envelope = fetch.envelope().context("无法解析邮件信封")?;
    
    // 显示邮件头信息
    println!("\n📧 邮件详情");
    println!("{}", "═".repeat(80));
    
    let subject = envelope.subject
        .and_then(|s| std::str::from_utf8(s).ok())
        .unwrap_or("(无主题)");
    println!("主题：{}", subject);
    
    let from = envelope.from
        .first()
        .map(|m| {
            let name = m.name().unwrap_or("");
            let mailbox = m.mailbox().unwrap_or("");
            format!("{} <{}>", name, mailbox)
        })
        .unwrap_or_else(|| "未知".to_string());
    println!("发件人：{}", from);
    
    let to: Vec<String> = envelope.to
        .iter()
        .map(|m| {
            let name = m.name().unwrap_or("");
            let mailbox = m.mailbox().unwrap_or("");
            format!("{} <{}>", name, mailbox)
        })
        .collect();
    println!("收件人：{}", to.join(", "));
    
    let date = envelope.date
        .and_then(|d| std::str::from_utf8(d).ok())
        .unwrap_or("未知日期");
    println!("日期：{}", date);
    
    println!("{}", "─".repeat(80));
    
    // 显示邮件正文
    if let Some(body) = fetch.body() {
        if let Ok(text) = std::str::from_utf8(body) {
            println!("\n{}", text);
        } else {
            println!("\n⚠ 无法解析邮件正文 (可能是 HTML 或编码问题)");
        }
    } else {
        println!("\n⚠ 邮件无正文");
    }
    
    println!("\n{}", "═".repeat(80));
    
    // 标记为已读
    session.store(&format!("{}", uid), "+Flags", &["\\Seen"])?;
    
    Ok(())
}

/// 搜索邮件
fn search_emails(config: &EmailConfig, query: &str, limit: usize) -> Result<()> {
    let mut session = connect_imap(config)?;
    
    println!("🔍 搜索：{}", query);
    println!("{}", "─".repeat(80));
    
    // 搜索主题或正文包含关键词的邮件
    let search_query = format!("(SUBJECT \"{}\" OR BODY \"{}\")", query, query);
    let messages = session.search(&search_query)?;
    
    if messages.is_empty() {
        println!("📭 未找到匹配的邮件");
        return Ok(());
    }
    
    println!("找到 {} 封匹配的邮件 (显示前 {} 封)\n", messages.len(), limit);
    
    let fetch_count = std::cmp::min(messages.len(), limit);
    let fetch_ids: Vec<u32> = messages.iter().rev().take(fetch_count).cloned().collect();
    
    for uid in fetch_ids {
        let fetches = session.fetch(&format!("{}", uid), "ENVELOPE")?;
        if let Some(fetch) = fetches.first() {
            if let Some(envelope) = fetch.envelope() {
                let subject = envelope.subject
                    .and_then(|s| std::str::from_utf8(s).ok())
                    .unwrap_or("(无主题)");
                
                let from = envelope.from
                    .first()
                    .map(|m| {
                        let mailbox = m.mailbox().unwrap_or("");
                        mailbox.to_string()
                    })
                    .unwrap_or_else(|| "未知".to_string());
                
                println!("UID: {} | 发件人：{} | 主题：{}", uid, from, subject);
            }
        }
    }
    
    Ok(())
}

/// 发送邮件
async fn send_email(config: &EmailConfig, to: &str, subject: &str, body: &str) -> Result<()> {
    use lettre::{
        message::header::ContentType,
        transport::smtp::authentication::Credentials,
        Message, SmtpTransport, Transport,
    };
    
    let email = Message::builder()
        .from(format!("{} <{}>", config.email.split('@').next().unwrap_or("User"), config.email).parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;
    
    let creds = Credentials::new(config.email.clone(), config.password.clone());
    
    let mailer = SmtpTransport::relay(&config.smtp_server)?
        .port(config.smtp_port)
        .credentials(creds)
        .build();
    
    match mailer.send(&email) {
        Ok(_) => println!("✓ 邮件已发送到：{}", to),
        Err(e) => println!("❌ 发送失败：{}", e),
    }
    
    Ok(())
}

/// 配置邮箱账户
fn configure_email(
    imap_server: Option<String>,
    smtp_server: Option<String>,
    email: Option<String>,
    password: Option<String>,
    imap_port: u16,
    smtp_port: u16,
) -> Result<()> {
    println!("📧 邮箱配置\n");
    
    // 如果已有配置，提示是否覆盖
    if let Ok(existing) = EmailConfig::load() {
        println!("发现现有配置：");
        println!("  邮箱：{}", existing.email);
        println!("  IMAP: {}:{}", existing.imap_server, existing.imap_port);
        println!("  SMTP: {}:{}", existing.smtp_server, existing.smtp_port);
        print!("\n是否覆盖配置？(y/N): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("ℹ 保留现有配置");
            return Ok(());
        }
    }
    
    // 交互式输入
    let get_input = |prompt: &str, default: Option<&str>| -> Result<String> {
        print!("{}", prompt);
        if let Some(d) = default {
            print!(" [{}]: ", d);
        } else {
            print!(": ");
        }
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            Ok(default.unwrap_or("").to_string())
        } else {
            Ok(input.to_string())
        }
    };
    
    let imap_srv = if let Some(s) = imap_server {
        s
    } else {
        get_input("IMAP 服务器", Some("imap.qq.com"))?
    };
    
    let smtp_srv = if let Some(s) = smtp_server {
        s
    } else {
        get_input("SMTP 服务器", Some("smtp.qq.com"))?
    };
    
    let email_addr = if let Some(e) = email {
        e
    } else {
        get_input("邮箱地址", None)?
    };
    
    let pwd = if let Some(p) = password {
        p
    } else {
        print!("邮箱密码/授权码: ");
        io::stdout().flush()?;
        // 简单隐藏输入（不显示字符）
        let mut pwd_input = String::new();
        io::stdin().read_line(&mut pwd_input)?;
        pwd_input.trim().to_string()
    };
    
    let config = EmailConfig {
        imap_server: imap_srv,
        imap_port,
        smtp_server: smtp_srv,
        smtp_port,
        email: email_addr,
        password: pwd,
    };
    
    config.save()?;
    
    println!("\n✓ 配置完成！");
    println!("\n常用命令:");
    println!("  email inbox          - 查看收件箱");
    println!("  email read -u <UID>  - 读取邮件详情");
    println!("  email search -q <关键词> - 搜索邮件");
    println!("  email send -t <收件人> -s <主题> -b <正文> - 发送邮件");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Config {
            imap_server,
            smtp_server,
            email,
            password,
            imap_port,
            smtp_port,
        } => {
            configure_email(imap_server, smtp_server, email, password, imap_port, smtp_port)?;
        }
        
        Commands::Inbox { count } => {
            let config = EmailConfig::load()?;
            view_inbox(&config, count)?;
        }
        
        Commands::Read { uid } => {
            let config = EmailConfig::load()?;
            read_email(&config, uid)?;
        }
        
        Commands::Send { to, subject, body } => {
            let config = EmailConfig::load()?;
            send_email(&config, &to, &subject, &body).await?;
        }
        
        Commands::Search { query, limit } => {
            let config = EmailConfig::load()?;
            search_emails(&config, &query, limit)?;
        }
        
        Commands::ShowConfig => {
            match EmailConfig::load() {
                Ok(config) => {
                    println!("📧 当前配置:");
                    println!("  邮箱：{}", config.email);
                    println!("  IMAP: {}:{}", config.imap_server, config.imap_port);
                    println!("  SMTP: {}:{}", config.smtp_server, config.smtp_port);
                }
                Err(e) => {
                    println!("❌ 无法加载配置：{}", e);
                }
            }
        }
        
        Commands::ClearConfig => {
            EmailConfig::delete()?;
        }
    }
    
    Ok(())
}
