use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

struct AppState {
    sink: Arc<Mutex<Sink>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let app_state = AppState {
        sink: Arc::new(Mutex::new(sink)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_sound, pause_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn play_sound(app: AppHandle, state: tauri::State<AppState>) {
    let sink = state.sink.lock().unwrap();
    if sink.is_paused() {
        sink.play();
    } else if sink.empty() {
        let file = File::open("../static/example.flac").unwrap();
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.play();

        let arc_sink = Arc::clone(&state.sink);
        let app_clone = app.clone();

        // Using time::Duration is probably terrible, might need a refactor.
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_micros(100));
            let sink = arc_sink.lock().unwrap();
            if sink.empty() {
                app_clone.emit("sound_done", ()).unwrap();
                break;
            }
        });
    }
}

#[tauri::command]
fn pause_sound(state: tauri::State<AppState>) {
    let sink = state.sink.lock().unwrap();
    if !sink.is_paused() {
        sink.pause();
    }
}
