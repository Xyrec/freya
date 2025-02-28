use lofty::{
    read_from_path,
    prelude::{AudioFile, TaggedFileExt},
    picture::PictureType,
    tag::Accessor,
};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Track {
    id: usize,
    title: String,
    artist: String,
    album: String,
    duration: String, // Formatted as mm:ss
    duration_seconds: f32,
    file_path: String,
    current: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct AlbumArt {
    mime_type: String,
    data: Vec<u8>,
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
    
    // Try to find a tag (prefer ID3v2, but fall back to others)
    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())
        .ok_or_else(|| format!("No tags found in {}", path.display()))?;
    
    // Extract basic metadata with proper Cow handling
    let default_title = path.file_stem()
        .map(|s| s.to_string_lossy())
        .unwrap_or(Cow::Borrowed("Unknown"));
    
    let title = tag.title()
        .unwrap_or(default_title)
        .to_string();
    
    let artist = tag.artist()
        .unwrap_or(Cow::Borrowed("Unknown Artist"))
        .to_string();
    
    let album = tag.album()
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
        if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
            for picture in tag.pictures() {
                if picture.pic_type() == PictureType::CoverFront || 
                   picture.pic_type() == PictureType::Other {
                    return Some(AlbumArt {
                        mime_type: picture.mime_type()
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

/// Creates a temporary playlist with example tracks
#[tauri::command]
pub async fn get_playlist() -> Result<Vec<Track>, String> {
    // For now, just read our example file
    let example_path = PathBuf::from("../music/example.flac");
    if example_path.exists() {
        let track = extract_metadata(&example_path, 1)?;
        
        // For demo purposes, let's create a few fake entries based on the real one
        let playlist = vec![
            track.clone(),
            Track {
                id: 2,
                title: "The Whistle Song".into(),
                artist: format!("{} feat. Dynamic", track.artist),
                album: track.album.clone(),
                duration: "4:39".into(),
                duration_seconds: 279.0,
                file_path: track.file_path.clone(),
                current: false,
            },
            Track {
                id: 3,
                title: "Wanna Die For You".into(),
                artist: format!("{} feat. Diane Char", track.artist),
                album: track.album.clone(),
                duration: "4:17".into(),
                duration_seconds: 257.0,
                file_path: track.file_path.clone(),
                current: false,
            },
            Track {
                id: 4,
                title: "Come Alive".into(),
                artist: track.artist.clone(),
                album: track.album.clone(),
                duration: "3:11".into(),
                duration_seconds: 191.0,
                file_path: track.file_path.clone(),
                current: true,
            },
            Track {
                id: 5,
                title: "Give & Take".into(),
                artist: track.artist,
                album: track.album,
                duration: "4:08".into(),
                duration_seconds: 248.0,
                file_path: track.file_path,
                current: false,
            },
        ];
        
        Ok(playlist)
    } else {
        Err("Example audio file not found".into())
    }
}

/// Get album art for a specific track by file path
#[tauri::command]
pub async fn get_track_album_art(file_path: String) -> Result<Option<AlbumArt>, String> {
    let path = PathBuf::from(&file_path);
    if path.exists() {
        Ok(get_album_art(&path))
    } else {
        Err(format!("File not found: {}", file_path))
    }
}

/// Play a specific track from the playlist
#[tauri::command]
pub async fn play_track(
    _app: AppHandle,
    file_path: String,
    track_id: usize,
) -> Result<(), String> {
    // For now this is just a placeholder
    // In a real implementation, this would update the current track and play it
    // by integrating with the existing audio playback code
    println!("Playing track {} from {}", track_id, file_path);
    
    // Here you would:
    // 1. Update which track is "current"
    // 2. Load and play the audio file
    // 3. Update the UI
    
    Ok(())
}

/// Scan a directory for audio files and create a playlist
#[tauri::command]
pub async fn scan_directory(_dir_path: String) -> Result<Vec<Track>, String> {
    // In a full implementation, this would scan a directory for audio files
    // and build a proper playlist. For now, we'll just return the example tracks.
    get_playlist().await
}