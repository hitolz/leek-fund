import React, { useState, useEffect, useCallback } from "react";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { StockQuote, StockSearchResult, StockHolding } from "../types";

const STOCK_WATCHLIST_KEY = "leek-fund-stock-watchlist";

interface StockPanelProps {
  showToast: (message: string, type?: "success" | "error") => void;
}

export const StockPanel: React.FC<StockPanelProps> = ({ showToast }) => {
  const [keyword, setKeyword] = useState("");
  const [searchResults, setSearchResults] = useState<StockSearchResult[]>([]);
  const [stocks, setStocks] = useState<StockQuote[]>([]);
  const [holdings, setHoldings] = useState<Record<string, StockHolding>>({});
  const [loading, setLoading] = useState(false);
  const [searchLoading, setSearchLoading] = useState(false);
  const [editingHolding, setEditingHolding] = useState<string | null>(null);
  const [holdingForm, setHoldingForm] = useState({ shares: "", costPrice: "" });
  const { searchStock, getStockQuote, getStockHolding, setStockHolding, clearStockHolding } =
    useTauriCommands();

  // 保存到 localStorage
  const saveWatchlist = useCallback((stockCodes: string[]) => {
    localStorage.setItem(STOCK_WATCHLIST_KEY, JSON.stringify(stockCodes));
  }, []);

  // 从 localStorage 加载
  const loadWatchlist = useCallback((): string[] => {
    try {
      const saved = localStorage.getItem(STOCK_WATCHLIST_KEY);
      return saved ? JSON.parse(saved) : [];
    } catch {
      return [];
    }
  }, []);

  // 初始化：从 localStorage 加载股票列表
  useEffect(() => {
    const init = async () => {
      const codes = loadWatchlist();
      if (codes.length === 0) return;

      setLoading(true);
      try {
        const loadedStocks: StockQuote[] = [];
        for (const code of codes) {
          try {
            const quote = await getStockQuote(code);
            loadedStocks.push(quote);
            // 加载持仓
            const holding = await getStockHolding(code);
            if (holding) {
              setHoldings((prev) => ({ ...prev, [code]: holding }));
            }
          } catch (error) {
            console.error(`Failed to load stock ${code}:`, error);
          }
        }
        setStocks(loadedStocks);
      } catch (error) {
        console.error("Failed to load watchlist:", error);
      } finally {
        setLoading(false);
      }
    };
    init();
  }, [loadWatchlist, getStockQuote, getStockHolding]);

  // 搜索股票
  const handleSearch = useCallback(async () => {
    if (!keyword.trim()) {
      setSearchResults([]);
      return;
    }
    setSearchLoading(true);
    try {
      const results = await searchStock(keyword.trim());
      setSearchResults(results);
    } catch (error) {
      showToast(String(error), "error");
      setSearchResults([]);
    } finally {
      setSearchLoading(false);
    }
  }, [keyword, searchStock, showToast]);

  // 加载持仓
  const loadHolding = useCallback(
    async (code: string) => {
      try {
        const holding = await getStockHolding(code);
        if (holding) {
          setHoldings((prev) => ({ ...prev, [code]: holding }));
        }
      } catch (error) {
        // ignore
      }
    },
    [getStockHolding]
  );

  // 添加股票到列表
  const handleAddStock = useCallback(
    async (code: string) => {
      if (stocks.some((s) => s.code === code)) {
        showToast("该股票已在列表中", "error");
        return;
      }
      setLoading(true);
      try {
        const quote = await getStockQuote(code);
        const newStocks = [...stocks, quote];
        setStocks(newStocks);
        // 保存到 localStorage
        saveWatchlist(newStocks.map((s) => s.code));
        setSearchResults([]);
        setKeyword("");
        showToast(`已添加 ${quote.name}`);
        // 加载持仓
        await loadHolding(code);
      } catch (error) {
        showToast(String(error), "error");
      } finally {
        setLoading(false);
      }
    },
    [stocks, getStockQuote, showToast, loadHolding, saveWatchlist]
  );

  // 删除股票
  const handleRemoveStock = useCallback((code: string) => {
    setStocks((prev) => {
      const newStocks = prev.filter((s) => s.code !== code);
      // 保存到 localStorage
      saveWatchlist(newStocks.map((s) => s.code));
      return newStocks;
    });
    setHoldings((prev) => {
      const next = { ...prev };
      delete next[code];
      return next;
    });
  }, [saveWatchlist]);

  // 刷新行情
  const handleRefresh = useCallback(async () => {
    if (stocks.length === 0) return;
    setLoading(true);
    try {
      const updatedStocks = await Promise.all(
        stocks.map((s) => getStockQuote(s.code).catch(() => s))
      );
      setStocks(updatedStocks);
    } catch (error) {
      // ignore
    } finally {
      setLoading(false);
    }
  }, [stocks, getStockQuote]);

  // 保存持仓
  const handleSaveHolding = useCallback(
    async (code: string) => {
      const shares = parseFloat(holdingForm.shares);
      const costPrice = parseFloat(holdingForm.costPrice);
      if (isNaN(shares) || isNaN(costPrice) || shares < 0 || costPrice < 0) {
        showToast("请输入有效的数量和成本价", "error");
        return;
      }
      // 持仓金额 = 数量 * 成本价
      const amount = shares * costPrice;
      try {
        const holding = await setStockHolding(code, amount, shares);
        setHoldings((prev) => ({ ...prev, [code]: holding }));
        setEditingHolding(null);
        setHoldingForm({ shares: "", costPrice: "" });
        showToast("持仓已保存");
      } catch (error) {
        showToast(String(error), "error");
      }
    },
    [holdingForm, setStockHolding, showToast]
  );

  // 清空持仓
  const handleClearHolding = useCallback(
    async (code: string) => {
      try {
        await clearStockHolding(code);
        setHoldings((prev) => {
          const next = { ...prev };
          delete next[code];
          return next;
        });
        showToast("持仓已清空");
      } catch (error) {
        showToast(String(error), "error");
      }
    },
    [clearStockHolding, showToast]
  );

  // 开始编辑持仓
  const startEditHolding = useCallback(
    (code: string) => {
      const holding = holdings[code];
      setHoldingForm({
        shares: holding?.holding_shares?.toString() || "",
        costPrice: holding?.cost_price?.toString() || "",
      });
      setEditingHolding(code);
    },
    [holdings]
  );

  // 自动刷新
  useEffect(() => {
    if (stocks.length === 0) return;
    const timer = setInterval(handleRefresh, 30000);
    return () => clearInterval(timer);
  }, [stocks.length, handleRefresh]);

  return (
    <div className="stock-panel">
      {/* 搜索区域 */}
      <div className="search-section">
        <div className="search-input-group">
          <input
            type="text"
            className="search-input"
            placeholder="输入股票名称或代码搜索..."
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
          />
          <button
            type="button"
            className="button primary"
            onClick={handleSearch}
            disabled={searchLoading}
          >
            {searchLoading ? "搜索中..." : "搜索"}
          </button>
        </div>

        {/* 搜索结果 */}
        {searchResults.length > 0 && (
          <div className="search-results">
            {searchResults.map((result) => (
              <div key={result.code} className="search-result-item">
                <div className="result-info">
                  <span className="result-code">{result.code}</span>
                  <span className="result-name">{result.name}</span>
                  <span className="result-market">
                    {result.market === "sh"
                      ? "沪"
                      : result.market === "sz"
                      ? "深"
                      : "港"}
                  </span>
                </div>
                <button
                  type="button"
                  className="button small ghost"
                  onClick={() => handleAddStock(result.code)}
                  disabled={loading}
                >
                  添加
                </button>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* 股票列表 */}
      <div className="stock-list">
        <div className="stock-list-header">
          <h3>自选股票</h3>
          <button
            type="button"
            className="button ghost small"
            onClick={handleRefresh}
            disabled={loading || stocks.length === 0}
          >
            {loading ? "刷新中..." : "刷新"}
          </button>
        </div>

        {stocks.length === 0 ? (
          <div className="empty-state">
            <p>暂无自选股票</p>
            <p className="hint">搜索并添加股票到列表</p>
          </div>
        ) : (
          <div className="stock-cards">
            {stocks.map((stock) => (
              <StockCard
                key={stock.code}
                stock={stock}
                holding={holdings[stock.code]}
                editingHolding={editingHolding === stock.code}
                holdingForm={holdingForm}
                onRemove={() => handleRemoveStock(stock.code)}
                onStartEdit={() => startEditHolding(stock.code)}
                onSaveHolding={() => handleSaveHolding(stock.code)}
                onCancelEdit={() => setEditingHolding(null)}
                onClearHolding={() => handleClearHolding(stock.code)}
                onHoldingFormChange={setHoldingForm}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

// 股票卡片组件
const StockCard: React.FC<{
  stock: StockQuote;
  holding?: StockHolding;
  editingHolding: boolean;
  holdingForm: { shares: string; costPrice: string };
  onRemove: () => void;
  onStartEdit: () => void;
  onSaveHolding: () => void;
  onCancelEdit: () => void;
  onClearHolding: () => void;
  onHoldingFormChange: (form: { shares: string; costPrice: string }) => void;
}> = ({
  stock,
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
  const isUp = (stock.change_percent ?? 0) >= 0;

  // 计算持仓盈亏
  const calculateProfit = () => {
    if (!holding || !stock.price) return null;
    const currentValue = holding.holding_shares * stock.price;
    const costPrice = holding.cost_price; // 成本价
    // 当日涨跌额 = (现价 - 成本价) * 持仓数量
    const dailyChangeAmount = (stock.price - costPrice) * holding.holding_shares;
    const profitPercent = costPrice > 0
      ? ((stock.price - costPrice) / costPrice) * 100
      : 0;
    return { dailyChangeAmount, profitPercent, currentValue, costPrice };
  };

  const profitInfo = calculateProfit();

  return (
    <div className="stock-card">
      <div className="stock-card-header">
        <div className="stock-name">{stock.name}</div>
        <button
          type="button"
          className="button ghost small remove-btn"
          onClick={onRemove}
        >
          ×
        </button>
      </div>
      <div className="stock-code">{stock.code}</div>
      <div className={`stock-price ${isUp ? "up" : "down"}`}>
        {stock.price?.toFixed(2) ?? "--"}
      </div>
      <div className={`stock-change ${isUp ? "up" : "down"}`}>
        {stock.change_percent !== null
          ? `${isUp ? "+" : ""}${stock.change_percent?.toFixed(2)}%`
          : "--"}
      </div>

      {/* 总计涨跌额（有持仓时显示） */}
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
      <div className="stock-details">
        <div className="detail-item">
          <span className="label">涨跌额</span>
          <span className={`value ${isUp ? "up" : "down"}`}>
            {stock.change_amount !== null
              ? `${isUp ? "+" : ""}${stock.change_amount?.toFixed(2)}`
              : "--"}
          </span>
        </div>
        <div className="detail-item">
          <span className="label">今开</span>
          <span className="value">{stock.open?.toFixed(2) ?? "--"}</span>
        </div>
        <div className="detail-item">
          <span className="label">最高</span>
          <span className="value up">{stock.high?.toFixed(2) ?? "--"}</span>
        </div>
        <div className="detail-item">
          <span className="label">最低</span>
          <span className="value down">{stock.low?.toFixed(2) ?? "--"}</span>
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
                value={holdingForm.shares}
                onChange={(e) =>
                  onHoldingFormChange({ ...holdingForm, shares: e.target.value })
                }
                placeholder="输入持仓数量"
              />
            </div>
            <div className="form-row">
              <label>成本价</label>
              <input
                type="number"
                value={holdingForm.costPrice}
                onChange={(e) =>
                  onHoldingFormChange({ ...holdingForm, costPrice: e.target.value })
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
              <span className="value">{holding.holding_shares}股</span>
            </div>
            <div className="holding-item">
              <span className="label">现价</span>
              <span className="value">¥{stock.price?.toFixed(2) ?? "--"}</span>
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
            onClick={onStartEdit}
          >
            + 添加持仓
          </button>
        )}
      </div>

      {stock.update_time && (
        <div className="stock-time">更新: {stock.update_time}</div>
      )}
    </div>
  );
};
