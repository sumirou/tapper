mod tosu;
mod database;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    println!("greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_from_tosu() {
    tosu::get_from_tosu().await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("start");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_from_tosu])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                get_from_tosu().await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
