use crate::db::agent;
use crate::errors::AppResult;
use crate::models::ChatAgentInfo;
use sqlx::SqlitePool;

const DEFAULT_AGENT_NAME: &str = "chat";
const DEFAULT_AGENT_DESC: &str = "默认对话代理";

pub async fn get_default_agent(pool: &SqlitePool) -> AppResult<ChatAgentInfo> {
    agent::ensure_default_agent(pool, DEFAULT_AGENT_NAME, DEFAULT_AGENT_DESC).await
}
