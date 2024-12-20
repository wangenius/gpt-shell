mod llm_provider;
mod config;
mod bots;

use clap::{Parser, Subcommand};
use colored::*;
use dotenv::dotenv;
use anyhow::Result;
use futures::StreamExt;
use std::io::{self, Write, BufRead};
use llm_provider::{LLMProvider, Message, OpenAIProvider};
use config::Config;
use bots::BotsConfig;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;

#[derive(Parser)]
#[command(author, version, about = "GPT Shell - 命令行AI助手")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 使用指定的机器人
    #[arg(short, long)]
    bot: Option<String>,

    /// 使用别名指定机器人（单字符）
    #[arg(short = 't')]
    alias: Option<String>,

    /// 要发送给GPT的提示文本
    #[arg(default_value = None, required = false)]
    prompt: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// 配置管理
    Config {
        #[command(subcommand)]
        action: Option<ConfigActions>,
    },
    /// 机器人管理
    Bots {
        #[command(subcommand)]
        action: Option<BotsActions>,
    },
}

#[derive(Subcommand)]
enum ConfigActions {
    /// 设置API密钥
    Key {
        /// API密钥
        key: String,
    },
    /// 设置API URL
    Url {
        /// API URL
        url: String,
    },
    /// 设置默认模型
    Model {
        /// 模型名称
        model: String,
    },
    /// 设置系统提示词
    System {
        /// 系统提示词（不提供则清除）
        prompt: Option<String>,
    },
    /// 设置是否使用流式输出
    Stream {
        /// 是否启用（true 或 false）
        enabled: bool,
    },
    /// 显示当前配置
    Show,
    /// 设置机器人别名
    Alias {
        /// 机器人名称
        bot: String,
        /// 单字符别名
        alias: String,
    },
    /// 删除机器人别名
    Unalias {
        /// 要删除的别名
        alias: String,
    },
    /// 列出所有别名
    Aliases,
}

#[derive(Subcommand)]
enum BotsActions {
    /// 添加新机器人
    Add {
        /// 机器人名称
        name: String,
        /// 系统提示词
        #[arg(short, long)]
        system: String,
    },
    /// 删除机器人
    Remove {
        /// 机器人名称
        name: String,
    },
    /// 列出所有机器人
    List,
}


async fn chat_once(config: &Config, messages: Vec<Message>, running: Arc<AtomicBool>) -> Result<String> {
    let provider = OpenAIProvider::new(config.api_key.clone())
        .with_url(config.api_url.clone())
        .with_model(config.model.clone());

    let mut stream = provider.chat(messages, config.stream, running.clone()).await?;
    let mut response = String::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(content) if !content.is_empty() => {
                print!("{}", content.green());
                io::stdout().flush()?;
                response.push_str(&content);
            }
            Err(e) => {
                eprintln!("\n错误: {}", e);
                break;
            }
            _ => {}
        }
    }

    // 如果是因为中断而退出，打印提示
    if !running.load(Ordering::SeqCst) {
        println!("\n已取消生成。");
    } else {
        println!();
    }

    Ok(response)
}

async fn interactive_mode(config: Config, bot_name: Option<String>, bots_config: BotsConfig, running: Arc<AtomicBool>) -> Result<()> {
    let mut messages = Vec::new();
    
    // 如果指定了机器人，使用机器人的系统提示词
    if let Some(bot_name) = bot_name {
        if let Some(bot) = bots_config.get_bot(&bot_name) {
            messages.push(Message {
                role: "system".to_string(),
                content: bot.system_prompt.clone(),
            });
            println!("使用机器人: {}", bot_name.green());
        } else {
            return Err(anyhow::anyhow!("未找到机器人: {}", bot_name));
        }
    } else if let Some(ref system_prompt) = config.system_prompt {
        // 否则使用默认系统提示词
        messages.push(Message {
            role: "system".to_string(),
            content: system_prompt.clone(),
        });
    }

    println!("进入交互模式 (输入 'exit' 或按 Ctrl+C 退出)");
    println!("---------------------------------------------");

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    loop {
        // 重置中断标志
        running.store(true, Ordering::SeqCst);

        print!("> ");
        io::stdout().flush()?;
        
        input.clear();
        if reader.read_line(&mut input)? == 0 {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" {
            break;
        }

        // 添加用户消息
        messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        // 获取助手回复
        let response = chat_once(&config, messages.clone(), running.clone()).await?;

        // 只有在有响应时才添加到历史记录
        if !response.is_empty() {
            messages.push(Message {
                role: "assistant".to_string(),
                content: response,
            });
        }
    }

    println!("再见！");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // 加载环境变量
    dotenv().ok();
    
    // 加载配置
    let mut config = Config::load()?;
    let mut bots_config = BotsConfig::load()?;
    
    // 获取命令行参数
    let cli = Cli::parse();

    // 设置中断处理
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    match (&cli.command, &cli.prompt) {
        (Some(Commands::Config { action }), _) => {
            match action {
                Some(ConfigActions::Key { key }) => {
                    config.set_key(key.clone())?;
                }
                Some(ConfigActions::Url { url }) => {
                    config.set_url(url.clone())?;
                }
                Some(ConfigActions::Model { model }) => {
                    config.set_model(model.clone())?;
                }
                Some(ConfigActions::System { prompt }) => {
                    config.set_system_prompt(prompt.clone())?;
                }
                Some(ConfigActions::Stream { enabled }) => {
                    config.set_stream(*enabled)?;
                }
                Some(ConfigActions::Show) => {
                    println!("当前配置：");
                    println!("API密钥: {}", if config.api_key.is_empty() { 
                        "未设置".red() 
                    } else { 
                        "已设置".green() 
                    });
                    println!("API URL: {}", config.api_url.green());
                    println!("默认模型: {}", config.model.green());
                    println!("流式输出: {}", if config.stream { 
                        "启用".green() 
                    } else { 
                        "禁用".yellow() 
                    });
                    println!("系统提示词: {}", config.system_prompt.as_ref().map(|p| p.as_str()).unwrap_or("未设置").green());
                }
                Some(ConfigActions::Alias { bot, alias }) => {
                    // 先检查机器人是否存在
                    if !bots_config.get_bot(bot).is_some() {
                        return Err(anyhow::anyhow!("未找到机器人: {}", bot));
                    }
                    config.set_alias(bot.clone(), alias.clone())?;
                }
                Some(ConfigActions::Unalias { alias }) => {
                    config.remove_alias(&alias)?;
                }
                Some(ConfigActions::Aliases) => {
                    config.list_aliases();
                }
                None => {
                    Config::open_config()?;
                }
            }
        }
        (Some(Commands::Bots { action }), _) => {
            match action {
                Some(BotsActions::Add { name, system }) => {
                    bots_config.add_bot(name.clone(), system.clone())?;
                }
                Some(BotsActions::Remove { name }) => {
                    bots_config.remove_bot(name)?;
                }
                Some(BotsActions::List) => {
                    bots_config.list_bots();
                }
                None => {
                    // 直接打开配置文件
                    BotsConfig::open_config()?;
                }
            }
        }
        (None, Some(prompt)) => {
            // 重置中断标志
            running.store(true, Ordering::SeqCst);

            // 创建消息列表
            let mut messages = Vec::new();

            // 检查是否使用了别名
            let bot_name = if let Some(alias) = &cli.alias {
                if let Some(bot_name) = config.get_bot_by_alias(alias) {
                    Some(bot_name.clone())
                } else {
                    return Err(anyhow::anyhow!("未找到别名对应的机器人: {}", alias));
                }
            } else {
                cli.bot.clone()
            };

            // 如果指定了机器人，使用机器人的系统提示词
            if let Some(bot_name) = &bot_name {
                if let Some(bot) = bots_config.get_bot(bot_name) {
                    messages.push(Message {
                        role: "system".to_string(),
                        content: bot.system_prompt.clone(),
                    });
                } else {
                    return Err(anyhow::anyhow!("未找到机器人: {}", bot_name));
                }
            } else if let Some(ref system_prompt) = config.system_prompt {
                // 否则使用默认系统提示词
                messages.push(Message {
                    role: "system".to_string(),
                    content: system_prompt.clone(),
                });
            }

            // 添加用户消息
            messages.push(Message {
                role: "user".to_string(),
                content: prompt.clone(),
            });

            // 发送消息并获取回复
            chat_once(&config, messages, running).await?;
        }
        (None, None) => {
            // 检查是否使用了别名
            let bot_name = if let Some(alias) = cli.alias {
                if let Some(bot_name) = config.get_bot_by_alias(&alias) {
                    Some(bot_name.clone())
                } else {
                    return Err(anyhow::anyhow!("未找到别名对应的机器人: {}", alias));
                }
            } else {
                cli.bot
            };

            // 进入交互模式
            interactive_mode(config, bot_name, bots_config, running).await?;
        }
    }
    
    Ok(())
}