import React, { useEffect, useState } from "react";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { FundInfo, FundList } from "../types";
import { FundInfoCard } from "./FundInfoCard";

interface FundSearchProps {
  lists: FundList[];
  onListsChange: () => void;
  showToast?: (message: string, type: "success" | "error") => void;
}

export const FundSearch: React.FC<FundSearchProps> = ({
  lists,
  onListsChange,
  showToast,
}) => {
  const [code, setCode] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [fund, setFund] = useState<FundInfo | null>(null);
  const { searchFund } = useTauriCommands();

  useEffect(() => {
    if (code.length !== 6) {
      setFund(null);
      setError(null);
      return;
    }

    const timer = setTimeout(async () => {
      setLoading(true);
      setError(null);
      try {
        const result = await searchFund(code);
        setFund(result);
      } catch (err) {
        const message = String(err);
        setError(message);
        setFund(null);
        if (showToast) {
          showToast(message, "error");
        }
      } finally {
        setLoading(false);
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [code, searchFund, showToast]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value.replace(/\D/g, "").slice(0, 6);
    setCode(value);
  };

  return (
    <div className="fund-search">
      <div className="search-bar">
        <input
          type="text"
          value={code}
          onChange={handleInputChange}
          placeholder="输入6位基金代码"
          maxLength={6}
          className="search-input"
          disabled={loading}
        />
        {loading && <span className="loading-spinner">🔄</span>}
      </div>

      {error && <div className="error-message">{error}</div>}
      {!loading && !error && code.length > 0 && code.length < 6 && (
        <div className="hint-message">请输入完整的6位基金代码</div>
      )}

      <FundInfoCard
        fund={fund}
        lists={lists}
        onListsChange={onListsChange}
        showToast={showToast}
      />
    </div>
  );
};
