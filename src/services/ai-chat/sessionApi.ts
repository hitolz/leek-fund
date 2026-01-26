import { ChatSession } from "../../types/ai-chat";
import { AI_CHAT_BASE_URL } from "./config";

export async function getRecentSession(): Promise<ChatSession | null> {
  const response = await fetch(`${AI_CHAT_BASE_URL}/api/sessions/recent`);
  if (response.status === 404) {
    return null;
  }
  if (!response.ok) {
    throw new Error(await response.text());
  }
  return response.json();
}

export async function createSession(): Promise<ChatSession> {
  const response = await fetch(`${AI_CHAT_BASE_URL}/api/sessions`, {
    method: "POST",
  });
  if (!response.ok) {
    throw new Error(await response.text());
  }
  return response.json();
}

export async function getOrCreateRecentSession(): Promise<ChatSession> {
  const recent = await getRecentSession();
  if (recent) {
    return recent;
  }
  return createSession();
}
