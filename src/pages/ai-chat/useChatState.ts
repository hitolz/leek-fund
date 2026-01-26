import { useCallback, useState } from "react";
import { ChatMessage, ChatSession, SavedState } from "../../types/ai-chat";
import { getOrCreateRecentSession } from "../../services/ai-chat/sessionApi";
import { listMessages } from "../../services/ai-chat/messageApi";

export function useChatState() {
  const [session, setSession] = useState<ChatSession | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const initialize = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const recent = await getOrCreateRecentSession();
      setSession(recent);
      const history = await listMessages(recent.session_id);
      setMessages(history);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const appendMessage = useCallback((msg: ChatMessage) => {
    setMessages((prev) => [...prev, msg]);
  }, []);

  const updateMessageContent = useCallback((id: number, content: string) => {
    setMessages((prev) =>
      prev.map((msg) => (msg.id === id ? { ...msg, content } : msg))
    );
  }, []);

  const updateMessageSavedState = useCallback(
    (id: number, savedState: SavedState) => {
      setMessages((prev) =>
        prev.map((msg) =>
          msg.id === id ? { ...msg, saved_state: savedState } : msg
        )
      );
    },
    []
  );

  return {
    session,
    messages,
    loading,
    error,
    initialize,
    setError,
    appendMessage,
    updateMessageContent,
    updateMessageSavedState,
  };
}
