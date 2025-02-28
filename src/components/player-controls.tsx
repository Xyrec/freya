"use client";

import { Pause, Play, SkipBack, SkipForward, Volume2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import Image from "next/image";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

interface CurrentTrack {
  id: number;
  title: string;
  artist: string;
  album: string;
}

export function PlayerControls() {
  const [isPlaying, setIsPlaying] = useState<boolean>(false);
  const [volume, setVolume] = useState<number>(70);
  const [currentPosition, setCurrentPosition] = useState<number>(0);
  const [duration, setDuration] = useState<number>(0);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [sliderPosition, setSliderPosition] = useState<number>(0);
  const [isInitialized, setIsInitialized] = useState<boolean>(false);
  const [currentTrack, setCurrentTrack] = useState<CurrentTrack>({
    id: 4, // Default track ID (matches the hardcoded "current" track in our playlist)
    title: "Come Alive",
    artist: "Netsky",
    album: "Example Album",
  });

  // Format time as mm:ss
  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  // Get initial playback state from backend
  useEffect(() => {
    const initializePlaybackState = async () => {
      try {
        const state = await invoke<{ is_playing: boolean; is_empty: boolean }>(
          "get_playback_state"
        );
        setIsPlaying(state.is_playing);
        setIsInitialized(true);
      } catch (error) {
        console.error("Failed to get playback state:", error);
        setIsInitialized(true);
      }
    };

    initializePlaybackState();

    // Listen for track change events
    const unlistenTrackChange = listen<CurrentTrack>("track_changed", (event) => {
      setCurrentTrack(event.payload);
    });

    return () => {
      unlistenTrackChange.then(unlisten => unlisten());
    };
  }, []);

  useEffect(() => {
    // Only set up listeners once we have the initial state
    if (!isInitialized) return;

    // Listen for progress updates from Rust
    const unlistenProgress = listen<{
      current_position: number;
      duration: number;
    }>("progress_update", (event) => {
      // Only update position from progress updates if not dragging
      if (!isDragging) {
        setCurrentPosition(event.payload.current_position);
      }
      setDuration(event.payload.duration);

      // If we're receiving progress updates, playback must be active
      setIsPlaying(true);
    });

    // Listen for position_changed events (after seeking)
    const unlistenPositionChanged = listen<{
      current_position: number;
      duration: number;
    }>("position_changed", (event) => {
      // Immediate position update after seeking
      setCurrentPosition(event.payload.current_position);
      setSliderPosition(event.payload.current_position);
      if (event.payload.duration > 0) {
        setDuration(event.payload.duration);
      }
    });

    // Listen for playback completion
    const unlistenDone = listen("sound_done", () => {
      setIsPlaying(false);
    });

    // Cleanup listeners when component unmounts
    return () => {
      unlistenProgress.then((unlisten) => unlisten());
      unlistenPositionChanged.then((unlisten) => unlisten());
      unlistenDone.then((unlisten) => unlisten());
    };
  }, [isDragging, isInitialized]);

  // Toggle play/pause
  const togglePlayback = async () => {
    try {
      if (isPlaying) {
        await invoke("pause_sound");
        setIsPlaying(false);
      } else {
        await invoke("play_sound");
        setIsPlaying(true);
      }
    } catch (error) {
      console.error("Playback error:", error);
    }
  };

  // Handle volume change
  const handleVolumeChange = async (values: number[]) => {
    const newVolume = values[0];
    setVolume(newVolume);
    await invoke("set_volume", { volume: newVolume });
  };

  // Handle slider value change (during drag)
  const handleSliderChange = (values: number[]) => {
    const position = values[0];
    setSliderPosition(position);
    // Only update visual position during drag, don't seek yet
    if (isDragging) {
      setCurrentPosition(position);
    }
  };

  // Handle seek when user starts dragging
  const handleSliderDragStart = () => {
    setIsDragging(true);
  };

  // Handle seek when user stops dragging
  const handleSliderDragEnd = async () => {
    // First update the visual state immediately to prevent flicker
    setCurrentPosition(sliderPosition);

    try {
      // Then perform the actual seek operation
      await invoke("seek_position", { position: sliderPosition });
    } catch (error) {
      console.error("Seek error:", error);
    } finally {
      // Always clear the dragging state when done
      setIsDragging(false);
    }
  };

  return (
    <div className="w-full border-t bg-background p-4 z-10">
      <div className="flex items-center gap-4">
        <Image
          src="album-art.jpg"
          alt={`Album art for ${currentTrack.album}`}
          className="aspect-square w-24 rounded-md"
          width={96}
          height={96}
          placeholder="empty"
        />
        <div className="flex-1">
          <div className="mb-2 flex items-center justify-between">
            <div>
              <div className="font-medium leading-none">{currentTrack.title}</div>
              <div className="text-sm text-muted-foreground">{currentTrack.artist}</div>
            </div>
            <div className="flex items-center gap-2 text-sm text-muted-foreground font-mono">
              <span>{formatTime(currentPosition)}</span>
              <span>/</span>
              <span>{formatTime(duration)}</span>
            </div>
          </div>
          <Slider
            value={[isDragging ? sliderPosition : currentPosition]}
            max={duration || 100}
            step={0.01}
            className="mb-3 mt-2 w-[calc(100%)]"
            onValueChange={handleSliderChange}
            onValueCommit={handleSliderDragEnd}
            onPointerDown={handleSliderDragStart}
          />
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <Button variant="ghost" size="icon">
                <SkipBack className="h-4 w-4" />
              </Button>
              <Button size="icon" onClick={togglePlayback}>
                {isPlaying ? (
                  <Pause className="h-4 w-4" />
                ) : (
                  <Play className="h-4 w-4" />
                )}
              </Button>
              <Button variant="ghost" size="icon">
                <SkipForward className="h-4 w-4" />
              </Button>
            </div>
            <div className="flex items-center gap-2">
              <Volume2 className="h-4 w-4" />
              <Slider
                value={[volume]}
                max={100}
                step={1}
                className="w-[100px]"
                onValueChange={handleVolumeChange}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
