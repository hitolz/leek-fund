import React from "react";
import {
  AlertTriangle,
  Calculator,
  CheckCircle2,
  Clock3,
  Database,
  HelpCircle,
  Lightbulb,
} from "lucide-react";
import { PortfolioSnapshot } from "../../types/ai-copilot";
import { formatSnapshotTime } from "./formatters";

interface EvidencePanelProps {
  snapshot: PortfolioSnapshot | null;
}

export const EvidencePanel: React.FC<EvidencePanelProps> = ({ snapshot }) => {
  if (!snapshot) {
    return (
      <aside className="evidence-panel">
        <div className="evidence-header"><h3>分析依据</h3></div>
        <div className="copilot-state compact"><Database size={18} /><span>等待快照</span></div>
      </aside>
    );
  }

  const completePercent = Math.round(snapshot.data_quality.quote_coverage_percent);

  return (
    <aside className="evidence-panel">
      <div className="evidence-header">
        <div>
          <span className="copilot-eyebrow">EVIDENCE</span>
          <h3>分析依据</h3>
        </div>
        <span className={`quality-score ${completePercent === 100 ? "complete" : "partial"}`}>
          {completePercent}%
        </span>
      </div>

      <div className="evidence-body">
        <section className="evidence-section">
          <h4><Clock3 size={13} /> 固定快照</h4>
          <dl className="evidence-list">
            <div><dt>截止时间</dt><dd>{formatSnapshotTime(snapshot.snapshot_at)}</dd></div>
            <div><dt>快照编号</dt><dd>{snapshot.id.slice(0, 8)}</dd></div>
            <div><dt>资产范围</dt><dd>{snapshot.assets.length} 项</dd></div>
          </dl>
        </section>

        <section className="evidence-section">
          <h4><Database size={13} /> 数据质量</h4>
          <div className="evidence-progress" aria-label={`行情完整度 ${completePercent}%`}>
            <span style={{ width: `${completePercent}%` }} />
          </div>
          <dl className="evidence-list">
            <div><dt>完整行情</dt><dd>{snapshot.data_quality.complete_assets}</dd></div>
            <div><dt>缺少行情</dt><dd>{snapshot.data_quality.missing_quote}</dd></div>
          </dl>
          {snapshot.data_quality.gaps.slice(0, 4).map((gap) => (
            <div className="evidence-gap" key={gap}>
              <AlertTriangle size={13} />
              <span>{gap}</span>
            </div>
          ))}
        </section>

        <section className="evidence-section">
          <h4>结论标记</h4>
          <ul className="evidence-legend">
            <li><CheckCircle2 className="fact" size={14} /><span>事实</span><small>行情与持仓</small></li>
            <li><Calculator className="calculation" size={14} /><span>计算</span><small>Rust 确定性结果</small></li>
            <li><Lightbulb className="inference" size={14} /><span>推断</span><small>模型解释</small></li>
            <li><HelpCircle className="unknown" size={14} /><span>未知</span><small>数据不足</small></li>
          </ul>
        </section>
      </div>
    </aside>
  );
};
