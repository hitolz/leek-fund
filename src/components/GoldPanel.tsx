import React, { useState, useEffect, useCallback } from "react";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { GoldQuote, CryptoHolding } from "../types";

interface GoldPanelProps {
  showToast: (message: string, type?: "success" | "error") => void;
}

export const GoldPanel: React.FC<GoldPanelProps> = ({ showToast }) => {
  const [gold, setGold] = useState<GoldQuote | null>(null);
  const [holding, setHolding] = useState<CryptoHolding | null>(null);
  const [loading, setLoading] = useState(false);
  const [editingHolding, setEditingHolding] = useState(false);
  const [holdingForm, setHoldingForm] = useState({ quantity: "", costPrice: "" });
  const { getGoldQuote, getGoldHolding, setGoldHolding, clearGoldHolding } =
    useTauriCommands();

  // 加载黄金行情
  const loadGoldQuote = useCallback(async () => {
    setLoading(true);
    try {
      const quote = await getGoldQuote();
      setGold(quote);
    } catch (error) {
      console.error("Failed to load gold quote:", error);
    } finally {
      setLoading(false);
    }
  }, [getGoldQuote]);

  // 加载持仓
  const loadHolding = useCallback(async () => {
    try {
      const h = await getGoldHolding();
      setHolding(h);
    } catch (error) {
      console.error("Failed to load gold holding:", error);
    }
  }, [getGoldHolding]);

  // 初始化
  useEffect(() => {
    loadGoldQuote();
    loadHolding();
  }, [loadGoldQuote, loadHolding]);

  // 自动刷新（每15秒）
  useEffect(() => {
    const timer = setInterval(loadGoldQuote, 15000);
    return () => clearInterval(timer);
  }, [loadGoldQuote]);

  // 开始编辑持仓
  const startEditHolding = useCallback(() => {
    setHoldingForm({
      quantity: holding?.holding_quantity?.toString() || "",
      costPrice: holding?.cost_price?.toString() || "",
    });
    setEditingHolding(true);
  }, [holding]);

  // 保存持仓
  const handleSaveHolding = useCallback(async () => {
    const quantity = parseFloat(holdingForm.quantity);
    const costPrice = parseFloat(holdingForm.costPrice);
    if (isNaN(quantity) || isNaN(costPrice) || quantity < 0 || costPrice < 0) {
      showToast("请输入有效的数量和成本价", "error");
      return;
    }
    const amount = quantity * costPrice;
    try {
      const h = await setGoldHolding(amount, quantity);
      setHolding(h);
      setEditingHolding(false);
      setHoldingForm({ quantity: "", costPrice: "" });
      showToast("持仓已保存");
    } catch (error) {
      showToast(String(error), "error");
    }
  }, [holdingForm, setGoldHolding, showToast]);

  // 清空持仓
  const handleClearHolding = useCallback(async () => {
    try {
      await clearGoldHolding();
      setHolding(null);
      showToast("持仓已清空");
    } catch (error) {
      showToast(String(error), "error");
    }
  }, [clearGoldHolding, showToast]);

  // 计算持仓盈亏
  const calculateProfit = () => {
    if (!holding || !gold?.price) return null;
    const currentValue = holding.holding_quantity * gold.price;
    const costPrice = holding.cost_price;
    const dailyChangeAmount = (gold.price - costPrice) * holding.holding_quantity;
    const profitPercent = costPrice > 0
      ? ((gold.price - costPrice) / costPrice) * 100
      : 0;
    return { dailyChangeAmount, profitPercent, currentValue, costPrice };
  };

  const profitInfo = calculateProfit();
  const isUp = (gold?.change_percent ?? 0) >= 0;

  return (
    <div className="gold-panel">
      {/* 黄金行情卡片 */}
      <div className="gold-card">
        <div className="gold-card-header">
          <div className="gold-icon">🥇</div>
          <div className="gold-title">
            <h3>黄金 AU9999</h3>
            <span className="gold-subtitle">上海黄金交易所</span>
          </div>
          <button
            type="button"
            className="button ghost small"
            onClick={loadGoldQuote}
            disabled={loading}
          >
            {loading ? "刷新中..." : "刷新"}
          </button>
        </div>

        {gold ? (
          <>
            <div className={`gold-price ${isUp ? "up" : "down"}`}>
              ¥{gold.price?.toFixed(2) ?? "--"}
            </div>
            <div className={`gold-change ${isUp ? "up" : "down"}`}>
              {gold.change_percent !== null
                ? `${isUp ? "+" : ""}${gold.change_percent?.toFixed(2)}%`
                : "--"}
            </div>

            {/* 总计涨跌额 */}
            {profitInfo && (
              <div className={`total-profit ${profitInfo.dailyChangeAmount >= 0 ? "up" : "down"}`}>
                <span className="profit-label">总计涨跌额</span>
                <span className="profit-value">
                  {profitInfo.dailyChangeAmount >= 0 ? "+" : ""}¥
                  {profitInfo.dailyChangeAmount.toFixed(2)}
                </span>
              </div>
            )}

            {/* 行情详情 */}
            <div className="gold-details">
              <div className="detail-item">
                <span className="label">今开</span>
                <span className="value">
                  ¥{gold.open?.toFixed(2) ?? "--"}
                </span>
              </div>
              <div className="detail-item">
                <span className="label">最高</span>
                <span className="value up">
                  ¥{gold.high?.toFixed(2) ?? "--"}
                </span>
              </div>
              <div className="detail-item">
                <span className="label">最低</span>
                <span className="value down">
                  ¥{gold.low?.toFixed(2) ?? "--"}
                </span>
              </div>
              <div className="detail-item">
                <span className="label">昨收</span>
                <span className="value">
                  ¥{gold.yesterday_close?.toFixed(2) ?? "--"}
                </span>
              </div>
              <div className="detail-item">
                <span className="label">涨跌额</span>
                <span className={`value ${isUp ? "up" : "down"}`}>
                  {gold.change_amount !== null
                    ? `${isUp ? "+" : ""}${gold.change_amount?.toFixed(2)}`
                    : "--"}
                </span>
              </div>
              <div className="detail-item">
                <span className="label">成交量</span>
                <span className="value">
                  {gold.volume?.toLocaleString() ?? "--"} 手
                </span>
              </div>
            </div>

            {gold.update_time && (
              <div className="gold-time">更新: {gold.update_time}</div>
            )}
          </>
        ) : (
          <div className="gold-loading">加载中...</div>
        )}
      </div>

      {/* 持仓信息 */}
      <div className="gold-holding-card">
        <div className="holding-header">
          <span className="holding-title">持仓信息</span>
          {holding && !editingHolding && (
            <div className="holding-actions">
              <button
                type="button"
                className="button ghost small"
                onClick={startEditHolding}
              >
                编辑
              </button>
              <button
                type="button"
                className="button ghost small danger"
                onClick={handleClearHolding}
              >
                清空
              </button>
            </div>
          )}
        </div>

        {editingHolding ? (
          <div className="holding-form">
            <div className="form-row">
              <label>数量(克)</label>
              <input
                type="number"
                value={holdingForm.quantity}
                onChange={(e) =>
                  setHoldingForm({ ...holdingForm, quantity: e.target.value })
                }
                placeholder="输入持仓数量"
              />
            </div>
            <div className="form-row">
              <label>成本价(元/克)</label>
              <input
                type="number"
                value={holdingForm.costPrice}
                onChange={(e) =>
                  setHoldingForm({ ...holdingForm, costPrice: e.target.value })
                }
                placeholder="输入成本价"
              />
            </div>
            <div className="form-actions">
              <button
                type="button"
                className="button primary small"
                onClick={handleSaveHolding}
              >
                保存
              </button>
              <button
                type="button"
                className="button ghost small"
                onClick={() => setEditingHolding(false)}
              >
                取消
              </button>
            </div>
          </div>
        ) : holding ? (
          <div className="holding-info">
            <div className="holding-item">
              <span className="label">数量</span>
              <span className="value">{holding.holding_quantity} 克</span>
            </div>
            <div className="holding-item">
              <span className="label">现价</span>
              <span className="value">
                ¥{gold?.price?.toFixed(2) ?? "--"}
              </span>
            </div>
            <div className="holding-item">
              <span className="label">成本</span>
              <span className="value">¥{holding.cost_price.toFixed(2)}</span>
            </div>
            {profitInfo && (
              <>
                <div className="holding-item">
                  <span className="label">当前市值</span>
                  <span className="value">
                    ¥{profitInfo.currentValue.toLocaleString()}
                  </span>
                </div>
                <div className="holding-item">
                  <span className="label">当日涨跌额</span>
                  <span
                    className={`value ${
                      profitInfo.dailyChangeAmount >= 0 ? "up" : "down"
                    }`}
                  >
                    {profitInfo.dailyChangeAmount >= 0 ? "+" : ""}¥
                    {profitInfo.dailyChangeAmount.toFixed(2)}
                  </span>
                </div>
                <div className="holding-item">
                  <span className="label">涨跌幅</span>
                  <span
                    className={`value ${
                      profitInfo.profitPercent >= 0 ? "up" : "down"
                    }`}
                  >
                    {profitInfo.profitPercent >= 0 ? "+" : ""}
                    {profitInfo.profitPercent.toFixed(2)}%
                  </span>
                </div>
              </>
            )}
          </div>
        ) : (
          <button
            type="button"
            className="button ghost small add-holding-btn"
            onClick={startEditHolding}
          >
            + 添加持仓
          </button>
        )}
      </div>
    </div>
  );
};
