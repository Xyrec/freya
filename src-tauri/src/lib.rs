use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

struct AppState {
    sink: Arc<Mutex<Sink>>,
    is_paused: Arc<AtomicBool>,
    is_seeking: Arc<AtomicBool>, // Flag to track seeking state
    current_duration: Arc<Mutex<std::time::Duration>>, // Store current track duration
}

#[derive(serde::Serialize, Clone)]
struct ProgressData {
    current_position: f32,
    duration: f32,
}

#[derive(serde::Serialize)]
struct PlaybackState {
    is_playing: bool,
    is_empty: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let app_state = AppState {
        sink: Arc::new(Mutex::new(sink)),
        is_paused: Arc::new(AtomicBool::new(false)),
        is_seeking: Arc::new(AtomicBool::new(false)),
        current_duration: Arc::new(Mutex::new(std::time::Duration::default())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            play_sound,
            pause_sound,
            set_volume,
            seek_position,
            get_playback_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_playback_state(state: tauri::State<'_, AppState>) -> Result<PlaybackState, String> {
    let sink = state
        .sink
        .lock()
        .map_err(|_| "Failed to acquire sink lock".to_string())?;

    let is_empty = sink.empty();
    let is_paused = state.is_paused.load(Ordering::SeqCst);

    // We consider playback active if sink is not empty and not paused
    let is_playing = !is_empty && !is_paused;

    Ok(PlaybackState {
        is_playing,
        is_empty,
    })
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn play_sound(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
    println!("Attempting to play sound");

    let sink = state
        .sink
        .lock()
        .map_err(|_| "Failed to acquire sink lock".to_string())?;

    if state.is_paused.load(Ordering::SeqCst) {
        println!("Resuming paused playback");
        sink.play();
        state.is_paused.store(false, Ordering::SeqCst);
        println!("Playback resumed");
        Ok(())
    } else if sink.empty() {
        println!("Starting new playback");

        let file = File::open("../music/example.flac")
            .map_err(|e| format!("Failed to open audio file: {}", e))?;

        let source =
            Decoder::new(file).map_err(|e| format!("Failed to decode audio file: {}", e))?;

        let duration = source.total_duration().unwrap_or_default();

        // Store the duration for later use
        *state
            .current_duration
            .lock()
            .map_err(|_| "Failed to update duration".to_string())? = duration;

        sink.append(source);

        // Reset position before starting playback
        sink.try_seek(std::time::Duration::from_secs(0))
            .unwrap_or_default();
        sink.play();

        let arc_sink = Arc::clone(&state.sink);
        let arc_paused = Arc::clone(&state.is_paused);
        let arc_seeking = Arc::clone(&state.is_seeking);
        let app_clone = app.clone();
        let arc_duration = Arc::clone(&state.current_duration);

        std::thread::spawn(move || {
            // Small initial delay to ensure proper position tracking
            std::thread::sleep(std::time::Duration::from_millis(50));

            println!("Started progress monitoring thread");

            while !arc_sink.lock().unwrap().empty() {
                // Only send progress updates if not seeking
                if !arc_paused.load(Ordering::SeqCst) && !arc_seeking.load(Ordering::SeqCst) {
                    let sink = arc_sink.lock().unwrap();
                    let position = sink.get_pos();
                    let duration = *arc_duration.lock().unwrap();

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
            println!("Playback finished, emitting sound_done event");
            app_clone.emit("sound_done", ()).unwrap();
        });

        println!("Playback started successfully");
        Ok(())
    } else {
        println!("Cannot play: Sink is not empty");
        Err("Sink is not empty".to_string())
    }
}

#[tauri::command]
async fn pause_sound(state: tauri::State<'_, AppState>) -> Result<(), String> {
    println!("Attempting to pause sound");

    let sink = state
        .sink
        .lock()
        .map_err(|_| "Failed to acquire sink lock".to_string())?;

    let current_state = sink.is_paused();

    if !current_state {
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
async fn seek_position(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    position: f32,
) -> Result<(), String> {
    println!("Seeking to position: {}s", position);

    // Set seeking flag before seeking
    state.is_seeking.store(true, Ordering::SeqCst);

    let sink = state
        .sink
        .lock()
        .map_err(|_| "Failed to acquire sink lock".to_string())?;

    // Instead of unpausing temporarily, we'll create a silent seek operation
    let seek_duration = std::time::Duration::from_secs_f32(position);

    // Perform the seek operation without unpausing
    match sink.try_seek(seek_duration) {
        Ok(_) => {
            // Drop the mutex lock before the delay
            drop(sink);

            // Small delay to ensure the backend has updated its position
            std::thread::sleep(std::time::Duration::from_millis(50));

            // Send immediate position update to frontend
            let actual_sink = state
                .sink
                .lock()
                .map_err(|_| "Failed to acquire sink lock after seek".to_string())?;

            let new_position = actual_sink.get_pos().as_secs_f32();

            // Get current duration from our stored value
            let duration = state
                .current_duration
                .lock()
                .map_err(|_| "Failed to get duration".to_string())?;

            // Emit position_changed event with accurate position
            app.emit(
                "position_changed",
                ProgressData {
                    current_position: new_position,
                    duration: duration.as_secs_f32(),
                },
            )
            .map_err(|e| format!("Failed to emit position_changed: {}", e))?;

            // Clear seeking flag after position update is sent
            state.is_seeking.store(false, Ordering::SeqCst);
            println!("Seek completed successfully to position: {}s", new_position);
            Ok(())
        }
        Err(e) => {
            // Clear seeking flag on error
            state.is_seeking.store(false, Ordering::SeqCst);
            println!("Seek failed: {:?}", e);
            Err(format!("Seek error: {:?}", e))
        }
    }
}

#[tauri::command]
async fn set_volume(state: tauri::State<'_, AppState>, volume: f32) -> Result<(), String> {
    let sink = state
        .sink
        .lock()
        .map_err(|_| "Failed to acquire sink lock".to_string())?;

    // Convert 0-100 range to 0.0-1.0 with quadratic curve for more natural volume control
    sink.set_volume((volume / 100.0).powf(2.0));

    Ok(())
}
