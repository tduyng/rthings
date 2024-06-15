use anyhow::{Context, Result};
use clap::Parser;
use futures::stream::StreamExt;
use langchain_rust::chain::Chain;
use langchain_rust::chain::{builder::ConversationalChainBuilder, ConversationalChain};
use langchain_rust::llm::{openai::OpenAI, OpenAIConfig};
use langchain_rust::memory::SimpleMemory;
use langchain_rust::prompt::HumanMessagePromptTemplate;
use langchain_rust::schemas::Message;
use langchain_rust::template_fstring;
use langchain_rust::{fmt_message, fmt_template};
use langchain_rust::{message_formatter, prompt_args};
use std::io::{self, Write};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The model to use
    #[clap(long, default_value = "llama3")]
    model: String,

    /// The base URL for the LLM API
    #[clap(short, long, default_value = "http://localhost:11434")]
    base_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let state = load_state(&args.base_url, &args.model).await?;

    interactive_mode(&state).await?;
    Ok(())
}

async fn handle_prompt(
    state: &State,
    question: &str,
    loading_sender: mpsc::Sender<()>,
) -> Result<()> {
    let input_variables = prompt_args! {
        "input" => question,
    };

    let stream_result = state.chain.stream(input_variables).await;
    // Stop the loading indicator
    let _ = loading_sender.send(()).await;

    match stream_result {
        Ok(mut stream) => {
            let mut response = String::new();
            while let Some(result) = stream.next().await {
                match result {
                    Ok(data) => {
                        response.push_str(&data.content);
                    }
                    Err(e) => {
                        eprintln!("Stream error: {:?}", e);
                    }
                }
            }
            println!("\n{}", response);
        }
        Err(e) => {
            eprintln!("Stream error: {:?}", e);
        }
    }

    Ok(())
}

async fn interactive_mode(state: &State) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    println!("Welcome to Ollar AI, can I help you?");
    println!("Enter your question or type 'Ctrl+C' to quit");

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        // Create a channel to communicate with the loading task
        let (loading_sender, mut loading_receiver) = mpsc::channel(1);

        // Spawn the loading indicator task
        let loading_handle = task::spawn(async move {
            let mut loading = true;
            while loading {
                print!("loading...\n");
                io::stdout().flush().unwrap();
                sleep(Duration::from_millis(500)).await;

                // Check if we received a message to stop the loading indicator
                if let Ok(_) = loading_receiver.try_recv() {
                    loading = false;
                }
            }
        });

        handle_prompt(state, input.trim(), loading_sender).await?;
        loading_handle.await?;

        input.clear();
    }
}

pub async fn create_conversation_chain(
    ollama_base_url: &str,
    model: &str,
) -> Result<ConversationalChain> {
    let llm = OpenAI::default()
        .with_config(
            OpenAIConfig::default()
                .with_api_base(format!("{}/v1", ollama_base_url))
                .with_api_key("ollama"),
        )
        .with_model(model);

    let memory = SimpleMemory::new();

    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            "You are a versatile AI assistant capable of answering a wide range of questions, including those related to programming, technology, general knowledge, and current events. Provide detailed, accurate, and helpful responses."
        )),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
            "{input}", "input"
        ))),
    ];

    ConversationalChainBuilder::new()
        .llm(llm)
        .prompt(prompt)
        .memory(memory.into())
        .build()
        .context("Error building ConversationalChain")
}

pub struct State {
    pub chain: ConversationalChain,
}

pub async fn load_state(base_url: &str, model: &str) -> Result<State> {
    let chain = create_conversation_chain(base_url, model).await?;

    Ok(State { chain })
}
