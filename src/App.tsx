import {
  KeyboardEvent,
  useDeferredValue,
  useEffect,
  useMemo,
  useRef,
  useState
} from "react";
import appIcon from "./assets/app-icon.svg";
import { rankItems } from "./lib/search";
import { getSheetTags, loadCheatSheets, registerFocusListener } from "./lib/tauri";
import type { CheatSheet, CheatSheetEntry } from "./lib/types";

const formatCommand = (entry: CheatSheetEntry) => {
  if (entry.command?.length) {
    return [entry.command];
  }

  return entry.commandSequence ?? [];
};

const stringifyEntryCommand = (entry: CheatSheetEntry) => {
  if (entry.commandText?.trim()) {
    return entry.commandText.trim();
  }

  const combos = formatCommand(entry);
  if (combos.length === 0) {
    return "";
  }

  return combos.map((combo) => combo.join("+")).join(", ");
};

const Keycap = ({ label }: { label: string }) => <kbd className="keycap">{label}</kbd>;

const EmptyState = ({ label }: { label: string }) => (
  <div className="empty-state">
    <p>{label}</p>
  </div>
);

type RankedSheet = { item: CheatSheet; score: number };
type RankedEntry = { item: CheatSheetEntry; score: number };

export default function App() {
  const [payload, setPayload] = useState<{ directory: string; sheets: CheatSheet[] } | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedSheet, setSelectedSheet] = useState<CheatSheet | null>(null);
  const [sheetQuery, setSheetQuery] = useState("");
  const [entryQuery, setEntryQuery] = useState("");
  const [activeIndex, setActiveIndex] = useState(0);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const resultsListRef = useRef<HTMLDivElement>(null);

  const deferredSheetQuery = useDeferredValue(sheetQuery);
  const deferredEntryQuery = useDeferredValue(entryQuery);

  useEffect(() => {
    let mounted = true;

    const bootstrap = async () => {
      try {
        const nextPayload = await loadCheatSheets();
        if (!mounted) {
          return;
        }

        setPayload(nextPayload);
        setError(null);
      } catch (loadError) {
        if (!mounted) {
          return;
        }

        setError(loadError instanceof Error ? loadError.message : "Unable to load cheat sheets.");
      } finally {
        if (mounted) {
          setIsLoading(false);
        }
      }
    };

    void bootstrap();

    return () => {
      mounted = false;
    };
  }, []);

  useEffect(() => {
    const bind = async () => {
      const cleanup = await registerFocusListener(() => {
        searchInputRef.current?.focus();
        searchInputRef.current?.select();
      });

      return cleanup;
    };

    let dispose: (() => void) | undefined;
    void bind().then((cleanup) => {
      dispose = cleanup;
    });

    return () => {
      dispose?.();
    };
  }, []);

  useEffect(() => {
    const onWindowKeyDown = (event: globalThis.KeyboardEvent) => {
      if (event.ctrlKey && event.key === "/") {
        event.preventDefault();
        searchInputRef.current?.focus();
        searchInputRef.current?.select();
      }
    };

    window.addEventListener("keydown", onWindowKeyDown);
    return () => {
      window.removeEventListener("keydown", onWindowKeyDown);
    };
  }, []);

  useEffect(() => {
    setActiveIndex(0);
  }, [deferredSheetQuery, deferredEntryQuery, selectedSheet]);

  useEffect(() => {
    const activeRow = resultsListRef.current?.querySelector<HTMLElement>(".result-row.active");
    activeRow?.scrollIntoView({ block: "nearest" });
  }, [activeIndex, selectedSheet, deferredSheetQuery, deferredEntryQuery]);

  const sheetResults = useMemo<RankedSheet[]>(() => {
    if (!payload) {
      return [];
    }

    return rankItems(payload.sheets, deferredSheetQuery, (sheet) => [
      sheet.name,
      ...getSheetTags(sheet),
      ...sheet.entries.flatMap((entry) => [entry.name, entry.action, ...(entry.aliases ?? [])])
    ]);
  }, [deferredSheetQuery, payload]);

  const entryResults = useMemo<RankedEntry[]>(() => {
    if (!selectedSheet) {
      return [];
    }

    return rankItems(selectedSheet.entries, deferredEntryQuery, (entry) => [
      entry.name,
      entry.action,
      ...(entry.commandText ? [entry.commandText] : []),
      ...(entry.aliases ?? []),
      ...(entry.tags ?? []),
      ...(entry.command ?? []),
      ...(entry.commandSequence?.flat() ?? [])
    ]);
  }, [deferredEntryQuery, selectedSheet]);

  const visibleList: Array<RankedSheet | RankedEntry> = selectedSheet ? entryResults : sheetResults;

  const copyEntryCommand = async (entry: CheatSheetEntry) => {
    const text = stringifyEntryCommand(entry);
    if (!text) {
      return;
    }

    await navigator.clipboard.writeText(text);
  };

  const activateCurrent = async () => {
    const current = visibleList[activeIndex]?.item;
    if (!current) {
      return;
    }

    if (selectedSheet) {
      await copyEntryCommand(current as CheatSheetEntry);
      return;
    }

    setSelectedSheet(current as CheatSheet);
    setEntryQuery("");
  };

  const moveSelection = (direction: 1 | -1) => {
    setActiveIndex((current) => {
      const next = current + direction;
      if (next < 0) {
        return Math.max(visibleList.length - 1, 0);
      }

      if (next >= visibleList.length) {
        return 0;
      }

      return next;
    });
  };

  const onKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "ArrowDown" || event.key === "Tab" || (event.ctrlKey && event.key === "n")) {
      event.preventDefault();
      moveSelection(1);
      return;
    }

    if (
      event.key === "ArrowUp" ||
      (event.shiftKey && event.key === "Tab") ||
      (event.ctrlKey && event.key === "p")
    ) {
      event.preventDefault();
      moveSelection(-1);
      return;
    }

    if (event.key === "Enter") {
      event.preventDefault();
      void activateCurrent();
      return;
    }

    if (event.key === "Backspace" && !inputValue && selectedSheet) {
      event.preventDefault();
      setSelectedSheet(null);
      setEntryQuery("");
      return;
    }

    if (event.key === "Escape") {
      event.preventDefault();
      if (selectedSheet) {
        setSelectedSheet(null);
        setEntryQuery("");
      } else {
        setSheetQuery("");
      }
    }
  };

  const inputValue = selectedSheet ? entryQuery : sheetQuery;
  const inputPlaceholder = selectedSheet
    ? `${selectedSheet.name}`
    : "Search cheat sheets";

  return (
    <main className="shell">
      <section className="panel">
        <div className="search-shell">
          <header className="search-header">
            <div className="header-left">
              <img alt="Vaise" className="brand-mark" src={appIcon} />
              <span className="scope-label">{selectedSheet?.name ?? "Search"}</span>
            </div>
            <div className="header-right">
              <span className="scope-pill">Ctrl+/</span>
            </div>
          </header>
          <label className="search-field">
            <span className="visually-hidden">Search</span>
            <span className="search-icon">⌕</span>
            <input
              onChange={(event) => {
                if (selectedSheet) {
                  setEntryQuery(event.target.value);
                } else {
                  setSheetQuery(event.target.value);
                }
              }}
              onKeyDown={onKeyDown}
              placeholder={inputPlaceholder}
              ref={searchInputRef}
              value={inputValue}
            />
          </label>
          <div className="search-hint">
            <span>{selectedSheet ? "Inside cheat sheet" : "Cheat sheets"}</span>
            <span>Enter open</span>
            <span>Tab move</span>
            <span>Esc back</span>
          </div>
        </div>

        {isLoading ? <EmptyState label="Loading cheat sheets..." /> : null}
        {error ? <EmptyState label={error} /> : null}

        {!isLoading && !error ? (
          <section className="results">
            <div className="results-header">
              <span className="eyebrow">{selectedSheet ? "Commands" : "Cheat sheets"}</span>
              <span className="results-count">
                {selectedSheet
                  ? `${entryResults.length} commands`
                  : `${sheetResults.length} sheets available`}
              </span>
            </div>

            <div className="results-list" ref={resultsListRef}>
              {visibleList.length === 0 ? (
                <EmptyState label="No results for this search." />
              ) : null}

              {selectedSheet
                ? entryResults.map(({ item, score }, index) => (
                    <button
                      key={item.id}
                      className={`result-row ${index === activeIndex ? "active" : ""}`}
                      onClick={() => {
                        void copyEntryCommand(item);
                      }}
                      onMouseEnter={() => setActiveIndex(index)}
                      type="button"
                    >
                      <span className="row-icon">⌘</span>
                      <div className="result-main">
                        <strong>{item.name}</strong>
                        <span>{item.action}</span>
                        {item.commandText ? <code className="command-text">{item.commandText}</code> : null}
                      </div>
                      <div className="shortcut-stack" aria-label={`score ${score}`}>
                        {formatCommand(item).map((combo, comboIndex) => (
                          <div className="shortcut-group" key={`${item.id}-${comboIndex}`}>
                            {combo.map((part) => (
                              <Keycap key={`${item.id}-${comboIndex}-${part}`} label={part} />
                            ))}
                          </div>
                        ))}
                      </div>
                    </button>
                  ))
                : sheetResults.map(({ item, score }, index) => (
                    <button
                      key={item.id}
                      className={`result-row ${index === activeIndex ? "active" : ""}`}
                      onClick={() => {
                        setSelectedSheet(item);
                        setEntryQuery("");
                      }}
                      onMouseEnter={() => setActiveIndex(index)}
                      type="button"
                    >
                      <span className="row-icon">◦</span>
                      <div className="result-main">
                        <strong>{item.name}</strong>
                        <span>{item.entries.length} shortcuts</span>
                      </div>
                      <div className="result-side">
                        <span>{getSheetTags(item).join(" • ")}</span>
                        <small>{score > 500 ? "exact" : "match"}</small>
                      </div>
                    </button>
                  ))}
            </div>
          </section>
        ) : null}

        <footer className="footer-strip">
          <span>{payload?.directory ?? "No storage path yet"}</span>
          <span>Ctrl+/ focus search</span>
        </footer>
      </section>
    </main>
  );
}
