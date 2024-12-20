# GPT Shell

A versatile command-line tool for interacting with various Large Language Models through the terminal.

## Key Features

- Supports any LLM service compatible with OpenAI API
- Multiple model configuration support (easily switch between different providers)
- Custom API endpoints support
- Streaming output (real-time response display)
- Custom system prompts
- Interactive conversations with context
- Preset bot roles with single-character aliases
- Generation interruption (Ctrl+C)
- Simple command-line interface
- Colored output
- Configuration via files and environment variables
- Built-in support for popular providers:
  - OpenAI
  - DeepSeek
  - Azure OpenAI
  - Claude API
  - Local LLMs (via compatible servers like LMStudio)

## Installation

Ensure you have Rust installed, then:

```bash
# Build
cargo build --release

# Install (copy binary to system path)
# Windows (PowerShell):
Copy-Item "target/release/gpt.exe" "$env:USERPROFILE/bin/gpt.exe"
# Linux/macOS:
cp target/release/gpt ~/.local/bin/
```

## Configuration

### Config File
The program automatically creates a configuration file in your home directory:
- Windows: `%USERPROFILE%\.gpt-shell\config.toml`
- Linux/macOS: `~/.gpt-shell/config.toml`

Example configuration:
```toml
# Model configurations
[models.openai]
api_key = "your-api-key-here"
api_url = "https://api.openai.com/v1/chat/completions"
model = "gpt-3.5-turbo"

[models.deepseek]
api_key = "your-deepseek-key"
api_url = "https://api.deepseek.com/v1/chat/completions"
model = "deepseek-chat"

# Current active model
current_model = "openai"

# Whether to use streaming output
stream = true

# System prompt (optional)
system_prompt = "You are a helpful AI assistant."
```

### Bot Configuration
Bot configurations are stored in:
- Windows: `%USERPROFILE%\.gpt-shell\bots.toml`
- Linux/macOS: `~/.gpt-shell/bots.toml`

Example bot configuration:
```toml
[bots.programmer]
name = "programmer"
system_prompt = "You are a professional programmer, proficient in various programming languages and software development best practices."

[bots.teacher]
name = "teacher"
system_prompt = "You are a patient teacher, good at explaining complex concepts in simple ways."

# Bot aliases (single character shortcuts)
[aliases]
p = "programmer"
t = "teacher"
```

## Usage

### Help Commands
```bash
# Show main help
gpt --help

# Show config command help
gpt config --help

# Show bots command help
gpt bots --help
```

### Interactive Mode
```bash
# Enter interactive mode
gpt

# Use a specific bot
gpt -b programmer

# Use a bot alias
gpt -p

Example conversation:
> Hello
Hello! How can I help you?

> What is a closure?
A closure is a function that captures its environment...
[Press Ctrl+C to cancel generation]
Generation cancelled.

> exit
Goodbye!
```

### Single Queries
```bash
# Direct question (no context)
gpt "Hello"

# Use a specific bot
gpt -b programmer "Explain closures"

# Use a bot alias
gpt -p "Explain closures"
```

### Model Management
```bash
# Add new model
gpt config model add openai sk-xxxxxxxxxxxxxxxx

# Add model with custom URL and model name
gpt config model add deepseek your-api-key --url https://api.deepseek.com/v1/chat/completions --model deepseek-chat

# Remove model
gpt config model remove openai

# List models
gpt config model list

# Switch model
gpt config model use deepseek
```

### Configuration Management
```bash
# Show current config
gpt config

# Edit config file
gpt config edit

# Set system prompt
gpt config system "You are a professional programmer"

# Clear system prompt
gpt config system

# Set streaming output
gpt config stream true
```

### Bot Management
```bash
# List all bots
gpt bots

# Edit bots config
gpt bots edit

# Add new bot
gpt bots add coder -s "You are a senior code review expert."

# Remove bot
gpt bots remove coder

# Manage aliases
gpt bots alias set programmer p    # Set alias
gpt bots alias remove p            # Remove alias
gpt bots alias list                # List aliases
```

## Important Notes

- Ensure you have valid API keys for the services you want to use
- API calls may incur costs - check provider pricing
- Keep your API keys secure - never commit them to version control
- Different models may have different pricing and limitations
- Configuration changes take effect immediately
- Config files and API keys are stored in your home directory
- Conversation history in interactive mode is cleared when the program closes
- Bot aliases provide quick access with single-character shortcuts
- You can cancel generation at any time with Ctrl+C

## Building

Use `cargo build --release` to build an optimized binary.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
