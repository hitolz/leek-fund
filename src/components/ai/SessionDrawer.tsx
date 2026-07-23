import React, { useCallback, useEffect, useState } from "react";
import { MessageSquareText, RefreshCw, X } from "lucide-react";
import { listSessions } from "../../services/ai-copilot";
import { ChatSession } from "../../types/ai-copilot";

interface SessionDrawerProps {
  open: boolean;
  currentSessionId: string | null;
  onClose: () => void;
  onSelect: (sessionId: string) => void;
  onNew: () => void;
}

function formatSessionTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const isToday =
    date.getFullYear() === now.getFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getDate() === now.getDate();

  if (isToday) {
    return `今天 ${date.toLocaleTimeString("zh-CN", {
      hour: "2-digit",
      minute: "2-digit",
    })}`;
  }

  const yesterday = new Date(now);
  yesterday.setDate(yesterday.getDate() - 1);
  const isYesterday =
    date.getFullYear() === yesterday.getFullYear() &&
    date.getMonth() === yesterday.getMonth() &&
    date.getDate() === yesterday.getDate();

  if (isYesterday) {
    return `昨天 ${date.toLocaleTimeString("zh-CN", {
      hour: "2-digit",
      minute: "2-digit",
    })}`;
  }

  const isThisYear = date.getFullYear() === now.getFullYear();
  if (isThisYear) {
    return date.toLocaleDateString("zh-CN", {
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  return date.toLocaleDateString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function sessionTitle(session: ChatSession): string {
  if (session.title) return session.title;
  const id = session.session_id;
  return `会话 ${id.slice(0, 8)}`;
}

export const SessionDrawer: React.FC<SessionDrawerProps> = ({
  open,
  currentSessionId,
  onClose,
  onSelect,
  onNew,
}) => {
  const [sessions, setSessions] = useState<ChatSession[]>([]);
  const [loading, setLoading] = useState(false);

  const loadSessions = useCallback(async () => {
    setLoading(true);
    try {
      const list = await listSessions(100);
      setSessions(list);
    } catch {
      // silent
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (open) loadSessions();
  }, [open, loadSessions]);

  if (!open) return null;

  return (
    <div className="session-drawer-overlay" onClick={onClose}>
      <div
        className="session-drawer"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="session-drawer-header">
          <h4>历史会话</h4>
          <div className="session-drawer-actions">
            <button
              type="button"
              className="session-new-button"
              onClick={() => {
                onNew();
                onClose();
              }}
            >
              新对话
            </button>
            <button
              type="button"
              className="session-close-button"
              onClick={onClose}
            >
              <X size={14} />
            </button>
          </div>
        </div>

        <div className="session-drawer-body">
          {loading ? (
            <div className="session-drawer-state">
              <RefreshCw className="spin" size={16} />
              <span>加载中...</span>
            </div>
          ) : sessions.length === 0 ? (
            <div className="session-drawer-state">
              <MessageSquareText size={16} />
              <span>暂无会话记录</span>
            </div>
          ) : (
            sessions.map((session) => (
              <button
                type="button"
                key={session.session_id}
                className={`session-item ${
                  session.session_id === currentSessionId ? "active" : ""
                }`}
                onClick={() => {
                  onSelect(session.session_id);
                  onClose();
                }}
              >
                <div className="session-item-info">
                  <span className="session-item-title">
                    {sessionTitle(session)}
                  </span>
                  <span className="session-item-time">
                    {formatSessionTime(session.created_at)}
                  </span>
                </div>
              </button>
            ))
          )}
        </div>
      </div>
    </div>
  );
};
