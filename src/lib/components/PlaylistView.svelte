<script lang="ts">
  interface Track {
    id: number;
    title: string;
    artist: string;
    duration: string;
    current?: boolean;
  }

  let playlist: Track[] = [];

  playlist = [
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
    { id: 6, title: "Midnight City", artist: "M83", duration: "4:03" },
    { id: 7, title: "Strobe", artist: "Deadmau5", duration: "10:37" },
    {
      id: 8,
      title: "Get Lucky",
      artist: "Daft Punk feat. Pharrell Williams",
      duration: "4:08",
    },
    { id: 9, title: "Runaway", artist: "Aurora", duration: "4:09" },
    { id: 10, title: "Take on Me", artist: "A-ha", duration: "3:48" },
    {
      id: 11,
      title: "Counting Stars",
      artist: "OneRepublic",
      duration: "4:17",
    },
    { id: 12, title: "Sweet Dreams", artist: "Eurythmics", duration: "3:36" },
    {
      id: 13,
      title: "Blinding Lights",
      artist: "The Weeknd",
      duration: "3:20",
    },
    {
      id: 14,
      title: "Starboy",
      artist: "The Weeknd feat. Daft Punk",
      duration: "3:50",
    },
    {
      id: 15,
      title: "Around the World",
      artist: "Daft Punk",
      duration: "7:09",
    },
  ];

  function setCurrentTrack(id: number) {
    playlist = playlist.map((track) => ({
      ...track,
      current: track.id === id,
    }));
  }
</script>

<div class="px-3 space-y-1 my-2 overscroll-auto">
  {#each playlist as track (track.id)}
    <div
      class={`grid grid-cols-[40px_1fr_auto] items-center rounded-lg px-4 py-2 hover:bg-accent cursor-pointer ${
        track.current ? "bg-accent shadow-inner shadow-border" : ""
      }`}
      role="button"
      tabindex="0"
      on:click={() => setCurrentTrack(track.id)}
      on:keydown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          setCurrentTrack(track.id);
        }
      }}
    >
      <span class="text-sm text-muted-foreground font-mono w-[40px]"
        >{track.id}</span
      >
      <div>
        <div class="font-medium leading-none">{track.title}</div>
        <div class="text-sm text-muted-foreground">{track.artist}</div>
      </div>
      <span class="text-sm text-muted-foreground font-mono"
        >{track.duration}</span
      >
    </div>
  {/each}
</div>
