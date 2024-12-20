mod llm_provider;
mod config;
mod bots;

use clap::{Command, Arg};
use colored::*;
use dotenv::dotenv;
use anyhow::Result;
use futures::StreamExt;
use std::io::{self, Write, BufRead};
use llm_provider::{LLMProvider, Message, Provider};
use config::Config;
use bots::BotsConfig;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;

fn build_cli() -> Command {
    let mut cmd = Command::new("gpt")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("GPT Shell - command line AI assistant")
        .arg(
            Arg::new("bot")
                .short('b')
                .long("bot")
                .help("use specified bot")
                .value_name("BOT")
        )
        .arg(
            Arg::new("prompt")
                .help("prompt text to send to GPT")
                .required(false)
        );

    // 添加子命令
    cmd = cmd.subcommand(
        Command::new("config")
            .about("configuration management")
            .subcommand(
                Command::new("model")
                    .about("model management")
                    .subcommand(
                        Command::new("add")
                            .about("add new model")
                            .arg(Arg::new("name").required(true))
                            .arg(Arg::new("key").required(true))
                            .arg(
                                Arg::new("url")
                                    .long("url")
                                    .help("API URL")
                            )
                            .arg(
                                Arg::new("model")
                                    .long("model")
                                    .help("model name")
                            )
                    )
                    .subcommand(
                        Command::new("remove")
                            .about("remove model")
                            .arg(Arg::new("name").required(true))
                    )
                    .subcommand(
                        Command::new("list")
                            .about("list all models")
                    )
                    .subcommand(
                        Command::new("use")
                            .about("switch to model")
                            .arg(Arg::new("name").required(true))
                    )
            )
            .subcommand(
                Command::new("system")
                    .about("set system prompt")
                    .arg(Arg::new("prompt"))
            )
            .subcommand(
                Command::new("stream")
                    .about("set stream output")
                    .arg(Arg::new("enabled").required(true))
            )
            .subcommand(
                Command::new("show")
                    .about("show current configuration")
            )
            .subcommand(
                Command::new("edit")
                    .about("edit configuration file")
            )
    );

    cmd = cmd.subcommand(
        Command::new("bots")
            .about("bot management")
            .subcommand(
                Command::new("add")
                    .about("add new bot")
                    .arg(Arg::new("name").required(true))
                    .arg(
                        Arg::new("system")
                            .short('s')
                            .long("system")
                            .required(true)
                    )
            )
            .subcommand(
                Command::new("remove")
                    .about("remove bot")
                    .arg(Arg::new("name").required(true))
            )
            .subcommand(
                Command::new("list")
                    .about("list all bots")
            )
            .subcommand(
                Command::new("edit")
                    .about("edit bots configuration file")
            )
            .subcommand(
                Command::new("alias")
                    .about("alias management")
                    .subcommand(
                        Command::new("set")
                            .about("set bot alias")
                            .arg(Arg::new("bot").required(true))
                            .arg(Arg::new("alias").required(true))
                    )
                    .subcommand(
                        Command::new("remove")
                            .about("remove alias")
                            .arg(Arg::new("alias").required(true))
                    )
                    .subcommand(
                        Command::new("list")
                            .about("list all aliases")
                    )
            )
    );

    cmd
}

async fn chat_once(config: &Config, messages: Vec<Message>, running: Arc<AtomicBool>) -> Result<String> {
    let (_, model_config) = match config.get_current_model() {
        Some(model) => model,
        None => {
            println!("tips: no model configured, please add a model first.");
            println!("you can use the following command to add a model:");
            println!("  gpt config model add <n> <key> [--url <url>] [--model <model>]");
            println!("for example, add deepseek:");
            println!("  gpt config model add deepseek your-api-key --url https://api.deepseek.com/v1/chat/completions --model deepseek-chat");
            return Ok(String::new());
        }
    };

    let provider = Provider::new(model_config.api_key.clone())
        .with_url(model_config.api_url.clone())
        .with_model(model_config.model.clone());

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
                eprintln!("\nerror: {}", e);
                break;
            }
            _ => {}
        }
    }

    // if cancelled by interrupt, print prompt
    if !running.load(Ordering::SeqCst) {
        println!("\ncancelled");
    } else {
        println!();
    }

    Ok(response)
}

async fn interactive_mode(config: Config, bot_name: Option<String>, bots_config: BotsConfig, running: Arc<AtomicBool>) -> Result<()> {
    // 首先检查是否配置了模型
    if config.get_current_model().is_none() {
        println!("tips: no model configured, please add a model first.");
        println!("you can use the following command to add a model:");
        println!("  gpt config model add <n> <key> [--url <url>] [--model <model>]");
        println!("for example, add deepseek:");
        println!("  gpt config model add deepseek your-api-key --url https://api.deepseek.com/v1/chat/completions --model deepseek-chat");
        return Ok(());
    }

    let mut messages = Vec::new();
    // if specified bot, use bot's system prompt
    if let Some(bot_name) = bot_name {
        if let Some(bot) = bots_config.get_bot(&bot_name) {
            messages.push(Message {
                role: "system".to_string(),
                content: bot.system_prompt.clone(),
            });
            println!("using bot: {}", bot_name.green());
        } else {
            println!("tips: bot not found: {}", bot_name);
            println!("you can use the following command to list all bots:");
            println!("  gpt bots list");
            return Ok(());
        }
    } else if let Some(ref system_prompt) = config.system_prompt {
        // otherwise use default system prompt
        messages.push(Message {
            role: "system".to_string(),
            content: system_prompt.clone(),
        });
    }

    println!("enter interactive mode (input 'exit' or press Ctrl+C to exit)");
    println!("---------------------------------------------");

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    loop {
        // reset interrupt flag
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

        // add user message
        messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        // get assistant response
        let response = chat_once(&config, messages.clone(), running.clone()).await?;

        // only add to history if there is a response
        if !response.is_empty() {
            messages.push(Message {
                role: "assistant".to_string(),
                content: response,
            });
        }
    }

    println!("bye!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // load environment variables
    dotenv().ok();
    
    // load config
    let mut config = Config::load()?;
    let mut bots_config = BotsConfig::load()?;
    
    // build and get command line arguments
    let mut cmd = build_cli();
    
    // 为每个别名添加短参数
    let aliases: Vec<(String, String)> = bots_config.aliases.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    // 添加别名参数
    let mut alias_args = Vec::new();
    for (alias, _) in &aliases {
        if let Some(c) = alias.chars().next() {
            let id = format!("alias_{}", alias);
            let id_static = Box::leak(id.into_boxed_str()) as &'static str;
            alias_args.push(
                Arg::new(id_static)
                    .short(c)
                    .help(format!("use bot alias '{}'", alias))
                    .action(clap::ArgAction::SetTrue)
            );
        }
    }
    
    // 将别名参数添加到命令中
    for arg in alias_args {
        cmd = cmd.arg(arg);
    }
    
    let matches = cmd.get_matches();

    // set interrupt handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // 检查是否使用了别名
    let mut bot_name = None;
    for (alias, _) in &aliases {
        let id = format!("alias_{}", alias);
        if matches.get_flag(&id) {
            if let Some(bot) = bots_config.get_bot_by_alias(alias) {
                bot_name = Some(bot.to_string());
                break;
            }
        }
    }

    // 如果没有使用别名，检查是否使用了 --bot 参数
    if bot_name.is_none() {
        bot_name = matches.get_one::<String>("bot").cloned();
    }

    // 处理子命令或提示词
    match matches.subcommand() {
        Some(("config", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("model", model_matches)) => {
                    match model_matches.subcommand() {
                        Some(("add", add_matches)) => {
                            if let (Some(name), Some(key), Some(url), Some(model)) = (
                                add_matches.get_one::<String>("name"),
                                add_matches.get_one::<String>("key"),
                                add_matches.get_one::<String>("url"),
                                add_matches.get_one::<String>("model")
                            ) {
                                config.add_model(
                                    name.clone(),
                                    key.clone(),
                                    url.clone(),
                                    model.clone()
                                )?;
                            }
                        }
                        Some(("remove", remove_matches)) => {
                            if let Some(name) = remove_matches.get_one::<String>("name") {
                                config.remove_model(name)?;
                            }
                        }
                        Some(("list", _)) => {
                            config.list_models();
                        }
                        Some(("use", use_matches)) => {
                            if let Some(name) = use_matches.get_one::<String>("name") {
                                config.set_current_model(name)?;
                            }
                        }
                        _ => {
                            println!("available model commands:");
                            println!("  gpt config model add <n> <key> [--url <url>] [--model <model>]");
                            println!("  gpt config model remove <n>");
                            println!("  gpt config model list");
                            println!("  gpt config model use <n>");
                        }
                    }
                }
                Some(("system", system_matches)) => {
                    let prompt = system_matches.get_one::<String>("prompt").cloned();
                    config.set_system_prompt(prompt)?;
                }
                Some(("stream", stream_matches)) => {
                    if let Some(enabled) = stream_matches.get_one::<String>("enabled") {
                        config.set_stream(enabled.parse()?)?;
                    }
                }
                Some(("show", _)) => {
                    println!("current config:");
                    if let Some((name, model_config)) = config.get_current_model() {
                        println!("  current model: {}", name.green());
                        println!("  api url: {}", model_config.api_url);
                        println!("  model: {}", model_config.model);
                        println!("  stream: {}", config.stream);
                        if let Some(ref system_prompt) = config.system_prompt {
                            println!("  system prompt: {}", system_prompt);
                        }
                    } else {
                        println!("  no model configured");
                    }
                }
                Some(("edit", _)) => {
                    Config::open_config()?;
                }
                _ => {
                    // 默认显示当前配置
                    println!("current config:");
                    if let Some((name, model_config)) = config.get_current_model() {
                        println!("  current model: {}", name.green());
                        println!("  api url: {}", model_config.api_url);
                        println!("  model: {}", model_config.model);
                        println!("  stream: {}", config.stream);
                        if let Some(ref system_prompt) = config.system_prompt {
                            println!("  system prompt: {}", system_prompt);
                        }
                    } else {
                        println!("  no model configured");
                    }
                }
            }
        }
        Some(("bots", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("add", add_matches)) => {
                    if let (Some(name), Some(system)) = (
                        add_matches.get_one::<String>("name"),
                        add_matches.get_one::<String>("system")
                    ) {
                        bots_config.add_bot(name.clone(), system.clone())?;
                    }
                }
                Some(("remove", remove_matches)) => {
                    if let Some(name) = remove_matches.get_one::<String>("name") {
                        bots_config.remove_bot(name)?;
                    }
                }
                Some(("list", _)) => {
                    bots_config.list_bots();
                }
                Some(("edit", _)) => {
                    BotsConfig::open_config()?;
                }
                Some(("alias", alias_matches)) => {
                    match alias_matches.subcommand() {
                        Some(("set", set_matches)) => {
                            if let (Some(bot), Some(alias)) = (
                                set_matches.get_one::<String>("bot"),
                                set_matches.get_one::<String>("alias")
                            ) {
                                bots_config.set_alias(bot.clone(), alias.clone())?;
                            }
                        }
                        Some(("remove", remove_matches)) => {
                            if let Some(alias) = remove_matches.get_one::<String>("alias") {
                                bots_config.remove_alias(alias)?;
                            }
                        }
                        Some(("list", _)) => {
                            bots_config.list_aliases();
                        }
                        _ => {
                            println!("available alias commands:");
                            println!("  gpt bots alias set <bot> <alias>  # set bot alias");
                            println!("  gpt bots alias remove <alias>     # remove alias");
                            println!("  gpt bots alias list               # list all aliases");
                        }
                    }
                }
                _ => {
                    // 默认显示机器人列表
                    bots_config.list_bots();
                }
            }
        }
        _ => {
            // 获取提示词
            if let Some(prompt) = matches.get_one::<String>("prompt") {
                // 单次对话模式
                let mut messages = Vec::new();

                // 如果指定了机器人，使用机器人的系统提示词
                if let Some(bot_name) = &bot_name {
                    if let Some(bot) = bots_config.get_bot(bot_name) {
                        messages.push(Message {
                            role: "system".to_string(),
                            content: bot.system_prompt.clone(),
                        });
                    } else {
                        return Err(anyhow::anyhow!("bot not found: {}", bot_name));
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
            } else {
                // 交互模式
                interactive_mode(config, bot_name, bots_config, running).await?;
            }
        }
    }
    
    Ok(())
}