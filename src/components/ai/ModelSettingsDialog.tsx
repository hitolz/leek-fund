import React, { useEffect, useMemo, useState } from "react";
import {
  Box,
  Check,
  CheckCircle2,
  ChevronDown,
  CircleAlert,
  Cloud,
  Eye,
  EyeOff,
  KeyRound,
  LoaderCircle,
  Save,
  Server,
  Settings2,
  SlidersHorizontal,
  Wifi,
  X,
} from "lucide-react";
import {
  getLlmConfig,
  saveLlmConfig,
  testLlmConfig,
} from "../../services/ai-copilot";
import { LlmConfig, LlmProvider } from "../../types/ai-copilot";

interface ModelSettingsDialogProps {
  open: boolean;
  onClose: () => void;
  onSaved: (config: LlmConfig) => void;
}

type ConnectionPreset = "ollama" | "openai" | "claude" | "updev" | "custom";

interface PresetDefinition {
  value: ConnectionPreset;
  label: string;
  caption: string;
  icon: React.ReactNode;
  config?: Pick<
    LlmConfig,
    "provider" | "base_url" | "model" | "max_tokens" | "temperature"
  >;
}

interface ConnectionResult {
  kind: "success" | "error";
  message: string;
}

const PROVIDERS: Array<{ value: LlmProvider; label: string }> = [
  { value: "openai_compatible", label: "OpenAI 兼容" },
  { value: "openai", label: "OpenAI" },
  { value: "claude", label: "Claude" },
  { value: "claude_compatible", label: "Claude 兼容" },
];

const PRESETS: PresetDefinition[] = [
  {
    value: "ollama",
    label: "Ollama",
    caption: "本机运行",
    icon: <Box size={18} />,
    config: {
      provider: "openai_compatible",
      base_url: "http://127.0.0.1:11434/v1",
      model: "qwen3:latest",
      max_tokens: 2048,
      temperature: 0.3,
    },
  },
  {
    value: "openai",
    label: "OpenAI",
    caption: "官方接口",
    icon: <Cloud size={18} />,
    config: {
      provider: "openai",
      base_url: "https://api.openai.com/v1",
      model: "gpt-4o-mini",
      max_tokens: 4096,
      temperature: 0.3,
    },
  },
  {
    value: "claude",
    label: "Claude",
    caption: "官方接口",
    icon: <Cloud size={18} />,
    config: {
      provider: "claude",
      base_url: "https://api.anthropic.com/v1",
      model: "claude-sonnet-4-20250514",
      max_tokens: 4096,
      temperature: 0.3,
    },
  },
  {
    value: "updev",
    label: "Updev",
    caption: "Claude 兼容",
    icon: <Server size={18} />,
    config: {
      provider: "claude_compatible",
      base_url: "https://oneapi.updev.cn/v1",
      model: "mimo-v2.5-pro",
      max_tokens: 4096,
      temperature: 0.3,
    },
  },
  {
    value: "custom",
    label: "自定义",
    caption: "其他服务",
    icon: <Settings2 size={18} />,
  },
];

const UPDEV_MODELS = [
  "mimo-v2.5-pro",
  "MiniMax-M3",
  "glm-latest[1M]",
  "claude-opus-4-6",
];

const DEFAULT_CONFIG: LlmConfig = {
  provider: "openai_compatible",
  base_url: "http://127.0.0.1:11434/v1",
  model: "qwen3:latest",
  max_tokens: 2048,
  temperature: 0.3,
  has_api_key: false,
  api_key: "",
};

function matchPreset(config: LlmConfig): ConnectionPreset {
  const matched = PRESETS.find(
    (item) =>
      item.config?.provider === config.provider &&
      item.config.base_url === config.base_url &&
      item.config.model === config.model
  );
  return matched?.value ?? "custom";
}

function providerNeedsKey(provider: LlmProvider): boolean {
  return provider !== "openai_compatible";
}

function validateConfig(config: LlmConfig): string | null {
  const baseUrl = config.base_url.trim();
  if (!baseUrl) return "请输入服务地址";
  try {
    const parsed = new URL(baseUrl);
    if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
      return "服务地址需要以 http:// 或 https:// 开头";
    }
  } catch {
    return "服务地址格式不正确";
  }
  if (!config.model.trim()) return "请输入模型名称";
  if (
    providerNeedsKey(config.provider) &&
    !config.has_api_key &&
    !config.api_key?.trim()
  ) {
    return "请输入 API Key";
  }
  if (config.max_tokens < 1 || config.max_tokens > 32768) {
    return "最大输出需在 1 到 32768 之间";
  }
  return null;
}

export const ModelSettingsDialog: React.FC<ModelSettingsDialogProps> = ({
  open,
  onClose,
  onSaved,
}) => {
  const [config, setConfig] = useState<LlmConfig>(DEFAULT_CONFIG);
  const [loadingConfig, setLoadingConfig] = useState(false);
  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);
  const [result, setResult] = useState<ConnectionResult | null>(null);
  const [preset, setPreset] = useState<ConnectionPreset>("ollama");
  const [showApiKey, setShowApiKey] = useState(false);
  const [showAdvanced, setShowAdvanced] = useState(false);

  useEffect(() => {
    if (!open) return;
    setResult(null);
    setShowApiKey(false);
    setLoadingConfig(true);
    getLlmConfig()
      .then((current) => {
        setConfig({ ...current, api_key: "" });
        setPreset(matchPreset(current));
      })
      .catch(() => {
        setConfig(DEFAULT_CONFIG);
        setPreset("ollama");
      })
      .finally(() => setLoadingConfig(false));
  }, [open]);

  useEffect(() => {
    if (!open) return;
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape" && !saving && !testing) onClose();
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, onClose, saving, testing]);

  const validationError = useMemo(() => validateConfig(config), [config]);
  const isLocal =
    config.base_url.includes("127.0.0.1") ||
    config.base_url.includes("localhost") ||
    config.base_url.includes("[::1]");

  if (!open) return null;

  const update = <K extends keyof LlmConfig>(key: K, value: LlmConfig[K]) => {
    setConfig((current) => ({ ...current, [key]: value }));
    if (key === "provider" || key === "base_url" || key === "model") {
      setPreset("custom");
    }
    setResult(null);
  };

  const applyPreset = (value: ConnectionPreset) => {
    setPreset(value);
    const selected = PRESETS.find((item) => item.value === value)?.config;
    if (!selected) {
      setResult(null);
      return;
    }
    setConfig((current) => {
      const sameProvider = current.provider === selected.provider;
      return {
        ...current,
        ...selected,
        api_key: sameProvider ? current.api_key : "",
        has_api_key: sameProvider ? current.has_api_key : false,
      };
    });
    setResult(null);
  };

  const handleTest = async () => {
    if (validationError) {
      setResult({ kind: "error", message: validationError });
      return;
    }
    setTesting(true);
    setResult(null);
    try {
      const response = await testLlmConfig(config);
      setResult({
        kind: response.success ? "success" : "error",
        message: response.message,
      });
    } catch (error) {
      setResult({ kind: "error", message: String(error) });
    } finally {
      setTesting(false);
    }
  };

  const handleSave = async () => {
    if (validationError) {
      setResult({ kind: "error", message: validationError });
      return;
    }
    setSaving(true);
    setResult(null);
    try {
      const response = await saveLlmConfig(config);
      if (!response.success) throw new Error(response.message);
      onSaved({
        ...config,
        base_url: config.base_url.trim().replace(/\/$/, ""),
        model: config.model.trim(),
        has_api_key: Boolean(config.api_key?.trim()) || config.has_api_key,
        api_key: "",
      });
      onClose();
    } catch (error) {
      setResult({ kind: "error", message: String(error) });
    } finally {
      setSaving(false);
    }
  };

  const selectedPreset = PRESETS.find((item) => item.value === preset);

  return (
    <div
      className="copilot-dialog-backdrop"
      role="presentation"
      onMouseDown={() => !saving && !testing && onClose()}
    >
      <section
        className="copilot-dialog model-connection-dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby="model-settings-title"
        onMouseDown={(event) => event.stopPropagation()}
      >
        <header className="model-dialog-header">
          <div className="model-dialog-heading">
            <span className="model-dialog-icon"><Wifi size={18} /></span>
            <div>
              <span className="copilot-eyebrow">AI CONNECTION</span>
              <h3 id="model-settings-title">模型连接</h3>
            </div>
          </div>
          <button
            type="button"
            className="copilot-icon-button"
            onClick={onClose}
            title="关闭"
            disabled={saving || testing}
          >
            <X size={17} />
          </button>
        </header>

        <div className="model-dialog-layout">
          <aside className="model-preset-panel" aria-label="连接方式">
            <div className="model-section-label">连接方式</div>
            <div className="model-preset-list">
              {PRESETS.map((item) => (
                <button
                  type="button"
                  key={item.value}
                  className={preset === item.value ? "active" : ""}
                  onClick={() => applyPreset(item.value)}
                >
                  <span className="model-preset-icon">{item.icon}</span>
                  <span>
                    <strong>{item.label}</strong>
                    <small>{item.caption}</small>
                  </span>
                  {preset === item.value && <Check size={15} />}
                </button>
              ))}
            </div>

            <div className="model-connection-summary">
              <span className={`model-status-dot ${result?.kind ?? "idle"}`} />
              <div>
                <small>当前配置</small>
                <strong>{selectedPreset?.label ?? "自定义"}</strong>
                <span>{isLocal ? "本地连接" : "云端连接"}</span>
              </div>
            </div>
          </aside>

          <form
            className="model-config-panel"
            onSubmit={(event) => {
              event.preventDefault();
              void handleSave();
            }}
          >
            {loadingConfig ? (
              <div className="model-config-loading">
                <LoaderCircle className="spin" size={20} />
                <span>正在读取配置</span>
              </div>
            ) : (
              <>
                <div className="model-config-heading">
                  <div>
                    <span className="copilot-eyebrow">CONFIGURATION</span>
                    <h4>{selectedPreset?.label ?? "自定义"} 配置</h4>
                  </div>
                  <span className={`model-location-badge ${isLocal ? "local" : "cloud"}`}>
                    {isLocal ? <Server size={13} /> : <Cloud size={13} />}
                    {isLocal ? "本地" : "云端"}
                  </span>
                </div>

                <div className="model-form-grid">
                  <label className="copilot-field model-field-wide">
                    <span>接口类型</span>
                    <select
                      className="copilot-select"
                      value={config.provider}
                      onChange={(event) =>
                        update("provider", event.target.value as LlmProvider)
                      }
                    >
                      {PROVIDERS.map((provider) => (
                        <option value={provider.value} key={provider.value}>
                          {provider.label}
                        </option>
                      ))}
                    </select>
                  </label>

                  <label className="copilot-field model-field-wide">
                    <span>服务地址</span>
                    <div className="model-input-with-icon">
                      <Server size={15} />
                      <input
                        value={config.base_url}
                        onChange={(event) => update("base_url", event.target.value)}
                        placeholder="http://127.0.0.1:11434/v1"
                        spellCheck={false}
                      />
                    </div>
                  </label>

                  <label className="copilot-field model-field-wide">
                    <span>模型名称</span>
                    <div className="model-input-with-icon">
                      <Box size={15} />
                      <input
                        list={preset === "updev" ? "updev-models" : undefined}
                        value={config.model}
                        onChange={(event) => update("model", event.target.value)}
                        placeholder="输入模型 ID"
                        spellCheck={false}
                      />
                    </div>
                    <datalist id="updev-models">
                      {UPDEV_MODELS.map((model) => (
                        <option value={model} key={model} />
                      ))}
                    </datalist>
                  </label>

                  <label className="copilot-field model-field-wide">
                    <span className="model-field-label">
                      API Key
                      {config.has_api_key && !config.api_key && (
                        <em><CheckCircle2 size={12} /> 已保存</em>
                      )}
                    </span>
                    <div className="model-input-with-icon model-secret-input">
                      <KeyRound size={15} />
                      <input
                        type={showApiKey ? "text" : "password"}
                        value={config.api_key ?? ""}
                        onChange={(event) => update("api_key", event.target.value)}
                        placeholder={
                          config.has_api_key
                            ? "留空则继续使用已保存的密钥"
                            : providerNeedsKey(config.provider)
                              ? "输入 API Key"
                              : "本地服务可留空"
                        }
                        autoComplete="new-password"
                        spellCheck={false}
                      />
                      <button
                        type="button"
                        onClick={() => setShowApiKey((current) => !current)}
                        title={showApiKey ? "隐藏 API Key" : "显示 API Key"}
                      >
                        {showApiKey ? <EyeOff size={15} /> : <Eye size={15} />}
                      </button>
                    </div>
                  </label>
                </div>

                <div className={`model-advanced ${showAdvanced ? "open" : ""}`}>
                  <button
                    type="button"
                    className="model-advanced-toggle"
                    onClick={() => setShowAdvanced((current) => !current)}
                    aria-expanded={showAdvanced}
                  >
                    <span><SlidersHorizontal size={14} />生成参数</span>
                    <span>{config.max_tokens} tokens · 温度 {config.temperature.toFixed(1)}</span>
                    <ChevronDown size={15} />
                  </button>
                  {showAdvanced && (
                    <div className="model-advanced-fields">
                      <label className="copilot-field">
                        <span>最大输出</span>
                        <input
                          type="number"
                          min={1}
                          max={32768}
                          value={config.max_tokens}
                          onChange={(event) =>
                            update("max_tokens", Number(event.target.value))
                          }
                        />
                      </label>
                      <label className="copilot-field">
                        <span>温度 <b>{config.temperature.toFixed(1)}</b></span>
                        <input
                          type="range"
                          min={0}
                          max={2}
                          step={0.1}
                          value={config.temperature}
                          onChange={(event) =>
                            update("temperature", Number(event.target.value))
                          }
                        />
                      </label>
                    </div>
                  )}
                </div>

                {result && (
                  <div className={`copilot-config-result ${result.kind}`} role="status">
                    {result.kind === "success" ? (
                      <CheckCircle2 size={16} />
                    ) : (
                      <CircleAlert size={16} />
                    )}
                    <div>
                      <strong>{result.kind === "success" ? "连接可用" : "连接失败"}</strong>
                      <span>{result.message}</span>
                    </div>
                  </div>
                )}

                <footer className="model-dialog-actions">
                  <button
                    type="button"
                    className="copilot-secondary-button"
                    onClick={() => void handleTest()}
                    disabled={testing || saving || Boolean(validationError)}
                    title={validationError ?? "向当前模型发送一条连接测试"}
                  >
                    {testing ? <LoaderCircle className="spin" size={15} /> : <Wifi size={15} />}
                    {testing ? "正在测试" : "测试连接"}
                  </button>
                  <button
                    type="submit"
                    className="copilot-primary-button"
                    disabled={saving || testing || Boolean(validationError)}
                    title={validationError ?? "保存并启用当前配置"}
                  >
                    {saving ? <LoaderCircle className="spin" size={15} /> : <Save size={15} />}
                    {saving ? "正在保存" : "保存并启用"}
                  </button>
                </footer>
              </>
            )}
          </form>
        </div>
      </section>
    </div>
  );
};
