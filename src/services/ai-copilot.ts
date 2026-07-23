import {
  ChatMessage,
  ChatSession,
  LlmActionResult,
  LlmConfig,
  StreamEvent,
} from "../types/ai-copilot";

const AI_CHAT_API = "http://127.0.0.1:18188";

async function parseResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const detail = await response.text();
    throw new Error(detail || `请求失败 (${response.status})`);
  }
  return response.json() as Promise<T>;
}

export async function getOrCreateSession(): Promise<ChatSession> {
  const recent = await fetch(`${AI_CHAT_API}/api/sessions/recent`);
  if (recent.ok) return recent.json() as Promise<ChatSession>;

  return parseResponse<ChatSession>(
    await fetch(`${AI_CHAT_API}/api/sessions`, { method: "POST" })
  );
}

export async function createSession(): Promise<ChatSession> {
  return parseResponse<ChatSession>(
    await fetch(`${AI_CHAT_API}/api/sessions`, { method: "POST" })
  );
}

export async function listSessions(limit = 50): Promise<ChatSession[]> {
  return parseResponse<ChatSession[]>(
    await fetch(`${AI_CHAT_API}/api/sessions/list?limit=${limit}`)
  );
}

export async function listSessionMessages(
  sessionId: string
): Promise<ChatMessage[]> {
  return parseResponse<ChatMessage[]>(
    await fetch(`${AI_CHAT_API}/api/sessions/${sessionId}/messages`)
  );
}

export async function streamPortfolioReply(
  sessionId: string,
  content: string,
  snapshotId: string | undefined,
  signal: AbortSignal,
  onEvent: (event: StreamEvent) => void
): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    xhr.open(
      "POST",
      `${AI_CHAT_API}/api/sessions/${sessionId}/messages/stream`
    );
    xhr.setRequestHeader("Content-Type", "application/json");

    let lastProcessed = 0;

    xhr.onreadystatechange = () => {
      if (xhr.readyState >= 3 && xhr.status === 200) {
        const text = xhr.responseText;
        const newPart = text.slice(lastProcessed);
        lastProcessed = text.length;

        const blocks = newPart.split("\n\n");
        // 最后一块可能不完整，保留在 buffer 中由下一次处理
        const completeBlocks = blocks.slice(0, -1);

        for (const block of completeBlocks) {
          if (!block.trim()) continue;
          let event = "message";
          const data: string[] = [];
          for (const line of block.split("\n")) {
            if (line.startsWith("event:")) event = line.slice(6).trim();
            if (line.startsWith("data:"))
              data.push(line.slice(5).trimStart());
          }
          if (data.length > 0) onEvent({ event, data: data.join("\n") });
        }
      }
    };

    xhr.onload = () => {
      // 处理最后剩余的数据
      const text = xhr.responseText.slice(lastProcessed);
      if (text.trim()) {
        const blocks = text.split("\n\n");
        for (const block of blocks) {
          if (!block.trim()) continue;
          let event = "message";
          const data: string[] = [];
          for (const line of block.split("\n")) {
            if (line.startsWith("event:")) event = line.slice(6).trim();
            if (line.startsWith("data:"))
              data.push(line.slice(5).trimStart());
          }
          if (data.length > 0) onEvent({ event, data: data.join("\n") });
        }
      }

      if (xhr.status >= 200 && xhr.status < 300) {
        resolve();
      } else {
        reject(new Error(xhr.responseText || `请求失败 (${xhr.status})`));
      }
    };

    xhr.onerror = () => {
      reject(new Error(`网络错误: ${xhr.status}`));
    };

    xhr.onabort = () => {
      resolve();
    };

    signal.addEventListener("abort", () => xhr.abort());

    xhr.send(
      JSON.stringify({
        content,
        context: {
          context_type: "portfolio",
          snapshot_id: snapshotId,
        },
      })
    );
  });
}

export async function getLlmConfig(): Promise<LlmConfig> {
  return parseResponse<LlmConfig>(await fetch(`${AI_CHAT_API}/api/llm/config`));
}

export async function saveLlmConfig(
  config: LlmConfig
): Promise<LlmActionResult> {
  return parseResponse<LlmActionResult>(
    await fetch(`${AI_CHAT_API}/api/llm/config`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(config),
    })
  );
}

export async function testLlmConfig(
  config: LlmConfig
): Promise<LlmActionResult> {
  return parseResponse<LlmActionResult>(
    await fetch(`${AI_CHAT_API}/api/llm/test`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(config),
    })
  );
}
