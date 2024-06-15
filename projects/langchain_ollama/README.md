# Ollar AI CLI

Ollar AI CLI is a command-line interface application powered by the Ollama conversational AI service. It allows users to interact with an AI assistant capable of answering questions on various topics.

## Requirements

To run this application, ensure you have the following installed:

- **Rust**: The Rust programming language and toolchain. Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: Rust's package manager and build system. It is included with Rust installation.
- **Ollama Account**: You need access to the Ollama API. Sign up and get an API key from [Ollama](https://ollama.com).

## Installation

### Local Installation (via Cargo)

If you have cloned the repository locally or downloaded the source code, you can install the Ollar AI CLI using Cargo:

```sh
cargo install --path .
```

### From Git

You can also install directly from the GitHub repository using Cargo:

```sh
cargo install --git https://github.com/tduyng/rthings/projects/lanchain_ollama
```

## Usage

After installation, you can start interacting with the Ollar AI CLI. Here's how to use it:

```sh
ollar
```

```sh
> ollar -h
Ollar AI CLI

Usage: ollar [OPTIONS]

Options:
      --model <MODEL>  The model to use [default: llama3]
      --url <URL>      The base URL for the LLM API [default: http://localhost:11434]
  -h, --help           Print help
  -V, --version        Print version
```
You can specify the model (default: llama3) and the url of the Ollama service running on your machine (default: http://localhost:11434)

Once launched, the application will prompt you to enter your question:

```sh
Welcome to Ollar AI, can I help you?
Enter your question or type 'Ctrl+C' to quit
→ What is the capital of France?
Loading...
```

The AI will process your question and provide an informative response based on its knowledge and the configured model.