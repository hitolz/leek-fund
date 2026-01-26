import React from "react";
import { ChatMessage } from "../../types/ai-chat";
import { MessageItem } from "./MessageItem";

interface MessageListProps {
  messages: ChatMessage[];
}

export const MessageList: React.FC<MessageListProps> = ({ messages }) => {
  return (
    <div className="message-list">
      {messages.map((message) => (
        <MessageItem key={`${message.id}-${message.role}`} message={message} />
      ))}
    </div>
  );
};
