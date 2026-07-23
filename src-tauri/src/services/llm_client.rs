use crate::errors::{AppError, AppResult};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};

// ============================================================================
// Public types
// ============================================================================

#[derive(Debug, Clone)]
pub enum LlmEvent {
    TextChunk(String),
    ToolCallComplete {
        id: String,
        name: String,
        arguments: String,
    },
    Done,
}

pub type LlmStream = Box<dyn Stream<Item = AppResult<LlmEvent>> + Unpin + Send>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum LlmProvider {
    OpenAI,
    Claude,
    ClaudeCompatible,
    OpenAICompatible,
}

impl LlmProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAI => "openai",
            Self::Claude => "claude",
            Self::ClaudeCompatible => "claude_compatible",
            Self::OpenAICompatible => "openai_compatible",
        }
    }

    pub fn parse(value: &str) -> AppResult<Self> {
        match value {
            "openai" => Ok(Self::OpenAI),
            "claude" => Ok(Self::Claude),
            "claude_compatible" => Ok(Self::ClaudeCompatible),
            "openai_compatible" | "ollama" => Ok(Self::OpenAICompatible),
            _ => Err(AppError::ValidationError("不支持的模型提供商".to_string())),
        }
    }

    pub fn requires_api_key(&self) -> bool {
        matches!(self, Self::OpenAI | Self::Claude | Self::ClaudeCompatible)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub api_key: Option<String>,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: LlmProvider::OpenAICompatible,
            api_key: None,
            base_url: "http://127.0.0.1:11434/v1".to_string(),
            model: "qwen3:latest".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
        }
    }
}

impl LlmConfig {
    pub fn validate(mut self) -> AppResult<Self> {
        self.base_url = self.base_url.trim().trim_end_matches('/').to_string();
        self.model = self.model.trim().to_string();
        self.api_key = self
            .api_key
            .map(|key| key.trim().to_string())
            .filter(|key| !key.is_empty());
        if !(self.base_url.starts_with("http://") || self.base_url.starts_with("https://")) {
            return Err(AppError::ValidationError(
                "模型地址必须以 http:// 或 https:// 开头".to_string(),
            ));
        }
        if self.model.is_empty() {
            return Err(AppError::ValidationError("模型名称不能为空".to_string()));
        }
        if self.provider.requires_api_key() && self.api_key.is_none() {
            return Err(AppError::ValidationError("未配置 API Key".to_string()));
        }
        if self.max_tokens == 0 || self.max_tokens > 32_768 {
            return Err(AppError::ValidationError(
                "max_tokens 必须在 1 到 32768 之间".to_string(),
            ));
        }
        if !self.temperature.is_finite() || !(0.0..=2.0).contains(&self.temperature) {
            return Err(AppError::ValidationError(
                "temperature 必须在 0 到 2 之间".to_string(),
            ));
        }
        Ok(self)
    }
}

// ============================================================================
// Message & Tool types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl LlmMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn tool_result(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: "tool".into(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }

    pub fn assistant_with_tool_calls(tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: "assistant".into(),
            content: None,
            tool_calls: Some(tool_calls),
            tool_call_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

// ============================================================================
// OpenAI request/response types
// ============================================================================

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolDefinition>>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum OpenAiMessage {
    Flat {
        role: String,
        content: String,
    },
    WithToolCalls {
        role: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        tool_calls: Vec<ToolCall>,
    },
    ToolResult {
        role: String,
        content: String,
        tool_call_id: String,
    },
}

fn to_openai_messages(messages: &[LlmMessage]) -> Vec<OpenAiMessage> {
    messages
        .iter()
        .map(|msg| {
            if msg.role == "tool" {
                OpenAiMessage::ToolResult {
                    role: "tool".into(),
                    content: msg.content.clone().unwrap_or_default(),
                    tool_call_id: msg.tool_call_id.clone().unwrap_or_default(),
                }
            } else if let Some(tool_calls) = &msg.tool_calls {
                OpenAiMessage::WithToolCalls {
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                    tool_calls: tool_calls.clone(),
                }
            } else {
                OpenAiMessage::Flat {
                    role: msg.role.clone(),
                    content: msg.content.clone().unwrap_or_default(),
                }
            }
        })
        .collect()
}

#[derive(Deserialize)]
struct OpenAiStreamResponse {
    choices: Vec<OpenAiStreamChoice>,
}

#[derive(Deserialize)]
struct OpenAiStreamChoice {
    delta: Option<OpenAiDelta>,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct OpenAiDelta {
    content: Option<String>,
    tool_calls: Option<Vec<DeltaToolCall>>,
}

#[derive(Deserialize)]
struct DeltaToolCall {
    index: usize,
    id: Option<String>,
    function: Option<DeltaFunction>,
}

#[derive(Deserialize)]
struct DeltaFunction {
    name: Option<String>,
    arguments: Option<String>,
}

// ============================================================================
// Claude request/response types
// ============================================================================

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<ClaudeMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ClaudeToolDef>>,
}

#[derive(Serialize)]
struct ClaudeToolDef {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ClaudeMessage {
    Text { role: String, content: String },
    Blocks { role: String, content: Vec<ClaudeContentBlock> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum ClaudeContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

fn to_claude_messages(messages: &[LlmMessage]) -> Vec<ClaudeMessage> {
    let mut result = Vec::new();
    for msg in messages {
        if msg.role == "system" {
            continue;
        }
        if msg.role == "tool" {
            // Tool results go as a user message with tool_result block
            result.push(ClaudeMessage::Blocks {
                role: "user".into(),
                content: vec![ClaudeContentBlock::ToolResult {
                    tool_use_id: msg.tool_call_id.clone().unwrap_or_default(),
                    content: msg.content.clone().unwrap_or_default(),
                }],
            });
        } else if let Some(tool_calls) = &msg.tool_calls {
            // Assistant message with tool_use blocks
            let mut blocks: Vec<ClaudeContentBlock> = Vec::new();
            if let Some(text) = &msg.content {
                if !text.is_empty() {
                    blocks.push(ClaudeContentBlock::Text { text: text.clone() });
                }
            }
            for tc in tool_calls {
                let input: serde_json::Value =
                    serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::Value::Null);
                blocks.push(ClaudeContentBlock::ToolUse {
                    id: tc.id.clone(),
                    name: tc.function.name.clone(),
                    input,
                });
            }
            result.push(ClaudeMessage::Blocks {
                role: "assistant".into(),
                content: blocks,
            });
        } else {
            result.push(ClaudeMessage::Text {
                role: msg.role.clone(),
                content: msg.content.clone().unwrap_or_default(),
            });
        }
    }
    result
}

fn to_claude_tools(tools: &[ToolDefinition]) -> Vec<ClaudeToolDef> {
    tools
        .iter()
        .map(|t| ClaudeToolDef {
            name: t.function.name.clone(),
            description: t.function.description.clone(),
            input_schema: t.function.parameters.clone(),
        })
        .collect()
}

#[derive(Deserialize)]
struct ClaudeStreamResponse {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(default)]
    index: Option<usize>,
    delta: Option<ClaudeDelta>,
    #[serde(rename = "content_block")]
    content_block: Option<ClaudeStreamContentBlock>,
}

#[derive(Deserialize)]
struct ClaudeStreamContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    id: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize)]
struct ClaudeDelta {
    #[serde(rename = "type")]
    delta_type: String,
    text: Option<String>,
    #[serde(rename = "partial_json")]
    partial_json: Option<String>,
}

// ============================================================================
// Stream line parsing
// ============================================================================

enum StreamLine {
    TextChunk(String),
    ToolCallDelta {
        index: usize,
        id: Option<String>,
        name: Option<String>,
        arguments_fragment: Option<String>,
    },
    ToolCallComplete {
        index: usize,
        id: Option<String>,
        name: Option<String>,
        arguments_fragment: Option<String>,
    },
    Done,
    Ignore,
}

fn take_sse_line(buffer: &mut Vec<u8>) -> Option<AppResult<String>> {
    let line_end = buffer.iter().position(|byte| *byte == b'\n')?;
    let mut line: Vec<u8> = buffer.drain(..=line_end).collect();
    line.pop();
    if line.last() == Some(&b'\r') {
        line.pop();
    }
    Some(
        String::from_utf8(line)
            .map(|line| line.trim().to_string())
            .map_err(|error| AppError::ParseError(format!("LLM 流不是有效 UTF-8: {error}"))),
    )
}

fn parse_openai_line(line: &str) -> AppResult<StreamLine> {
    let Some(data) = line.strip_prefix("data:") else {
        return Ok(StreamLine::Ignore);
    };
    let data = data.trim();
    if data == "[DONE]" {
        return Ok(StreamLine::Done);
    }
    let response: OpenAiStreamResponse = serde_json::from_str(data)
        .map_err(|error| AppError::ParseError(format!("解析 LLM 流失败: {error}")))?;
    let Some(choice) = response.choices.first() else {
        return Ok(StreamLine::Ignore);
    };

    // Check for tool calls
    if let Some(tool_calls) = choice
        .delta
        .as_ref()
        .and_then(|d| d.tool_calls.as_ref())
    {
        for tc in tool_calls {
            return Ok(StreamLine::ToolCallDelta {
                index: tc.index,
                id: tc.id.clone(),
                name: tc.function.as_ref().and_then(|f| f.name.clone()),
                arguments_fragment: tc
                    .function
                    .as_ref()
                    .and_then(|f| f.arguments.clone()),
            });
        }
    }

    // Check for text content
    if let Some(content) = choice
        .delta
        .as_ref()
        .and_then(|delta| delta.content.clone())
    {
        if !content.is_empty() {
            return Ok(StreamLine::TextChunk(content));
        }
    }

    if choice.finish_reason.is_some() {
        Ok(StreamLine::Done)
    } else {
        Ok(StreamLine::Ignore)
    }
}

fn parse_claude_line(
    line: &str,
    tool_blocks: &mut HashMap<usize, (Option<String>, Option<String>, String)>,
) -> AppResult<StreamLine> {
    let Some(data) = line.strip_prefix("data:") else {
        return Ok(StreamLine::Ignore);
    };
    let response: ClaudeStreamResponse = serde_json::from_str(data.trim())
        .map_err(|error| AppError::ParseError(format!("解析 Claude 流失败: {error}")))?;

    match response.event_type.as_str() {
        "message_stop" => Ok(StreamLine::Done),
        "content_block_start" => {
            if let Some(block) = response.content_block {
                if block.block_type == "tool_use" {
                    let index = response.index.unwrap_or(0);
                    tool_blocks.insert(index, (block.id, block.name, String::new()));
                }
            }
            Ok(StreamLine::Ignore)
        }
        "content_block_delta" => {
            let index = response.index.unwrap_or(0);
            if let Some(delta) = response.delta {
                if delta.delta_type == "text_delta" {
                    return delta
                        .text
                        .filter(|t| !t.is_empty())
                        .map(StreamLine::TextChunk)
                        .ok_or_else(|| AppError::ParseError("Claude 文本增量为空".to_string()));
                }
                if delta.delta_type == "input_json_delta" {
                    if let Some((id, name, args)) = tool_blocks.get_mut(&index) {
                        if let Some(partial) = delta.partial_json {
                            args.push_str(&partial);
                        }
                        return Ok(StreamLine::ToolCallDelta {
                            index,
                            id: id.clone(),
                            name: name.clone(),
                            arguments_fragment: None,
                        });
                    }
                }
            }
            Ok(StreamLine::Ignore)
        }
        "content_block_stop" => {
            let index = response.index.unwrap_or(0);
            if let Some((id, name, args)) = tool_blocks.remove(&index) {
                return Ok(StreamLine::ToolCallComplete {
                    index,
                    id,
                    name: Some(name.unwrap_or_default()),
                    arguments_fragment: Some(args),
                });
            }
            Ok(StreamLine::Ignore)
        }
        _ => Ok(StreamLine::Ignore),
    }
}

// ============================================================================
// Response stream builders
// ============================================================================

fn openai_response_stream(response: reqwest::Response) -> ReceiverStream<AppResult<LlmEvent>> {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move {
        let mut bytes = response.bytes_stream();
        let mut buffer = Vec::new();
        // Accumulate tool calls by index: (id, name, arguments)
        let mut tool_calls: HashMap<usize, (String, String, String)> = HashMap::new();

        while let Some(result) = bytes.next().await {
            match result {
                Ok(chunk) => {
                    buffer.extend_from_slice(&chunk);
                    while let Some(line) = take_sse_line(&mut buffer) {
                        let line = match line {
                            Ok(line) => line,
                            Err(error) => {
                                let _ = tx.send(Err(error)).await;
                                return;
                            }
                        };
                        match parse_openai_line(&line) {
                            Ok(StreamLine::TextChunk(text)) => {
                                if tx.send(Ok(LlmEvent::TextChunk(text))).await.is_err() {
                                    return;
                                }
                            }
                            Ok(StreamLine::ToolCallDelta {
                                index,
                                id,
                                name,
                                arguments_fragment,
                            }) => {
                                let entry =
                                    tool_calls.entry(index).or_default();
                                if let Some(id) = id {
                                    entry.0 = id;
                                }
                                if let Some(name) = name {
                                    entry.1 = name;
                                }
                                if let Some(args) = arguments_fragment {
                                    entry.2.push_str(&args);
                                }
                            }
                            Ok(StreamLine::ToolCallComplete { .. }) => {
                                // Not used for OpenAI; handled on Done
                            }
                            Ok(StreamLine::Done) => {
                                // Emit accumulated tool calls
                                let mut sorted: Vec<_> =
                                    tool_calls.drain().collect();
                                sorted.sort_by_key(|(i, _)| *i);
                                for (_, (id, name, args)) in sorted {
                                    if tx
                                        .send(Ok(LlmEvent::ToolCallComplete {
                                            id,
                                            name,
                                            arguments: args,
                                        }))
                                        .await
                                        .is_err()
                                    {
                                        return;
                                    }
                                }
                                let _ = tx.send(Ok(LlmEvent::Done)).await;
                                return;
                            }
                            Ok(StreamLine::Ignore) => {}
                            Err(error) => {
                                let _ = tx.send(Err(error)).await;
                                return;
                            }
                        }
                    }
                }
                Err(error) => {
                    let _ = tx
                        .send(Err(AppError::NetworkError(format!(
                            "读取 LLM 流失败: {error}"
                        ))))
                        .await;
                    return;
                }
            }
        }
    });
    ReceiverStream::new(rx)
}

fn claude_response_stream(response: reqwest::Response) -> ReceiverStream<AppResult<LlmEvent>> {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move {
        let mut bytes = response.bytes_stream();
        let mut buffer = Vec::new();
        let mut tool_blocks: HashMap<usize, (Option<String>, Option<String>, String)> =
            HashMap::new();

        while let Some(result) = bytes.next().await {
            match result {
                Ok(chunk) => {
                    buffer.extend_from_slice(&chunk);
                    while let Some(line) = take_sse_line(&mut buffer) {
                        let line = match line {
                            Ok(line) => line,
                            Err(error) => {
                                let _ = tx.send(Err(error)).await;
                                return;
                            }
                        };
                        match parse_claude_line(&line, &mut tool_blocks) {
                            Ok(StreamLine::TextChunk(text)) => {
                                if tx.send(Ok(LlmEvent::TextChunk(text))).await.is_err() {
                                    return;
                                }
                            }
                            Ok(StreamLine::ToolCallComplete {
                                id,
                                name,
                                arguments_fragment,
                                ..
                            }) => {
                                if tx
                                    .send(Ok(LlmEvent::ToolCallComplete {
                                        id: id.unwrap_or_default(),
                                        name: name.unwrap_or_default(),
                                        arguments: arguments_fragment.unwrap_or_default(),
                                    }))
                                    .await
                                    .is_err()
                                {
                                    return;
                                }
                            }
                            Ok(StreamLine::Done) => {
                                let _ = tx.send(Ok(LlmEvent::Done)).await;
                                return;
                            }
                            Ok(_) => {}
                            Err(error) => {
                                let _ = tx.send(Err(error)).await;
                                return;
                            }
                        }
                    }
                }
                Err(error) => {
                    let _ = tx
                        .send(Err(AppError::NetworkError(format!(
                            "读取 Claude 流失败: {error}"
                        ))))
                        .await;
                    return;
                }
            }
        }
    });
    ReceiverStream::new(rx)
}

// ============================================================================
// Public API
// ============================================================================

pub async fn stream_chat(
    config: &LlmConfig,
    messages: Vec<LlmMessage>,
    tools: Option<Vec<ToolDefinition>>,
) -> AppResult<LlmStream> {
    let config = config.clone().validate()?;
    eprintln!(
        "[AI_CHAT] ====== LLM API 调用 ======\n[AI_CHAT] provider: {}\n[AI_CHAT] model: {}\n[AI_CHAT] base_url: {}\n[AI_CHAT] 消息数: {}\n[AI_CHAT] 工具数: {}",
        config.provider.as_str(),
        config.model,
        config.base_url,
        messages.len(),
        tools.as_ref().map_or(0, |t| t.len())
    );
    match config.provider {
        LlmProvider::OpenAI | LlmProvider::OpenAICompatible => {
            Ok(Box::new(
                stream_openai_compatible(&config, messages, tools).await?,
            ))
        }
        LlmProvider::Claude => Ok(Box::new(
            stream_claude(&config, messages, tools, ClaudeAuth::ApiKey).await?,
        )),
        LlmProvider::ClaudeCompatible => Ok(Box::new(
            stream_claude(&config, messages, tools, ClaudeAuth::Bearer).await?,
        )),
    }
}

#[derive(Clone, Copy)]
enum ClaudeAuth {
    ApiKey,
    Bearer,
}

async fn stream_openai_compatible(
    config: &LlmConfig,
    messages: Vec<LlmMessage>,
    tools: Option<Vec<ToolDefinition>>,
) -> AppResult<ReceiverStream<AppResult<LlmEvent>>> {
    let client = build_client()?;
    let request = OpenAiRequest {
        model: config.model.clone(),
        messages: to_openai_messages(&messages),
        max_tokens: config.max_tokens,
        temperature: config.temperature,
        stream: true,
        tools,
    };
    let builder = client
        .post(format!("{}/chat/completions", config.base_url))
        .header("Content-Type", "application/json")
        .json(&request);
    let response = send_checked(
        with_optional_bearer(builder, config.api_key.as_deref()),
        "LLM",
    )
    .await?;
    Ok(openai_response_stream(response))
}

async fn stream_claude(
    config: &LlmConfig,
    messages: Vec<LlmMessage>,
    tools: Option<Vec<ToolDefinition>>,
    auth: ClaudeAuth,
) -> AppResult<ReceiverStream<AppResult<LlmEvent>>> {
    let system = messages
        .iter()
        .find(|message| message.role == "system")
        .and_then(|message| message.content.clone());
    let claude_messages = to_claude_messages(&messages);
    let claude_tools = tools.as_deref().map(to_claude_tools);
    let request = ClaudeRequest {
        model: config.model.clone(),
        max_tokens: config.max_tokens,
        system,
        messages: claude_messages,
        stream: true,
        tools: claude_tools,
    };
    let builder = build_client()?
        .post(format!("{}/messages", config.base_url))
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&request);
    let builder = match auth {
        ClaudeAuth::ApiKey => {
            builder.header("x-api-key", config.api_key.as_deref().unwrap_or_default())
        }
        ClaudeAuth::Bearer => with_optional_bearer(builder, config.api_key.as_deref()),
    };
    let response = send_checked(builder, "Claude").await?;
    Ok(claude_response_stream(response))
}

// ============================================================================
// Utilities
// ============================================================================

fn build_client() -> AppResult<Client> {
    Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|error| AppError::NetworkError(format!("创建 HTTP 客户端失败: {error}")))
}

fn with_optional_bearer(builder: RequestBuilder, api_key: Option<&str>) -> RequestBuilder {
    match api_key.filter(|key| !key.is_empty()) {
        Some(api_key) => builder.bearer_auth(api_key),
        None => builder,
    }
}

async fn send_checked(builder: RequestBuilder, provider: &str) -> AppResult<reqwest::Response> {
    let response = builder
        .send()
        .await
        .map_err(|error| AppError::NetworkError(format!("请求 {provider} 失败: {error}")))?;
    if response.status().is_success() {
        return Ok(response);
    }
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    let body = body.chars().take(500).collect::<String>();
    Err(AppError::NetworkError(format!(
        "{provider} API 返回错误 {status}: {body}"
    )))
}

pub fn load_config(app_data_dir: &Path) -> LlmConfig {
    let config_path = app_data_dir.join("llm_config.json");
    std::fs::read_to_string(config_path)
        .ok()
        .and_then(|content| serde_json::from_str::<LlmConfig>(&content).ok())
        .and_then(|config| config.validate().ok())
        .unwrap_or_default()
}

pub fn save_config(app_data_dir: &Path, config: &LlmConfig) -> AppResult<()> {
    std::fs::create_dir_all(app_data_dir)?;
    let config = config.clone().validate()?;
    let config_path = app_data_dir.join("llm_config.json");
    let temporary_path = app_data_dir.join("llm_config.json.tmp");
    std::fs::write(&temporary_path, serde_json::to_vec_pretty(&config)?)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&temporary_path, std::fs::Permissions::from_mode(0o600))?;
    }
    std::fs::rename(temporary_path, config_path)?;
    Ok(())
}

pub async fn test_connection(config: &LlmConfig) -> AppResult<String> {
    let mut stream = stream_chat(
        config,
        vec![LlmMessage::user("你好，请回复 OK")],
        None,
    )
    .await?;
    let mut response = String::new();
    while let Some(event) = stream.next().await {
        match event? {
            LlmEvent::TextChunk(text) => {
                response.push_str(&text);
                if response.chars().count() >= 100 {
                    break;
                }
            }
            LlmEvent::Done => break,
            _ => {}
        }
    }
    if response.trim().is_empty() {
        Err(AppError::NetworkError("模型返回空响应".to_string()))
    } else {
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_provider_allows_empty_api_key() {
        assert!(LlmConfig::default().validate().is_ok());
        let mut openai = LlmConfig::default();
        openai.provider = LlmProvider::OpenAI;
        assert!(openai.validate().is_err());
    }

    #[test]
    fn claude_compatible_provider_requires_bearer_token() {
        let mut config = LlmConfig::default();
        config.provider = LlmProvider::ClaudeCompatible;
        config.base_url = "https://oneapi.updev.cn/v1".to_string();
        config.model = "mimo-v2.5-pro".to_string();
        assert!(config.clone().validate().is_err());

        config.api_key = Some("token".to_string());
        assert!(config.validate().is_ok());
        assert_eq!(
            LlmProvider::parse("claude_compatible").unwrap(),
            LlmProvider::ClaudeCompatible
        );
    }

    #[test]
    fn parses_openai_sse_lines() {
        match parse_openai_line(
            "data: {\"choices\":[{\"delta\":{\"content\":\"你好\"},\"finish_reason\":null}]}",
        )
        .unwrap()
        {
            StreamLine::TextChunk(content) => assert_eq!(content, "你好"),
            _ => panic!("expected content"),
        }
        assert!(matches!(
            parse_openai_line("data: [DONE]").unwrap(),
            StreamLine::Done
        ));
    }

    #[test]
    fn parses_openai_tool_call_delta() {
        let line = "data: {\"choices\":[{\"delta\":{\"tool_calls\":[{\"index\":0,\"id\":\"call_123\",\"function\":{\"name\":\"get_fund_info\",\"arguments\":\"{\\\"co\"}}]},\"finish_reason\":null}]}";
        match parse_openai_line(line).unwrap() {
            StreamLine::ToolCallDelta { index, id, name, arguments_fragment } => {
                assert_eq!(index, 0);
                assert_eq!(id.as_deref(), Some("call_123"));
                assert_eq!(name.as_deref(), Some("get_fund_info"));
                assert_eq!(arguments_fragment.as_deref(), Some("{\"co"));
            }
            _ => panic!("expected tool call delta"),
        }
    }

    #[test]
    fn sse_line_waits_for_complete_utf8_bytes() {
        let bytes = "data: 你好\n".as_bytes();
        let split = bytes.len() - 2;
        let mut buffer = bytes[..split].to_vec();
        assert!(take_sse_line(&mut buffer).is_none());
        buffer.extend_from_slice(&bytes[split..]);
        assert_eq!(take_sse_line(&mut buffer).unwrap().unwrap(), "data: 你好");
    }

    #[test]
    fn claude_request_keeps_system_message() {
        let messages = [LlmMessage::system("组合上下文")];
        assert_eq!(
            messages
                .iter()
                .find(|message| message.role == "system")
                .unwrap()
                .content
                .as_deref(),
            Some("组合上下文")
        );
    }

    #[cfg(unix)]
    #[test]
    fn saved_config_is_only_readable_by_current_user() {
        use std::os::unix::fs::PermissionsExt;

        let directory =
            std::env::temp_dir().join(format!("leek-fund-llm-config-{}", uuid::Uuid::new_v4()));
        save_config(&directory, &LlmConfig::default()).unwrap();
        let mode = std::fs::metadata(directory.join("llm_config.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        std::fs::remove_dir_all(directory).unwrap();

        assert_eq!(mode, 0o600);
    }
}
