<script lang="ts">
  export const playlist = [
    { id: 1, title: "Love Has Gone", artist: "Netsky", duration: "4:11" },
    {
      id: 2,
      title: "The Whistle Song",
      artist: "Netsky feat. Dynamic",
      duration: "4:39",
    },
    {
      id: 3,
      title: "Wanna Die For You",
      artist: "Netsky feat. Diane Char",
      duration: "4:17",
    },
    {
      id: 4,
      title: "Come Alive",
      artist: "Netsky",
      duration: "3:11",
      current: true,
    },
    { id: 5, title: "Give & Take", artist: "Netsky", duration: "4:08" },
  ];
</script>

<div class="space-y-1 p-4">
  {#each playlist as track (track.id)}
    <div
      class={`flex items-center justify-between rounded-lg px-4 py-2 hover:bg-accent ${
        track.current ? "bg-accent" : ""
      }`}
    >
      <div class="flex items-center gap-4">
        <span class="text-sm text-muted-foreground">{track.id}</span>
        <div>
          <div class="font-medium leading-none">{track.title}</div>
          <div class="text-sm text-muted-foreground">{track.artist}</div>
        </div>
      </div>
      <span class="text-sm text-muted-foreground">{track.duration}</span>
    </div>
  {/each}
</div>
