import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import {
  defaultSettings,
  getLaunchContext,
  listPresetsForPath,
  loadSettings,
  runConversion,
  saveSettings,
} from "./tauri";
import type {
  ConversionError,
  ConversionPreset,
  ConversionResult,
  LaunchContext,
  ProgressEvent,
  Settings,
} from "./types";

const emptyLaunchContext: LaunchContext = {
  mode: "gui",
  inputPath: null,
};

function App() {
  const [settings, setSettings] = useState<Settings>(defaultSettings);
  const [launchContext, setLaunchContext] = useState<LaunchContext>(emptyLaunchContext);
  const [presets, setPresets] = useState<ConversionPreset[]>([]);
  const [selectedPresetId, setSelectedPresetId] = useState<string>("");
  const [isConverting, setIsConverting] = useState(false);
  const [progress, setProgress] = useState<ProgressEvent | null>(null);
  const [result, setResult] = useState<ConversionResult | null>(null);
  const [error, setError] = useState<ConversionError | null>(null);

  useEffect(() => {
    const boot = async () => {
      const [loadedSettings, context] = await Promise.all([
        loadSettings().catch(() => defaultSettings),
        getLaunchContext(),
      ]);
      setSettings(loadedSettings);
      setLaunchContext(context);
      const availablePresets = await listPresetsForPath(context.inputPath);
      setPresets(availablePresets);
      setSelectedPresetId(availablePresets[0]?.id ?? "");
    };

    void boot();
  }, []);

  useEffect(() => {
    let dispose: null | (() => void) = null;

    const subscribe = async () => {
      const unlisten = await listen<ProgressEvent>("conversion-progress", (event) => {
        setProgress(event.payload);
      });
      dispose = unlisten;
    };

    void subscribe();
    return () => {
      dispose?.();
    };
  }, []);

  const selectedPreset = presets.find((preset) => preset.id === selectedPresetId) ?? null;

  const onSaveSettings = async (nextSettings: Settings) => {
    setSettings(nextSettings);
    if (nextSettings.rememberSettings) {
      await saveSettings(nextSettings);
    }
  };

  const handleConvert = async () => {
    if (!launchContext.inputPath || !selectedPresetId) {
      return;
    }

    setError(null);
    setResult(null);
    setProgress({ stage: "queued", message: "Queued for conversion" });
    setIsConverting(true);

    try {
      const conversionResult = await runConversion({
        inputPath: launchContext.inputPath,
        presetId: selectedPresetId,
        qualityPreset: settings.defaultQualityPreset,
        openFolderAfterConvert: settings.openFolderAfterConvert,
      });
      setResult(conversionResult);
    } catch (reason) {
      const message = reason instanceof Error ? reason.message : String(reason);
      setError({
        message: "Conversion failed",
        details: message,
      });
    } finally {
      setIsConverting(false);
    }
  };

  return (
    <main className="app-shell">
      <section className="hero-card">
        <div className="eyebrow">ConvertIT</div>
        <h1>Quick conversion for Windows files</h1>
        <p className="subtitle">
          Classic Explorer integration for one-click conversions, plus a lightweight advanced
          window when you need a little more control.
        </p>
      </section>

      <section className="panel-grid">
        <article className="panel">
          <h2>Source</h2>
          <div className="source-box">
            <span className="meta-label">Launch mode</span>
            <strong>{launchContext.mode === "advanced" ? "Advanced" : "GUI"}</strong>
            <span className="meta-label">Input file</span>
            <code>{launchContext.inputPath ?? "No input provided"}</code>
          </div>
        </article>

        <article className="panel">
          <h2>Convert to</h2>
          <label className="field">
            <span>Preset</span>
            <select
              value={selectedPresetId}
              onChange={(event) => setSelectedPresetId(event.target.value)}
              disabled={isConverting || presets.length === 0}
            >
              {presets.length === 0 ? <option value="">No compatible targets</option> : null}
              {presets.map((preset) => (
                <option value={preset.id} key={preset.id}>
                  {preset.label}
                </option>
              ))}
            </select>
          </label>
          <div className="preset-hint">
            {selectedPreset ? (
              <>
                <span>{selectedPreset.description}</span>
                <code>{selectedPreset.targetExtension}</code>
              </>
            ) : (
              <span>Select a supported file to see conversion targets.</span>
            )}
          </div>
        </article>

        <article className="panel">
          <h2>Quality</h2>
          <div className="segmented-control">
            {(["fast", "balanced", "best"] as const).map((quality) => (
              <button
                key={quality}
                className={quality === settings.defaultQualityPreset ? "active" : ""}
                onClick={() =>
                  void onSaveSettings({
                    ...settings,
                    defaultQualityPreset: quality,
                  })
                }
                disabled={isConverting}
                type="button"
              >
                {quality}
              </button>
            ))}
          </div>
          <label className="checkbox">
            <input
              type="checkbox"
              checked={settings.openFolderAfterConvert}
              onChange={(event) =>
                void onSaveSettings({
                  ...settings,
                  openFolderAfterConvert: event.target.checked,
                })
              }
              disabled={isConverting}
            />
            <span>Open output folder after conversion</span>
          </label>
          <label className="checkbox">
            <input
              type="checkbox"
              checked={settings.rememberSettings}
              onChange={(event) =>
                void onSaveSettings({
                  ...settings,
                  rememberSettings: event.target.checked,
                })
              }
              disabled={isConverting}
            />
            <span>Remember advanced settings</span>
          </label>
        </article>
      </section>

      <section className="status-strip">
        <div>
          <span className="meta-label">Status</span>
          <strong>{progress?.message ?? "Idle"}</strong>
        </div>
        <button
          className="primary-action"
          type="button"
          onClick={handleConvert}
          disabled={!launchContext.inputPath || !selectedPresetId || isConverting}
        >
          {isConverting ? "Converting..." : "Start conversion"}
        </button>
      </section>

      {result ? (
        <section className="feedback success">
          <h2>Finished</h2>
          <p>{result.outputPath}</p>
          <small>{result.logSummary.join(" · ")}</small>
        </section>
      ) : null}

      {error ? (
        <section className="feedback error">
          <h2>{error.message}</h2>
          <p>{error.details}</p>
        </section>
      ) : null}
    </main>
  );
}

export default App;