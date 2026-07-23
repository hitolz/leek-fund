import React from "react";
import { AlertTriangle, Database, ShieldCheck } from "lucide-react";
import { PortfolioSnapshot } from "../../types/ai-copilot";
import { AssetCategoryIcon } from "./AssetCategoryIcon";
import { formatMoney, formatPercent, formatSignedMoney } from "./formatters";

interface PortfolioOverviewProps {
  snapshot: PortfolioSnapshot | null;
  loading: boolean;
  onAsk: (question: string) => void;
}

export const PortfolioOverview: React.FC<PortfolioOverviewProps> = ({
  snapshot,
  loading,
  onAsk,
}) => {
  if (!snapshot) {
    return (
      <div className="copilot-state">
        <Database size={22} />
        <strong>{loading ? "正在整理组合" : "暂无可分析持仓"}</strong>
      </div>
    );
  }

  const completeRate = snapshot.data_quality.quote_coverage_percent;
  const concentrated = snapshot.concentration.max_single_percent >= 30;

  return (
    <section className="portfolio-view" aria-label="组合体检">
      <div className="portfolio-kpis">
        <div className="portfolio-kpi">
          <span>最大单项占比</span>
          <strong>{snapshot.concentration.max_single_percent.toFixed(1)}%</strong>
          <small>{snapshot.concentration.max_single_name || "暂无持仓"}</small>
        </div>
        <div className="portfolio-kpi">
          <span>前五项占比</span>
          <strong>{snapshot.concentration.top5_percent.toFixed(1)}%</strong>
          <small>{snapshot.concentration.top5_names.length} 项资产</small>
        </div>
        <div className="portfolio-kpi">
          <span>行情完整度</span>
          <strong>{completeRate.toFixed(0)}%</strong>
          <small>{snapshot.data_quality.complete_assets}/{snapshot.data_quality.total_assets} 项完整</small>
        </div>
      </div>

      <div className={`portfolio-finding ${concentrated ? "attention" : "healthy"}`}>
        {concentrated ? <AlertTriangle size={18} /> : <ShieldCheck size={18} />}
        <div>
          <strong>{concentrated ? "单项持仓集中度需要关注" : "未发现明显单项集中风险"}</strong>
          <p>
            {concentrated
              ? `${snapshot.concentration.max_single_name} 占组合 ${snapshot.concentration.max_single_percent.toFixed(1)}%，可进一步比较保持不变与降低占比的差异。`
              : "当前最大单项占比低于 30%，仍需结合投资期限与资产相关性判断。"}
          </p>
        </div>
        <button
          type="button"
          onClick={() =>
            onAsk(
              concentrated
                ? `分析${snapshot.concentration.max_single_name}的集中度风险`
                : "从集中度角度检查我的组合"
            )
          }
        >
          继续分析
        </button>
      </div>

      <div className="portfolio-table-wrap">
        <div className="copilot-section-heading">
          <div>
            <span className="copilot-eyebrow">HOLDINGS</span>
            <h3>持仓贡献</h3>
          </div>
          <span>{snapshot.assets.length} 项</span>
        </div>
        <table className="portfolio-table">
          <thead>
            <tr>
              <th>资产</th>
              <th>类别</th>
              <th className="numeric">持仓金额</th>
              <th className="numeric">组合占比</th>
              <th className="numeric">今日影响</th>
            </tr>
          </thead>
          <tbody>
            {[...snapshot.assets]
              .sort((a, b) => b.holding_amount - a.holding_amount)
              .map((asset, index) => {
                const weight = snapshot.total_value
                  ? (asset.holding_amount / snapshot.total_value) * 100
                  : 0;
                return (
                  <tr key={`${asset.category}-${asset.code}-${asset.group_name ?? index}`}>
                    <td>
                      <button
                        type="button"
                        className="portfolio-asset-button"
                        onClick={() => onAsk(`分析${asset.name}对我组合的影响`)}
                      >
                        <AssetCategoryIcon category={asset.category} />
                        <span>
                          <strong>{asset.name}</strong>
                          <small>{asset.code}</small>
                        </span>
                      </button>
                    </td>
                    <td>{asset.group_name || asset.category}</td>
                    <td className="numeric">{formatMoney(asset.holding_amount)}</td>
                    <td className="numeric">{weight.toFixed(1)}%</td>
                    <td
                      className={`numeric ${
                        (asset.daily_change_amount ?? 0) >= 0 ? "positive" : "negative"
                      }`}
                    >
                      {asset.daily_change_amount === null
                        ? "--"
                        : formatSignedMoney(asset.daily_change_amount)}
                      {asset.change_percent !== null && (
                        <small>{formatPercent(asset.change_percent)}</small>
                      )}
                    </td>
                  </tr>
                );
              })}
          </tbody>
        </table>
      </div>
    </section>
  );
};
