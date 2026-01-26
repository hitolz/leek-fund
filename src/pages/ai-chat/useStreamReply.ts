import { useCallback, useState } from "react";
import { streamAssistantReply } from "../../services/ai-chat/streamApi";
import { SavedState } from "../../types/ai-chat";

interface StreamHandlers {
  onChunk: (chunk: string) => void;
  onSavedState: (state: SavedState) => void;
  onDone: () => void;
}

export function useStreamReply() {
  const [streaming, setStreaming] = useState(false);

  const streamReply = useCallback(
    async (sessionId: string, content: string, handlers: StreamHandlers) => {
      setStreaming(true);
      try {
        await streamAssistantReply(sessionId, content, (event) => {
          if (event.event === "chunk") {
            handlers.onChunk(event.data);
          } else if (event.event === "saved_state") {
            handlers.onSavedState(event.data as SavedState);
          } else if (event.event === "done") {
            handlers.onDone();
          }
        });
      } finally {
        setStreaming(false);
      }
    },
    []
  );

  return { streaming, streamReply };
}
