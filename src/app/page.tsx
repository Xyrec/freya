"use client";

import { useEffect } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { info } from "@tauri-apps/plugin-log";
import { PlayerControls } from "@/components/player-controls";
import { PlaylistView } from "@/components/playlist-view";
import { ThemeToggleButton } from "@/components/theme-provider";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

export default function MusicPlayer() {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    (async () => {
      unlisten = await getCurrentWebview().onDragDropEvent((event: any) => {
        if (event.payload.type === "drop") {
          info(`User dropped: ${JSON.stringify(event.payload.paths)}`);
        }
      });
    })();
    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  return (
    <div className="flex flex-col h-screen bg-background text-foreground">
      <Tabs
        defaultValue="playlist"
        className="flex-1 flex flex-col overflow-hidden"
      >
        <TabsList className="w-full justify-start rounded-none border-b bg-background p-0 z-50 shadow">
          <TabsTrigger
            value="playlist"
            className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary hover:cursor-pointer"
          >
            Playlist
          </TabsTrigger>
          <TabsTrigger
            value="lyrics"
            className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary hover:cursor-pointer"
          >
            Lyrics
          </TabsTrigger>
          <TabsTrigger
            value="bio"
            className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary hover:cursor-pointer"
          >
            Bio
          </TabsTrigger>
          <div className="flex-1" />
          <ThemeToggleButton />
        </TabsList>
        <TabsContent value="playlist" className="flex-1 p-0 overflow-hidden">
          <ScrollArea className="h-full">
            <PlaylistView />
          </ScrollArea>
        </TabsContent>
      </Tabs>
      <PlayerControls />
    </div>
  );
}
