import React, { useEffect } from "react";
import { MessageInput } from "../../components/ai-chat/MessageInput";
import { MessageList } from "../../components/ai-chat/MessageList";
import { useToast } from "../../components/ToastContext";
import { ChatMessage } from "../../types/ai-chat";
import { useChatState } from "./useChatState";
import { useStreamReply } from "./useStreamReply";
import "./ai-chat.css";

export const AiChatPage: React.FC = () => {
  const showToast = useToast();
  const {
    session,
    messages,
    loading,
    error,
    initialize,
    setError,
    appendMessage,
    updateMessageContent,
    updateMessageSavedState,
  } = useChatState();
  const { streaming, streamReply } = useStreamReply();

  useEffect(() => {
    initialize();
  }, [initialize]);

  useEffect(() => {
    if (error) {
      showToast(error, "error");
      setError(null);
    }
  }, [error, setError, showToast]);

  const handleSend = async (content: string) => {
    if (!session) {
      showToast("会话未就绪，请稍后重试", "error");
      return;
    }

    const now = Date.now();
    const userMessage: ChatMessage = {
      id: now,
      session_id: session.session_id,
      role: "user",
      content,
      created_at: now,
      updated_at: now,
      saved_state: "saved",
    };

    const assistantMessageId = -now;
    const assistantMessage: ChatMessage = {
      id: assistantMessageId,
      session_id: session.session_id,
      role: "assistant",
      content: "",
      created_at: now,
      updated_at: now,
      saved_state: "saved",
    };

    appendMessage(userMessage);
    appendMessage(assistantMessage);

    let buffer = "";
    try {
      await streamReply(session.session_id, content, {
        onChunk: (chunk) => {
          buffer += chunk;
          updateMessageContent(assistantMessageId, buffer);
        },
        onSavedState: (state) => {
          updateMessageSavedState(assistantMessageId, state);
        },
        onDone: () => {
          // no-op
        },
      });
    } catch (err) {
      showToast("回复失败，可重试", "error");
    }
  };

  return (
    <div className="ai-chat-page">
      <div className="ai-chat-header">
        <h2>AI 对话</h2>
        {loading && <span className="ai-chat-status">加载中...</span>}
        {streaming && <span className="ai-chat-status">AI 正在思考...</span>}
      </div>
      <div className="ai-chat-body">
        <MessageList messages={messages} />
      </div>
      <div className="ai-chat-footer">
        <MessageInput onSend={handleSend} disabled={loading || streaming} />
      </div>
    </div>
  );
};
