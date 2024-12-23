<script>
  import { Play, SkipBack, SkipForward, Volume2 } from "lucide-svelte";
  import { Button } from "./ui/button";
  import { Slider } from "./ui/slider";
</script>

<div class="w-full border-t bg-background p-4 z-10">
  <div class="flex items-center gap-4">
    <img src="#" alt="Album art" class="h-12 w-12 rounded-md" />
    <div class="flex-1">
      <div class="mb-2 flex items-center justify-between">
        <div>
          <div class="font-medium leading-none">Come Alive</div>
          <div class="text-sm text-muted-foreground">Netsky</div>
        </div>
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <span>2:14</span>
          <span>/</span>
          <span>3:11</span>
        </div>
      </div>
      <Slider max={100} step={1} class="my-2" />
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <Button variant="ghost" size="icon">
            <SkipBack class="h-4 w-4" />
          </Button>
          <Button size="icon">
            <Play class="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon">
            <SkipForward class="h-4 w-4" />
          </Button>
        </div>
        <div class="flex items-center gap-2">
          <Volume2 class="h-4 w-4" />
          <Slider max={100} step={1} class="w-[100px]" />
        </div>
      </div>
    </div>
  </div>
</div>
