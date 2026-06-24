use ai_agent::models::action_plan::ActionPlan;
use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use ai_agent::{
    constant::KIMI_2_6, llm::complete::chat_complete, llm::structured::chat_complete_structured,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let plan: ActionPlan = chat_complete_structured(
        KIMI_2_6,
        Some("你是一个全能的助手"),
        "我要去美加墨世界杯观看比赛如何安排？",
    )
    .await?;
    println!("Response: {plan:#?}");
    Ok(())
}
