<script lang="ts">
  import MusicHeader from "$lib/components/MusicHeader.svelte";
  import PlayerControls from "$lib/components/PlayerControls.svelte";
  import PlaylistView from "$lib/components/PlaylistView.svelte";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import {
    Tabs,
    TabsContent,
    TabsList,
    TabsTrigger,
  } from "$lib/components/ui/tabs";

  let activeTab = $state("playlist");
</script>

<div class="flex flex-col h-screen bg-background text-foreground">
  <MusicHeader />
  <Tabs bind:value={activeTab} class="flex-1 flex flex-col overflow-hidden">
    <TabsList
      class="w-full justify-start rounded-none border-b bg-background p-0 z-50 shadow"
    >
      <TabsTrigger
        value="playlist"
        class="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary"
      >
        Playlist
      </TabsTrigger>
      <TabsTrigger
        value="lyrics"
        class="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary"
      >
        Lyrics
      </TabsTrigger>
      <TabsTrigger
        value="bio"
        class="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary"
      >
        Bio
      </TabsTrigger>
    </TabsList>
    <TabsContent value="playlist" class="flex-1 p-0 overflow-hidden">
      <ScrollArea class="h-full">
        <PlaylistView />
      </ScrollArea>
    </TabsContent>
  </Tabs>
  <PlayerControls />
</div>
