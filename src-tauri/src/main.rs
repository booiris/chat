#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::tcp::client::send_msg;

use app::tcp::server::server;
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
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            let main_window = app.get_window("main").unwrap();

            main_window.listen("send-msg", |event| {
                tokio::spawn(async move {
                    match send_msg(event.payload()).await {
                        Ok(()) => (),
                        Err(err) => eprintln!("error: {}", err),
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
