import React, { useState, useEffect, useCallback } from "react";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { CryptoQuote, CryptoHolding } from "../types";

const CRYPTO_WATCHLIST_KEY = "leek-fund-crypto-watchlist";

interface CryptoPanelProps {
  showToast: (message: string, type?: "success" | "error") => void;
}

// 常用加密货币
const POPULAR_CRYPTOS = [
  { symbol: "BTCUSDT", name: "比特币" },
  { symbol: "ETHUSDT", name: "以太坊" },
  { symbol: "BNBUSDT", name: "币安币" },
  { symbol: "SOLUSDT", name: "Solana" },
  { symbol: "XRPUSDT", name: "瑞波币" },
  { symbol: "DOGEUSDT", name: "狗狗币" },
  { symbol: "ADAUSDT", name: "艾达币" },
  { symbol: "AVAXUSDT", name: "雪崩" },
];

export const CryptoPanel: React.FC<CryptoPanelProps> = ({ showToast }) => {
  const [cryptos, setCryptos] = useState<CryptoQuote[]>([]);
  const [holdings, setHoldings] = useState<Record<string, CryptoHolding>>({});
  const [loading, setLoading] = useState(false);
  const [editingHolding, setEditingHolding] = useState<string | null>(null);
  const [holdingForm, setHoldingForm] = useState({ quantity: "", costPrice: "" });
  const {
    getCryptoQuote,
    getCryptoQuotes,
    getCryptoHolding,
    setCryptoHolding,
    clearCryptoHolding,
  } = useTauriCommands();

  // 保存到 localStorage
  const saveWatchlist = useCallback((symbols: string[]) => {
    localStorage.setItem(CRYPTO_WATCHLIST_KEY, JSON.stringify(symbols));
  }, []);

  // 从 localStorage 加载
  const loadWatchlist = useCallback((): string[] => {
    try {
      const saved = localStorage.getItem(CRYPTO_WATCHLIST_KEY);
      return saved ? JSON.parse(saved) : [];
    } catch {
      return [];
    }
  }, []);

  // 初始化：从 localStorage 加载加密货币列表
  useEffect(() => {
    const init = async () => {
      const symbols = loadWatchlist();
      if (symbols.length === 0) return;

      setLoading(true);
      try {
        const loadedCryptos: CryptoQuote[] = [];
        for (const symbol of symbols) {
          try {
            const quote = await getCryptoQuote(symbol);
            loadedCryptos.push(quote);
            // 加载持仓
            const holding = await getCryptoHolding(symbol);
            if (holding) {
              setHoldings((prev) => ({ ...prev, [symbol]: holding }));
            }
          } catch (error) {
            console.error(`Failed to load crypto ${symbol}:`, error);
          }
        }
        setCryptos(loadedCryptos);
      } catch (error) {
        console.error("Failed to load watchlist:", error);
      } finally {
        setLoading(false);
      }
    };
    init();
  }, [loadWatchlist, getCryptoQuote, getCryptoHolding]);

  // 加载持仓
  const loadHolding = useCallback(
    async (symbol: string) => {
      try {
        const holding = await getCryptoHolding(symbol);
        if (holding) {
          setHoldings((prev) => ({ ...prev, [symbol]: holding }));
        }
      } catch (error) {
        // ignore
      }
    },
    [getCryptoHolding]
  );

  // 添加加密货币
  const handleAddCrypto = useCallback(
    async (symbol: string) => {
      if (cryptos.some((c) => c.symbol === symbol)) {
        showToast("该币种已在列表中", "error");
        return;
      }
      setLoading(true);
      try {
        const quote = await getCryptoQuote(symbol);
        const newCryptos = [...cryptos, quote];
        setCryptos(newCryptos);
        // 保存到 localStorage
        saveWatchlist(newCryptos.map((c) => c.symbol));
        showToast(`已添加 ${quote.name}`);
        // 加载持仓
        await loadHolding(symbol);
      } catch (error) {
        showToast(String(error), "error");
      } finally {
        setLoading(false);
      }
    },
    [cryptos, getCryptoQuote, showToast, loadHolding, saveWatchlist]
  );

  // 删除加密货币
  const handleRemoveCrypto = useCallback((symbol: string) => {
    setCryptos((prev) => {
      const newCryptos = prev.filter((c) => c.symbol !== symbol);
      // 保存到 localStorage
      saveWatchlist(newCryptos.map((c) => c.symbol));
      return newCryptos;
    });
    setHoldings((prev) => {
      const next = { ...prev };
      delete next[symbol];
      return next;
    });
  }, [saveWatchlist]);

  // 刷新行情
  const handleRefresh = useCallback(async () => {
    if (cryptos.length === 0) return;
    setLoading(true);
    try {
      const symbols = cryptos.map((c) => c.symbol);
      const updated = await getCryptoQuotes(symbols);
      setCryptos(updated);
    } catch (error) {
      // ignore
    } finally {
      setLoading(false);
    }
  }, [cryptos, getCryptoQuotes]);

  // 保存持仓
  const handleSaveHolding = useCallback(
    async (symbol: string) => {
      const quantity = parseFloat(holdingForm.quantity);
      const costPrice = parseFloat(holdingForm.costPrice);
      if (isNaN(quantity) || isNaN(costPrice) || quantity < 0 || costPrice < 0) {
        showToast("请输入有效的数量和成本价", "error");
        return;
      }
      // 持仓金额 = 数量 * 成本价
      const amount = quantity * costPrice;
      try {
        const holding = await setCryptoHolding(symbol, amount, quantity);
        setHoldings((prev) => ({ ...prev, [symbol]: holding }));
        setEditingHolding(null);
        setHoldingForm({ quantity: "", costPrice: "" });
        showToast("持仓已保存");
      } catch (error) {
        showToast(String(error), "error");
      }
    },
    [holdingForm, setCryptoHolding, showToast]
  );

  // 清空持仓
  const handleClearHolding = useCallback(
    async (symbol: string) => {
      try {
        await clearCryptoHolding(symbol);
        setHoldings((prev) => {
          const next = { ...prev };
          delete next[symbol];
          return next;
        });
        showToast("持仓已清空");
      } catch (error) {
        showToast(String(error), "error");
      }
    },
    [clearCryptoHolding, showToast]
  );

  // 开始编辑持仓
  const startEditHolding = useCallback(
    (symbol: string) => {
      const holding = holdings[symbol];
      setHoldingForm({
        quantity: holding?.holding_quantity?.toString() || "",
        costPrice: holding?.cost_price?.toString() || "",
      });
      setEditingHolding(symbol);
    },
    [holdings]
  );

  // 自动刷新
  useEffect(() => {
    if (cryptos.length === 0) return;
    const timer = setInterval(handleRefresh, 15000);
    return () => clearInterval(timer);
  }, [cryptos.length, handleRefresh]);

  return (
    <div className="crypto-panel">
      {/* 快速添加 */}
      <div className="quick-add-section">
        <h3>快速添加</h3>
        <div className="quick-add-grid">
          {POPULAR_CRYPTOS.map((crypto) => (
            <button
              key={crypto.symbol}
              type="button"
              className={`quick-add-btn ${
                cryptos.some((c) => c.symbol === crypto.symbol) ? "active" : ""
              }`}
              onClick={() => handleAddCrypto(crypto.symbol)}
              disabled={
                loading || cryptos.some((c) => c.symbol === crypto.symbol)
              }
            >
              <span className="crypto-name">{crypto.name}</span>
              <span className="crypto-symbol">{crypto.symbol}</span>
            </button>
          ))}
        </div>
      </div>

      {/* 加密货币列表 */}
      <div className="crypto-list">
        <div className="crypto-list-header">
          <h3>自选加密货币</h3>
          <button
            type="button"
            className="button ghost small"
            onClick={handleRefresh}
            disabled={loading || cryptos.length === 0}
          >
            {loading ? "刷新中..." : "刷新"}
          </button>
        </div>

        {cryptos.length === 0 ? (
          <div className="empty-state">
            <p>暂无自选加密货币</p>
            <p className="hint">点击上方快速添加按钮</p>
          </div>
        ) : (
          <div className="crypto-cards">
            {cryptos.map((crypto) => (
              <CryptoCard
                key={crypto.symbol}
                crypto={crypto}
                holding={holdings[crypto.symbol]}
                editingHolding={editingHolding === crypto.symbol}
                holdingForm={holdingForm}
                onRemove={() => handleRemoveCrypto(crypto.symbol)}
                onStartEdit={() => startEditHolding(crypto.symbol)}
                onSaveHolding={() => handleSaveHolding(crypto.symbol)}
                onCancelEdit={() => setEditingHolding(null)}
                onClearHolding={() => handleClearHolding(crypto.symbol)}
                onHoldingFormChange={setHoldingForm}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

// 加密货币卡片组件
const CryptoCard: React.FC<{
  crypto: CryptoQuote;
  holding?: CryptoHolding;
  editingHolding: boolean;
  holdingForm: { quantity: string; costPrice: string };
  onRemove: () => void;
  onStartEdit: () => void;
  onSaveHolding: () => void;
  onCancelEdit: () => void;
  onClearHolding: () => void;
  onHoldingFormChange: (form: { quantity: string; costPrice: string }) => void;
}> = ({
  crypto,
  holding,
  editingHolding,
  holdingForm,
  onRemove,
  onStartEdit,
  onSaveHolding,
  onCancelEdit,
  onClearHolding,
  onHoldingFormChange,
}) => {
  const isUp = (crypto.change_percent ?? 0) >= 0;

  // 计算持仓盈亏
  const calculateProfit = () => {
    if (!holding || !crypto.price) return null;
    const currentValue = holding.holding_quantity * crypto.price;
    const costPrice = holding.cost_price; // 成本价
    // 总盈亏 = (现价 - 成本价) * 持仓数量
    const totalProfit = (crypto.price - costPrice) * holding.holding_quantity;
    // 当日涨跌额 = 从涨跌幅反推前一日价格，再计算差额
    const changePercent = crypto.change_percent ?? 0;
    const previousPrice = changePercent !== -100
      ? crypto.price / (1 + changePercent / 100)
      : crypto.price;
    const dailyChangeAmount = (crypto.price - previousPrice) * holding.holding_quantity;
    const profitPercent = costPrice > 0
      ? ((crypto.price - costPrice) / costPrice) * 100
      : 0;
    return { totalProfit, dailyChangeAmount, profitPercent, currentValue, costPrice };
  };

  const profitInfo = calculateProfit();

  return (
    <div className="crypto-card">
      <div className="crypto-card-header">
        <div className="crypto-info">
          <div className="crypto-name">{crypto.name}</div>
          <div className="crypto-symbol">{crypto.symbol}</div>
        </div>
        <button
          type="button"
          className="button ghost small remove-btn"
          onClick={onRemove}
        >
          ×
        </button>
      </div>
      <div className={`crypto-price ${isUp ? "up" : "down"}`}>
        $
        {crypto.price?.toLocaleString(undefined, {
          minimumFractionDigits: 2,
          maximumFractionDigits: 2 }) ?? "--"}
      </div>
      <div className={`crypto-change ${isUp ? "up" : "down"}`}>
        {crypto.change_percent !== null
          ? `${isUp ? "+" : ""}${crypto.change_percent?.toFixed(2)}%`
          : "--"}
      </div>

      {/* 总计涨跌额（有持仓时显示） */}
      {profitInfo && (
        <div className={`total-profit ${profitInfo.totalProfit >= 0 ? "up" : "down"}`}>
          <span className="profit-label">总计涨跌额</span>
          <span className="profit-value">
            {profitInfo.totalProfit >= 0 ? "+" : ""}$
            {profitInfo.totalProfit.toFixed(2)}
          </span>
        </div>
      )}

      {/* 行情详情 */}
      <div className="crypto-details">
        <div className="detail-item">
          <span className="label">24h最高</span>
          <span className="value up">
            $
            {crypto.high_24h?.toLocaleString(undefined, {
              minimumFractionDigits: 2 }) ?? "--"}
          </span>
        </div>
        <div className="detail-item">
          <span className="label">24h最低</span>
          <span className="value down">
            $
            {crypto.low_24h?.toLocaleString(undefined, {
              minimumFractionDigits: 2 }) ?? "--"}
          </span>
        </div>
        <div className="detail-item">
          <span className="label">24h成交量</span>
          <span className="value">
            {crypto.volume_24h?.toLocaleString(undefined, {
              maximumFractionDigits: 0 }) ?? "--"}
          </span>
        </div>
      </div>

      {/* 持仓信息 */}
      <div className="holding-section">
        <div className="holding-header">
          <span className="holding-title">持仓信息</span>
          {holding && !editingHolding && (
            <div className="holding-actions">
              <button
                type="button"
                className="button ghost small"
                onClick={onStartEdit}
              >
                编辑
              </button>
              <button
                type="button"
                className="button ghost small danger"
                onClick={onClearHolding}
              >
                清空
              </button>
            </div>
          )}
        </div>

        {editingHolding ? (
          <div className="holding-form">
            <div className="form-row">
              <label>数量</label>
              <input
                type="number"
                value={holdingForm.quantity}
                onChange={(e) =>
                  onHoldingFormChange({
                    ...holdingForm,
                    quantity: e.target.value,
                  })
                }
                placeholder="输入持仓数量"
              />
            </div>
            <div className="form-row">
              <label>成本价(USDT)</label>
              <input
                type="number"
                value={holdingForm.costPrice}
                onChange={(e) =>
                  onHoldingFormChange({
                    ...holdingForm,
                    costPrice: e.target.value,
                  })
                }
                placeholder="输入成本价"
              />
            </div>
            <div className="form-actions">
              <button
                type="button"
                className="button primary small"
                onClick={onSaveHolding}
              >
                保存
              </button>
              <button
                type="button"
                className="button ghost small"
                onClick={onCancelEdit}
              >
                取消
              </button>
            </div>
          </div>
        ) : holding ? (
          <div className="holding-info">
            <div className="holding-item">
              <span className="label">数量</span>
              <span className="value">{holding.holding_quantity}</span>
            </div>
            <div className="holding-item">
              <span className="label">现价</span>
              <span className="value">
                ${crypto.price?.toLocaleString(undefined, { minimumFractionDigits: 2 }) ?? "--"}
              </span>
            </div>
            <div className="holding-item">
              <span className="label">成本</span>
              <span className="value">
                ${holding.cost_price.toFixed(2)}
              </span>
            </div>
            {profitInfo && (
              <>
                <div className="holding-item">
                  <span className="label">当前市值</span>
                  <span className="value">
                    ${profitInfo.currentValue.toLocaleString()}
                  </span>
                </div>
                <div className="holding-item">
                  <span className="label">当日涨跌额</span>
                  <span
                    className={`value ${
                      profitInfo.dailyChangeAmount >= 0 ? "up" : "down"
                    }`}
                  >
                    {profitInfo.dailyChangeAmount >= 0 ? "+" : ""}$
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
            onClick={onStartEdit}
          >
            + 添加持仓
          </button>
        )}
      </div>

      {crypto.update_time && (
        <div className="crypto-time">更新: {crypto.update_time}</div>
      )}
    </div>
  );
};
