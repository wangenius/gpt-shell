# GPT Shell

一个简单的命令行工具，用于在终端中与各种大语言模型进行对话。

## 功能特点

- 支持任意兼容OpenAI API的大语言模型服务
- 支持自定义API端点
- 支持流式输出（实时显示回答）
- 支持自定义系统提示词
- 支持交互式对话（带上下文）
- 支持预设机器人角色
- 支持生成过程中断（Ctrl+C）
- 命令行界面，使用简单
- 彩色输出响应
- 支持配置文件和环境变量配置

## 安装

确保你的系统已安装Rust开发环境，然后执行：

```bash
# 构建
cargo build --release

# 安装（将生成的二进制文件复制到系统路径）
# Windows (PowerShell):
Copy-Item "target/release/gpt.exe" "$env:USERPROFILE/bin/gpt.exe"
# Linux/macOS:
cp target/release/gpt ~/.local/bin/
```

## 配置

### 配置文件
程序会在首次运行时自动在用户主目录创建配置文件：
- Windows: `%USERPROFILE%\.gpt-shell\config.toml`
- Linux/macOS: `~/.gpt-shell\config.toml`

配置文件示例：
```toml
# API密钥
api_key = "your-api-key-here"

# API基础URL
api_url = "https://api.openai.com/v1/chat/completions"

# 默认使用的模型
model = "gpt-3.5-turbo"

# 是否使用流式输出
stream = true

# 系统提示词（可选）
system_prompt = "你是一个有用的AI助手。"
```

### 机器人配置
程序会自动创建机器人配置文件：
- Windows: `%USERPROFILE%\.gpt-shell\bots.toml`
- Linux/macOS: `~/.gpt-shell\bots.toml`

机器人配置示例：
```toml
[bots.programmer]
name = "programmer"
system_prompt = "你是一个专业的程序员，精通各种编程语言和软件开发最佳实践。"

[bots.teacher]
name = "teacher"
system_prompt = "你是一个耐心的��师，善于用简单的方式解释复杂的概念。"
```

## 使用方法

### 查看帮助
```bash
# 显示帮助信息
gpt --help

# 显示配置命令帮助
gpt config --help

# 显示机器人命令帮助
gpt bots --help
```

### 交互式对话
```bash
# 进入交互模式（支持上下文的对话）
gpt

# 使用特定机器人进入交互模式
gpt -b programmer

# 示例对话：
> 你好
你好！有什么我可以帮你的吗？

> 1+1等于几？
1+1等于2。

> 为什么？
因为在基础数学中，1+1表示将两个单位数1相加...
[按 Ctrl+C 取消生成]
已取消生成。

> exit
再见！
```

### 单次对话
```bash
# 直接提问（不保留上下文）
gpt 你好

# 使用特定机器人
gpt -b programmer "解释一下什么是闭包"

# 复杂问题
gpt "解释一下什么是闭包"
[按 Ctrl+C 可以随时取消生成]
```

### 配置管理
```bash
# 打开配置文件
gpt config

# 显示当前配置
gpt config show

# 设置API密钥
gpt config key sk-xxxxxxxxxxxxxxxx

# 设置API URL（使用其他兼容服务）
gpt config url https://api.example.com/v1/chat/completions

# 设置默认模型
gpt config model gpt-4

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
gpt bots list

# 添加新机器人
gpt bots add coder -s "你是一个资深的代码审查专家，擅长代码优化和最佳实践建议。"

# 删除机器人
gpt bots remove coder

# 使用机器人（单次对话）
gpt -b programmer "解释一下设计模式"

# 使用机器人（交互模式）
gpt -b programmer
```

## 支持的服务

本工具支持所有兼容OpenAI API的服务，包括但不限于：

1. OpenAI
   ```bash
   gpt config url https://api.openai.com/v1/chat/completions
   ```

2. DeepSeek
   ```bash
   gpt config url https://api.deepseek.com/v1/chat/completions
   ```

3. 其他兼容服务
   - Azure OpenAI
   - Claude API
   - 本地部署的开源模型（如使用LMStudio）

## 注意事项

- 请确保你有相应服务的有效API密钥
- API调用可能会产生费用，请参考各服务提供商的定价策略
- 请妥善保管你的API密钥，不要将其提交到版本控制系统中
- 不同的模型可能有不同的定价和功能限制
- 各服务提供商的API可能有不同的使用限制和响应特点
- 配置文件修改后立即生效，无需重启程序
- 配置文件和API密钥都保存在用户主目录下，便于管理
- 交互模式下的对话历史会保留在内存中，关闭程序后会清除
- 预设机器人可以帮助你快速切换不同的对话角色和场景
- 机器人可以在单次对话和交互模式下使用
- 在生成回答过程中，可以随时按 Ctrl+C 取消生成

# 打包

使用 `cargo build --release` 构建。
