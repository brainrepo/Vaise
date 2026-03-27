#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use image::GenericImageView;
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{ActivationPolicy, AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSStatusWindowLevel, NSWindow, NSWindowCollectionBehavior,
};
#[cfg(target_os = "macos")]
use tauri::WebviewWindow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheatSheetEntry {
    id: String,
    name: String,
    action: String,
    #[serde(default)]
    command: Option<Vec<String>>,
    #[serde(default)]
    command_text: Option<String>,
    #[serde(default)]
    command_sequence: Option<Vec<Vec<String>>>,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheatSheet {
    id: String,
    name: String,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    entries: Vec<CheatSheetEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CheatSheetsPayload {
    directory: String,
    sheets: Vec<CheatSheet>,
}

struct TrayState {
    _icon: TrayIcon,
}

fn tray_icon_image() -> Result<Image<'static>, String> {
    let image_bytes = include_bytes!("../icons/tray.png");
    let decoded = image::load_from_memory(image_bytes)
        .map_err(|error| format!("Unable to decode tray icon PNG: {error}"))?;
    let (width, height) = decoded.dimensions();
    Ok(Image::new_owned(decoded.to_rgba8().into_raw(), width, height))
}

fn default_cheat_sheets() -> Vec<CheatSheet> {
    vec![
        CheatSheet {
            id: "vscode".into(),
            name: "VS Code".into(),
            icon: Some("code".into()),
            tags: vec!["editor".into(), "typescript".into()],
            entries: vec![
                CheatSheetEntry {
                    id: "command-palette".into(),
                    name: "Command Palette".into(),
                    action: "Open the command palette".into(),
                    command: Some(vec!["Cmd".into(), "Shift".into(), "P".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["palette".into(), "commands".into()],
                    tags: vec!["search".into()],
                },
                CheatSheetEntry {
                    id: "quick-open".into(),
                    name: "Quick Open".into(),
                    action: "Search files in the workspace".into(),
                    command: Some(vec!["Cmd".into(), "P".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["go to file".into(), "open file".into()],
                    tags: vec!["files".into()],
                },
                CheatSheetEntry {
                    id: "terminal".into(),
                    name: "Integrated Terminal".into(),
                    action: "Toggle the integrated terminal".into(),
                    command: Some(vec!["Ctrl".into(), "`".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["shell".into(), "console".into()],
                    tags: vec!["terminal".into()],
                },
            ],
        },
        CheatSheet {
            id: "figma".into(),
            name: "Figma".into(),
            icon: Some("fig".into()),
            tags: vec!["design".into(), "ui".into()],
            entries: vec![
                CheatSheetEntry {
                    id: "frame-tool".into(),
                    name: "Frame Tool".into(),
                    action: "Create a new frame".into(),
                    command: Some(vec!["F".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["artboard".into()],
                    tags: vec!["layout".into()],
                },
                CheatSheetEntry {
                    id: "scale-tool".into(),
                    name: "Scale Tool".into(),
                    action: "Scale a selection proportionally".into(),
                    command: Some(vec!["K".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["resize".into()],
                    tags: vec!["selection".into()],
                },
                CheatSheetEntry {
                    id: "format-painter".into(),
                    name: "Copy and Paste Properties".into(),
                    action: "Copy properties, then paste them elsewhere".into(),
                    command: None,
                    command_text: None,
                    command_sequence: Some(vec![
                        vec!["Option".into(), "Cmd".into(), "C".into()],
                        vec!["Option".into(), "Cmd".into(), "V".into()],
                    ]),
                    aliases: vec!["styles".into(), "properties".into()],
                    tags: vec!["style".into()],
                },
            ],
        },
        CheatSheet {
            id: "terminal".into(),
            name: "Terminal".into(),
            icon: Some("prompt".into()),
            tags: vec!["shell".into(), "cli".into()],
            entries: vec![
                CheatSheetEntry {
                    id: "clear-line".into(),
                    name: "Clear Line".into(),
                    action: "Delete from cursor to start of line".into(),
                    command: Some(vec!["Ctrl".into(), "U".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["kill line".into()],
                    tags: vec!["editing".into()],
                },
                CheatSheetEntry {
                    id: "last-command".into(),
                    name: "Last Command Arguments".into(),
                    action: "Insert the last command argument".into(),
                    command: Some(vec!["Esc".into(), ".".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["argument".into()],
                    tags: vec!["history".into()],
                },
                CheatSheetEntry {
                    id: "reverse-search".into(),
                    name: "Reverse Search".into(),
                    action: "Search shell history backwards".into(),
                    command: Some(vec!["Ctrl".into(), "R".into()]),
                    command_text: None,
                    command_sequence: None,
                    aliases: vec!["history search".into()],
                    tags: vec!["history".into()],
                },
            ],
        },
        CheatSheet {
            id: "tmux".into(),
            name: "tmux".into(),
            icon: Some("mux".into()),
            tags: vec!["terminal".into(), "multiplexer".into()],
            entries: vec![
                CheatSheetEntry {
                    id: "new-session".into(),
                    name: "New Session".into(),
                    action: "Start a new tmux session".into(),
                    command: None,
                    command_text: Some("tmux new-session".into()),
                    command_sequence: Some(vec![vec!["tmux".into()], vec!["new-session".into()]]),
                    aliases: vec!["tmux new".into(), "session start".into()],
                    tags: vec!["sessions".into(), "cli".into()],
                },
                CheatSheetEntry {
                    id: "list-sessions".into(),
                    name: "List Sessions".into(),
                    action: "Show all tmux sessions".into(),
                    command: None,
                    command_text: Some("tmux list-sessions".into()),
                    command_sequence: Some(vec![vec!["tmux".into()], vec!["list-sessions".into()]]),
                    aliases: vec!["tmux ls".into(), "sessions list".into()],
                    tags: vec!["sessions".into(), "cli".into()],
                },
                CheatSheetEntry {
                    id: "attach-session".into(),
                    name: "Attach Session".into(),
                    action: "Attach to the last or named tmux session".into(),
                    command: None,
                    command_text: Some("tmux attach-session -t <name>".into()),
                    command_sequence: Some(vec![
                        vec!["tmux".into()],
                        vec!["attach-session".into(), "-t".into(), "<name>".into()],
                    ]),
                    aliases: vec!["attach".into(), "tmux attach".into(), "reattach".into()],
                    tags: vec!["sessions".into(), "cli".into()],
                },
                CheatSheetEntry {
                    id: "new-window".into(),
                    name: "New Window".into(),
                    action: "Create a new tmux window".into(),
                    command: None,
                    command_text: Some("new-window".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["C".into()]]),
                    aliases: vec!["window".into(), "create window".into()],
                    tags: vec!["windows".into()],
                },
                CheatSheetEntry {
                    id: "split-horizontal".into(),
                    name: "Split Horizontal".into(),
                    action: "Split the current pane horizontally".into(),
                    command: None,
                    command_text: Some("split-window -v".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["\"".into()]]),
                    aliases: vec!["pane split".into(), "horizontal split".into()],
                    tags: vec!["panes".into()],
                },
                CheatSheetEntry {
                    id: "split-vertical".into(),
                    name: "Split Vertical".into(),
                    action: "Split the current pane vertically".into(),
                    command: None,
                    command_text: Some("split-window -h".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["%".into()]]),
                    aliases: vec!["vertical split".into(), "pane vertical".into()],
                    tags: vec!["panes".into()],
                },
                CheatSheetEntry {
                    id: "next-window".into(),
                    name: "Next Window".into(),
                    action: "Move to the next tmux window".into(),
                    command: None,
                    command_text: Some("next-window".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["n".into()]]),
                    aliases: vec!["cycle window".into(), "forward window".into()],
                    tags: vec!["windows".into()],
                },
                CheatSheetEntry {
                    id: "previous-window".into(),
                    name: "Previous Window".into(),
                    action: "Move to the previous tmux window".into(),
                    command: None,
                    command_text: Some("previous-window".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["p".into()]]),
                    aliases: vec!["back window".into(), "prev window".into()],
                    tags: vec!["windows".into()],
                },
                CheatSheetEntry {
                    id: "list-windows".into(),
                    name: "List Windows".into(),
                    action: "Open the tmux window list".into(),
                    command: None,
                    command_text: Some("list-windows".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["w".into()]]),
                    aliases: vec!["window list".into(), "choose window".into()],
                    tags: vec!["windows".into()],
                },
                CheatSheetEntry {
                    id: "last-window".into(),
                    name: "Last Window".into(),
                    action: "Toggle the last active window".into(),
                    command: None,
                    command_text: Some("last-window".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["l".into()]]),
                    aliases: vec!["previous active window".into(), "toggle window".into()],
                    tags: vec!["windows".into()],
                },
                CheatSheetEntry {
                    id: "rename-window".into(),
                    name: "Rename Window".into(),
                    action: "Rename the current tmux window".into(),
                    command: None,
                    command_text: Some("rename-window".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec![",".into()]]),
                    aliases: vec!["window name".into(), "rename pane group".into()],
                    tags: vec!["windows".into()],
                },
                CheatSheetEntry {
                    id: "detach-session".into(),
                    name: "Detach Session".into(),
                    action: "Detach from the current tmux session".into(),
                    command: None,
                    command_text: Some("detach-client".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["D".into()]]),
                    aliases: vec!["detach".into(), "leave session".into()],
                    tags: vec!["sessions".into()],
                },
                CheatSheetEntry {
                    id: "choose-sessions".into(),
                    name: "Choose Session".into(),
                    action: "Open the tmux session switcher".into(),
                    command: None,
                    command_text: Some("choose-tree -s".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["s".into()]]),
                    aliases: vec!["session list".into(), "switch session".into()],
                    tags: vec!["sessions".into()],
                },
                CheatSheetEntry {
                    id: "previous-session".into(),
                    name: "Previous Session".into(),
                    action: "Move to the previous tmux session".into(),
                    command: None,
                    command_text: Some("switch-client -p".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["(".into()]]),
                    aliases: vec!["prev session".into(), "back session".into()],
                    tags: vec!["sessions".into()],
                },
                CheatSheetEntry {
                    id: "next-session".into(),
                    name: "Next Session".into(),
                    action: "Move to the next tmux session".into(),
                    command: None,
                    command_text: Some("switch-client -n".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec![")".into()]]),
                    aliases: vec!["forward session".into(), "cycle session".into()],
                    tags: vec!["sessions".into()],
                },
                CheatSheetEntry {
                    id: "copy-mode".into(),
                    name: "Copy Mode".into(),
                    action: "Enter tmux copy mode".into(),
                    command: None,
                    command_text: Some("copy-mode".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["[".into()]]),
                    aliases: vec!["scrollback".into(), "copy".into()],
                    tags: vec!["copy mode".into()],
                },
                CheatSheetEntry {
                    id: "last-pane".into(),
                    name: "Last Pane".into(),
                    action: "Jump to the previously active pane".into(),
                    command: None,
                    command_text: Some("last-pane".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec![";".into()]]),
                    aliases: vec!["previous pane".into(), "toggle pane".into()],
                    tags: vec!["panes".into()],
                },
                CheatSheetEntry {
                    id: "move-pane-left".into(),
                    name: "Move Pane Left".into(),
                    action: "Focus the pane on the left".into(),
                    command: None,
                    command_text: Some("select-pane -L".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["Left".into()]]),
                    aliases: vec!["pane left".into(), "focus left".into()],
                    tags: vec!["panes".into(), "navigation".into()],
                },
                CheatSheetEntry {
                    id: "move-pane-right".into(),
                    name: "Move Pane Right".into(),
                    action: "Focus the pane on the right".into(),
                    command: None,
                    command_text: Some("select-pane -R".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["Right".into()]]),
                    aliases: vec!["pane right".into(), "focus right".into()],
                    tags: vec!["panes".into(), "navigation".into()],
                },
                CheatSheetEntry {
                    id: "next-pane".into(),
                    name: "Next Pane".into(),
                    action: "Switch to the next tmux pane".into(),
                    command: None,
                    command_text: Some("select-pane -t :.+".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["o".into()]]),
                    aliases: vec!["cycle pane".into(), "pane next".into()],
                    tags: vec!["panes".into()],
                },
                CheatSheetEntry {
                    id: "show-pane-numbers".into(),
                    name: "Show Pane Numbers".into(),
                    action: "Temporarily show pane numbers".into(),
                    command: None,
                    command_text: Some("display-panes".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["q".into()]]),
                    aliases: vec!["pane ids".into(), "pane numbers".into()],
                    tags: vec!["panes".into()],
                },
                CheatSheetEntry {
                    id: "toggle-pane-zoom".into(),
                    name: "Toggle Pane Zoom".into(),
                    action: "Zoom or unzoom the current pane".into(),
                    command: None,
                    command_text: Some("resize-pane -Z".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["z".into()]]),
                    aliases: vec!["maximize pane".into(), "zoom pane".into()],
                    tags: vec!["panes".into()],
                },
                CheatSheetEntry {
                    id: "kill-pane".into(),
                    name: "Kill Pane".into(),
                    action: "Close the current pane".into(),
                    command: None,
                    command_text: Some("kill-pane".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["x".into()]]),
                    aliases: vec!["close pane".into(), "delete pane".into()],
                    tags: vec!["panes".into()],
                },
                CheatSheetEntry {
                    id: "resize-pane-left".into(),
                    name: "Resize Pane Left".into(),
                    action: "Resize the current pane to the left".into(),
                    command: None,
                    command_text: Some("resize-pane -L".into()),
                    command_sequence: Some(vec![
                        vec!["Ctrl".into(), "B".into()],
                        vec!["Ctrl".into(), "Left".into()],
                    ]),
                    aliases: vec!["shrink pane".into(), "resize left".into()],
                    tags: vec!["panes".into(), "resize".into()],
                },
                CheatSheetEntry {
                    id: "resize-pane-right".into(),
                    name: "Resize Pane Right".into(),
                    action: "Resize the current pane to the right".into(),
                    command: None,
                    command_text: Some("resize-pane -R".into()),
                    command_sequence: Some(vec![
                        vec!["Ctrl".into(), "B".into()],
                        vec!["Ctrl".into(), "Right".into()],
                    ]),
                    aliases: vec!["grow pane".into(), "resize right".into()],
                    tags: vec!["panes".into(), "resize".into()],
                },
                CheatSheetEntry {
                    id: "pane-layout".into(),
                    name: "Toggle Pane Layout".into(),
                    action: "Cycle through pane layouts".into(),
                    command: None,
                    command_text: Some("select-layout".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["Space".into()]]),
                    aliases: vec!["layout".into(), "pane layout".into()],
                    tags: vec!["panes".into(), "layout".into()],
                },
                CheatSheetEntry {
                    id: "paste-buffer".into(),
                    name: "Paste Buffer".into(),
                    action: "Paste the current tmux buffer".into(),
                    command: None,
                    command_text: Some("paste-buffer".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["]".into()]]),
                    aliases: vec!["paste".into(), "buffer paste".into()],
                    tags: vec!["copy mode".into(), "buffers".into()],
                },
                CheatSheetEntry {
                    id: "command-prompt".into(),
                    name: "Command Prompt".into(),
                    action: "Open the tmux command prompt".into(),
                    command: None,
                    command_text: Some("command-prompt".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec![":".into()]]),
                    aliases: vec!["tmux command".into(), "prompt".into()],
                    tags: vec!["misc".into()],
                },
                CheatSheetEntry {
                    id: "list-keys".into(),
                    name: "List Keys".into(),
                    action: "Show all tmux key bindings".into(),
                    command: None,
                    command_text: Some("list-keys".into()),
                    command_sequence: Some(vec![vec!["Ctrl".into(), "B".into()], vec!["?".into()]]),
                    aliases: vec!["help".into(), "bindings".into(), "shortcuts".into()],
                    tags: vec!["help".into()],
                },
            ],
        },
    ]
}

fn app_cheatsheet_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let mut dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve app data directory: {error}"))?;
    dir.push("cheatsheets");

    fs::create_dir_all(&dir)
        .map_err(|error| format!("Unable to create cheat sheet directory: {error}"))?;

    Ok(dir)
}

fn has_json_files(dir: &Path) -> Result<bool, String> {
    let entries =
        fs::read_dir(dir).map_err(|error| format!("Unable to inspect cheat sheet directory: {error}"))?;

    for entry in entries {
        let entry = entry.map_err(|error| format!("Unable to read directory entry: {error}"))?;
        if entry.path().extension().and_then(|extension| extension.to_str()) == Some("json") {
            return Ok(true);
        }
    }

    Ok(false)
}

fn ensure_default_cheat_sheets(dir: &Path) -> Result<(), String> {
    if has_json_files(dir)? {
        return Ok(());
    }

    for sheet in default_cheat_sheets() {
        let file_path = dir.join(format!("{}.json", sheet.id));
        let body = serde_json::to_string_pretty(&sheet)
            .map_err(|error| format!("Unable to serialize default cheat sheet: {error}"))?;
        fs::write(&file_path, body)
            .map_err(|error| format!("Unable to write default cheat sheet {:?}: {error}", file_path))?;
    }

    Ok(())
}

fn read_cheat_sheets(dir: &Path) -> Result<Vec<CheatSheet>, String> {
    let mut sheets = Vec::new();

    let entries =
        fs::read_dir(dir).map_err(|error| format!("Unable to read cheat sheet directory: {error}"))?;

    for entry in entries {
        let path = entry
            .map_err(|error| format!("Unable to read directory entry: {error}"))?
            .path();

        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }

        let content = fs::read_to_string(&path)
            .map_err(|error| format!("Unable to read cheat sheet {:?}: {error}", path))?;
        let sheet: CheatSheet = serde_json::from_str(&content)
            .map_err(|error| format!("Invalid cheat sheet {:?}: {error}", path))?;
        sheets.push(sheet);
    }

    sheets.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(sheets)
}

fn toggle_main_window(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window is not available".to_string())?;

    let is_visible = window
        .is_visible()
        .map_err(|error| format!("Unable to inspect window visibility: {error}"))?;

    if is_visible {
        window
            .hide()
            .map_err(|error| format!("Unable to hide main window: {error}"))?;
        return Ok(());
    }

    window
        .show()
        .map_err(|error| format!("Unable to show main window: {error}"))?;
    #[cfg(target_os = "macos")]
    configure_launcher_window(&window)?;
    window
        .set_focus()
        .map_err(|error| format!("Unable to focus main window: {error}"))?;
    window
        .emit("vaise://focus-search", ())
        .map_err(|error| format!("Unable to emit focus event: {error}"))?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn configure_launcher_window(window: &WebviewWindow) -> Result<(), String> {
    let launcher_window = window.clone();

    window
        .run_on_main_thread(move || unsafe {
            let ns_window = launcher_window
                .ns_window()
                .expect("Unable to access macOS window handle on main thread");
            let ns_window: &NSWindow = &*ns_window.cast();
            let behavior = NSWindowCollectionBehavior::CanJoinAllSpaces
                | NSWindowCollectionBehavior::FullScreenAuxiliary
                | NSWindowCollectionBehavior::Stationary
                | NSWindowCollectionBehavior::Transient;

            ns_window.setCollectionBehavior(behavior);
            ns_window.setLevel(NSStatusWindowLevel);
            ns_window.setHidesOnDeactivate(true);
            ns_window.orderFrontRegardless();
            ns_window.makeKeyAndOrderFront(None);
        })
        .map_err(|error| format!("Unable to configure macOS launcher window: {error}"))?;

    Ok(())
}

#[tauri::command]
fn load_cheat_sheets(app: AppHandle) -> Result<CheatSheetsPayload, String> {
    let directory = app_cheatsheet_dir(&app)?;
    ensure_default_cheat_sheets(&directory)?;
    let sheets = read_cheat_sheets(&directory)?;

    Ok(CheatSheetsPayload {
        directory: directory.to_string_lossy().to_string(),
        sheets,
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let _ = toggle_main_window(app);
                    }
                })
                .build(),
        )
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let toggle_item = MenuItemBuilder::with_id("toggle-launcher", "Open / Hide Vaise")
                .build(app)
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;
            let quit_item = MenuItemBuilder::with_id("quit-vaise", "Quit Vaise")
                .build(app)
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;
            let tray_menu = MenuBuilder::new(app)
                .item(&toggle_item)
                .separator()
                .item(&quit_item)
                .build()
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;

            let tray_icon = TrayIconBuilder::with_id("vaise-tray")
                .menu(&tray_menu)
                .tooltip("Vaise")
                .title("Vaise")
                .icon(
                    tray_icon_image()
                        .map_err(|error| -> Box<dyn std::error::Error> { error.into() })?,
                )
                .build(app)
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;

            #[cfg(target_os = "macos")]
            tray_icon
                .set_icon_as_template(false)
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;

            app.manage(TrayState { _icon: tray_icon });

            app.on_menu_event(|app_handle, event| match event.id().as_ref() {
                "toggle-launcher" => {
                    let _ = toggle_main_window(app_handle);
                }
                "quit-vaise" => app_handle.exit(0),
                _ => {}
            });

            app.on_tray_icon_event(|app_handle, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    let _ = toggle_main_window(app_handle);
                }
            });

            let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::CONTROL), Code::KeyK);
            app.global_shortcut()
                .register(shortcut)
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;

            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "macos")]
                configure_launcher_window(&window)
                    .map_err(|error| -> Box<dyn std::error::Error> { error.into() })?;

                let launcher_window = window.clone();
                window.on_window_event(move |event| match event {
                    WindowEvent::Focused(false) => {
                        let _ = launcher_window.hide();
                    }
                    WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        let _ = launcher_window.hide();
                    }
                    _ => {}
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_cheat_sheets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
