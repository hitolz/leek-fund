import React from "react";
import { Database, LockKeyhole } from "lucide-react";
import { PortfolioSnapshot } from "../../types/ai-copilot";
import { formatSnapshotTime } from "./formatters";

interface ContextBarProps {
  snapshot: PortfolioSnapshot | null;
}

export const ContextBar: React.FC<ContextBarProps> = ({ snapshot }) => {
  return (
    <div className="context-bar">
      <div className="context-tags">
        {snapshot ? (
          <>
            <span className="context-tag active">
              <Database size={12} />
              全组合 · {snapshot.assets.length} 项
            </span>
            <span className="context-tag">
              {formatSnapshotTime(snapshot.snapshot_at)} · {snapshot.id.slice(0, 8)}
            </span>
          </>
        ) : (
          <span className="context-tag">组合上下文未就绪</span>
        )}
      </div>
      <span className="context-privacy"><LockKeyhole size={12} /> 本地快照</span>
    </div>
  );
};
