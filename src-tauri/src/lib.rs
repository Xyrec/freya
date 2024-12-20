use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn play_sound() {
    thread::spawn(|| {
        // Get an output stream handle to the default physical sound device.
        // Note that no sound will be played if _stream is dropped
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("../static/example.flac").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Get the length of the source
        let duration = source.total_duration().unwrap();
        // Play the sound directly on the device
        let _result = stream_handle.play_raw(source.convert_samples());

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_nanos(duration.as_nanos() as u64));
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
