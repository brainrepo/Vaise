import type { CheatSheet, CheatSheetsPayload } from "./types";

const isTauriRuntime =
  typeof window !== "undefined" &&
  Object.prototype.hasOwnProperty.call(window, "__TAURI_INTERNALS__");

const fallbackPayload: CheatSheetsPayload = {
  directory: "~/Library/Application Support/com.vaise.app/cheatsheets",
  sheets: [
    {
      id: "vscode",
      name: "VS Code",
      icon: "code",
      tags: ["editor", "typescript"],
      entries: [
        {
          id: "command-palette",
          name: "Command Palette",
          action: "Open the command palette",
          command: ["Cmd", "Shift", "P"],
          aliases: ["palette", "commands"]
        },
        {
          id: "quick-open",
          name: "Quick Open",
          action: "Search files in the workspace",
          command: ["Cmd", "P"],
          aliases: ["go to file", "open file"]
        }
      ]
    },
    {
      id: "figma",
      name: "Figma",
      icon: "fig",
      tags: ["design"],
      entries: [
        {
          id: "frame",
          name: "Frame Tool",
          action: "Create a frame",
          command: ["F"],
          aliases: ["artboard"]
        },
        {
          id: "scale",
          name: "Scale Tool",
          action: "Scale a selection",
          command: ["K"]
        }
      ]
    },
    {
      id: "tmux",
      name: "tmux",
      icon: "mux",
      tags: ["terminal", "multiplexer"],
      entries: [
        {
          id: "new-session",
          name: "New Session",
          action: "Start a new tmux session",
          commandText: "tmux new-session",
          commandSequence: [["tmux"], ["new-session"]],
          aliases: ["tmux new", "session start"],
          tags: ["sessions", "cli"]
        },
        {
          id: "list-sessions",
          name: "List Sessions",
          action: "Show all tmux sessions",
          commandText: "tmux list-sessions",
          commandSequence: [["tmux"], ["list-sessions"]],
          aliases: ["tmux ls", "sessions list"],
          tags: ["sessions", "cli"]
        },
        {
          id: "attach-session",
          name: "Attach Session",
          action: "Attach to the last or named tmux session",
          commandText: "tmux attach-session -t <name>",
          commandSequence: [["tmux"], ["attach-session", "-t", "<name>"]],
          aliases: ["attach", "tmux attach", "reattach"],
          tags: ["sessions", "cli"]
        },
        {
          id: "new-window",
          name: "New Window",
          action: "Create a new tmux window",
          commandText: "new-window",
          commandSequence: [["Ctrl", "B"], ["C"]],
          aliases: ["window", "create window"]
        },
        {
          id: "split-horizontal",
          name: "Split Horizontal",
          action: "Split the current pane horizontally",
          commandText: "split-window -v",
          commandSequence: [["Ctrl", "B"], ["\""]],
          aliases: ["pane split", "horizontal split"]
        },
        {
          id: "split-vertical",
          name: "Split Vertical",
          action: "Split the current pane vertically",
          commandText: "split-window -h",
          commandSequence: [["Ctrl", "B"], ["%"]],
          aliases: ["vertical split", "pane vertical"]
        },
        {
          id: "next-window",
          name: "Next Window",
          action: "Move to the next tmux window",
          commandText: "next-window",
          commandSequence: [["Ctrl", "B"], ["n"]],
          aliases: ["cycle window", "forward window"]
        },
        {
          id: "previous-window",
          name: "Previous Window",
          action: "Move to the previous tmux window",
          commandText: "previous-window",
          commandSequence: [["Ctrl", "B"], ["p"]],
          aliases: ["back window", "prev window"],
          tags: ["windows"]
        },
        {
          id: "list-windows",
          name: "List Windows",
          action: "Open the tmux window list",
          commandText: "list-windows",
          commandSequence: [["Ctrl", "B"], ["w"]],
          aliases: ["window list", "choose window"],
          tags: ["windows"]
        },
        {
          id: "last-window",
          name: "Last Window",
          action: "Toggle the last active window",
          commandText: "last-window",
          commandSequence: [["Ctrl", "B"], ["l"]],
          aliases: ["previous active window", "toggle window"],
          tags: ["windows"]
        },
        {
          id: "rename-window",
          name: "Rename Window",
          action: "Rename the current tmux window",
          commandText: "rename-window",
          commandSequence: [["Ctrl", "B"], [","]],
          aliases: ["window name", "rename pane group"]
        },
        {
          id: "detach-session",
          name: "Detach Session",
          action: "Detach from the current tmux session",
          commandText: "detach-client",
          commandSequence: [["Ctrl", "B"], ["D"]],
          aliases: ["detach", "leave session"]
        },
        {
          id: "choose-sessions",
          name: "Choose Session",
          action: "Open the tmux session switcher",
          commandText: "choose-tree -s",
          commandSequence: [["Ctrl", "B"], ["s"]],
          aliases: ["session list", "switch session"],
          tags: ["sessions"]
        },
        {
          id: "previous-session",
          name: "Previous Session",
          action: "Move to the previous tmux session",
          commandText: "switch-client -p",
          commandSequence: [["Ctrl", "B"], ["("]],
          aliases: ["prev session", "back session"],
          tags: ["sessions"]
        },
        {
          id: "next-session",
          name: "Next Session",
          action: "Move to the next tmux session",
          commandText: "switch-client -n",
          commandSequence: [["Ctrl", "B"], [")"]],
          aliases: ["forward session", "cycle session"],
          tags: ["sessions"]
        },
        {
          id: "copy-mode",
          name: "Copy Mode",
          action: "Enter tmux copy mode",
          commandText: "copy-mode",
          commandSequence: [["Ctrl", "B"], ["["]],
          aliases: ["scrollback", "copy"]
        },
        {
          id: "last-pane",
          name: "Last Pane",
          action: "Jump to the previously active pane",
          commandText: "last-pane",
          commandSequence: [["Ctrl", "B"], [";"]],
          aliases: ["previous pane", "toggle pane"]
        },
        {
          id: "move-pane-left",
          name: "Move Pane Left",
          action: "Focus the pane on the left",
          commandText: "select-pane -L",
          commandSequence: [["Ctrl", "B"], ["Left"]],
          aliases: ["pane left", "focus left"]
        },
        {
          id: "move-pane-right",
          name: "Move Pane Right",
          action: "Focus the pane on the right",
          commandText: "select-pane -R",
          commandSequence: [["Ctrl", "B"], ["Right"]],
          aliases: ["pane right", "focus right"]
        },
        {
          id: "next-pane",
          name: "Next Pane",
          action: "Switch to the next tmux pane",
          commandText: "select-pane -t :.+" ,
          commandSequence: [["Ctrl", "B"], ["o"]],
          aliases: ["cycle pane", "pane next"],
          tags: ["panes"]
        },
        {
          id: "show-pane-numbers",
          name: "Show Pane Numbers",
          action: "Temporarily show pane numbers",
          commandText: "display-panes",
          commandSequence: [["Ctrl", "B"], ["q"]],
          aliases: ["pane ids", "pane numbers"],
          tags: ["panes"]
        },
        {
          id: "toggle-pane-zoom",
          name: "Toggle Pane Zoom",
          action: "Zoom or unzoom the current pane",
          commandText: "resize-pane -Z",
          commandSequence: [["Ctrl", "B"], ["z"]],
          aliases: ["maximize pane", "zoom pane"],
          tags: ["panes"]
        },
        {
          id: "kill-pane",
          name: "Kill Pane",
          action: "Close the current pane",
          commandText: "kill-pane",
          commandSequence: [["Ctrl", "B"], ["x"]],
          aliases: ["close pane", "delete pane"],
          tags: ["panes"]
        },
        {
          id: "resize-pane-left",
          name: "Resize Pane Left",
          action: "Resize the current pane to the left",
          commandText: "resize-pane -L",
          commandSequence: [["Ctrl", "B"], ["Ctrl", "Left"]],
          aliases: ["shrink pane", "resize left"]
        },
        {
          id: "resize-pane-right",
          name: "Resize Pane Right",
          action: "Resize the current pane to the right",
          commandText: "resize-pane -R",
          commandSequence: [["Ctrl", "B"], ["Ctrl", "Right"]],
          aliases: ["grow pane", "resize right"]
        },
        {
          id: "pane-layout",
          name: "Toggle Pane Layout",
          action: "Cycle through pane layouts",
          commandText: "select-layout",
          commandSequence: [["Ctrl", "B"], ["Space"]],
          aliases: ["layout", "pane layout"],
          tags: ["panes", "layout"]
        },
        {
          id: "paste-buffer",
          name: "Paste Buffer",
          action: "Paste the current tmux buffer",
          commandText: "paste-buffer",
          commandSequence: [["Ctrl", "B"], ["]"]],
          aliases: ["paste", "buffer paste"],
          tags: ["copy mode", "buffers"]
        },
        {
          id: "command-prompt",
          name: "Command Prompt",
          action: "Open the tmux command prompt",
          commandText: "command-prompt",
          commandSequence: [["Ctrl", "B"], [":"]],
          aliases: ["tmux command", "prompt"],
          tags: ["misc"]
        },
        {
          id: "list-keys",
          name: "List Keys",
          action: "Show all tmux key bindings",
          commandText: "list-keys",
          commandSequence: [["Ctrl", "B"], ["?"]],
          aliases: ["help", "bindings", "shortcuts"],
          tags: ["help"]
        }
      ]
    }
  ]
};

export const loadCheatSheets = async (): Promise<CheatSheetsPayload> => {
  if (!isTauriRuntime) {
    return fallbackPayload;
  }

  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<CheatSheetsPayload>("load_cheat_sheets");
};

export const registerFocusListener = async (callback: () => void) => {
  if (!isTauriRuntime) {
    return () => undefined;
  }

  const { listen } = await import("@tauri-apps/api/event");
  const unlisten = await listen("vaise://focus-search", callback);
  return () => {
    unlisten();
  };
};

export const hideCurrentWindow = async () => {
  if (!isTauriRuntime) {
    return;
  }

  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  await getCurrentWindow().hide();
};

export const getSheetTags = (sheet: CheatSheet) => sheet.tags ?? [];
