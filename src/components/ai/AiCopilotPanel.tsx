import React, { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Bot,
  PanelRightClose,
  PanelRightOpen,
  RefreshCw,
  Settings2,
} from "lucide-react";
import { getLlmConfig } from "../../services/ai-copilot";
import { LlmConfig, PortfolioSnapshot } from "../../types/ai-copilot";
import "../../ai-copilot.css";
import { ChatWorkspace, ChatWorkspaceQuestion } from "./ChatWorkspace";
import { EvidencePanel } from "./EvidencePanel";
import { ModelSettingsDialog } from "./ModelSettingsDialog";
import { PortfolioOverview } from "./PortfolioOverview";
import { TodayBriefing } from "./TodayBriefing";

type ActiveView = "today" | "portfolio" | "chat";

interface AiCopilotPanelProps {
  showToast: (message: string, type?: "success" | "error") => void;
}

function providerLabel(config: LlmConfig | null): string {
  if (!config) return "模型状态未知";
  if (config.provider === "claude") return `Claude · ${config.model}`;
  if (config.provider === "openai") return `OpenAI · ${config.model}`;
  return `兼容接口 · ${config.model}`;
}

export const AiCopilotPanel: React.FC<AiCopilotPanelProps> = ({ showToast }) => {
  const [activeView, setActiveView] = useState<ActiveView>("today");
  const [snapshot, setSnapshot] = useState<PortfolioSnapshot | null>(null);
  const [snapshotLoading, setSnapshotLoading] = useState(false);
  const [snapshotError, setSnapshotError] = useState<string | null>(null);
  const [pendingQuestion, setPendingQuestion] = useState<ChatWorkspaceQuestion | null>(null);
  const [showEvidence, setShowEvidence] = useState(true);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [llmConfig, setLlmConfig] = useState<LlmConfig | null>(null);

  const loadSnapshot = useCallback(async (forceRefresh = false) => {
    setSnapshotLoading(true);
    setSnapshotError(null);
    try {
      const command = forceRefresh
        ? "refresh_portfolio_snapshot"
        : "get_portfolio_snapshot";
      setSnapshot(await invoke<PortfolioSnapshot>(command));
    } catch (error) {
      setSnapshotError(String(error));
    } finally {
      setSnapshotLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadSnapshot();
    getLlmConfig().then(setLlmConfig).catch(() => null);
  }, [loadSnapshot]);

  const handleQuickQuestion = useCallback((question: string) => {
    setPendingQuestion({ text: question, nonce: Date.now() });
    setActiveView("chat");
  }, []);

  const isLocal = llmConfig?.base_url.includes("127.0.0.1") ||
    llmConfig?.base_url.includes("localhost");

  return (
    <div className={`ai-copilot ${showEvidence ? "with-evidence" : ""}`}>
      <header className="copilot-header">
        <div className="copilot-brand">
          <span className="copilot-brand-mark"><Bot size={18} /></span>
          <div><span>LEEK INTELLIGENCE</span><h2>投资驾驶舱</h2></div>
        </div>
        <nav className="copilot-nav" aria-label="AI 驾驶舱视图">
          <button type="button" className={activeView === "today" ? "active" : ""} onClick={() => setActiveView("today")}>今日</button>
          <button type="button" className={activeView === "portfolio" ? "active" : ""} onClick={() => setActiveView("portfolio")}>组合</button>
          <button type="button" className={activeView === "chat" ? "active" : ""} onClick={() => setActiveView("chat")}>问 AI</button>
        </nav>
        <div className="copilot-toolbar">
          <span className={`copilot-mode ${isLocal ? "local" : "cloud"}`}>
            <span />{providerLabel(llmConfig)}
          </span>
          <button type="button" className="copilot-icon-button" onClick={() => setSettingsOpen(true)} title="模型设置"><Settings2 size={17} /></button>
          <button type="button" className="copilot-icon-button" onClick={() => setShowEvidence((current) => !current)} title={showEvidence ? "隐藏分析依据" : "显示分析依据"}>
            {showEvidence ? <PanelRightClose size={17} /> : <PanelRightOpen size={17} />}
          </button>
          <button type="button" className="copilot-icon-button" onClick={() => void loadSnapshot(true)} disabled={snapshotLoading} title="刷新组合快照"><RefreshCw className={snapshotLoading ? "spin" : ""} size={17} /></button>
        </div>
      </header>

      <div className="copilot-body">
        <main className={`copilot-workspace ${activeView === "chat" ? "chat-active" : ""}`}>
          {activeView === "today" && (
            <div className="copilot-view">
              <TodayBriefing snapshot={snapshot} loading={snapshotLoading} error={snapshotError} onRefresh={() => void loadSnapshot(true)} onQuickQuestion={handleQuickQuestion} />
            </div>
          )}
          {activeView === "portfolio" && (
            <div className="copilot-view">
              <PortfolioOverview snapshot={snapshot} loading={snapshotLoading} onAsk={handleQuickQuestion} />
            </div>
          )}
          {activeView === "chat" && (
            <ChatWorkspace snapshot={snapshot} initialQuestion={pendingQuestion} showToast={showToast} />
          )}
        </main>
        {showEvidence && <EvidencePanel snapshot={snapshot} />}
      </div>

      <ModelSettingsDialog
        open={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        onSaved={(config) => {
          setLlmConfig({ ...config, has_api_key: Boolean(config.api_key) || config.has_api_key });
          showToast("模型配置已保存", "success");
        }}
      />
    </div>
  );
};
