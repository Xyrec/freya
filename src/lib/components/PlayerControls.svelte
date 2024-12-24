<script lang="ts">
  import { Pause, Play, SkipBack, SkipForward, Volume2 } from "lucide-svelte";
  import { Button } from "./ui/button";
  import { Slider } from "./ui/slider";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { info } from "@tauri-apps/plugin-log";

  let userVolume = $state<[number]>([70]);
  let trackProgress = $state<[number]>([0]);
  let trackLength = $state<number>(191);
  let isPlaying = $state<boolean>(false);

  async function togglePlayback() {
    if (isPlaying) {
      await invoke("pause_sound");
      isPlaying = false;
    } else {
      await invoke("play_sound");
      isPlaying = true;
    }
  }

  async function handleVolumeChange(volume: number[]) {
    userVolume = [volume[0]];
    await invoke("set_volume", { volume: volume[0] });
  }

  onMount(() => {
    const unlistenPromise = listen("sound_done", () => {
      isPlaying = false;
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
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
          <span>2:14</span>
          <span>/</span>
          <span>3:11</span>
        </div>
      </div>
      <Slider
        value={trackProgress}
        max={trackLength}
        step={1}
        class="mb-3 mt-2 w-[calc(100%)]"
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
