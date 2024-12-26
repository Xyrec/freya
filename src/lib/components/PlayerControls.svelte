<script lang="ts">
  import { Pause, Play, SkipBack, SkipForward, Volume2 } from "lucide-svelte";
  import { Button } from "./ui/button";
  import { Slider } from "./ui/slider";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { error, info } from "@tauri-apps/plugin-log";

  let userVolume = $state<[number]>([70]);
  let trackProgress = $state<[number]>([0]);
  let trackLength = $state<number>(191);
  let lastPosition = $state<number>(0);
  let isPlaying = $state<boolean>(false);

  async function togglePlayback() {
    try {
      if (isPlaying) {
        await invoke("pause_sound");
        isPlaying = false;
      } else {
        if (lastPosition >= trackLength) {
          trackProgress = [0];
          lastPosition = 0;
        }
        await invoke("play_sound");
        isPlaying = true;
      }
    } catch (err) {
      error(String(err));
      // Optionally reset state if operation failed
      isPlaying = false;
    }
  }

  async function handleVolumeChange(volume: number[]) {
    userVolume = [volume[0]];
    await invoke("set_volume", { volume: volume[0] });
  }

  async function handleProgressChange(progress: number[]) {
    trackProgress = [progress[0]];
    await invoke("seek_position", { position: progress[0] });
  }

  function formatTime(seconds: number): string {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = Math.floor(seconds % 60);
    return `${minutes}:${remainingSeconds.toString().padStart(2, "0")}`;
  }

  onMount(() => {
    const unlistenProgress = listen<{
      current_position: number;
      duration: number;
    }>("progress_update", (event) => {
      trackProgress = [event.payload.current_position];
      lastPosition = event.payload.current_position;
      trackLength = event.payload.duration;
    });

    const unlistenDone = listen("sound_done", () => {
      isPlaying = false;
      lastPosition = trackLength; // Keep the position at the end
      trackProgress = [trackLength]; // Keep progress bar at the end
    });

    return () => {
      unlistenProgress.then((unlisten) => unlisten());
      unlistenDone.then((unlisten) => unlisten());
    };
  });
</script>

<div class="w-full border-t bg-background p-4 z-10">
  <div class="flex items-center gap-4">
    <img
      src="album-art.jpg"
      alt="Album art"
      class="aspect-square w-24 rounded-md"
    />
    <div class="flex-1">
      <div class="mb-2 flex items-center justify-between">
        <div>
          <div class="font-medium leading-none">Come Alive</div>
          <div class="text-sm text-muted-foreground">Netsky</div>
        </div>
        <div
          class="flex items-center gap-2 text-sm text-muted-foreground font-mono"
        >
          <span>{formatTime(trackProgress[0])}</span>
          <span>/</span>
          <span>{formatTime(trackLength)}</span>
        </div>
      </div>
      <Slider
        value={trackProgress}
        max={trackLength}
        step={0.01}
        class="mb-3 mt-2 w-[calc(100%)]"
        onValueChange={handleProgressChange}
      />
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <Button variant="ghost" size="icon">
            <SkipBack class="h-4 w-4" />
          </Button>
          <Button size="icon" on:click={togglePlayback}>
            {#if isPlaying}
              <Pause class="h-4 w-4" />
            {:else}
              <Play class="h-4 w-4" />
            {/if}
          </Button>
          <Button variant="ghost" size="icon">
            <SkipForward class="h-4 w-4" />
          </Button>
        </div>
        <div class="flex items-center gap-2">
          <Volume2 class="h-4 w-4" />
          <Slider
            value={userVolume}
            max={100}
            step={1}
            class="w-[100px]"
            onValueChange={handleVolumeChange}
          />
        </div>
      </div>
    </div>
  </div>
</div>
