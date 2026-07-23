import React from "react";
import {
  AlertTriangle,
  ArrowRight,
  CircleGauge,
  Database,
  RefreshCw,
  ShieldCheck,
  TrendingDown,
  TrendingUp,
} from "lucide-react";
import { PortfolioSnapshot } from "../../types/ai-copilot";
import { AssetCategoryIcon } from "./AssetCategoryIcon";
import { formatMoney, formatPercent, formatSignedMoney } from "./formatters";

interface TodayBriefingProps {
  snapshot: PortfolioSnapshot | null;
  loading: boolean;
  error: string | null;
  onRefresh: () => void;
  onQuickQuestion: (question: string) => void;
}

export const TodayBriefing: React.FC<TodayBriefingProps> = ({
  snapshot,
  loading,
  error,
  onRefresh,
  onQuickQuestion,
}) => {
  if (loading && !snapshot) {
    return (
      <div className="copilot-state">
        <RefreshCw className="spin" size={22} />
        <strong>正在生成组合快照</strong>
      </div>
    );
  }

  if (error && !snapshot) {
    return (
      <div className="copilot-state error">
        <AlertTriangle size={22} />
        <strong>组合快照加载失败</strong>
        <span>{error}</span>
        <button type="button" onClick={onRefresh}>重试</button>
      </div>
    );
  }

  if (!snapshot || snapshot.assets.length === 0) {
    return (
      <div className="copilot-state">
        <Database size={22} />
        <strong>暂无可分析持仓</strong>
        <span>在资产页面录入持仓后，这里会生成组合摘要。</span>
      </div>
    );
  }

  const positive = snapshot.daily_change_amount >= 0;
  const completeRate = Math.round(snapshot.data_quality.quote_coverage_percent);
  const concentrated = snapshot.concentration.max_single_percent >= 30;

  return (
    <section className="briefing" aria-label="今日组合简报">
      <div className="briefing-hero">
        <div>
          <span className="copilot-eyebrow">PORTFOLIO PULSE</span>
          <h3>{positive ? "今日组合保持上行" : "今日组合承受回撤"}</h3>
          <p>
            {snapshot.top_movers[0]
              ? `${snapshot.top_movers[0].name}是当前影响最大的持仓项。`
              : "当前没有足够行情计算持仓影响。"}
          </p>
        </div>
        <div className="briefing-total">
          <span>组合金额</span>
          <strong>{formatMoney(snapshot.total_value, true)}</strong>
        </div>
        <div className={`briefing-daily ${positive ? "positive" : "negative"}`}>
          {positive ? <TrendingUp size={18} /> : <TrendingDown size={18} />}
          <div>
            <strong>{formatSignedMoney(snapshot.daily_change_amount, true)}</strong>
            <span>{formatPercent(snapshot.daily_change_percent)}</span>
          </div>
        </div>
      </div>

      <div className="briefing-kpis">
        <div>
          <span>资产数量</span>
          <strong>{snapshot.assets.length}</strong>
          <small>{snapshot.allocation.length} 个类别</small>
        </div>
        <div>
          <span>最大单项占比</span>
          <strong>{snapshot.concentration.max_single_percent.toFixed(1)}%</strong>
          <small>{snapshot.concentration.max_single_name}</small>
        </div>
        <div>
          <span>行情完整度</span>
          <strong>{completeRate}%</strong>
          <small>{snapshot.data_quality.freshness}</small>
        </div>
      </div>

      <div className="briefing-grid">
        <section className="briefing-block">
          <div className="copilot-section-heading">
            <div>
              <span className="copilot-eyebrow">ALLOCATION</span>
              <h3>资产配置</h3>
            </div>
            <button type="button" onClick={() => onQuickQuestion("详细分析我的资产配置")}>分析 <ArrowRight size={13} /></button>
          </div>
          <div className="allocation-list">
            {snapshot.allocation.map((allocation) => (
              <div className="allocation-row" key={allocation.category}>
                <span className={`asset-mark ${allocation.category}`}>
                  <AssetCategoryIcon category={allocation.category} size={15} />
                </span>
                <div className="allocation-name">
                  <strong>{allocation.label}</strong>
                  <small>{allocation.count} 项</small>
                </div>
                <div className="allocation-track"><span style={{ width: `${Math.min(100, allocation.percent)}%` }} /></div>
                <strong className="allocation-percent">{allocation.percent.toFixed(1)}%</strong>
                <span className={allocation.daily_change >= 0 ? "positive" : "negative"}>
                  {formatSignedMoney(allocation.daily_change, true)}
                </span>
              </div>
            ))}
          </div>
        </section>

        <section className="briefing-block">
          <div className="copilot-section-heading">
            <div>
              <span className="copilot-eyebrow">IMPACT</span>
              <h3>今日影响</h3>
            </div>
            <span>按金额绝对值</span>
          </div>
          <div className="mover-list">
            {snapshot.top_movers.slice(0, 4).map((mover, index) => (
              <button
                type="button"
                className="mover-row"
                key={`${mover.category}-${mover.code}-${index}`}
                onClick={() => onQuickQuestion(`解释${mover.name}今天对组合的影响`)}
              >
                <span className="mover-rank">{String(index + 1).padStart(2, "0")}</span>
                <span className={`asset-mark ${mover.category}`}><AssetCategoryIcon category={mover.category} size={14} /></span>
                <span className="mover-name"><strong>{mover.name}</strong><small>{mover.code}</small></span>
                <span className={mover.daily_change_amount >= 0 ? "positive" : "negative"}>
                  <strong>{formatSignedMoney(mover.daily_change_amount, true)}</strong>
                  {mover.change_percent !== null && <small>{formatPercent(mover.change_percent)}</small>}
                </span>
              </button>
            ))}
          </div>
        </section>
      </div>

      <section className={`briefing-insight ${concentrated ? "attention" : "healthy"}`}>
        {concentrated ? <AlertTriangle size={18} /> : <ShieldCheck size={18} />}
        <div>
          <span className="copilot-eyebrow">RISK CHECK</span>
          <strong>{concentrated ? "存在单项集中暴露" : "今日未发现明显集中风险"}</strong>
          <p>
            {concentrated
              ? `${snapshot.concentration.max_single_name}占组合${snapshot.concentration.max_single_percent.toFixed(1)}%，建议先理解风险来源再考虑调整。`
              : "最大单项占比低于 30%。普通波动不必自动转化为操作。"}
          </p>
        </div>
        <button type="button" onClick={() => onQuickQuestion("从集中度和数据缺口检查我的组合")}>
          <CircleGauge size={15} /> 查看依据
        </button>
      </section>
    </section>
  );
};
