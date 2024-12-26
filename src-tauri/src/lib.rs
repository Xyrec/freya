use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

struct AppState {
    sink: Arc<Mutex<Sink>>,
    is_paused: Arc<AtomicBool>,
}

#[derive(serde::Serialize, Clone)]
struct ProgressData {
    current_position: f32,
    duration: f32,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let app_state = AppState {
        sink: Arc::new(Mutex::new(sink)),
        is_paused: Arc::new(AtomicBool::new(false)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            play_sound,
            pause_sound,
            set_volume,
            seek_position
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn play_sound(app: AppHandle, state: tauri::State<AppState>) {
    let sink = state.sink.lock().unwrap();
    if state.is_paused.load(Ordering::SeqCst) {
        sink.play();
        state.is_paused.store(false, Ordering::SeqCst);
    } else if sink.empty() {
        let file = File::open("../static/example.flac").unwrap();
        let source = Decoder::new(file).unwrap();
        let duration = source.total_duration().unwrap_or_default();
        sink.append(source);

        // Reset position before starting playback
        sink.try_seek(std::time::Duration::from_secs(0))
            .unwrap_or_default();
        sink.play();

        let arc_sink = Arc::clone(&state.sink);
        let arc_paused = Arc::clone(&state.is_paused);
        let app_clone = app.clone();

        std::thread::spawn(move || {
            // Small initial delay to ensure proper position tracking
            std::thread::sleep(std::time::Duration::from_millis(50));

            while !arc_sink.lock().unwrap().empty() {
                if !arc_paused.load(Ordering::SeqCst) {
                    let sink = arc_sink.lock().unwrap();
                    let position = sink.get_pos();
                    // Ensure position never exceeds duration
                    let current_pos = position.as_secs_f32().min(duration.as_secs_f32());
                    let progress = ProgressData {
                        current_position: current_pos,
                        duration: duration.as_secs_f32(),
                    };
                    app_clone.emit("progress_update", progress).unwrap();
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            arc_paused.store(false, Ordering::SeqCst);
            app_clone.emit("sound_done", ()).unwrap();
        });
    }
}

#[tauri::command]
async fn pause_sound(state: tauri::State<'_, AppState>) -> Result<(), String> {
    // Log pause attempt
    println!("Attempting to pause sound");

    let sink = state
        .sink
        .lock()
        .map_err(|_| "Failed to acquire sink lock")?;
    let current_state = sink.is_paused();

    if !current_state {
        // Log current state
        println!("Sink is playing, pausing now");
        sink.pause();
        state.is_paused.store(true, Ordering::SeqCst);
        println!("Pause completed");
    } else {
        println!("Sink is already paused");
    }

    Ok(())
}

#[tauri::command]
fn seek_position(state: tauri::State<AppState>, position: f32) {
    let sink = state.sink.lock().unwrap();
    let was_paused = sink.is_paused();

    // Temporarily unpause to perform seek
    if was_paused {
        sink.play();
    }

    // Perform the seek operation
    if let Ok(_) = sink.try_seek(std::time::Duration::from_secs_f32(position)) {
        // If seek was successful and sink was paused, restore pause state
        if was_paused {
            sink.pause();
        }
    }
}

#[tauri::command]
fn set_volume(state: tauri::State<AppState>, volume: f32) {
    let sink = state.sink.lock().unwrap();
    // Convert 0-100 range to 0.0-1.0
    sink.set_volume(volume / 100.0);
}
