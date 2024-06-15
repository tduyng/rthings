use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use langchain_rust::chain::Chain;
use langchain_rust::prompt_args;
use std::io::{self, Write};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;

use crate::state::State;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// Ollar AI CLI
pub struct Cli {
    /// The model to use
    #[clap(long, default_value = "llama3")]
    pub model: String,

    /// The base URL for the LLM API
    #[clap(long, default_value = "http://localhost:11434")]
    pub url: String,
}

pub async fn interactive_mode(state: &State) -> Result<()> {
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
                print!("> loading...");
                io::stdout().flush().unwrap();
                sleep(Duration::from_millis(500)).await;

                // Check if we received a message to stop the loading indicator
                if loading_receiver.try_recv().is_ok() {
                    loading = false;
                }
            }
        });

        handle_prompt(state, input.trim(), loading_sender).await?;
        loading_handle.await?;

        input.clear();
    }
}

pub async fn handle_prompt(
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
