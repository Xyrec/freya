use lofty::{
    picture::PictureType,
    prelude::{AudioFile, TaggedFileExt},
    read_from_path,
    tag::Accessor,
};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::AppHandle;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Track {
    id: usize,
    title: String,
    artist: String,
    album: String,
    duration: String,
    duration_seconds: f32,
    file_path: String,
    current: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct AlbumArt {
    mime_type: String,
    data: Vec<u8>,
}

// PlaylistState for managing the playlist
pub struct PlaylistState {
    tracks: Mutex<Vec<Track>>,
    next_id: Mutex<usize>,
}

impl PlaylistState {
    pub fn new() -> Self {
        Self {
            tracks: Mutex::new(Vec::new()),
            next_id: Mutex::new(1),
        }
    }

    fn get_next_id(&self) -> usize {
        let mut id = self.next_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }
}

/// Format duration in seconds to mm:ss format
fn format_duration(seconds: f32) -> String {
    let mins = (seconds / 60.0).floor() as u32;
    let secs = (seconds % 60.0).floor() as u32;
    format!("{}:{:02}", mins, secs)
}

/// Extract metadata from a music file using Lofty
fn extract_metadata(path: &Path, id: usize) -> Result<Track, String> {
    let tagged_file = read_from_path(path)
        .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

    let properties = tagged_file.properties();
    let duration_seconds = properties.duration().as_secs_f32();

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())
        .ok_or_else(|| format!("No tags found in {}", path.display()))?;

    let default_title = path
        .file_stem()
        .map(|s| s.to_string_lossy())
        .unwrap_or(Cow::Borrowed("Unknown"));

    let title = tag.title().unwrap_or(default_title).to_string();

    let artist = tag
        .artist()
        .unwrap_or(Cow::Borrowed("Unknown Artist"))
        .to_string();

    let album = tag
        .album()
        .unwrap_or(Cow::Borrowed("Unknown Album"))
        .to_string();

    Ok(Track {
        id,
        title,
        artist,
        album,
        duration: format_duration(duration_seconds),
        duration_seconds,
        file_path: path.to_string_lossy().to_string(),
        current: false,
    })
}

/// Get album art for a specific track
fn get_album_art(path: &Path) -> Option<AlbumArt> {
    if let Ok(tagged_file) = read_from_path(path) {
        if let Some(tag) = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())
        {
            for picture in tag.pictures() {
                if picture.pic_type() == PictureType::CoverFront
                    || picture.pic_type() == PictureType::Other
                {
                    return Some(AlbumArt {
                        mime_type: picture
                            .mime_type()
                            .map(|mt| mt.to_string())
                            .unwrap_or_else(|| "image/jpeg".to_string()),
                        data: picture.data().to_vec(),
                    });
                }
            }
        }
    }
    None
}

#[tauri::command]
pub async fn get_playlist(state: tauri::State<'_, PlaylistState>) -> Result<Vec<Track>, String> {
    let tracks = state.tracks.lock().unwrap();
    Ok(tracks.clone())
}

#[tauri::command]
pub async fn add_files(
    state: tauri::State<'_, PlaylistState>,
    paths: Vec<String>,
) -> Result<Vec<Track>, String> {
    let mut tracks = state.tracks.lock().unwrap();
    let mut new_tracks = Vec::new();

    for path in paths {
        let path = PathBuf::from(path);
        if path.exists() {
            let id = state.get_next_id();
            match extract_metadata(&path, id) {
                Ok(track) => {
                    new_tracks.push(track.clone());
                    tracks.push(track);
                }
                Err(e) => {
                    log::error!("Failed to read metadata from {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(new_tracks)
}

#[tauri::command]
pub async fn get_track_album_art(file_path: String) -> Result<Option<AlbumArt>, String> {
    let path = PathBuf::from(&file_path);
    if path.exists() {
        Ok(get_album_art(&path))
    } else {
        Err(format!("File not found: {}", file_path))
    }
}

#[tauri::command]
pub async fn play_track(_app: AppHandle, file_path: String, track_id: usize) -> Result<(), String> {
    println!("Playing track {} from {}", track_id, file_path);
    Ok(())
}

#[tauri::command]
pub async fn scan_directory(_dir_path: String) -> Result<Vec<Track>, String> {
    Err("Directory scanning not implemented yet".into())
}
