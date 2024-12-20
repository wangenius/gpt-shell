# GPT Shell

A simple command line tool for chatting with various large language models in the terminal.

## Features

- Supports any large language model service compatible with OpenAI API
- Supports custom API endpoints
- Supports streaming output (real-time display of answers)
- Supports custom system prompt
- Supports interactive conversation (with context)
- Supports preset bot roles
- Supports interrupting generation (Ctrl+C)
- Command line interface, easy to use
- Colored output response
- Supports configuration file and environment variable configuration

## Installation

Ensure your system has Rust development environment installed, then execute:

```bash
# build
cargo build --release

# install (copy the generated binary file to the system path)
# Windows (PowerShell):
Copy-Item "target/release/gpt.exe" "$env:USERPROFILE/bin/gpt.exe"
# Linux/macOS:
cp target/release/gpt ~/.local/bin/
```

## Configuration

### Configuration file
The program will automatically create a configuration file in the user's home directory:
- Windows: `%USERPROFILE%\.gpt-shell\config.toml`
- Linux/macOS: `~/.gpt-shell\config.toml`

Configuration file example:
```toml
# API key
api_key = "your-api-key-here"

# API base URL
api_url = "https://api.openai.com/v1/chat/completions"

# Default model
model = "gpt-3.5-turbo"

# Whether to use streaming output
stream = true

# System prompt (optional)
system_prompt = "You are a useful AI assistant."
```

### Bot configuration
The program will automatically create a bot configuration file:
- Windows: `%USERPROFILE%\.gpt-shell\bots.toml`
- Linux/macOS: `~/.gpt-shell\bots.toml`

Bot configuration example:
```toml
[bots.programmer]
name = "programmer"
system_prompt = "You are a professional programmer, proficient in various programming languages and software development best practices."

[bots.teacher]
name = "teacher"
system_prompt = "You are a patient teacher, good at explaining complex concepts in simple ways."
```

## Usage

### View help
```bash
# Display help information
gpt --help

# Display configuration command help
gpt config --help

# Display bot command help
gpt bots --help
```

### Interactive conversation
```bash
# Enter interactive mode (supports context-based conversation)
gpt

# Enter interactive mode using a specific bot
gpt -b programmer

# Example conversation:
> Hello
Hello! How can I help you?

> What is 1+1?
1+1 equals 2.

> Why?
Because in basic mathematics, 1+1 represents adding two units of 1...
[Press Ctrl+C to cancel generation at any time]
Generation cancelled.

> exit
Goodbye!
```

### Single conversation
```bash
# Direct question (without context)
gpt "Hello"

# Use a specific bot
gpt -b programmer "Explain what closures are"

# Complex question
gpt "Explain what closures are"
[Press Ctrl+C to cancel generation at any time]
```

### Configuration management
```bash
# Open configuration file
gpt config

# Display current configuration
gpt config show

# Set API key
gpt config key sk-xxxxxxxxxxxxxxxx

# Set API URL (use other compatible services)
gpt config url https://api.example.com/v1/chat/completions

# Set default model
gpt config model gpt-4

# Set system prompt
gpt config system "You are a professional programmer"

# Clear system prompt
gpt config system

# Set streaming output
gpt config stream true
```

### Bot management
```bash
# List all bots
gpt bots list

# Add new bot
gpt bots add coder -s "You are a senior code review expert, good at code optimization and best practice suggestions."

# Delete bot
gpt bots remove coder

# Use bot (single conversation)
gpt -b programmer "Explain design patterns"

# Use bot (interactive mode)
gpt -b programmer

# Manage aliases
gpt bots alias                           # show alias help
gpt bots alias set programmer p          # set alias 'p' for bot 'programmer'
gpt bots alias remove p                  # remove alias 'p'
gpt bots alias list                      # list all aliases

# Use bot with alias (single conversation)
gpt -t p "Explain design patterns"

# Use bot with alias (interactive mode)
gpt -t p
```

### Alias management
```bash
# List all aliases
gpt config alias list
```

## Supported services

This tool supports all compatible services with the OpenAI API, including but not limited to:

1. OpenAI
   ```bash
   gpt config url https://api.openai.com/v1/chat/completions
   ```

2. DeepSeek
   ```bash
   gpt config url https://api.deepseek.com/v1/chat/completions
   ```

3. Other compatible services
   - Azure OpenAI
   - Claude API
   - Local deployment of open source models (such as using LMStudio)

## Notes

- Ensure you have a valid API key for the corresponding service
- API calls may incur costs, please refer to the pricing strategies of the service providers
- Please keep your API key secure and do not submit it to version control systems
- Different models may have different pricing and feature limitations
- The APIs of different service providers may have different usage limits and response characteristics
- Configuration file changes take effect immediately without restarting the program
- Configuration files and API keys are stored in the user's home directory for easy management
- Conversation history in interactive mode is retained in memory and cleared when the program is closed
- Preset bots help you quickly switch between different conversation roles and scenarios
- Bots can be used in single conversations and interactive mode
- You can press Ctrl+C to cancel generation at any time during the generation process

# Build

Use `cargo build --release` to build.
