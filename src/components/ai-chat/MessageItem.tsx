import React from "react";
import { ChatMessage } from "../../types/ai-chat";

interface MessageItemProps {
  message: ChatMessage;
}

export const MessageItem: React.FC<MessageItemProps> = ({ message }) => {
  const roleLabel = message.role === "user" ? "你" : "AI";
  const savedBadge =
    message.saved_state === "unsaved" ? (
      <span className="message-unsaved">未保存</span>
    ) : null;

  return (
    <div className={`message-item ${message.role}`}>
      <div className="message-meta">
        <span className="message-role">{roleLabel}</span>
        {savedBadge}
      </div>
      <div className="message-content">{message.content}</div>
    </div>
  );
};
