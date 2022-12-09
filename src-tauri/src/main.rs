#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Window};

#[tauri::command]
fn command(text: String) -> String {
    format!("invoke: {}", text)
}

#[tauri::command]
fn hello() {
    println!("hello world");
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

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

    std::thread::spawn(move || loop {
        window
            .emit(
                "backend-event",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            let main_window = app.get_window("main").unwrap();

            main_window.listen("front-event", |event| {
              println!("got window event-name with payload {:?}", event.payload());
            });
      
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command, hello, init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
