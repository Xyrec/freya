"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// Define the Track type to match our Rust structure
interface Track {
  id: number;
  title: string;
  artist: string;
  album: string;
  duration: string;
  duration_seconds: number;
  file_path: string;
  current: boolean;
}

export function PlaylistView() {
  const [playlist, setPlaylist] = useState<Track[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // Fetch playlist data from Rust backend
  useEffect(() => {
    async function fetchPlaylist() {
      try {
        setLoading(true);
        const tracks = await invoke<Track[]>("get_playlist");
        setPlaylist(tracks);
        setError(null);
      } catch (err) {
        console.error("Failed to load playlist:", err);
        setError("Failed to load playlist");
      } finally {
        setLoading(false);
      }
    }

    fetchPlaylist();
  }, []);

  // Handle track selection
  const handleTrackSelect = async (track: Track) => {
    try {
      await invoke("play_track", {
        filePath: track.file_path,
        trackId: track.id,
      });
      
      // Update current track in UI (in a real app, this would be done via an event from Rust)
      setPlaylist(prev => prev.map(t => ({
        ...t,
        current: t.id === track.id
      })));
    } catch (err) {
      console.error("Failed to play track:", err);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-muted-foreground">Loading playlist...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-destructive">{error}</p>
      </div>
    );
  }

  return (
    <div className="px-3 space-y-1 my-2 overscroll-auto">
      {playlist.map((track) => (
        <div
          key={track.id}
          className={`grid grid-cols-[40px_1fr_auto] items-center rounded-lg px-4 py-2 hover:bg-accent cursor-pointer ${
            track.current ? "bg-accent shadow-inner shadow-border" : ""
          }`}
          role="button"
          tabIndex={0}
          onClick={() => handleTrackSelect(track)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              handleTrackSelect(track);
            }
          }}
        >
          <span className="text-sm text-muted-foreground font-mono w-[40px]">
            {track.id}
          </span>
          <div>
            <div className="font-medium leading-none">{track.title}</div>
            <div className="text-sm text-muted-foreground">{track.artist}</div>
          </div>
          <span className="text-sm text-muted-foreground font-mono">
            {track.duration}
          </span>
        </div>
      ))}
    </div>
  );
}
