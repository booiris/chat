#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use app::service::tcp::client::Client;

use app::service::tcp::server::server;
use env_logger::Env;
use tauri::{Manager, Window};

static mut FLAG: bool = false;

#[tauri::command]
fn init_process(window: Window) {
    unsafe {
        if FLAG {
            println!("init_process has been called");
            return;
        }
        FLAG = true;
    }
    println!("init_process is called");
    tokio::spawn(server(window));
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let client = Arc::new(Client::new());
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            let main_window = app.get_window("main").unwrap();

            main_window.listen("send-msg", move |event| {
                let client = client.clone();
                tokio::spawn(async move { client.send_msg(123, event.payload()).await });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
