import { ChatMessage } from "../../types/ai-chat";
import { AI_CHAT_BASE_URL } from "./config";

export async function listMessages(
  sessionId: string,
  limit?: number
): Promise<ChatMessage[]> {
  const url = new URL(`${AI_CHAT_BASE_URL}/api/sessions/${sessionId}/messages`);
  if (limit) {
    url.searchParams.set("limit", String(limit));
  }
  const response = await fetch(url.toString());
  if (!response.ok) {
    throw new Error(await response.text());
  }
  return response.json();
}
