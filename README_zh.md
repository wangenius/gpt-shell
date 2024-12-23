# GPT Shell

![icon](assets/icon.png)

一个用于在终端中与各种大语言模型进行交互的多功能命令行工具。

## 主要特性

- 支持任何兼容 OpenAI API 的 LLM 服务
- 支持多模型配置（轻松切换不同的服务提供商）
- 支持自定义 API 端点
- 流式输出（实时显示响应）
- 自定义系统提示词
- 支持上下文的交互式对话
- 支持单字符别名的预设机器人角色
- 支持生成过程中断（Ctrl+C）
- 简单的命令行界面
- 彩色输出
- 支持通过文件和环境变量进行配置
- 内置支持主流服务提供商：
  - OpenAI
  - DeepSeek
  - Azure OpenAI
  - Claude API
  - 本地 LLMs（通过兼容服务器如 LMStudio）


## 安装

在 PowerShell (Windows) 中执行以下命令：

```powershell
irm https://raw.githubusercontent.com/wangenius/gpt-shell/refs/heads/master/install.ps1 | iex
```

安装完成后即可开始使用。

## 使用方法

### 帮助命令
```bash
# 显示主要帮助信息
gpt --help

# 显示配置命令帮助
gpt config --help

# 显示机器人命令帮助
gpt bots --help
```

### 交互模式
```bash
# 进入交互模式
gpt

# 使用特定机器人
gpt -b programmer

# 使用机器人别名
gpt -p

示例对话：
> Hello
你好！我能帮你什么忙？

> 什么是闭包？
闭包是一个可以捕获其环境的函数...
[按 Ctrl+C 可取消生成]
已取消生成。

> exit
再见！
```

### 单次查询
```bash
# 直接提问（无上下文）
gpt "你好"

# 使用特定机器人
gpt -bot programmer "解释闭包"

# 使用机器人别名
gpt -p "解释闭包"
```

### 模型管理
```bash
# 添加新模型
gpt config model add openai sk-xxxxxxxxxxxxxxxx

# 添加带自定义 URL 和模型名称的模型
gpt config model add deepseek your-api-key --url https://api.deepseek.com/v1/chat/completions --model deepseek-chat

# 移除模型
gpt config model remove openai

# 列出模型
gpt config model list

# 切换模型
gpt config model use deepseek
```

### 配置管理
```bash
# 显示当前配置
gpt config

# 编辑配置文件
gpt config edit

# 设置系统提示词
gpt config system "你是一个专业的程序员"

# 清除系统提示词
gpt config system

# 设置流式输出
gpt config stream true
```

### 机器人管理
```bash
# 列出所有机器人
gpt bots

# 编辑机器人配置
gpt bots edit

# 添加新机器人
gpt bots add coder -s "你是一个资深的代码审查专家。"

# 移除机器人
gpt bots remove coder

# 管理别名
gpt bots alias set programmer p    # 设置别名
gpt bots alias remove p            # 移除别名
gpt bots alias list                # 列出别名
```

## 重要说明

- 确保你拥有要使用的服务的有效 API 密钥
- API 调用可能产生费用 - 请查看提供商的定价
- 确保 API 密钥安全 - 切勿提交到版本控制系统
- 不同模型可能有不同的定价和限制
- 配置更改立即生效
- 配置文件和 API 密钥存储在你的主目录中
- 交互模式下的对话历史在程序关闭时会被清除
- 机器人别名提供单字符快捷访问
- 你可以随时使用 Ctrl+C 取消生成