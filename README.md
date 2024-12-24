# GPT Shell

<div align="center">
  
  <img src="assets/icon.png" alt="GPT Shell Logo" width="200">

  <h1>GPT Shell</h1>
  <p><strong>Professional Terminal Assistant for LLM Interactions</strong></p>

  [![GitHub stars](https://img.shields.io/github/stars/wangenius/gpt-shell)](https://github.com/wangenius/gpt-shell/stargazers)
  [![License](https://img.shields.io/github/license/wangenius/gpt-shell)](https://github.com/wangenius/gpt-shell/blob/master/LICENSE)
  ![Platform](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-blue)
  [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/wangenius/gpt-shell/pulls)
  ![Version](https://img.shields.io/badge/version-${CURRENT_VERSION}-blue)

  <p align="center">
    <a href="#key-features">Key Features</a> •
    <a href="#quick-start">Quick Start</a> •
    <a href="#usage-guide">Usage Guide</a> •
    <a href="#advanced-configuration">Advanced Configuration</a>
  </p>

</div>

## 📖 Overview

GPT Shell is a powerful terminal AI assistant built with Rust, enabling direct interaction with various Large Language Models (LLMs) through the command line. With its clean interface and support for multiple AI providers, it's the ideal choice for developers and terminal users. Whether you need programming assistance, text creation, or task automation, GPT Shell provides professional support.

## 🌟 Key Features

<table>
<tr>
<td>

### 🔌 Core Features
- **Multi-Model Support**: Compatible with OpenAI, DeepSeek, Tongyi, ZhiPu, and more
- **Custom Endpoints**: Support for private deployments and compatible services
- **Real-time Response**: Stream API-based output
- **Role Presets**: Quick switching between different AI assistant scenarios
- **Command Aliases**: Single-character shortcut support
- **Cross-Platform**: Full support for Windows, MacOS, and Linux
- **Auto Updates**: Built-in version checking and updating

</td>
<td>

### 🎯 User Experience
- **Interactive Mode**: Natural conversation flow
- **Direct Queries**: Quick one-off questions
- **Colored Output**: Enhanced terminal display
- **Context Memory**: Automatic conversation history management
- **Generation Control**: Ctrl+C interrupt support
- **Flexible Config**: Environment variables and config file support
- **JSON Mode**: Structured data interaction support

</td>
</tr>
</table>

### 🤖 Supported Providers

- ✅ **OpenAI**: GPT-3.5/4 models with complete streaming response
- ✅ **DeepSeek**: DeepSeek-Chat models with custom API endpoints
- ✅ **Tongyi**: Qianwen models with configurable parameters
- ✅ **ZhiPu**: ChatGLM models with flexible interface adaptation
- ✅ **Other models**: Compatible with OpenAI API format

## ⚡ Quick Start

### Installation

```powershell
# Windows PowerShell
irm https://raw.githubusercontent.com/wangenius/gpt-shell/refs/heads/master/install.ps1 | iex

# For other platforms, please refer to the online documentation
```

### Command Line Arguments

```bash
gpt [OPTIONS] [PROMPT]

Options:
  -b, --bot <BOT>      Use specified preset role
  -a, --agent <AGENT>  Use specified intelligent agent
  -h, --help          Display help information
  -V, --version       Display version information

Subcommands:
  update              Check and install updates
  config              Configuration management
  bots                Role management
```

### Basic Commands

```bash
# Start interactive session
gpt

# Direct question
gpt "How to use Docker?"

# Use specific role
gpt -b programmer "Code review"

# Show help
gpt --help
```

## 📚 Usage Guide

### Interactive Mode
```bash
gpt
> Hello
Hello! How can I help you today?

> What is a closure?
A closure is a function that can access free variables...
[Press Ctrl+C to interrupt generation]

> exit
Goodbye!
```

### Model Configuration
```bash
# Add model
gpt config model add openai sk-xxxxxxxxxxxxxxxx

# Add custom model
gpt config model add deepseek your-api-key \
  --url https://api.deepseek.com/v1/chat/completions \
  --model deepseek-chat

# View and switch models
gpt config model list
gpt config model use deepseek
gpt config model remove openai
```

Note: `--url` should be a complete API address, like `https://api.deepseek.com/v1/chat/completions`, not just the base URL.

### Role Management
```bash
# View all roles
gpt bots

# Add custom role
gpt bots add reviewer -s "You are a professional code review expert"

# Manage role aliases
gpt bots alias set reviewer r
gpt bots alias list
gpt bots alias remove r
```

## 🤖 Intelligent Agent System

GPT Shell provides a powerful intelligent agent system with high-performance command execution and state management implemented in Rust:

- **System Prompts**: Define agent roles and behavior characteristics
- **Environment Variables**: Configure system environment variables for command execution
- **Command Templates**: Predefined common command templates
- **Smart Command Execution**: Automatic variable replacement and execution
- **State Management**: Maintain conversation context and execution state
- **Concurrency Control**: Support for async operations and task cancellation
- **Standardized Interaction**: JSON format request-response

### Agent Configuration Example

```toml
name = "Development Assistant"
description = "Professional development tool"
system_prompt = "You are an experienced developer"

[env]
editor = "code"
browser = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe"

[templates]
open_file = "{{editor}} {{file}}"
open_browser = "start {{browser}} {{url}}"
```

## ⚙️ Advanced Configuration

### System Settings
```bash
# View configuration
gpt config

# Edit settings
gpt config edit

# Update system prompt
gpt config system "You are a professional developer"

# Feature toggles
gpt config stream true
```

### Configuration Storage
- Config file location: `~/.gpt-shell/`
- Secure API key storage
- Automatic session history management
- .env environment variable support

### Technical Features
- **Async Processing**: tokio-based async runtime
- **Stream Transfer**: Real-time response handling
- **Memory Safety**: Rust-guaranteed memory safety
- **Error Handling**: Comprehensive error handling
- **Type System**: Strong typing for data safety

## 🔒 Security Best Practices

- **API Keys**: Secure storage, avoid version control
- **Access Control**: Careful permission management
- **Cost Management**: Monitor API usage
- **Data Privacy**: Mind data handling policies
- **Updates**: Keep tools and dependencies current

## 🤝 Contributing

Welcome to contribute to the project:

- 🐛 Report issues and bugs
- 💡 Suggest new features
- 🔧 Submit code improvements
- 📖 Improve project documentation
- 🌍 Enhance translations

### Development Guide
1. Clone project and install dependencies
2. Follow Rust coding standards
3. Write test cases
4. Ensure all tests pass before submitting PR

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

<div align="center">
  
**[Documentation](https://github.com/wangenius/gpt-shell/wiki)** • 
**[Report Bug](https://github.com/wangenius/gpt-shell/issues)** • 
**[Request Feature](https://github.com/wangenius/gpt-shell/issues)**

</div>

## Acknowledgments

- [FREE-CHATGPT-API](https://github.com/popjane/free_chatgpt_api)


