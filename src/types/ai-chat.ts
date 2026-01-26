export type ChatRole = "user" | "assistant";

export type SavedState = "saved" | "unsaved";

export interface ChatSession {
  id: number;
  session_id: string;
  title?: string | null;
  created_at: number;
  updated_at: number;
}

export interface ChatMessage {
  id: number;
  session_id: string;
  role: ChatRole;
  content: string;
  saved_state?: SavedState;
  created_at: number;
  updated_at: number;
}

export interface StreamChunk {
  event: string;
  data: string;
}
