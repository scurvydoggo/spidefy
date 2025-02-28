// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts(["ctrl+d", "alt+space"]).expect("Shortcut syntax error")
            .with_handler(|app, shortcut, event| {
                use tauri::Emitter;
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
                if event.state == ShortcutState::Pressed {
                    if shortcut.matches(Modifiers::CONTROL, Code::KeyD) {
                        let _ = app.emit("shortcut-event", "Ctrl+D triggered");
                        print!("Ctrl+D");
                    }
                    if shortcut.matches(Modifiers::ALT, Code::Space) {
                        let _ = app.emit("shortcut-event", "Alt+Space triggered");
                        print!("Alt+Space!");
                    }
                }
            })
            .build()
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
