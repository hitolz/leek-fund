import React, { useCallback, useEffect, useRef, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import {
  ChartNoAxesCombined,
  Copy,
  Database,
  History,
  MessageSquareText,
  RefreshCw,
  Send,
  Square,
} from "lucide-react";
import {
  getOrCreateSession,
  createSession,
  listSessionMessages,
  streamPortfolioReply,
} from "../../services/ai-copilot";
import { ChatMessage, PortfolioSnapshot } from "../../types/ai-copilot";
import { ContextBar } from "./ContextBar";
import { formatSnapshotTime } from "./formatters";
import { SessionDrawer } from "./SessionDrawer";

export interface ChatWorkspaceQuestion {
  text: string;
  nonce: number;
}

interface ChatWorkspaceProps {
  snapshot: PortfolioSnapshot | null;
  initialQuestion: ChatWorkspaceQuestion | null;
  showToast: (message: string, type?: "success" | "error") => void;
}

const suggestions = [
  "今天我的组合主要受什么影响？",
  "哪些持仓的数据不完整？",
  "从集中度角度检查我的组合",
];

export const ChatWorkspace: React.FC<ChatWorkspaceProps> = ({
  snapshot,
  initialQuestion,
  showToast,
}) => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState("");
  const [streaming, setStreaming] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [sessionLoading, setSessionLoading] = useState(true);
  const [sessionError, setSessionError] = useState<string | null>(null);
  const [sessionAttempt, setSessionAttempt] = useState(0);
  const [copiedId, setCopiedId] = useState<number | null>(null);
  const [drawerOpen, setDrawerOpen] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const abortControllerRef = useRef<AbortController | null>(null);
  const showToastRef = useRef(showToast);

  useEffect(() => {
    showToastRef.current = showToast;
  }, [showToast]);

  useEffect(() => {
    let active = true;
    setSessionLoading(true);
    setSessionError(null);

    getOrCreateSession()
      .then(async (session) => {
        const history = await listSessionMessages(session.session_id);
        if (!active) return;
        setSessionId(session.session_id);
        setMessages(history);
      })
      .catch((error) => {
        if (!active) return;
        const detail = String(error);
        setSessionError(detail);
        showToastRef.current(`AI 会话初始化失败：${detail}`, "error");
      })
      .finally(() => {
        if (active) setSessionLoading(false);
      });

    return () => {
      active = false;
      abortControllerRef.current?.abort();
    };
  }, [sessionAttempt]);

  useEffect(() => {
    if (!initialQuestion) return;
    setInputValue(initialQuestion.text);
    requestAnimationFrame(() => inputRef.current?.focus());
  }, [initialQuestion]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleNewSession = useCallback(async () => {
    if (streaming) return;
    try {
      const session = await createSession();
      setSessionId(session.session_id);
      setMessages([]);
      setInputValue("");
      showToastRef.current("已开启新对话", "success");
    } catch (error) {
      showToastRef.current(`创建新对话失败：${String(error)}`, "error");
    }
  }, [streaming]);

  const handleSelectSession = useCallback(
    async (id: string) => {
      if (streaming || id === sessionId) return;
      try {
        const history = await listSessionMessages(id);
        setSessionId(id);
        setMessages(history);
        setInputValue("");
      } catch (error) {
        showToastRef.current(`加载会话失败：${String(error)}`, "error");
      }
    },
    [streaming, sessionId]
  );

  const handleSend = useCallback(async () => {
    const content = inputValue.trim();
    if (!content || !sessionId || streaming) return;

    if (content === "/new") {
      await handleNewSession();
      return;
    }

    const now = Date.now();
    const assistantId = -now;
    setMessages((current) => [
      ...current,
      {
        id: now,
        role: "user",
        content,
        created_at: Math.floor(now / 1000),
        saved_state: "saved",
      },
      {
        id: assistantId,
        role: "assistant",
        content: "",
        created_at: Math.floor(now / 1000),
        saved_state: "saved",
      },
    ]);
    setInputValue("");
    setStreaming(true);
    const controller = new AbortController();
    abortControllerRef.current = controller;
    let fullContent = "";

    try {
      await streamPortfolioReply(
        sessionId,
        content,
        snapshot?.id,
        controller.signal,
        (event) => {
          if (event.event === "chunk") {
            fullContent += event.data;
            setMessages((current) =>
              current.map((message) =>
                message.id === assistantId
                  ? { ...message, content: fullContent }
                  : message
              )
            );
          }
          if (event.event === "saved_state") {
            setMessages((current) =>
              current.map((message) =>
                message.id === assistantId
                  ? {
                      ...message,
                      saved_state:
                        event.data === "unsaved" ? "unsaved" : "saved",
                    }
                  : message
              )
            );
          }
          if (event.event === "error") {
            let detail = "模型不可用，当前显示的是本地快照摘要";
            try {
              const payload = JSON.parse(event.data) as { message?: string };
              if (payload.message) detail = payload.message;
            } catch {
              if (event.data.trim()) detail = event.data;
            }
            showToastRef.current(`AI 模型调用失败：${detail}`, "error");
          }
        }
      );
    } catch (error) {
      if (!controller.signal.aborted) {
        showToastRef.current(`AI 回复失败：${String(error)}`, "error");
      }
    } finally {
      setStreaming(false);
      abortControllerRef.current = null;
    }
  }, [inputValue, sessionId, snapshot?.id, streaming, handleNewSession]);

  const handleCopy = useCallback(async (id: number, content: string) => {
    try {
      await navigator.clipboard.writeText(content);
      setCopiedId(id);
      setTimeout(() => setCopiedId(null), 1500);
    } catch {
      showToastRef.current("复制失败", "error");
    }
  }, []);

  const handleSuggestion = useCallback((question: string) => {
    setInputValue(question);
    requestAnimationFrame(() => inputRef.current?.focus());
  }, []);

  return (
    <section className="chat-workspace" aria-label="问 AI">
      <header className="chat-workspace-header">
        <div className="chat-workspace-title">
          <button
            type="button"
            className="session-history-button"
            onClick={() => setDrawerOpen(true)}
            title="历史会话"
          >
            <History size={15} />
          </button>
          <span className="chat-workspace-icon"><MessageSquareText size={17} /></span>
          <div>
            <span className="copilot-eyebrow">PORTFOLIO CHAT</span>
            <h3>问 AI</h3>
          </div>
        </div>
        <div className="chat-workspace-summary">
          {snapshot ? (
            <>
              <span><Database size={12} /> 快照 {snapshot.id.slice(0, 8)}</span>
              <span>{snapshot.assets.length} 项资产</span>
              <span>{formatSnapshotTime(snapshot.snapshot_at)}</span>
            </>
          ) : (
            <span>组合快照未就绪</span>
          )}
        </div>
        {streaming && <span className="chat-streaming"><i /> 正在分析</span>}
      </header>

      <div className="chat-workspace-main">
        {sessionLoading ? (
          <div className="copilot-state">
            <RefreshCw className="spin" size={20} />
            <strong>正在连接 AI 会话</strong>
          </div>
        ) : sessionError ? (
          <div className="copilot-state error">
            <MessageSquareText size={21} />
            <strong>AI 会话连接失败</strong>
            <span>{sessionError}</span>
            <button type="button" onClick={() => setSessionAttempt((current) => current + 1)}>
              重试
            </button>
          </div>
        ) : (
          <div className="copilot-messages">
            {messages.length === 0 ? (
              <div className="copilot-chat-empty">
                <ChartNoAxesCombined size={21} />
                <strong>从当前组合开始</strong>
                <div className="copilot-suggestions">
                  {suggestions.map((question) => (
                    <button type="button" key={question} onClick={() => handleSuggestion(question)}>
                      {question}
                    </button>
                  ))}
                </div>
              </div>
            ) : (
              messages.map((message) => (
                <article className={`copilot-message ${message.role}`} key={`${message.id}-${message.role}`}>
                  <span className="copilot-message-role">{message.role === "user" ? "你" : "LEEK AI"}</span>
                  {message.role === "assistant" ? (
                    <div className="copilot-message-content">
                      {message.content ? (
                        <ReactMarkdown remarkPlugins={[remarkGfm]}>{message.content}</ReactMarkdown>
                      ) : (
                        streaming ? "正在组织分析..." : "未生成内容"
                      )}
                    </div>
                  ) : (
                    <div>{message.content}</div>
                  )}
                  {message.content && (
                    <button
                      type="button"
                      className="copilot-copy-button"
                      onClick={() => handleCopy(message.id, message.content)}
                      title="复制"
                    >
                      <Copy size={11} />
                      {copiedId === message.id ? "已复制" : "复制"}
                    </button>
                  )}
                  {message.saved_state === "unsaved" && <small>本条回复未保存</small>}
                </article>
              ))
            )}
            <div ref={messagesEndRef} />
          </div>
        )}
      </div>

      <footer className="copilot-footer">
        <ContextBar snapshot={snapshot} />
        <div className="copilot-composer">
          <textarea
            ref={inputRef}
            className="copilot-input"
            value={inputValue}
            onChange={(event) => setInputValue(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter" && !event.shiftKey) {
                event.preventDefault();
                void handleSend();
              }
            }}
            placeholder={sessionId ? "问我的组合..." : "AI 会话连接中..."}
            disabled={!sessionId}
            rows={1}
          />
          {streaming ? (
            <button type="button" className="copilot-stop-button" onClick={() => abortControllerRef.current?.abort()} title="停止生成">
              <Square size={15} />
            </button>
          ) : (
            <button type="button" className="copilot-send-button" onClick={() => void handleSend()} disabled={!sessionId || !inputValue.trim()} title="发送">
              <Send size={16} />
            </button>
          )}
        </div>
      </footer>

      <SessionDrawer
        open={drawerOpen}
        currentSessionId={sessionId}
        onClose={() => setDrawerOpen(false)}
        onSelect={handleSelectSession}
        onNew={handleNewSession}
      />
    </section>
  );
};
