import React, { useState, useEffect, useRef, useCallback } from "react";

interface AiChatPanelProps {
  showToast: (message: string, type?: "success" | "error") => void;
}

interface ChatMessage {
  id: number;
  role: "user" | "assistant";
  content: string;
  timestamp: number;
}

const AI_CHAT_API = "http://127.0.0.1:18188";

export const AiChatPanel: React.FC<AiChatPanelProps> = ({ showToast }) => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState("");
  const [loading] = useState(false);
  const [streaming, setStreaming] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // 滚动到底部
  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, []);

  // 初始化会话
  useEffect(() => {
    const initSession = async () => {
      try {
        const res = await fetch(`${AI_CHAT_API}/api/sessions/recent`);
        if (res.ok) {
          const session = await res.json();
          setSessionId(session.session_id);

          // 加载历史消息
          const msgRes = await fetch(
            `${AI_CHAT_API}/api/sessions/${session.session_id}/messages`
          );
          if (msgRes.ok) {
            const history = await msgRes.json();
            setMessages(
              history.map((msg: any) => ({
                id: msg.id,
                role: msg.role,
                content: msg.content,
                timestamp: msg.created_at,
              }))
            );
          }
        } else {
          // 创建新会话
          const createRes = await fetch(`${AI_CHAT_API}/api/sessions`, {
            method: "POST",
          });
          if (createRes.ok) {
            const session = await createRes.json();
            setSessionId(session.session_id);
          }
        }
      } catch (error) {
        console.error("Failed to init AI chat session:", error);
        showToast("AI 服务连接失败，请检查服务是否启动", "error");
      }
    };
    initSession();
  }, [showToast]);

  // 滚动到底部
  useEffect(() => {
    scrollToBottom();
  }, [messages, scrollToBottom]);

  // 发送消息
  const handleSend = useCallback(async () => {
    const content = inputValue.trim();
    if (!content || !sessionId || streaming) return;

    const now = Date.now();
    const userMessage: ChatMessage = {
      id: now,
      role: "user",
      content,
      timestamp: now,
    };

    const assistantMessage: ChatMessage = {
      id: -now,
      role: "assistant",
      content: "",
      timestamp: now,
    };

    setMessages((prev) => [...prev, userMessage, assistantMessage]);
    setInputValue("");
    setStreaming(true);

    let buffer = "";
    try {
      const response = await fetch(
        `${AI_CHAT_API}/api/sessions/${sessionId}/messages/stream`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ content }),
        }
      );

      if (!response.ok) {
        throw new Error(await response.text());
      }

      const reader = response.body?.getReader();
      if (!reader) throw new Error("Stream unavailable");

      const decoder = new TextDecoder("utf-8");
      let streamBuffer = "";

      while (true) {
        const { value, done } = await reader.read();
        if (done) break;

        streamBuffer += decoder.decode(value, { stream: true });
        const parts = streamBuffer.split("\n\n");
        streamBuffer = parts.pop() || "";

        for (const part of parts) {
          const lines = part.split("\n");
          let event = "message";
          let data = "";

          for (const line of lines) {
            if (line.startsWith("event:")) {
              event = line.slice(6).trim();
            } else if (line.startsWith("data:")) {
              data += line.slice(5).trim();
            }
          }

          if (event === "chunk" && data) {
            buffer += data;
            setMessages((prev) =>
              prev.map((msg) =>
                msg.id === -now ? { ...msg, content: buffer } : msg
              )
            );
          }
        }
      }
    } catch (error) {
      console.error("Stream error:", error);
      showToast("AI 回复失败", "error");
    } finally {
      setStreaming(false);
    }
  }, [inputValue, sessionId, streaming, showToast]);

  // 清空聊天
  const handleClear = useCallback(() => {
    setMessages([]);
    showToast("聊天记录已清空");
  }, [showToast]);

  return (
    <div className="ai-chat-panel">
      {/* 头部 */}
      <div className="ai-chat-header">
        <h3>AI 对话</h3>
        <div className="ai-chat-actions">
          {streaming && <span className="ai-status">AI 思考中...</span>}
          <button
            type="button"
            className="button ghost small"
            onClick={handleClear}
            disabled={messages.length === 0}
          >
            清空
          </button>
        </div>
      </div>

      {/* 消息列表 */}
      <div className="ai-messages">
        {messages.length === 0 ? (
          <div className="ai-empty-state">
            <div className="ai-icon">🤖</div>
            <p>有什么可以帮你的？</p>
            <p className="hint">输入消息开始对话</p>
          </div>
        ) : (
          messages.map((msg) => (
            <div
              key={`${msg.id}-${msg.role}`}
              className={`ai-message ${msg.role}`}
            >
              <div className="ai-message-role">
                {msg.role === "user" ? "你" : "AI"}
              </div>
              <div className="ai-message-content">
                {msg.content || (streaming && msg.role === "assistant" ? "..." : "")}
              </div>
            </div>
          ))
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* 输入区域 */}
      <div className="ai-input-area">
        <input
          type="text"
          className="ai-input"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && !e.shiftKey && handleSend()}
          placeholder="输入消息..."
          disabled={loading || streaming || !sessionId}
        />
        <button
          type="button"
          className="button primary"
          onClick={handleSend}
          disabled={loading || streaming || !inputValue.trim() || !sessionId}
        >
          发送
        </button>
      </div>
    </div>
  );
};
