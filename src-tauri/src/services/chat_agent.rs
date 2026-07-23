use crate::errors::AppResult;
use crate::services::llm_client::{self, LlmConfig, LlmEvent, LlmMessage, ToolDefinition};
use crate::services::tool_registry;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

const MAX_TOOL_ROUNDS: usize = 5;

/// 流式回复，支持 tool calling agent loop
pub async fn stream_reply(
    config: &LlmConfig,
    conversation: Vec<LlmMessage>,
    portfolio_context: Option<String>,
    pool: &SqlitePool,
) -> AppResult<ReceiverStream<AppResult<LlmEvent>>> {
    let system_prompt = build_system_prompt(portfolio_context.as_deref());
    let mut messages = vec![LlmMessage::system(system_prompt)];
    messages.extend(conversation);

    let tools = tool_registry::get_tool_definitions();
    let config = config.clone();
    let pool = pool.clone();
    let (tx, rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        run_agent_loop(&config, &mut messages, &tools, &pool, &tx).await;
    });

    Ok(ReceiverStream::new(rx))
}

async fn run_agent_loop(
    config: &LlmConfig,
    messages: &mut Vec<LlmMessage>,
    tools: &[ToolDefinition],
    pool: &SqlitePool,
    tx: &mpsc::Sender<AppResult<LlmEvent>>,
) {
    for round in 0..MAX_TOOL_ROUNDS {
        eprintln!("[AI_CHAT] Agent loop round {}", round + 1);

        let mut stream = match llm_client::stream_chat(config, messages.clone(), Some(tools.to_vec())).await {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("[AI_CHAT] LLM 调用失败: {}", error.details());
                let _ = tx.send(Err(error)).await;
                return;
            }
        };

        let mut text_parts = Vec::new();
        let mut tool_calls = Vec::new();

        while let Some(event) = stream.next().await {
            match event {
                Ok(LlmEvent::TextChunk(text)) => {
                    text_parts.push(text.clone());
                    if tx.send(Ok(LlmEvent::TextChunk(text))).await.is_err() {
                        return;
                    }
                }
                Ok(LlmEvent::ToolCallComplete { id, name, arguments }) => {
                    tool_calls.push((id, name, arguments));
                }
                Ok(LlmEvent::Done) => break,
                Err(error) => {
                    let _ = tx.send(Err(error)).await;
                    return;
                }
            }
        }

        // No tool calls — we're done
        if tool_calls.is_empty() {
            let _ = tx.send(Ok(LlmEvent::Done)).await;
            return;
        }

        // Tool calls requested — execute them
        let full_text: String = text_parts.concat();
        let llm_tool_calls: Vec<llm_client::ToolCall> = tool_calls
            .iter()
            .map(|(id, name, args)| llm_client::ToolCall {
                id: id.clone(),
                call_type: "function".into(),
                function: llm_client::FunctionCall {
                    name: name.clone(),
                    arguments: args.clone(),
                },
            })
            .collect();

        // Append assistant message with tool calls
        messages.push(LlmMessage {
            role: "assistant".into(),
            content: if full_text.is_empty() {
                None
            } else {
                Some(full_text)
            },
            tool_calls: Some(llm_tool_calls),
            tool_call_id: None,
        });

        // Execute each tool and append results
        for (id, name, args) in &tool_calls {
            let result = match tool_registry::execute_tool(name, args, pool).await {
                Ok(data) => data,
                Err(error) => {
                    serde_json::json!({"error": error.details()}).to_string()
                }
            };
            messages.push(LlmMessage::tool_result(id, result));
        }

        // Continue loop — LLM will see tool results and produce final answer
    }

    eprintln!("[AI_CHAT] 达到最大工具调用轮次 {}", MAX_TOOL_ROUNDS);
    let _ = tx.send(Ok(LlmEvent::Done)).await;
}

/// 构建系统提示
fn build_system_prompt(portfolio_context: Option<&str>) -> String {
    let mut prompt = String::from(
        "你是 Leek Fund 的 AI 投资顾问。你是一位专业的量化投资分析师，擅长基于数据做出科学的投资决策。\n\n\
         核心原则：\n\
         1. 所有分析和建议必须基于工具查询到的真实数据，不能凭空猜测\n\
         2. 明确区分事实（来自数据）、推断（基于证据的推测）和未知（数据不足）\n\
         3. 使用中文回答，回答简洁有力\n\
         4. 保持多轮对话连续性，可以引用用户在当前会话中明确提供的信息\n\
         5. 用户询问之前明确提供的信息时，优先从会话历史查找并原样回答\n\n\
         投资建议框架（当用户询问买卖操作时）：\n\
         你应当给出明确的操作建议（买入/卖出/持有/定投/减仓/加仓），并遵循以下分析框架：\n\n\
         1. **估值分析**：当前净值/价格处于历史什么分位？低估区间优先考虑买入，高估区间考虑减仓\n\
         2. **趋势判断**：近期涨跌趋势如何？是否出现拐点信号？\n\
         3. **风险评估**：最大回撤数据、波动率、集中度是否合理？\n\
         4. **仓位管理**：\n\
            - 单一资产占比不超过 20%\n\
            - 行业/板块过度集中时建议分散\n\
            - 根据用户风险偏好调整（保守/均衡/进取）\n\
         5. **定投策略**：对于长期看好的标的，推荐定投而非择时\n\
         6. **止损止盈**：给出明确的止损位和止盈目标\n\n\
         建议输出格式：\n\
         - 🟢 建议操作：买入/加仓/定投（附理由）\n\
         - 🟡 建议操作：持有/观望（附理由）\n\
         - 🔴 建议操作：卖出/减仓/止损（附理由）\n\
         - 📊 分析依据：列出支撑建议的关键数据点\n\
         - ⚠️ 风险提示：列出可能的风险因素\n\n\
         重要约束：\n\
         - 不使用\"稳赚\"\"必涨\"\"零风险\"等绝对化表述\n\
         - 每个建议必须有数据支撑和逻辑推理过程\n\
         - 承认市场的不确定性，建议中体现概率思维\n\
         - 优先保护本金安全，其次追求收益\n\
         - 对于数据不足的情况，明确告知用户需要更多信息\n\n\
         工具使用说明：\n\
         - 当用户提到具体的基金代码（如 012733）时，请主动调用 get_fund_info 获取实时信息\n\
         - 当用户提到股票代码时，调用 get_stock_quote 获取行情。A股格式如 sh600519，港股如 hk00700\n\
         - 如果用户只说了一个数字（如 01810），可能是港股代码，尝试用 hk 前缀查询\n\
         - 当用户提到加密货币、比特币、BTC、以太坊等时，调用 get_crypto_quote\n\
         - 当用户提到黄金、金价时，调用 get_gold_quote\n\
         - 如果需要深入分析基金，可以调用 get_local_fund_profile 获取本地存储的历史数据\n\
         - 如果用户已在自选列表中添加了该基金，可以调用 get_fund_detail 获取持仓详情\n\
         - 如果不确定资产类型，可以用 search_stock 搜索，或直接尝试最可能的工具\n\
         - 获取到数据后，基于实际数据为用户提供分析和建议\n\
         - 在给出建议前，尽量多调用工具获取完整数据，不要在数据不足时仓促给出建议\n\n",
    );

    if let Some(context) = portfolio_context {
        prompt.push_str("用户当前投资组合数据：\n");
        prompt.push_str(context);
        prompt.push_str("\n\n请结合会话历史与以上数据回答用户问题。\n");
    }

    prompt
}
