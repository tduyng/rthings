use anyhow::{Context, Result};
use langchain_rust::chain::{builder::ConversationalChainBuilder, ConversationalChain};
use langchain_rust::llm::{openai::OpenAI, OpenAIConfig};
use langchain_rust::memory::SimpleMemory;
use langchain_rust::message_formatter;
use langchain_rust::prompt::HumanMessagePromptTemplate;
use langchain_rust::schemas::Message;
use langchain_rust::template_fstring;
use langchain_rust::{fmt_message, fmt_template};

pub struct State {
    pub chain: ConversationalChain,
}

pub async fn load_state(base_url: &str, model: &str) -> Result<State> {
    let chain = create_conversation_chain(base_url, model).await?;

    Ok(State { chain })
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
